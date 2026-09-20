# claude-statusline

A fast, minimal statusline for [Claude Code](https://claude.com/claude-code).

[![CI](https://github.com/xuedi/claude-statusline/actions/workflows/ci.yml/badge.svg)](https://github.com/xuedi/claude-statusline/actions/workflows/ci.yml)
[![Release](https://github.com/xuedi/claude-statusline/actions/workflows/release.yml/badge.svg)](https://github.com/xuedi/claude-statusline/actions/workflows/release.yml)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![Made with Braille](https://img.shields.io/badge/progress_bars-%E2%A3%BF%E2%A3%BF%E2%A3%BF%E2%A3%B7-brightgreen)](#what-it-shows)
[![Rust](https://img.shields.io/badge/Rust-1.85+-orange?logo=rust&logoColor=white)](https://www.rust-lang.org)

```
MeetAgain | Claude Sonnet 4.6 1M | Effort: high | git@main (+12 -3) | 250k/1m [⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀] | HourlyReset: 3% @03:30 | WeeklyReset: 11% @May 18, 22:00
```

## What it shows

- **Project** - the project folder name, skipping role subfolders (`FileFin/app` shows `FileFin`)
- **Model** - active Claude model, with the context window summarized (`Sonnet 4.6 1M`)
- **Git** - branch and unstaged diff size in the current working directory
- **Tokens** - used/total of the context window plus a 20-cell braille progress bar
- **Effort** - the `effortLevel` setting (`low`, `med`, `high`)
- **Rate limits** - 5-hour and 7-day usage percentages with reset times

If Claude Code does not pass rate-limit data in the statusline payload, the binary
fetches it from Anthropic's `/api/oauth/usage` endpoint and caches the result for 60s.

## Install

```bash
git clone https://github.com/xuedi/claude-statusline
cd claude-statusline
just install        # builds release and copies to ~/.claude/claude-statusline
```

Or directly with cargo:

```bash
cargo install --path .
```

> Prefer a package? Every release ships static `x86_64` and `aarch64` builds for
> Debian/Ubuntu, Fedora/RHEL/openSUSE, Arch, and a plain tarball - see
> [docs/install.md](docs/install.md).

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

| Variable                   | Purpose                                                        |
|----------------------------|----------------------------------------------------------------|
| `HOME`                     | Required - used to locate `~/.claude/`                         |
| `CLAUDE_CONFIG_DIR`        | Overrides `~/.claude` when looking up credentials and settings |
| `CLAUDE_CODE_EFFORT_LEVEL` | Fallback effort level when `settings.json` does not set one    |

## Security note

This binary reads `~/.claude/.credentials.json` to fetch usage data from Anthropic's
`/api/oauth/usage` endpoint **only when Claude Code does not supply rate-limit data
in the statusline payload**. The token never leaves the binary; results are cached
under `/tmp/claude/`.

If this bothers you, the binary still produces useful output (model, git, tokens,
effort) without it - those segments do not touch credentials.

## Docs

- [Install](docs/install.md) - prebuilt packages per distro
- [Architecture](docs/architecture.md) - how the ~500 lines fit together, and why
- [Contributing](docs/contributing.md) - dev loop, branching, style, releasing

## License

[EUPL-1.2](LICENSE)
