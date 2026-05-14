<script lang="ts">
  import * as m from '$lib/paraglide/messages.js';
  import { localeState } from '$lib/i18n';
  import { Home, Library } from '@lucide/svelte';
  import { untrack } from 'svelte';
  import { gsap, getMotionDuration, killTweens } from '$lib/design/gsap';
  import type { AppView } from '$lib/features/shell/store.svelte';

  interface Props {
    currentView: AppView;
    onNavigate: (view: AppView) => void;
    collapsed?: boolean;
  }

  let { currentView, onNavigate, collapsed = false }: Props = $props();

  const labels = $derived.by(() => {
    void localeState.current;
    return {
      home: m.shell_nav_home(),
      library: m.shell_nav_library(),
    };
  });

  const navItems: {
    view: AppView;
    icon: typeof Home;
    labelKey: 'home' | 'library';
  }[] = [
    { view: 'home', icon: Home, labelKey: 'home' },
    { view: 'overview', icon: Library, labelKey: 'library' },
  ];

  const labelEls: (HTMLSpanElement | null)[] = $state(
    Array(navItems.length).fill(null)
  );
  let prevCollapsed: boolean | null = $state(null);

  $effect(() => {
    const curr = collapsed;
    const prev = untrack(() => prevCollapsed);

    if (prev === null) {
      prevCollapsed = curr;
      return;
    }
    if (prev === curr) return;
    prevCollapsed = curr;

    const validEls = labelEls.filter(
      (el): el is HTMLSpanElement => el !== null
    );
    if (validEls.length === 0) return;

    killTweens(validEls);
    const dur = getMotionDuration(220);

    if (curr) {
      gsap.to(validEls, {
        maxWidth: 0,
        opacity: 0,
        duration: dur,
        stagger: 0.02,
        ease: 'ios-in',
      });
    } else {
      gsap.fromTo(
        validEls,
        { maxWidth: 0, opacity: 0 },
        {
          maxWidth: 120,
          opacity: 1,
          duration: dur,
          stagger: 0.02,
          ease: 'ios-out',
        }
      );
    }
  });

  $effect(() => {
    return () => {
      killTweens(labelEls.filter(Boolean));
    };
  });
</script>

<nav class="sidebar-nav" class:collapsed aria-label="Main navigation">
  {#each navItems as item, i (item.view)}
    <button
      type="button"
      class="nav-item"
      class:active={currentView === item.view}
      onclick={() => onNavigate(item.view)}
      aria-current={currentView === item.view ? 'page' : undefined}
      title={collapsed ? labels[item.labelKey] : undefined}
    >
      <item.icon size={16} aria-hidden="true" />
      <span
        class="nav-label"
        class:hidden={collapsed && prevCollapsed !== null}
        bind:this={labelEls[i]}>{labels[item.labelKey]}</span
      >
    </button>
  {/each}
</nav>

<style>
  .sidebar-nav {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 0 8px;
  }

  .sidebar-nav.collapsed {
    padding: 0;
    align-items: center;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: 100%;
    height: 36px;
    padding: 0 0.75rem;
    border: none;
    border-radius: 8px;
    background: none;
    color: var(--text-secondary, rgba(255, 255, 255, 0.6));
    font-family: var(--font-body);
    font-size: 0.8125rem;
    font-weight: 500;
    cursor: pointer;
    transition:
      background var(--motion-fast) ease,
      color var(--motion-fast) ease;
  }

  .collapsed .nav-item {
    width: 36px;
    height: 36px;
    padding: 0;
    justify-content: center;
  }

  .nav-label {
    overflow: hidden;
    white-space: nowrap;
  }

  .nav-label.hidden {
    max-width: 0;
    opacity: 0;
  }

  .nav-item:hover {
    background: var(--hover-bg-elevated);
    color: var(--text-primary);
  }

  .nav-item.active {
    background: var(--surface-state);
    color: var(--text-primary);
    font-weight: 600;
  }

  @media (prefers-reduced-motion: reduce) {
    .nav-label {
      transition: none;
    }
  }
</style>
