export type SidebarGroup = 'Start' | 'Concepts' | 'Reference' | 'Review';

interface BaseEntry {
  slug: string;
  title: string;
  seoTitle: string;
  description: string;
  group: SidebarGroup;
  order: number;
}

/** A repository Markdown file rendered as a page. */
export interface DocEntry extends BaseEntry {
  kind: 'doc';
  source: string;
}

/** A repository config file shown in full with an explanation. */
export interface ConfigEntry extends BaseEntry {
  kind: 'config';
  source: string;
  lang: string;
  download: string;
  intro: string;
}

export type ContentEntry = DocEntry | ConfigEntry;

export const contentMap: ContentEntry[] = [
  {
    kind: 'doc',
    source: 'docs/adopting.md',
    slug: 'adopting',
    title: 'Adopting in depth',
    seoTitle: 'Adopt the Google Rust Style Guide in an Existing Project',
    description:
      'Wire the kit into a workspace or single crate, add it to CI, phase lints into an existing codebase, and relax API lints in application crates.',
    group: 'Start',
    order: 2,
  },
  {
    kind: 'doc',
    source: 'docs/how-it-works.md',
    slug: 'how-it-works',
    title: 'How it works',
    seoTitle: 'How the Google Rust Style Guide Kit Enforces Each Rule',
    description:
      'Three layers enforce Google\'s Rust style guide: rustfmt, clippy and cargo-deny block merges, AGENTS.md guides coding agents, and humans review the rest.',
    group: 'Concepts',
    order: 1,
  },
  {
    kind: 'doc',
    source: 'docs/why.md',
    slug: 'why',
    title: 'Why these choices',
    seoTitle: 'Why These Clippy Lints for the Google Rust Style Guide',
    description:
      'The reasoning behind every lint, threshold and setting in the kit, from absolute_paths limits to disallowing Rc and Arc without a written reason.',
    group: 'Concepts',
    order: 2,
  },
  {
    kind: 'doc',
    source: 'docs/why-not.md',
    slug: 'why-not',
    title: 'Why not',
    seoTitle: 'What Clippy Cannot Enforce in the Google Rust Style Guide',
    description:
      'Rules no linter can check, known gaps in enabled lints, lints rejected because they contradict the guide, and Chromium or Fuchsia specific rules.',
    group: 'Concepts',
    order: 3,
  },
  {
    kind: 'doc',
    source: 'docs/coverage.md',
    slug: 'coverage',
    title: 'Coverage',
    seoTitle: 'Google Rust Style Guide Coverage: Clippy Lint per Rule',
    description:
      'Every section of Google\'s Rust style guide mapped to the clippy, rustfmt or cargo-deny check that enforces it, or to review when no tool can.',
    group: 'Reference',
    order: 1,
  },
  {
    kind: 'doc',
    source: 'AGENTS.md',
    slug: 'agents-md',
    title: 'AGENTS.md for coding agents',
    seoTitle: 'AGENTS.md for Rust: Google Style Guide Rules for AI Agents',
    description:
      'Drop-in AGENTS.md that teaches Claude Code, Codex, Cursor and other coding agents the Google Rust style rules that no linter can check.',
    group: 'Reference',
    order: 2,
  },
  {
    kind: 'config',
    source: 'Cargo.toml',
    slug: 'config/cargo-lints',
    title: 'Cargo.toml lints',
    seoTitle: 'Clippy Lints for the Google Rust Style Guide (Cargo.toml)',
    description:
      'The 41 rustc, rustdoc and clippy lints that enforce Google\'s Rust style guide, each annotated with the guide section it serves.',
    group: 'Reference',
    order: 3,
    lang: 'toml',
    download: 'workspace-lints.toml',
    intro:
      'These `[workspace.lints]` tables turn on every rustc, rustdoc and clippy lint the kit uses. Paste them into your workspace root `Cargo.toml` and add `[lints] workspace = true` to each member crate. A single crate uses the same tables without `workspace.`; download `crate-lints.toml` for that version.',
  },
  {
    kind: 'config',
    source: 'clippy.toml',
    slug: 'config/clippy-toml',
    title: 'clippy.toml',
    seoTitle: 'clippy.toml for the Google Rust Style Guide',
    description:
      'Clippy thresholds, test exemptions, and the disallowed types and methods that enforce the guide\'s rules on Rc, Arc, catch_unwind and poisoned locks.',
    group: 'Reference',
    order: 4,
    lang: 'toml',
    download: 'clippy.toml',
    intro:
      'Put this file at your repository root. It sets the thresholds the lints in `Cargo.toml` use and lists the types and methods the guide asks you to avoid.',
  },
  {
    kind: 'config',
    source: 'rustfmt.toml',
    slug: 'config/rustfmt-toml',
    title: 'rustfmt.toml',
    seoTitle: 'rustfmt.toml for the Google Rust Style Guide',
    description:
      'The rustfmt options Google\'s Rust style guide names: 100 columns, lowercase hex, macro matcher formatting and one module per use statement.',
    group: 'Reference',
    order: 5,
    lang: 'toml',
    download: 'rustfmt.toml',
    intro:
      'Put this file at your repository root and run `cargo +nightly fmt`. Stable rustfmt applies `max_width` and skips the three nightly-only options with a warning.',
  },
  {
    kind: 'config',
    source: 'deny.toml',
    slug: 'config/deny-toml',
    title: 'deny.toml',
    seoTitle: 'cargo-deny Config for the Google Rust Style Guide',
    description:
      'A cargo-deny configuration that bans ctor, inventory and static_init, the crates that run code before main(), as Google\'s Rust style guide requires.',
    group: 'Reference',
    order: 6,
    lang: 'toml',
    download: 'deny.toml',
    intro:
      'Put this file at your repository root and run `cargo deny check bans`. It rejects dependencies that run code before `main()`.',
  },
  {
    kind: 'doc',
    source: 'docs/human-review.md',
    slug: 'human-review',
    title: 'Human review guide',
    seoTitle: 'Rust Code Review Checklist for the Google Rust Style Guide',
    description:
      'What reviewers can skip because CI enforces it, what to check in which order, acceptable reasons for each suppressed lint, and how to review agent code.',
    group: 'Review',
    order: 1,
  },
];

/** Files copied to `/files/` for download, in the order the quick start lists them. */
export const downloadableFiles = [
  'rustfmt.toml',
  'clippy.toml',
  'deny.toml',
  'rust-toolchain.toml',
  'AGENTS.md',
] as const;
