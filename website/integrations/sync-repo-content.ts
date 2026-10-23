import { execFileSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import type { AstroIntegration } from 'astro';
import { contentMap, downloadableFiles, type ConfigEntry, type ContentEntry } from '../content-map';
import { BASE, REPO_URL, SITE_URL } from '../site.config';

const siteRoot = fileURLToPath(new URL('..', import.meta.url));
const repoRoot = path.resolve(siteRoot, '..');
const generatedDocsDir = path.join(siteRoot, 'src/content/docs/repo');
const generatedSnippetsDir = path.join(siteRoot, 'src/generated');
const publicFilesDir = path.join(siteRoot, 'public/files');
const siteOnlyPages: Record<string, string> = {
  '': 'website/src/content/docs/index.mdx',
  'quick-start': 'website/src/content/docs/quick-start.mdx',
  faq: 'website/src/content/docs/faq.md',
};

const entryBySource = new Map<string, ContentEntry>(contentMap.map((entry) => [entry.source, entry]));
const lastModifiedByUrl = new Map<string, string>();

/** Copies repository docs and configs into the site so the repo stays the single source. */
export function syncRepoContent(): AstroIntegration {
  return {
    name: 'sync-repo-content',
    hooks: {
      'astro:config:setup': ({ addWatchFile, logger }) => {
        syncAll();
        for (const source of [...entryBySource.keys(), ...downloadableFiles]) {
          addWatchFile(path.join(repoRoot, source));
        }
        logger.info(`synced ${contentMap.length} pages and ${downloadableFiles.length + 2} downloads`);
      },
    },
  };
}

/** The git date of the source behind a sitemap URL, for an accurate `<lastmod>`. */
export function lastModifiedBySlug(url: string): string | undefined {
  return lastModifiedByUrl.get(url);
}

function syncAll() {
  for (const dir of [generatedDocsDir, generatedSnippetsDir, publicFilesDir]) {
    fs.rmSync(dir, { recursive: true, force: true });
    fs.mkdirSync(dir, { recursive: true });
  }

  const workspaceLints = extractWorkspaceLints();
  const crateLints = toCrateLints(workspaceLints);
  writeFile(path.join(generatedSnippetsDir, 'workspace-lints.toml'), workspaceLints);
  writeFile(path.join(generatedSnippetsDir, 'crate-lints.toml'), crateLints);
  writeFile(path.join(publicFilesDir, 'workspace-lints.toml'), workspaceLints);
  writeFile(path.join(publicFilesDir, 'crate-lints.toml'), crateLints);
  for (const file of downloadableFiles) {
    fs.copyFileSync(path.join(repoRoot, file), path.join(publicFilesDir, file));
  }

  lastModifiedByUrl.clear();
  for (const entry of contentMap) {
    const body = entry.kind === 'doc' ? renderDoc(entry.source) : renderConfig(entry, workspaceLints);
    const lastUpdated = gitDate(entry.source);
    writeFile(path.join(generatedDocsDir, `${entry.slug}.md`), frontmatter(entry, lastUpdated) + body);
    if (lastUpdated) lastModifiedByUrl.set(`${SITE_URL}${entry.slug}/`, lastUpdated);
  }
  for (const [slug, source] of Object.entries(siteOnlyPages)) {
    const lastUpdated = gitDate(source);
    if (lastUpdated) lastModifiedByUrl.set(slug ? `${SITE_URL}${slug}/` : SITE_URL, lastUpdated);
  }
}

function renderDoc(source: string): string {
  const markdown = fs.readFileSync(path.join(repoRoot, source), 'utf8');
  const withoutTitle = markdown.replace(/^# .*\n+/, '');
  return rewriteLinks(withoutTitle, source);
}

function renderConfig(entry: ConfigEntry, workspaceLints: string): string {
  const isCargo = entry.source === 'Cargo.toml';
  const code = isCargo ? workspaceLints.trimStart() : fs.readFileSync(path.join(repoRoot, entry.source), 'utf8');
  const title = isCargo ? 'Cargo.toml (workspace root)' : entry.source;
  const downloads = isCargo
    ? `[Download workspace-lints.toml](${BASE}/files/workspace-lints.toml) · [Download crate-lints.toml](${BASE}/files/crate-lints.toml)`
    : `[Download ${entry.download}](${BASE}/files/${entry.download})`;
  return [
    entry.intro,
    '',
    downloads,
    '',
    `\`\`\`${entry.lang} title="${title}"`,
    code.trimEnd(),
    '```',
    '',
    `Source: [\`${entry.source}\` on GitHub](${REPO_URL}/blob/main/${entry.source}). New to the kit? Start with the [quick start](${BASE}/quick-start/).`,
    '',
  ].join('\n');
}

/** Rewrites relative repository links to site pages or GitHub URLs, skipping fenced code. */
function rewriteLinks(markdown: string, source: string): string {
  let inFence = false;
  return markdown
    .split('\n')
    .map((line) => {
      if (/^\s*(```|~~~)/.test(line)) inFence = !inFence;
      if (inFence) return line;
      return line.replace(/\]\(([^)\s]+)\)/g, (match, target: string) => {
        const rewritten = rewriteTarget(target, source);
        return rewritten ? `](${rewritten})` : match;
      });
    })
    .join('\n');
}

function rewriteTarget(target: string, source: string): string | undefined {
  if (/^[a-z][a-z0-9+.-]*:/i.test(target) || target.startsWith('#') || target.startsWith('/')) {
    return undefined;
  }
  const [pathPart = '', hash] = target.split('#');
  const anchor = hash ? `#${hash}` : '';
  const resolved = path.posix.normalize(path.posix.join(path.posix.dirname(source), pathPart)).replace(/\/$/, '');
  const entry = entryBySource.get(resolved);
  if (entry) return `${BASE}/${entry.slug}/${anchor}`;
  if (resolved === 'docs/README.md' || resolved === 'docs') return `${BASE}/${anchor}`;

  const absolute = path.join(repoRoot, resolved);
  if (!fs.existsSync(absolute)) return undefined;
  const kind = fs.statSync(absolute).isDirectory() ? 'tree' : 'blob';
  return `${REPO_URL}/${kind}/main/${resolved}${anchor}`;
}

function extractWorkspaceLints(): string {
  const cargo = fs.readFileSync(path.join(repoRoot, 'Cargo.toml'), 'utf8');
  const start = cargo.indexOf('[workspace.lints');
  if (start === -1) throw new Error('Cargo.toml has no [workspace.lints] tables to publish');
  const header = [
    '',
    `# Google Rust Style Guide lints (unofficial kit): ${SITE_URL}`,
    '# Paste into your workspace root Cargo.toml, then add `[lints] workspace = true` to each member.',
    '',
  ].join('\n');
  return `${header}${cargo.slice(start).trimEnd()}\n`;
}

function toCrateLints(workspaceLints: string): string {
  return workspaceLints
    .replaceAll('[workspace.lints.', '[lints.')
    .replace(
      '# Paste into your workspace root Cargo.toml, then add `[lints] workspace = true` to each member.',
      "# Paste into your crate's Cargo.toml.",
    );
}

function frontmatter(entry: ContentEntry, lastUpdated: string | undefined): string {
  const lines = [
    '---',
    `title: ${JSON.stringify(entry.title)}`,
    `description: ${JSON.stringify(entry.description)}`,
    `slug: ${entry.slug}`,
    `editUrl: ${REPO_URL}/edit/main/${entry.source}`,
    lastUpdated ? `lastUpdated: ${lastUpdated}` : undefined,
    `sidebar: { order: ${entry.order} }`,
    `head: [${JSON.stringify({ tag: 'title', content: entry.seoTitle })}]`,
    '---',
    '',
  ];
  return lines.filter((line) => line !== undefined).join('\n');
}

function gitDate(source: string): string | undefined {
  try {
    const date = execFileSync('git', ['log', '-1', '--format=%cI', '--', source], {
      cwd: repoRoot,
      encoding: 'utf8',
    }).trim();
    return date || undefined;
  } catch {
    return undefined;
  }
}

function writeFile(file: string, content: string) {
  fs.mkdirSync(path.dirname(file), { recursive: true });
  fs.writeFileSync(file, content);
}
