import sitemap from '@astrojs/sitemap';
import starlight from '@astrojs/starlight';
import { defineConfig } from 'astro/config';
import mermaid from 'astro-mermaid';
import starlightLinksValidator from 'starlight-links-validator';
import starlightLlmsTxt from 'starlight-llms-txt';
import starlightPageActions from 'starlight-page-actions';
import { lastModifiedBySlug, syncRepoContent } from './integrations/sync-repo-content';
import { BASE, FILES_URL, GUIDE_URL, REPO_URL, SITE } from './site.config';

export default defineConfig({
  site: SITE,
  base: BASE,
  trailingSlash: 'always',
  integrations: [
    syncRepoContent(),
    mermaid({ autoTheme: true }),
    starlight({
      title: 'Google Rust Style Guide Kit',
      description:
        "Unofficial kit that enforces Google's Rust style guide with rustfmt, clippy, cargo-deny, AGENTS.md for coding agents, and a human review guide. Adopt it in 5 minutes.",
      logo: { src: './src/assets/logo.svg' },
      social: [{ icon: 'github', label: 'GitHub', href: REPO_URL }],
      lastUpdated: true,
      components: { Footer: './src/components/Footer.astro' },
      routeMiddleware: './src/route-middleware.ts',
      customCss: ['./src/styles/theme.css'],
      plugins: [
        starlightLinksValidator({
          exclude: [`${BASE}/files/**`, `${BASE}/og/**`, `${BASE}/**/*.md`, `${BASE}/llms*.txt`, `${BASE}/_llms-txt/**`],
        }),
        // No baseUrl: with it, this plugin writes its own llms.txt over starlight-llms-txt's.
        starlightPageActions({
          prompt: 'Read {url} and help me apply the Google Rust Style Guide kit to my Rust project.',
          actions: { chatgpt: true, claude: true, perplexity: true, cursor: true, markdown: true },
        }),
        starlightLlmsTxt({
          projectName: 'Google Rust Style Guide Kit (unofficial)',
          description:
            "Drop-in rustfmt, clippy and cargo-deny configuration plus AGENTS.md that enforces Google's Rust style guide. 32 of 62 in-scope rules are tool-enforced; the rest are covered by agent instructions and a human review guide.",
          details: [
            `Not affiliated with Google. The guide itself: ${GUIDE_URL}`,
            `Adopt: download rustfmt.toml, clippy.toml, deny.toml, rust-toolchain.toml and AGENTS.md from ${FILES_URL}/, append ${FILES_URL}/crate-lints.toml (single crate) or ${FILES_URL}/workspace-lints.toml (workspace root, then \`[lints] workspace = true\` in each member) to Cargo.toml, then run \`cargo +nightly fmt\`, \`cargo clippy --workspace --all-targets -- -D warnings\` and \`cargo deny check bans\`. Never edit the lint configuration to silence a warning; fix the code or add \`#[expect(lint, reason = "...")]\` where the guide allows an exception.`,
          ].join('\n\n'),
          promote: ['index*', 'quick-start', 'adopting', 'coverage', 'agents-md'],
          demote: ['why-not', 'config/**'],
          customSets: [
            {
              label: 'For coding agents',
              paths: ['quick-start', 'agents-md', 'adopting', 'config/**'],
              description: 'Everything a coding agent needs to apply the kit to a repository.',
            },
            {
              label: 'For reviewers',
              paths: ['human-review', 'coverage', 'why-not'],
              description: 'What the tools enforce and what a human reviewer must judge.',
            },
          ],
          optionalLinks: [
            { label: 'Google Rust Style Guide (original)', url: GUIDE_URL },
            { label: 'Source repository', url: REPO_URL },
          ],
        }),
      ],
      sidebar: [
        {
          label: 'Start',
          items: [{ slug: 'quick-start' }, { slug: 'adopting' }, { slug: 'faq' }],
        },
        {
          label: 'Concepts',
          items: [{ slug: 'how-it-works' }, { slug: 'why' }, { slug: 'why-not' }],
        },
        {
          label: 'Reference',
          items: [
            { slug: 'coverage' },
            { slug: 'agents-md' },
            { slug: 'config/cargo-lints' },
            { slug: 'config/clippy-toml' },
            { slug: 'config/rustfmt-toml' },
            { slug: 'config/deny-toml' },
          ],
        },
        {
          label: 'Review',
          items: [{ slug: 'human-review' }],
        },
      ],
    }),
    // Replaces Starlight's built-in sitemap so <lastmod> can come from git.
    sitemap({
      serialize(item) {
        const lastmod = lastModifiedBySlug(item.url);
        return lastmod ? { ...item, lastmod } : item;
      },
    }),
  ],
});
