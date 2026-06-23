#!/usr/bin/env bash
set -euo pipefail

cargo install --path .

if ! command -v btcmap-cli >/dev/null 2>&1; then
    echo "btcmap-cli not found in PATH"
    echo "Add the following to your shell rc file (~/.bashrc, ~/.zshrc, etc.):"
    echo '  export PATH="$HOME/.cargo/bin:$PATH"'
    exit 1
fi

echo "Installed: $(command -v btcmap-cli)"