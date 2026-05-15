# claude-statusline

A fast, minimal statusline for [Claude Code](https://claude.com/claude-code).

[![License: EUPL-1.2](https://img.shields.io/badge/License-EUPL--1.2-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.85+-orange?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![Built for Claude Code](https://img.shields.io/badge/Built_for-Claude_Code-blueviolet?logo=anthropic&logoColor=white)](https://claude.ai/claude-code)
[![Made with Braille](https://img.shields.io/badge/progress_bars-%E2%A3%BF%E2%A3%BF%E2%A3%BF%E2%A3%B7-brightgreen)](#what-it-shows)

```
Claude Sonnet 4.6 1M | git@main (+12 -3) | 250k/1m [⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀] | effort: high | 5h 3% @03:30 | 7d 11% @May 18, 22:00
```

## What it shows

- **Model** - active Claude model, with the context window summarized (`Sonnet 4.6 1M`)
- **Git** - branch and unstaged diff size in the current working directory
- **Tokens** - used/total of the context window plus a 20-cell braille progress bar
- **Effort** - the `effortLevel` setting (`low`, `med`, `high`)
- **Rate limits** - 5-hour and 7-day usage percentages with reset times

If Claude Code does not pass rate-limit data in the statusline payload, the binary
fetches it from Anthropic's `/api/oauth/usage` endpoint and caches the result for 60s.

## Install

### From source

```bash
git clone https://github.com/xuedi/claude-statusline
cd claude-statusline
just install        # builds release and copies to ~/.claude/claude-statusline
```

Or directly with cargo:

```bash
cargo install --path .
```

Prebuilt binaries: coming once the project hits crates.io.

## Configure Claude Code

Add to `~/.claude/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/claude-statusline"
  }
}
```

## Environment variables

| Variable                   | Purpose                                                            |
|----------------------------|--------------------------------------------------------------------|
| `HOME`                     | Required - used to locate `~/.claude/`                             |
| `CLAUDE_CONFIG_DIR`        | Overrides `~/.claude` when looking up credentials and settings     |
| `CLAUDE_CODE_EFFORT_LEVEL` | Fallback effort level when `settings.json` does not set one        |

## Security note

This binary reads `~/.claude/.credentials.json` to fetch usage data from Anthropic's
`/api/oauth/usage` endpoint **only when Claude Code does not supply rate-limit data
in the statusline payload**. The token never leaves the binary; results are cached
under `/tmp/claude/`.

If this bothers you, the binary still produces useful output (model, git, tokens,
effort) without it - those segments do not touch credentials.

## Development

```bash
just            # list recipes
just check      # fmt-check + clippy + test - run this before pushing
just test       # cargo test
just demo       # pipe a sample payload through the binary
```

### Adding a segment

1. Write `src/segments/<name>.rs` exposing `pub fn render(input: &Input) -> ...`
2. Wire it into `segments::all()` in `src/segments/mod.rs`
3. Add a unit test in the same module

There is no `Segment` trait - five hand-listed segments do not need the indirection.

## Contributing

PRs welcome. Run `just check` before pushing. Keep the binary single-purpose; no new
mandatory dependencies without discussion.

## License

[EUPL-1.2](LICENSE)
