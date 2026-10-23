import { getCollection } from 'astro:content';
import { OGImageRoute } from 'astro-og-canvas';

const entries = await getCollection('docs');
const pages = Object.fromEntries(
  entries.map(({ id, data }) => [id || 'index', { title: data.title, description: data.description }]),
);

export const { getStaticPaths, GET } = await OGImageRoute({
  pages,
  getImageOptions: (_path, page: (typeof pages)[string]) => ({
    title: page.title,
    description: page.description,
    logo: { path: './src/assets/logo-og.png', size: [96] },
    bgGradient: [[24, 24, 27], [39, 39, 42]],
    border: { color: [194, 65, 12], width: 16, side: 'inline-start' },
    padding: 72,
    font: {
      title: { size: 64, weight: 'Bold', color: [250, 250, 250] },
      description: { size: 32, color: [212, 212, 216], lineHeight: 1.4 },
    },
  }),
});
