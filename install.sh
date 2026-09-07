#!/usr/bin/env bash
set -euo pipefail

repository="${GABRIEL_REPOSITORY:-GabrielEVP/gabriel-configurations}"
version="${GABRIEL_VERSION:-latest}"
install_dir="${GABRIEL_INSTALL_DIR:-$HOME/.local/bin}"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

case "$(uname -s):$(uname -m)" in
  Linux:x86_64) artifact="gabriel-linux-x86_64" ;;
  Linux:aarch64|Linux:arm64) artifact="gabriel-linux-aarch64" ;;
  Darwin:x86_64) artifact="gabriel-macos-x86_64" ;;
  Darwin:arm64|Darwin:aarch64) artifact="gabriel-macos-aarch64" ;;
  *) printf '%s\n' 'Unsupported operating system or architecture' >&2; exit 2 ;;
esac

command -v curl >/dev/null || { printf '%s\n' 'curl is required' >&2; exit 2; }
command -v sha256sum >/dev/null || command -v shasum >/dev/null || { printf '%s\n' 'sha256sum or shasum is required' >&2; exit 2; }

if [ "$version" = latest ]; then base="https://github.com/$repository/releases/latest/download"; else base="https://github.com/$repository/releases/download/$version"; fi
curl --fail --silent --show-error --location "$base/$artifact" -o "$tmp_dir/gabriel"
curl --fail --silent --show-error --location "$base/$artifact.sha256" -o "$tmp_dir/checksum"
expected="$(awk '{print $1}' "$tmp_dir/checksum")"
actual="$(if command -v sha256sum >/dev/null; then sha256sum "$tmp_dir/gabriel"; else shasum -a 256 "$tmp_dir/gabriel"; fi | awk '{print $1}')"
[ "$expected" = "$actual" ] || { printf '%s\n' 'Release checksum mismatch' >&2; exit 1; }
mkdir -p "$install_dir"
chmod 0755 "$tmp_dir/gabriel"
mv "$tmp_dir/gabriel" "$install_dir/gabriel"
printf 'Installed gabriel at %s/gabriel\n' "$install_dir"
printf 'Run: %s/gabriel setup --all\n' "$install_dir"
