# Install

The standard install is [from source](../README.md#install). This page covers the
prebuilt packages attached to every [tagged release](https://github.com/xuedi/claude-statusline/releases).

All binaries are static, dependency-free and built for `x86_64` and `aarch64`. The
examples below use `x86_64`; swap the architecture in the filename for arm64. Each
release also carries a `SHA256SUMS` file.

## Debian, Ubuntu, Mint

```bash
VERSION=1.0.0
curl -LO https://github.com/xuedi/claude-statusline/releases/download/v$VERSION/claude-statusline_$VERSION-1_amd64.deb
sudo dpkg -i claude-statusline_$VERSION-1_amd64.deb
```

## Fedora, RHEL, openSUSE

```bash
VERSION=1.0.0
curl -LO https://github.com/xuedi/claude-statusline/releases/download/v$VERSION/claude-statusline-$VERSION-1.x86_64.rpm
sudo rpm -i claude-statusline-$VERSION-1.x86_64.rpm
```

## Arch Linux

Prebuilt package straight from the release:

```bash
VERSION=1.0.0
curl -LO https://github.com/xuedi/claude-statusline/releases/download/v$VERSION/claude-statusline-bin-$VERSION-1-x86_64.pkg.tar.zst
sudo pacman -U claude-statusline-bin-$VERSION-1-x86_64.pkg.tar.zst
```

Or build it with the `PKGBUILD` that ships with every release:

```bash
curl -LO https://github.com/xuedi/claude-statusline/releases/latest/download/PKGBUILD
makepkg -si
```

## Any other distro

```bash
VERSION=1.0.0
curl -L https://github.com/xuedi/claude-statusline/releases/download/v$VERSION/claude-statusline-$VERSION-x86_64-linux.tar.gz | tar xz
install -Dm755 claude-statusline ~/.claude/claude-statusline
```

The binary is statically linked against musl, so it runs on glibc and musl systems
alike with no runtime dependencies.

Once installed, point Claude Code at it - see
[Configure Claude Code](../README.md#configure-claude-code).
