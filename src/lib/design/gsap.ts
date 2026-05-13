import gsap from 'gsap';
import { Flip } from 'gsap/Flip';

gsap.registerPlugin(Flip);

export { gsap, Flip };

const reducedMotionQuery =
  typeof window !== 'undefined'
    ? window.matchMedia('(prefers-reduced-motion: reduce)')
    : null;

export function getMotionDuration(baseMs: number): number {
  return reducedMotionQuery?.matches ? 0 : baseMs / 1000;
}

export function killTweens(targets: gsap.TweenTarget): void {
  gsap.killTweensOf(targets);
}
