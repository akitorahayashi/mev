#!/usr/bin/env bash
set -euo pipefail

repo="${MEV_GITHUB_REPO:-akitorahayashi/mev}"
version="${MEV_VERSION:-latest}"
install_dir="${MEV_INSTALL_DIR:-$HOME/.local/bin}"
binary_name="mev"

if [[ "$(uname -s)" != "Darwin" ]]; then
	echo "mev installer supports macOS only." >&2
	exit 1
fi

arch="$(uname -m)"
case "$arch" in
arm64 | aarch64) target="darwin-aarch64" ;;
*)
	echo "Unsupported architecture: $arch (available: darwin-aarch64)" >&2
	exit 1
	;;
esac

if ! command -v curl >/dev/null 2>&1; then
	echo "curl is required but was not found in PATH." >&2
	exit 1
fi

if [[ -n "${MEV_BINARY_URL:-}" ]]; then
	binary_url="$MEV_BINARY_URL"
elif [[ "$version" == "latest" ]]; then
	binary_url="https://github.com/${repo}/releases/latest/download/${binary_name}-${target}"
else
	binary_url="https://github.com/${repo}/releases/download/${version}/${binary_name}-${target}"
fi

tmp_file="$(mktemp "${TMPDIR:-/tmp}/mev.XXXXXX")"
trap 'rm -f "$tmp_file"' EXIT

echo "Downloading ${binary_name} for ${target} from ${binary_url}..."
curl -fsSL "$binary_url" -o "$tmp_file"

mkdir -p "$install_dir"
chmod +x "$tmp_file"
mv "$tmp_file" "${install_dir}/${binary_name}"

echo "Installed to ${install_dir}/${binary_name}"
echo "Ensure ${install_dir} is in PATH, then run: ${binary_name} --version"
