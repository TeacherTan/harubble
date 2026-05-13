<script lang="ts">
  import { tick, untrack } from 'svelte';

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

  const charEls: (HTMLSpanElement | null)[] = $state(Array(12).fill(null));

  const STAGGER = 30;
  const ROTATE_DUR = 280;
  const MOVE_DUR = 320;
  const TOTAL_ROTATE = 11 * STAGGER + ROTATE_DUR;
  const TOTAL_MOVE = 11 * STAGGER + MOVE_DUR;

  let layoutCollapsed = $state(collapsed);
  let charTransforms: string[] = $state(Array(12).fill(''));
  let animating = $state(false);
  let prevCollapsed: boolean | null = $state(null);

  /** 展开时：等侧边栏宽度展开后再开始移动阶段 */
  let expandResolve: (() => void) | null = null;

  // 当 expandReady 变为 true 时，解除等待
  $effect(() => {
    if (expandReady && expandResolve) {
      expandResolve();
      expandResolve = null;
    }
  });

  function getRect(el: HTMLSpanElement | null) {
    if (!el) return { x: 0, y: 0 };
    const r = el.getBoundingClientRect();
    return { x: r.left, y: r.top };
  }

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

  async function runAnimation(toCollapsed: boolean) {
    animating = true;
    const targetAngle = toCollapsed ? -90 : 0;

    // ── 阶段1: 原地旋转 ──
    charTransforms = charEls.map((_, i) => {
      const d = i * STAGGER;
      return `transform: rotate(${targetAngle}deg); transition: transform ${ROTATE_DUR}ms cubic-bezier(0.2, 0, 0, 1) ${d}ms;`;
    });

    await sleep(TOTAL_ROTATE + 50);
    onRotateEnd?.();

    if (!toCollapsed) {
      // 展开：旋转完毕后等父组件展开宽度
      await new Promise<void>((resolve) => {
        expandResolve = resolve;
      });
    }

    // ── 阶段2: FLIP 移动 ──
    const currentRects = charEls.map(getRect);

    // 瞬间切换布局以测量目标位置
    layoutCollapsed = toCollapsed;
    charTransforms = charEls.map(() => `transform: rotate(${targetAngle}deg);`);
    await tick();
    await new Promise((r) => requestAnimationFrame(r));

    const targetRects = charEls.map(getRect);

    // INVERT
    charTransforms = charEls.map((_, i) => {
      const dx = currentRects[i].x - targetRects[i].x;
      const dy = currentRects[i].y - targetRects[i].y;
      return `transform: rotate(${targetAngle}deg) translate(${dx}px, ${dy}px); transition: none;`;
    });
    await tick();
    await new Promise((r) => requestAnimationFrame(r));

    // PLAY
    charTransforms = charEls.map((_, i) => {
      const d = i * STAGGER;
      return `transform: rotate(${targetAngle}deg) translate(0, 0); transition: transform ${MOVE_DUR}ms cubic-bezier(0.16, 1, 0.3, 1) ${d}ms;`;
    });

    await sleep(TOTAL_MOVE + 50);
    onMoveEnd?.();

    charTransforms = Array(12).fill('');
    animating = false;
  }

  function sleep(ms: number): Promise<void> {
    return new Promise((r) => setTimeout(r, ms));
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
        style={charTransforms[i] || null}
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
        style={charTransforms[i + 6] || null}
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
