import starlight from '@astrojs/starlight';
import { defineConfig } from 'astro/config';
import { syncRepoContent } from './integrations/sync-repo-content';
import { BASE, REPO_URL, SITE } from './site.config';

export default defineConfig({
  site: SITE,
  base: BASE,
  trailingSlash: 'always',
  integrations: [
    syncRepoContent(),
    starlight({
      title: 'Google Rust Style Guide Kit',
      description:
        "Unofficial kit that enforces Google's Rust style guide with rustfmt, clippy, cargo-deny, AGENTS.md for coding agents, and a human review guide. Adopt it in 5 minutes.",
      logo: { src: './src/assets/logo.svg' },
      social: [{ icon: 'github', label: 'GitHub', href: REPO_URL }],
      lastUpdated: true,
      sidebar: [
        {
          label: 'Start',
          items: [{ slug: 'adopting' }],
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
  ],
});
