import { tick } from 'svelte';
import { gsap, Flip } from '$lib/design/gsap';

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
  LABEL_DUR: 220,
  PHASE_GAP: 50,
  FLIP_DELAY: 0,
} as const;

const EXPANDED_WIDTH = '248px';
const COLLAPSED_WIDTH = '56px';

function getAnimationParams() {
  const reduced =
    typeof window !== 'undefined'
      ? window.matchMedia('(prefers-reduced-motion: reduce)').matches
      : false;
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
    if (collapsed) {
      gsap.set(el, { rotation: -90 });
    } else {
      gsap.set(el, { clearProps: 'transform' });
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
  gsap.set(config.logoContainerEl, {
    clearProps: 'height,overflow,width,visibility',
  });
  gsap.set(config.bottomLabelEl, { clearProps: 'maxWidth,opacity' });
  gsap.set(config.navRegionEl, { clearProps: 'opacity' });
  gsap.set(config.collectionsRegionEl, { clearProps: 'opacity' });
  gsap.set(config.collectionsCollapsedEl, { clearProps: 'opacity' });

  config.logoCharEls.forEach((el) => {
    gsap.set(el, { clearProps: 'visibility' });
  });

  if (target === 'expanded') {
    config.logoCharEls.forEach((el) =>
      gsap.set(el, { clearProps: 'transform' })
    );
  }
}

export function createSidebarAnimator(
  config: SidebarAnimatorConfig
): SidebarAnimator {
  let currentAnimationId = 0;
  let currentTimeline: gsap.core.Timeline | null = null;
  let lastCommittedCollapsed = config.initialCollapsed;

  syncToState(config, config.initialCollapsed);

  function startNewAnimation(): number {
    currentAnimationId++;
    currentTimeline?.kill();
    currentTimeline = null;
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

  async function flipPhase(
    id: number,
    toCollapsed: boolean,
    params: ReturnType<typeof getAnimationParams>
  ): Promise<gsap.core.Timeline | null> {
    const els = config.logoCharEls;

    if (toCollapsed) {
      config.logoContainerEl.style.height = `${config.logoContainerEl.offsetHeight}px`;
    } else {
      config.logoContainerEl.style.height = `${config.logoContainerEl.offsetHeight}px`;
      config.logoContainerEl.style.width = `${config.logoContainerEl.offsetWidth}px`;
      config.logoContainerEl.style.overflow = 'hidden';
    }

    const state = Flip.getState(els);

    els.forEach((el) => (el.style.visibility = 'hidden'));

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

    gsap.to(config.logoContainerEl, {
      height: targetHeight,
      duration: params.moveDur,
      ease: 'ios-spring',
      onComplete: () => {
        config.logoContainerEl.style.height = '';
        config.logoContainerEl.style.overflow = '';
      },
    });

    const flipTl = Flip.from(state, {
      duration: params.moveDur,
      stagger: params.stagger,
      ease: 'ios-spring',
      absolute: true,
    });
    currentTimeline = flipTl;

    return flipTl;
  }

  async function runCollapse(id: number) {
    const params = getAnimationParams();

    config.onContentInteractive(false);

    const phase1 = gsap.timeline();
    currentTimeline = phase1;

    phase1.to(config.logoCharEls, {
      rotation: -90,
      duration: params.rotateDur,
      stagger: params.stagger,
      ease: 'ios-spring',
    });
    phase1.to(
      [config.collectionsRegionEl, config.navRegionEl],
      {
        opacity: 0,
        duration: params.contentFade,
        ease: 'ios-in',
      },
      0
    );
    phase1.to(
      config.bottomLabelEl,
      {
        maxWidth: 0,
        opacity: 0,
        duration: params.labelDur,
        ease: 'ios-in',
      },
      0
    );

    await new Promise<void>((resolve) => {
      phase1.eventCallback('onComplete', resolve);
    });
    if (isStale(id)) return;

    const flipResult = await flipPhase(id, true, params);
    if (!flipResult || isStale(id)) return;

    await new Promise<void>((resolve) => {
      flipResult.eventCallback('onComplete', resolve);
    });
    if (isStale(id)) return;

    const phase3 = gsap.timeline();
    currentTimeline = phase3;

    phase3.to(config.shellEl, {
      '--sidebar-width': COLLAPSED_WIDTH,
      duration: params.widthDur,
      ease: 'ios-spring',
    });

    await new Promise<void>((resolve) => {
      phase3.eventCallback('onComplete', resolve);
    });
    if (isStale(id)) return;

    lastCommittedCollapsed = true;
    config.onContentSwitch(true);
    cleanupTransientStyles(config, 'collapsed');
    config.onComplete?.(true);
    currentTimeline = null;
  }

  async function runExpand(id: number) {
    const params = getAnimationParams();

    const phase1 = gsap.timeline();
    currentTimeline = phase1;

    phase1.to(config.shellEl, {
      '--sidebar-width': EXPANDED_WIDTH,
      duration: params.widthDur,
      ease: 'ios-spring',
    });

    await new Promise<void>((resolve) => {
      phase1.eventCallback('onComplete', resolve);
    });
    if (isStale(id)) return;

    const flipResult = await flipPhase(id, false, params);
    if (!flipResult || isStale(id)) return;

    const rotateTl = gsap.timeline();
    rotateTl.to(config.logoCharEls, {
      rotation: 0,
      duration: params.rotateDur,
      stagger: params.stagger,
      ease: 'ios-spring',
    });

    await new Promise<void>((resolve) => {
      flipResult.eventCallback('onComplete', resolve);
    });
    if (isStale(id)) return;

    config.onContentSwitch(false);
    await tick();
    if (isStale(id)) return;

    gsap.set(config.collectionsRegionEl, { opacity: 0 });
    gsap.set(config.navRegionEl, { opacity: 0 });
    gsap.set(config.bottomLabelEl, { maxWidth: 0, opacity: 0 });

    const phase3 = gsap.timeline();
    currentTimeline = phase3;

    const measuredWidth = config.bottomLabelEl.scrollWidth;

    phase3.to([config.collectionsRegionEl, config.navRegionEl], {
      opacity: 1,
      duration: params.contentFade,
      ease: 'ios-out',
    });
    phase3.to(
      config.bottomLabelEl,
      {
        maxWidth: measuredWidth || 120,
        opacity: 1,
        duration: params.labelDur,
        ease: 'ios-out',
      },
      0
    );

    await new Promise<void>((resolve) => {
      phase3.eventCallback('onComplete', resolve);
    });
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
    void runCollapse(id);
  }

  function expand() {
    const id = startNewAnimation();
    if (!lastCommittedCollapsed) return;
    void runExpand(id);
  }

  function interrupt() {
    currentAnimationId++;
    currentTimeline?.kill();
    currentTimeline = null;
    normalizeToCommittedState();
  }

  function dispose() {
    currentAnimationId++;
    currentTimeline?.kill();
    currentTimeline = null;
    const allEls = [
      config.shellEl,
      config.sidebarEl,
      config.logoContainerEl,
      config.navRegionEl,
      config.collectionsRegionEl,
      config.collectionsCollapsedEl,
      config.bottomLabelEl,
      ...config.logoCharEls,
    ];
    allEls.forEach((el) => gsap.set(el, { clearProps: 'all' }));
  }

  return { collapse, expand, interrupt, dispose };
}
