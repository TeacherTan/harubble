<script lang="ts">
  import { tick, untrack } from 'svelte';
  import { gsap, Flip, getMotionDuration, killTweens } from '$lib/design/gsap';

  interface Props {
    isMacOS?: boolean;
    collapsed?: boolean;
    onMoveEnd?: () => void;
    expandReady?: boolean;
  }

  let {
    isMacOS = false,
    collapsed = false,
    onMoveEnd,
    expandReady = false,
  }: Props = $props();

  const BRAND_LETTERS = [
    { char: 'H', outline: false },
    { char: 'A', outline: false },
    { char: 'R', outline: false },
    { char: 'U', outline: false },
    { char: 'K', outline: true },
    { char: 'A', outline: true },
    { char: 'B', outline: true },
    { char: 'U', outline: true },
    { char: 'B', outline: false },
    { char: 'B', outline: false },
    { char: 'L', outline: false },
    { char: 'E', outline: false },
  ] as const;

  const ROW1 = BRAND_LETTERS.slice(0, 6);
  const ROW2 = BRAND_LETTERS.slice(6);

  const STAGGER = 0.02;
  const ROTATE_DUR = 200;
  const MOVE_DUR = 240;

  const charEls: (HTMLSpanElement | null)[] = $state(Array(12).fill(null));
  let containerEl: HTMLDivElement | null = $state(null);
  let layoutCollapsed = $state(collapsed);
  let animating = $state(false);
  let prevCollapsed: boolean | null = $state(null);

  let currentTimeline: gsap.core.Timeline | null = null;
  let expandResolve: (() => void) | null = null;

  $effect(() => {
    if (expandReady && expandResolve) {
      expandResolve();
      expandResolve = null;
    }
  });

  $effect(() => {
    const curr = collapsed;
    const prev = untrack(() => prevCollapsed);

    if (prev === null) {
      prevCollapsed = curr;
      layoutCollapsed = curr;
      return;
    }
    if (prev === curr) return;
    prevCollapsed = curr;

    void runAnimation(curr);
  });

  $effect(() => {
    return () => {
      currentTimeline?.kill();
      killTweens(charEls.filter(Boolean));
    };
  });

  async function runAnimation(toCollapsed: boolean) {
    if (currentTimeline) {
      currentTimeline.kill();
      currentTimeline = null;
    }
    if (expandResolve) {
      expandResolve();
      expandResolve = null;
    }

    const validEls = charEls.filter((el): el is HTMLSpanElement => el !== null);
    if (validEls.length === 0) return;

    animating = true;
    const rotateDur = getMotionDuration(ROTATE_DUR);
    const moveDur = getMotionDuration(MOVE_DUR);
    const targetAngle = toCollapsed ? -90 : 0;

    // 展开方向：字母立即原地旋转，同时锁住容器尺寸防止侧栏展开导致字母位移
    if (!toCollapsed) {
      if (containerEl) {
        containerEl.style.height = `${containerEl.offsetHeight}px`;
        containerEl.style.width = `${containerEl.offsetWidth}px`;
        containerEl.style.overflow = 'hidden';
      }

      // ── Phase 1: Stagger Rotate（与侧栏宽度展开同步） ──
      const tl = gsap.timeline();
      currentTimeline = tl;

      const rotatePromise = new Promise<void>((resolve) => {
        tl.to(validEls, {
          rotation: targetAngle,
          duration: rotateDur,
          stagger: STAGGER,
          ease: 'ios-spring',
          onComplete: resolve,
        });
      });

      // 等侧栏宽度展开完毕 + 旋转完毕，两者都完成后再进入 FLIP
      const expandPromise = new Promise<void>((resolve) => {
        expandResolve = resolve;
      });
      await Promise.all([rotatePromise, expandPromise]);
    } else {
      // ── 收起方向：正常旋转 ──
      const tl = gsap.timeline();
      currentTimeline = tl;

      await new Promise<void>((resolve) => {
        tl.to(validEls, {
          rotation: targetAngle,
          duration: rotateDur,
          stagger: STAGGER,
          ease: 'ios-spring',
          onComplete: resolve,
        });
      });
    }

    // ── Phase 2: FLIP Move ──
    // 在释放宽度前捕获字母位置，防止容器变宽后 align-items:center 导致跳变
    const state = Flip.getState(validEls);

    if (containerEl && toCollapsed) {
      containerEl.style.height = `${containerEl.offsetHeight}px`;
    }

    // 释放宽度锁定（展开方向，侧栏已展开完毕）
    if (containerEl && !toCollapsed) {
      containerEl.style.width = '';
    }

    // 切换布局前隐藏字母，防止布局切换到 FLIP 接管之间的一帧闪烁
    validEls.forEach((el) => (el.style.visibility = 'hidden'));

    layoutCollapsed = toCollapsed;
    await tick();

    // FLIP 接管后立即恢复可见
    validEls.forEach((el) => (el.style.visibility = ''));

    if (containerEl && !toCollapsed) {
      // 展开方向：用克隆节点测量目标高度（FLIP absolute 会影响实际容器内容高度）
      const clone = containerEl.cloneNode(true) as HTMLDivElement;
      clone.style.position = 'absolute';
      clone.style.visibility = 'hidden';
      clone.style.height = 'auto';
      clone.style.width = '';
      clone.style.overflow = '';
      clone.style.pointerEvents = 'none';
      containerEl.parentElement!.appendChild(clone);
      const naturalHeight = clone.offsetHeight;
      clone.remove();

      gsap.to(containerEl, {
        height: naturalHeight,
        duration: moveDur,
        delay: 0.25,
        ease: 'ios-spring',
        onComplete: () => {
          containerEl!.style.height = '';
          containerEl!.style.overflow = '';
        },
      });
    }

    const flipTl = Flip.from(state, {
      duration: moveDur,
      stagger: STAGGER,
      ease: 'ios-spring',
      absolute: true,
      onComplete: () => {
        if (containerEl) {
          containerEl.style.height = '';
          containerEl.style.overflow = '';
        }
        onMoveEnd?.();
        animating = false;
        currentTimeline = null;
      },
    });
    currentTimeline = flipTl;
  }
</script>

<div
  class="brand-logo"
  class:macos={isMacOS}
  class:collapsed={layoutCollapsed}
  aria-hidden="true"
  bind:this={containerEl}
>
  <span class="brand-row">
    {#each ROW1 as letter, i (i)}
      <span
        class="brand-char"
        class:outline={letter.outline}
        class:rotated={layoutCollapsed && !animating}
        bind:this={charEls[i]}>{letter.char}</span
      >
    {/each}
  </span>
  <span class="brand-row">
    {#each ROW2 as letter, i (i)}
      <span
        class="brand-char"
        class:outline={letter.outline}
        class:rotated={layoutCollapsed && !animating}
        bind:this={charEls[i + 6]}>{letter.char}</span
      >
    {/each}
  </span>
</div>

<style>
  .brand-logo {
    display: flex;
    flex-direction: column;
    padding: 20px 24px 12px;
    line-height: 1;
    user-select: none;
    -webkit-user-select: none;
  }

  .brand-logo.macos {
    padding-top: 48px;
  }

  .brand-logo.collapsed {
    align-items: center;
    padding: 20px 0 12px;
    flex-direction: column-reverse;
  }

  .brand-logo.collapsed.macos {
    padding-top: 48px;
  }

  .brand-row {
    display: flex;
    flex-direction: row;
    font-family: var(--font-wide);
    font-size: 22px;
    font-weight: 700;
    letter-spacing: 0.04em;
    color: var(--accent);
    white-space: nowrap;
  }

  .collapsed .brand-row {
    flex-direction: column-reverse;
    align-items: center;
    line-height: 0.88;
  }

  .brand-char {
    display: inline-block;
  }

  .brand-char.rotated {
    transform: rotate(-90deg);
  }

  .brand-char.outline {
    color: transparent;
    -webkit-text-stroke: 1.5px var(--accent);
  }

  @media (prefers-reduced-motion: reduce) {
    .brand-char {
      transition: none !important;
      animation: none !important;
    }
  }
</style>
