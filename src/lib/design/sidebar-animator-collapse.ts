/**
 * 侧栏收缩动画
 *
 * Phase 1 — 宽度收缩 + 旋转 + 内容淡出（并行，200ms）
 *  ├─ 侧栏宽度 248px → 56px (ios-spring)
 *  ├─ 字符旋转 0° → -90° (ios-spring, stagger 50ms)
 *  ├─ 导航/收藏区域淡出至 opacity 0
 *  └─ 文字标签 maxWidth → 0, opacity → 0
 *
 * Phase 2 — FLIP 布局切换（字符飞向竖向堆栈）
 */
import { gsap } from '$lib/design/gsap';
import {
  collectSidebarAnimatorLabelEls,
  chainTimelineComplete,
} from './sidebar-animator';
import type { AnimatorContext } from './sidebar-animator';

export async function runCollapse(id: number, ctx: AnimatorContext) {
  const { config, logoGlyphEls, params, isStale, setTimeline } = ctx;

  config.onContentInteractive(false);

  const labelEls = collectSidebarAnimatorLabelEls(config);
  const phase1 = gsap.timeline();
  setTimeline(phase1);

  phase1.to(
    config.shellEl,
    {
      '--sidebar-width': ctx.constants.COLLAPSED_WIDTH,
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

  const flipResult = await ctx.flipPhase(id, true);
  if (!flipResult || isStale(id)) return;

  await chainTimelineComplete(flipResult);
  if (isStale(id)) return;

  config.onContentSwitch(true);
  ctx.commitState(true);
}
