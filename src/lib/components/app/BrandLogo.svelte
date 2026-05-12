<script lang="ts">
  interface Props {
    isMacOS?: boolean;
  }

  let { isMacOS = false }: Props = $props();
</script>

<div class="brand-logo" class:macos={isMacOS} aria-hidden="true">
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
  }

  .brand-logo.macos {
    padding-top: 48px;
  }

  .brand-text {
    font-family: var(--font-wide);
    font-size: 22px;
    font-weight: 700;
    letter-spacing: 0.04em;
    color: var(--accent);
  }

  .brand-outline {
    color: transparent;
    -webkit-text-stroke: 1.5px var(--accent);
  }
</style>
