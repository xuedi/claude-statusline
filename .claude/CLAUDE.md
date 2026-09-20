# CLAUDE.md

## What this is

A tiny Rust CLI that reads the Claude Code statusline JSON payload from stdin and
prints one line: project name, model, git branch and diff, token usage with a braille
bar, effort level, and 5h/7d rate limits. Around 500 lines of code total, library
plus a thin binary.

The project docs are the source of truth - read them instead of re-deriving:

| Doc                      | What is in it                                                 |
|--------------------------|----------------------------------------------------------------|
| `docs/architecture.md`   | File layout table, data flow, settled design decisions          |
| `docs/contributing.md`   | Dev loop, branching, code style, adding a segment, tests, release |
| `docs/install.md`        | Distro packages                                                 |

## Run it

```bash
just check     # cargo fmt --check + clippy -D warnings + cargo test
```

Run that before reporting work done.

## Code style

Full list in `docs/contributing.md#code-style`. The ones that bite:

- ASCII `-` only - never em dash (`—`) in code, prose, or mermaid labels
- Comments are scarce: doc comments on public items, `// WHY:` for non-obvious
  workarounds, AAA markers in tests. No narrative comments.
- Use `use` imports - never fully-qualified inline paths
- `.unwrap()` is for tests. Production code returns `Option` / `Result` and
  collapses to the documented fallback at the top level.

## Settled questions

Do not re-propose these; the rationale is in `docs/architecture.md#decisions`.

- No `Segment` trait. Five hand-listed segments do not need the indirection.
- Hand-rolled `Error` enum, not `anyhow` / `thiserror`.
- New dependencies need to clear the bar in `docs/contributing.md#adding-a-dependency`.
- `segments::git`, `api::fetch_usage` and the `cache` filesystem paths are
  deliberately untested. Prefer a smoke test over a mocking layer.

## Git workflow

Never create commits yourself unless the user asks for one. `git status` and
`git diff` are fine for inspection.

`main` is protected and only moves through pull requests. All work happens on
`development` or a topic branch cut from it; if a task starts while `main` is
checked out, switch first. Full flow in `docs/contributing.md#branching`.

## Planning

If asked for a plan, write it to `.claude/plans/YYYY-MM-DD_<slug>.md` (create the
directory on demand). Freeform is fine for a project this size - no template.
