<script lang="ts">
  import { tick, untrack } from 'svelte';
  import { gsap, Flip, getMotionDuration, killTweens } from '$lib/design/gsap';

  interface Props {
    isMacOS?: boolean;
    collapsed?: boolean;
    onRotateEnd?: () => void;
    onMoveEnd?: () => void;
    expandReady?: boolean;
  }

  let {
    isMacOS = false,
    collapsed = false,
    onRotateEnd,
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

  const STAGGER = 0.03;
  const ROTATE_DUR = 280;
  const MOVE_DUR = 320;

  const charEls: (HTMLSpanElement | null)[] = $state(Array(12).fill(null));
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

    // ── Phase 1: Stagger Rotate ──
    const tl = gsap.timeline();
    currentTimeline = tl;

    await new Promise<void>((resolve) => {
      tl.to(validEls, {
        rotation: targetAngle,
        duration: rotateDur,
        stagger: STAGGER,
        ease: 'power3.out',
        onComplete: resolve,
      });
    });

    onRotateEnd?.();

    if (!toCollapsed) {
      await new Promise<void>((resolve) => {
        expandResolve = resolve;
      });
    }

    // ── Phase 2: FLIP Move ──
    const state = Flip.getState(validEls);

    layoutCollapsed = toCollapsed;
    await tick();

    const flipTl = Flip.from(state, {
      duration: moveDur,
      stagger: STAGGER,
      ease: 'power2.out',
      absolute: true,
      onComplete: () => {
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
