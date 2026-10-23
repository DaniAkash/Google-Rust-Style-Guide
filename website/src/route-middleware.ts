import { defineRouteMiddleware } from '@astrojs/starlight/route-data';
import type { BreadcrumbList, Graph, SoftwareSourceCode, TechArticle } from 'schema-dts';
import { contentMap } from '../content-map';
import { AUTHOR, BASE, GUIDE_URL, REPO_URL, SITE, SITE_URL } from '../site.config';

const groupBySlug = new Map<string, string>([
  ['quick-start', 'Start'],
  ['faq', 'Start'],
  ...contentMap.map((entry): [string, string] => [entry.slug, entry.group]),
]);

const author = { '@type': 'Person', name: AUTHOR.name, url: AUTHOR.url } as const;

/** Adds the social image, the markdown twin link and JSON-LD to every page. */
export const onRequest = defineRouteMiddleware((context) => {
  const route = context.locals.starlightRoute;
  const slug = route.id === 'index' ? '' : route.id;
  if (slug === '404') return;

  const pageUrl = slug ? `${SITE_URL}${slug}/` : SITE_URL;
  const image = `${SITE}${BASE}/og/${slug || 'index'}.png`;
  const { title, description } = route.entry.data;

  route.head.push(
    { tag: 'meta', attrs: { property: 'og:image', content: image } },
    { tag: 'meta', attrs: { property: 'og:image:width', content: '1200' } },
    { tag: 'meta', attrs: { property: 'og:image:height', content: '630' } },
    { tag: 'meta', attrs: { property: 'og:image:alt', content: title } },
    { tag: 'meta', attrs: { name: 'twitter:image', content: image } },
  );
  if (slug) {
    route.head.push({
      tag: 'link',
      attrs: { rel: 'alternate', type: 'text/markdown', href: `${BASE}/${slug}.md` },
    });
  }

  const graph: Graph = {
    '@context': 'https://schema.org',
    '@graph': slug
      ? [article(pageUrl, title, description, route.lastUpdated, image), breadcrumbs(slug, title, pageUrl)]
      : [project(description)],
  };
  route.head.push({
    tag: 'script',
    attrs: { type: 'application/ld+json' },
    content: JSON.stringify(graph).replaceAll('<', '\\u003c'),
  });
});

function project(description: string | undefined): SoftwareSourceCode {
  return {
    '@type': 'SoftwareSourceCode',
    '@id': `${SITE_URL}#project`,
    name: 'Google Rust Style Guide Kit',
    alternateName: 'Google Rust Style Guide enforcement kit (unofficial)',
    url: SITE_URL,
    description,
    codeRepository: REPO_URL,
    programmingLanguage: 'Rust',
    license: 'https://www.apache.org/licenses/LICENSE-2.0',
    isBasedOn: GUIDE_URL,
    author,
  };
}

function article(
  url: string,
  headline: string,
  description: string | undefined,
  lastUpdated: Date | undefined,
  image: string,
): TechArticle {
  return {
    '@type': 'TechArticle',
    headline,
    description,
    url,
    mainEntityOfPage: url,
    image,
    dateModified: lastUpdated?.toISOString(),
    author,
    isBasedOn: GUIDE_URL,
    isPartOf: { '@id': `${SITE_URL}#project` },
    inLanguage: 'en',
  };
}

function breadcrumbs(slug: string, title: string, url: string): BreadcrumbList {
  const group = groupBySlug.get(slug);
  const crumbs = [{ name: 'Home', item: SITE_URL }, ...(group ? [{ name: group }] : []), { name: title, item: url }];
  return {
    '@type': 'BreadcrumbList',
    itemListElement: crumbs.map((crumb, index) => ({
      '@type': 'ListItem',
      position: index + 1,
      name: crumb.name,
      ...('item' in crumb ? { item: crumb.item } : {}),
    })),
  };
}
