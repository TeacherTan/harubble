import { getImageDataUrl } from './api';

type ImageSource = string | null | undefined;

export function imageDataSrc(node: HTMLImageElement, source: ImageSource) {
  let requestSeq = 0;
  let activeSource: ImageSource = undefined;

  function setState(state: 'empty' | 'loading' | 'loaded' | 'error') {
    node.dataset.imageState = state;
  }

  async function load(nextSource: ImageSource) {
    const normalizedSource = nextSource || null;
    if (normalizedSource === activeSource) return;

    activeSource = normalizedSource;
    const seq = ++requestSeq;
    node.removeAttribute('src');
    setState(normalizedSource ? 'loading' : 'empty');

    if (!normalizedSource) return;

    try {
      const dataUrl = await getImageDataUrl(normalizedSource);
      if (seq !== requestSeq) return;

      node.src = dataUrl;
      setState('loaded');
    } catch {
      if (seq !== requestSeq) return;

      node.removeAttribute('src');
      setState('error');
    }
  }

  function handleImageError() {
    if (node.dataset.imageState !== 'loaded') return;

    node.removeAttribute('src');
    setState('error');
  }

  node.decoding = 'async';
  node.addEventListener('error', handleImageError);
  void load(source);

  return {
    update(nextSource: ImageSource) {
      void load(nextSource);
    },
    destroy() {
      requestSeq += 1;
      node.removeEventListener('error', handleImageError);
    },
  };
}
