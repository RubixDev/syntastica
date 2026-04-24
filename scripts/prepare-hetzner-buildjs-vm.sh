#!/usr/bin/env bash
set -euo pipefail

# Preps a fresh Ubuntu VM to reproduce/perform the syntastica build-js CI build.
# Designed for Hetzner Cloud Ubuntu 24.04, but should work on most recent Ubuntu.
#
# Usage:
#   ./scripts/prepare-hetzner-buildjs-vm.sh --repo https://github.com/casualjim/syntastica --ref expand-languages
#   ./scripts/prepare-hetzner-buildjs-vm.sh --full-build
#
# What it does:
# - Installs OS deps
# - Enables swap
# - Installs Rust toolchain + wasm32-unknown-emscripten target
# - Installs Node via nvm + tree-sitter-cli
# - Installs Emscripten SDK (emsdk)
# - Clones the repo and runs the "prep" steps that should populate the parser cache

REPO_URL="https://github.com/casualjim/syntastica"
REF="expand-languages"
WORKDIR="$HOME/syntastica"
CACHE_ROOT="/var/cache/syntastica"
SWAP_GB="16"
NODE_VERSION="23"
TREE_SITTER_CLI_VERSION="0.25.10"
EMSDK_VERSION="latest"
FULL_BUILD="0"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --repo) REPO_URL="$2"; shift 2 ;;
    --ref) REF="$2"; shift 2 ;;
    --workdir) WORKDIR="$2"; shift 2 ;;
    --cache-root) CACHE_ROOT="$2"; shift 2 ;;
    --swap-gb) SWAP_GB="$2"; shift 2 ;;
    --node) NODE_VERSION="$2"; shift 2 ;;
    --emsdk) EMSDK_VERSION="$2"; shift 2 ;;
    --full-build) FULL_BUILD="1"; shift 1 ;;
    -h|--help)
      sed -n '1,80p' "$0"; exit 0
      ;;
    *)
      echo "Unknown arg: $1" >&2
      exit 2
      ;;
  esac
done

log() { echo "[$(date -Is)] $*"; }

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || {
    echo "Missing required command: $1" >&2
    exit 1
  }
}

log "Installing system dependencies..."
sudo apt-get update -y
sudo apt-get install -y \
  build-essential pkg-config libssl-dev \
  curl git cmake unzip python3 python3-pip \
  ca-certificates xz-utils jq \
  time htop

log "Ensuring swap is enabled (${SWAP_GB}G)..."
if ! swapon --show | grep -q '^/swapfile'; then
  sudo fallocate -l "${SWAP_GB}G" /swapfile || sudo dd if=/dev/zero of=/swapfile bs=1M count=$((SWAP_GB*1024))
  sudo chmod 600 /swapfile
  sudo mkswap /swapfile
  sudo swapon /swapfile
  echo '/swapfile none swap sw 0 0' | sudo tee -a /etc/fstab >/dev/null
fi
free -h

log "Installing rustup (if needed)..."
if ! command -v rustup >/dev/null 2>&1; then
  curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
fi
# shellcheck disable=SC1090
source "$HOME/.cargo/env"

log "Installing wasm32-unknown-emscripten target..."
rustup target add wasm32-unknown-emscripten

log "Installing Node ${NODE_VERSION} via nvm (if needed)..."
if [[ ! -d "$HOME/.nvm" ]]; then
  curl -fsSL https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
fi
# shellcheck disable=SC1091
source "$HOME/.nvm/nvm.sh"
if ! command -v node >/dev/null 2>&1 || [[ "$(node -v | tr -d v | cut -d. -f1)" != "${NODE_VERSION}" ]]; then
  nvm install "${NODE_VERSION}"
  nvm alias default "${NODE_VERSION}"
fi
node -v
npm -v

log "Installing tree-sitter CLI ${TREE_SITTER_CLI_VERSION}..."
npm i -g "tree-sitter-cli@${TREE_SITTER_CLI_VERSION}"
require_cmd tree-sitter

log "Installing emsdk (${EMSDK_VERSION})..."
if [[ ! -d "$HOME/emsdk" ]]; then
  git clone https://github.com/emscripten-core/emsdk.git "$HOME/emsdk"
fi
pushd "$HOME/emsdk" >/dev/null
./emsdk install "${EMSDK_VERSION}"
./emsdk activate "${EMSDK_VERSION}"
# shellcheck disable=SC1091
source ./emsdk_env.sh
popd >/dev/null

log "Configuring syntastica parser cache env vars..."
sudo mkdir -p "$CACHE_ROOT/parsers/clone" "$CACHE_ROOT/parsers/cache"
sudo chown -R "$USER:$USER" "$CACHE_ROOT"

export SYNTASTICA_PARSERS_CLONE_DIR="$CACHE_ROOT/parsers/clone"
export SYNTASTICA_PARSERS_CACHE_DIR="$CACHE_ROOT/parsers/cache"
export SYNTASTICA_PARSERS_JS_RUNTIME=node

log "Persisting env vars to ~/.bashrc..."
if ! grep -q "SYNTASTICA_PARSERS_CLONE_DIR" "$HOME/.bashrc" 2>/dev/null; then
  echo '' >> "$HOME/.bashrc"
  echo '# Syntastica parser cache' >> "$HOME/.bashrc"
  echo "export SYNTASTICA_PARSERS_CLONE_DIR=\"$CACHE_ROOT/parsers/clone\"" >> "$HOME/.bashrc"
  echo "export SYNTASTICA_PARSERS_CACHE_DIR=\"$CACHE_ROOT/parsers/cache\"" >> "$HOME/.bashrc"
  echo 'export SYNTASTICA_PARSERS_JS_RUNTIME=node' >> "$HOME/.bashrc"
fi

log "Configuring emsdk in ~/.bashrc..."
if ! grep -q "emsdk_env.sh" "$HOME/.bashrc" 2>/dev/null; then
  echo '' >> "$HOME/.bashrc"
  echo '# Emscripten SDK' >> "$HOME/.bashrc"
  echo 'source "$HOME/emsdk/emsdk_env.sh" 2>/dev/null || true' >> "$HOME/.bashrc"
fi
export CARGO_BUILD_JOBS=1
export CARGO_INCREMENTAL=0

log "Cloning/updating repo into $WORKDIR..."
if [[ ! -d "$WORKDIR/.git" ]]; then
  git clone "$REPO_URL" "$WORKDIR"
fi
pushd "$WORKDIR" >/dev/null

git fetch --all --tags
# best-effort checkout
if git show-ref --verify --quiet "refs/heads/$REF"; then
  git switch "$REF"
else
  git switch -C "$REF" "origin/$REF" || git checkout "$REF"
fi

git reset --hard "origin/$REF" || true

log "Installing repo toolchain prerequisites (if any)..."
# (This repo's _prepare_ci.sh is not a full prep script; keep as a no-op-safe call)
if [[ -f "./_prepare_ci.sh" ]]; then
  bash ./_prepare_ci.sh || true
fi

log "Prebuilding generated grammars (tree-sitter generate where needed)..."
# This only touches languages marked parser.generate=true.
# It should reduce work done inside Rust build scripts.
( time cargo xtask build-grammars )

log "Precompiling parser cache for wasm32-unknown-emscripten (this is where SIGKILL happens in CI)..."
log "Cache dir: $SYNTASTICA_PARSERS_CACHE_DIR"
( time cargo build -p syntastica-parsers-git --profile release-wasm --target wasm32-unknown-emscripten --features all )

log "If the above failed, collect diagnostics:"
echo "  free -h"
echo "  df -h"
echo "  sudo dmesg -T | tail -200"

echo
log "Prep complete."
log "To run the full JS build (core + langs):"
cat <<'EOF'
cd "$HOME/syntastica"
# core JS package
cd syntastica-js
npm ci
npm run build
cd ..
# language packages
cargo xtask build-js-langs
EOF

echo
if [[ "$FULL_BUILD" == "1" ]]; then
  log "Running full build now (--full-build)..."
  pushd syntastica-js >/dev/null
  npm ci
  npm run build
  popd >/dev/null
  cargo xtask build-js-langs
fi

popd >/dev/null
