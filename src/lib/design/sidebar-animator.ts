import { tick } from 'svelte';
import { gsap, Flip, reducedMotionQuery } from '$lib/design/gsap';

export interface SidebarAnimatorConfig {
  shellEl: HTMLElement;
  sidebarEl: HTMLElement;
  logoCharEls: HTMLSpanElement[];
  logoContainerEl: HTMLDivElement;
  navRegionEl: HTMLElement;
  collectionsRegionEl: HTMLElement;
  collectionsCollapsedEl: HTMLElement;
  bottomLabelEl: HTMLSpanElement;
  initialCollapsed: boolean;
  onContentInteractive: (interactive: boolean) => void;
  onContentSwitch: (collapsed: boolean) => void;
  onLayoutSwitch: (collapsed: boolean) => void;
  onComplete?: (collapsed: boolean) => void;
}

export interface SidebarAnimator {
  collapse(): void;
  expand(): void;
  interrupt(): void;
  dispose(): void;
}

const TIMING = {
  WIDTH_DUR: 300,
  ROTATE_DUR: 200,
  MOVE_DUR: 240,
  STAGGER: 0.02,
  CONTENT_FADE: 200,
  LABEL_DUR: 300,
  PHASE_GAP: 50,
  FLIP_DELAY: 0,
} as const;

const EXPANDED_WIDTH = '248px';
const COLLAPSED_WIDTH = '56px';
const EXPANDED_WIDTH_VALUE = Number.parseFloat(EXPANDED_WIDTH);
const COLLAPSED_WIDTH_VALUE = Number.parseFloat(COLLAPSED_WIDTH);

export function getLogoCenterPinX(
  collapsedWidth: number,
  currentWidth: number
): number {
  const offset = -(currentWidth - collapsedWidth) / 2;
  return Object.is(offset, -0) ? 0 : offset;
}

export function getLogoCenterPinFrame(
  collapsedWidth: number,
  expandedWidth: number,
  progress: number
): { width: number; x: number } {
  const clampedProgress = Math.min(Math.max(progress, 0), 1);
  const width =
    collapsedWidth + (expandedWidth - collapsedWidth) * clampedProgress;

  return {
    width,
    x: getLogoCenterPinX(collapsedWidth, width),
  };
}

export function getPinnedLogoWidthFrame(
  collapsedWidth: number,
  expandedWidth: number,
  progress: number,
  measuredLogoWidth = collapsedWidth
): { sidebarWidth: number; logoWidth: number; alignSelf: 'flex-start' } {
  const clampedProgress = Math.min(Math.max(progress, 0), 1);
  const sidebarWidth =
    collapsedWidth + (expandedWidth - collapsedWidth) * clampedProgress;

  return {
    sidebarWidth,
    logoWidth: measuredLogoWidth,
    alignSelf: 'flex-start',
  };
}

type RectLike = Pick<DOMRect, 'left' | 'top' | 'width' | 'height'>;

export function getCenterLockTransform(
  initialRect: RectLike,
  currentRect: RectLike,
  currentTransform: { x: number; y: number }
): { x: number; y: number } {
  const initialCenterX = initialRect.left + initialRect.width / 2;
  const initialCenterY = initialRect.top + initialRect.height / 2;
  const currentCenterX = currentRect.left + currentRect.width / 2;
  const currentCenterY = currentRect.top + currentRect.height / 2;

  return {
    x: currentTransform.x + initialCenterX - currentCenterX,
    y: currentTransform.y + initialCenterY - currentCenterY,
  };
}

export function resolveLogoGlyphEl(charEl: HTMLSpanElement): HTMLElement {
  return charEl.querySelector<HTMLElement>('[data-logo-glyph]') ?? charEl;
}

export function getLogoFlipTargets(
  charEls: HTMLSpanElement[],
  _glyphEls: HTMLElement[]
): HTMLElement[] {
  return charEls;
}

export function getLogoSlotClearProps(): string {
  return 'all';
}

export function getLogoGlyphClearProps(
  target: 'collapsed' | 'expanded'
): string {
  const props = [
    'gridArea',
    'height',
    'left',
    'maxHeight',
    'maxWidth',
    'minHeight',
    'minWidth',
    'padding',
    'position',
    'top',
    'transition',
    'width',
  ];

  if (target === 'expanded') {
    props.push('transform', 'translate', 'rotate', 'scale');
  }

  return props.join(',');
}

export function collectSidebarItemLabelEls(
  regions: HTMLElement[],
  extraLabelEls: HTMLSpanElement[] = []
): HTMLSpanElement[] {
  const labels = [
    ...regions.flatMap((region) =>
      Array.from(
        region.querySelectorAll<HTMLSpanElement>('[data-sidebar-item-label]')
      )
    ),
    ...extraLabelEls,
  ];

  return Array.from(new Set(labels));
}

export function collectSidebarAnimatorLabelEls(
  config: Pick<
    SidebarAnimatorConfig,
    'navRegionEl' | 'collectionsRegionEl' | 'bottomLabelEl'
  >
): HTMLSpanElement[] {
  return collectSidebarItemLabelEls(
    [config.navRegionEl, config.collectionsRegionEl],
    [config.bottomLabelEl]
  );
}

function getAnimationParams() {
  const reduced = reducedMotionQuery?.matches ?? false;
  return {
    widthDur: reduced ? 0 : TIMING.WIDTH_DUR / 1000,
    rotateDur: reduced ? 0 : TIMING.ROTATE_DUR / 1000,
    moveDur: reduced ? 0 : TIMING.MOVE_DUR / 1000,
    stagger: reduced ? 0 : TIMING.STAGGER,
    contentFade: reduced ? 0 : TIMING.CONTENT_FADE / 1000,
    labelDur: reduced ? 0 : TIMING.LABEL_DUR / 1000,
    phaseGap: reduced ? 0 : TIMING.PHASE_GAP / 1000,
    flipDelay: reduced ? 0 : TIMING.FLIP_DELAY / 1000,
    reduced,
  };
}

function syncToState(config: SidebarAnimatorConfig, collapsed: boolean) {
  config.shellEl.style.setProperty(
    '--sidebar-width',
    collapsed ? COLLAPSED_WIDTH : EXPANDED_WIDTH
  );

  config.logoCharEls.forEach((el) => {
    const glyphEl = resolveLogoGlyphEl(el);
    if (collapsed) {
      gsap.set(glyphEl, { rotation: -90 });
    } else {
      gsap.set(glyphEl, { clearProps: 'transform' });
    }
  });

  config.onLayoutSwitch(collapsed);
  config.onContentSwitch(collapsed);
  config.onContentInteractive(!collapsed);
}

function cleanupTransientStyles(
  config: SidebarAnimatorConfig,
  target: 'collapsed' | 'expanded'
) {
  const labelEls = collectSidebarAnimatorLabelEls(config);

  gsap.set(config.logoContainerEl, {
    clearProps: 'height,overflow,width,visibility,transform,alignSelf',
  });
  gsap.set(labelEls, { clearProps: 'maxWidth,opacity' });
  gsap.set(config.navRegionEl, { clearProps: 'opacity' });
  gsap.set(config.collectionsRegionEl, { clearProps: 'opacity' });
  gsap.set(config.collectionsCollapsedEl, { clearProps: 'opacity' });

  config.logoCharEls.forEach((el) => {
    gsap.set(el, { clearProps: `visibility,${getLogoSlotClearProps()}` });
  });

  config.logoCharEls.forEach((el) =>
    gsap.set(resolveLogoGlyphEl(el), {
      clearProps: getLogoGlyphClearProps(target),
    })
  );
}

interface TimelineLike {
  totalDuration(): number;
  progress(): number;
  eventCallback(name: 'onComplete'): (() => void) | undefined;
  eventCallback(name: 'onComplete', callback: () => void): unknown;
}

type AwaitableTimeline = Omit<gsap.core.Timeline, 'then'> | TimelineLike;

export function chainTimelineComplete(tl: AwaitableTimeline): Promise<void> {
  return new Promise<void>((resolve) => {
    if (tl.totalDuration() === 0 || tl.progress() >= 1) {
      resolve();
      return;
    }
    const existingOnComplete = tl.eventCallback('onComplete');
    tl.eventCallback('onComplete', () => {
      existingOnComplete?.();
      resolve();
    });
  });
}

export function createSidebarAnimator(
  config: SidebarAnimatorConfig
): SidebarAnimator {
  let currentAnimationId = 0;
  let currentTimeline: gsap.core.Timeline | null = null;
  let heightTween: gsap.core.Tween | null = null;
  let lastCommittedCollapsed = config.initialCollapsed;

  const logoGlyphEls = config.logoCharEls.map(resolveLogoGlyphEl);

  syncToState(config, config.initialCollapsed);

  function startNewAnimation(): number {
    currentAnimationId++;
    currentTimeline?.kill();
    currentTimeline = null;
    heightTween?.kill();
    heightTween = null;
    normalizeToCommittedState();
    return currentAnimationId;
  }

  function normalizeToCommittedState() {
    syncToState(config, lastCommittedCollapsed);
    cleanupTransientStyles(
      config,
      lastCommittedCollapsed ? 'collapsed' : 'expanded'
    );
  }

  function isStale(id: number): boolean {
    return id !== currentAnimationId;
  }

  function applyExpandedWidthFrame(
    currentWidth: number,
    measuredLogoWidth = COLLAPSED_WIDTH_VALUE
  ) {
    const { sidebarWidth, logoWidth, alignSelf } = getPinnedLogoWidthFrame(
      COLLAPSED_WIDTH_VALUE,
      EXPANDED_WIDTH_VALUE,
      (currentWidth - COLLAPSED_WIDTH_VALUE) /
        (EXPANDED_WIDTH_VALUE - COLLAPSED_WIDTH_VALUE),
      measuredLogoWidth
    );

    config.shellEl.style.setProperty('--sidebar-width', `${sidebarWidth}px`);
    gsap.set(config.logoContainerEl, {
      width: logoWidth,
      alignSelf,
    });
  }

  async function flipPhase(
    id: number,
    toCollapsed: boolean,
    params: ReturnType<typeof getAnimationParams>
  ): Promise<gsap.core.Timeline | null> {
    const els = getLogoFlipTargets(config.logoCharEls, logoGlyphEls);

    config.logoContainerEl.style.height = `${config.logoContainerEl.offsetHeight}px`;
    if (!toCollapsed) {
      config.logoContainerEl.style.width = `${config.logoContainerEl.offsetWidth}px`;
      config.logoContainerEl.style.overflow = 'hidden';
    }

    const state = Flip.getState(els);

    els.forEach((el) => (el.style.visibility = 'hidden'));

    if (!toCollapsed) {
      gsap.set(config.logoContainerEl, { clearProps: 'x' });
      gsap.set(logoGlyphEls, { clearProps: 'transform' });
    }

    config.onLayoutSwitch(toCollapsed);
    await tick();
    if (isStale(id)) return null;

    els.forEach((el) => (el.style.visibility = ''));

    if (!toCollapsed) {
      config.logoContainerEl.style.width = '';
    }

    const clone = config.logoContainerEl.cloneNode(true) as HTMLDivElement;
    clone.style.position = 'absolute';
    clone.style.visibility = 'hidden';
    clone.style.height = 'auto';
    clone.style.width = '';
    clone.style.overflow = '';
    clone.style.pointerEvents = 'none';
    config.logoContainerEl.parentElement!.appendChild(clone);
    const targetHeight = clone.offsetHeight;
    clone.remove();

    heightTween = gsap.to(config.logoContainerEl, {
      height: targetHeight,
      duration: params.moveDur,
      ease: 'ios-spring',
      onComplete: () => {
        heightTween = null;
      },
    });

    const flipTl = Flip.from(state, {
      duration: params.moveDur,
      stagger: params.stagger,
      ease: 'ios-spring',
      absolute: true,
    });
    currentTimeline = flipTl as gsap.core.Timeline;

    return flipTl as gsap.core.Timeline;
  }

  async function runCollapse(id: number) {
    const params = getAnimationParams();
    const labelEls = collectSidebarAnimatorLabelEls(config);

    config.onContentInteractive(false);

    const phase1 = gsap.timeline();
    currentTimeline = phase1;

    // collapse 宽度收缩与字母旋转同步，故共用 rotateDur 而非 widthDur
    phase1.to(
      config.shellEl,
      {
        '--sidebar-width': COLLAPSED_WIDTH,
        duration: params.rotateDur,
        ease: 'ios-spring',
      },
      0
    );

    phase1.to(
      logoGlyphEls,
      {
        rotation: -90,
        duration: params.rotateDur,
        stagger: params.stagger,
        ease: 'ios-spring',
      },
      0
    );

    phase1.to(
      [config.collectionsRegionEl, config.navRegionEl],
      {
        opacity: 0,
        duration: params.rotateDur,
        ease: 'ios-in',
      },
      0
    );

    phase1.to(
      labelEls,
      {
        maxWidth: 0,
        opacity: 0,
        duration: params.rotateDur,
        ease: 'ios-in',
      },
      0
    );

    await chainTimelineComplete(phase1);
    if (isStale(id)) return;

    const flipResult = await flipPhase(id, true, params);
    if (!flipResult || isStale(id)) return;

    await chainTimelineComplete(flipResult);
    if (isStale(id)) return;

    lastCommittedCollapsed = true;
    config.onContentSwitch(true);
    cleanupTransientStyles(config, 'collapsed');
    config.onComplete?.(true);
    currentTimeline = null;
  }

  async function runExpand(id: number) {
    const params = getAnimationParams();
    const collapsedLogoWidth =
      config.logoContainerEl.getBoundingClientRect().width ||
      COLLAPSED_WIDTH_VALUE;

    applyExpandedWidthFrame(COLLAPSED_WIDTH_VALUE, collapsedLogoWidth);
    gsap.set(config.logoContainerEl, { overflow: 'hidden' });

    config.onContentSwitch(false);
    await tick();
    if (isStale(id)) return;

    gsap.set(config.collectionsRegionEl, { clearProps: 'opacity' });
    gsap.set(config.navRegionEl, { clearProps: 'opacity' });
    const labelEls = collectSidebarAnimatorLabelEls(config);
    gsap.set(labelEls, { maxWidth: 0, opacity: 0 });

    const measuredLabelWidths = labelEls.map((el) => el.scrollWidth);

    const lockedGlyphRects = logoGlyphEls.map((el) =>
      el.getBoundingClientRect()
    );
    const lockLogoGlyphCenters = () => {
      logoGlyphEls.forEach((glyphEl, index) => {
        const currentTransform = {
          x: Number(gsap.getProperty(glyphEl, 'x')) || 0,
          y: Number(gsap.getProperty(glyphEl, 'y')) || 0,
        };
        const nextTransform = getCenterLockTransform(
          lockedGlyphRects[index],
          glyphEl.getBoundingClientRect(),
          currentTransform
        );
        gsap.set(glyphEl, nextTransform);
      });
    };

    const widthFrame = { width: COLLAPSED_WIDTH_VALUE };
    const phase1 = gsap.timeline();
    currentTimeline = phase1;

    phase1.to(
      widthFrame,
      {
        width: EXPANDED_WIDTH_VALUE,
        duration: params.widthDur,
        ease: 'ios-spring',
        onUpdate: () => {
          applyExpandedWidthFrame(widthFrame.width, collapsedLogoWidth);
          lockLogoGlyphCenters();
        },
        onComplete: () => {
          applyExpandedWidthFrame(EXPANDED_WIDTH_VALUE, collapsedLogoWidth);
        },
      },
      0
    );

    phase1.to(
      logoGlyphEls,
      {
        rotation: 0,
        duration: params.widthDur,
        stagger: params.stagger,
        ease: 'ios-spring',
        onUpdate: lockLogoGlyphCenters,
      },
      0
    );

    await chainTimelineComplete(phase1);
    if (isStale(id)) return;

    applyExpandedWidthFrame(EXPANDED_WIDTH_VALUE, collapsedLogoWidth);

    const labelTl = gsap.timeline();
    currentTimeline = labelTl;

    labelTl.to(labelEls, {
      maxWidth: (index) => measuredLabelWidths[index] || 120,
      opacity: 1,
      duration: params.labelDur,
      ease: 'ios-out',
    });

    await chainTimelineComplete(labelTl);
    if (isStale(id)) return;

    const flipResult = await flipPhase(id, false, params);
    if (!flipResult || isStale(id)) return;

    await chainTimelineComplete(flipResult);
    if (isStale(id)) return;

    config.onContentInteractive(true);

    lastCommittedCollapsed = false;
    cleanupTransientStyles(config, 'expanded');
    config.onComplete?.(false);
    currentTimeline = null;
  }

  function collapse() {
    const id = startNewAnimation();
    if (lastCommittedCollapsed) return;
    runCollapse(id).catch(() => {
      if (!isStale(id)) {
        lastCommittedCollapsed = true;
        syncToState(config, true);
        cleanupTransientStyles(config, 'collapsed');
      }
    });
  }

  function expand() {
    const id = startNewAnimation();
    if (!lastCommittedCollapsed) return;
    runExpand(id).catch(() => {
      if (!isStale(id)) {
        lastCommittedCollapsed = false;
        syncToState(config, false);
        cleanupTransientStyles(config, 'expanded');
      }
    });
  }

  function interrupt() {
    currentAnimationId++;
    currentTimeline?.kill();
    currentTimeline = null;
    heightTween?.kill();
    heightTween = null;
    normalizeToCommittedState();
  }

  function dispose() {
    currentAnimationId++;
    currentTimeline?.kill();
    currentTimeline = null;
    heightTween?.kill();
    heightTween = null;
    const allEls = [
      config.shellEl,
      config.sidebarEl,
      config.logoContainerEl,
      config.navRegionEl,
      config.collectionsRegionEl,
      config.collectionsCollapsedEl,
      config.bottomLabelEl,
      ...collectSidebarAnimatorLabelEls(config),
      ...config.logoCharEls,
      ...logoGlyphEls,
    ];
    allEls.forEach((el) => gsap.set(el, { clearProps: 'all' }));
  }

  return { collapse, expand, interrupt, dispose };
}
