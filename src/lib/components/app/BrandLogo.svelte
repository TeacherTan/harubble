<script lang="ts">
  interface Props {
    isMacOS?: boolean;
    collapsed?: boolean;
  }

  let { isMacOS = false, collapsed = false }: Props = $props();
</script>

<div
  class="brand-logo"
  class:macos={isMacOS}
  class:collapsed
  aria-hidden="true"
>
  <span class="brand-text">HARU<span class="brand-outline">KA</span></span>
  <span class="brand-text"><span class="brand-outline">BU</span>BBLE</span>
</div>

<style>
  .brand-logo {
    display: flex;
    flex-direction: column;
    padding: 20px 24px 12px;
    line-height: 1;
    user-select: none;
    -webkit-user-select: none;
    transition:
      transform 300ms ease,
      padding 300ms ease;
    transform-origin: center center;
  }

  .brand-logo.macos {
    padding-top: 48px;
  }

  .brand-logo.collapsed {
    transform: rotate(-90deg);
    padding: 24px 0 12px;
  }

  .brand-logo.collapsed.macos {
    padding-top: 24px;
  }

  .brand-text {
    font-family: var(--font-wide);
    font-size: 22px;
    font-weight: 700;
    letter-spacing: 0.04em;
    color: var(--accent);
    transition: font-size 300ms ease;
    white-space: nowrap;
  }

  .collapsed .brand-text {
    font-size: 11px;
  }

  .brand-outline {
    color: transparent;
    -webkit-text-stroke: 1.5px var(--accent);
  }

  @media (prefers-reduced-motion: reduce) {
    .brand-logo {
      transition: none;
    }
    .brand-text {
      transition: none;
    }
  }
</style>
