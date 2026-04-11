#!/usr/bin/env bash
set -euo pipefail

# ==========================================================
# DeckClip CLI 构建脚本
#
# 编译 deckclip Rust CLI 二进制（release + strip），
# 可选地将产物注入到 Deck.app 的 Resources 目录中。
#
# 用法:
#   ./build-release.sh                          # 仅编译
#   ./build-release.sh /path/to/Deck.app        # 编译 + 注入到 .app
#
# 环境变量:
#   CARGO      cargo 路径 (默认: cargo)
#   TARGET     编译目标 (默认: 当前架构)
# ==========================================================

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# Ensure cargo is available (rustup installs to ~/.cargo/bin)
if [[ -f "$HOME/.cargo/env" ]]; then
    source "$HOME/.cargo/env"
fi

CARGO="${CARGO:-cargo}"
BINARY_NAME="deckclip"

# Detect target triple
if [[ -z "${TARGET:-}" ]]; then
    ARCH="$(uname -m)"
    case "$ARCH" in
        arm64) TARGET="aarch64-apple-darwin" ;;
        x86_64) TARGET="x86_64-apple-darwin" ;;
        *) echo "ERROR: unsupported arch: $ARCH"; exit 1 ;;
    esac
fi

echo "=== DeckClip Build ==="
echo "Project: $PROJECT_DIR"
echo "Target:  $TARGET"
echo ""

# Build
echo "1) cargo build --release --target $TARGET ..."
cd "$PROJECT_DIR"
$CARGO build --release --target "$TARGET"

BINARY="$PROJECT_DIR/target/$TARGET/release/$BINARY_NAME"
if [[ ! -f "$BINARY" ]]; then
    echo "ERROR: binary not found at $BINARY"
    exit 1
fi

# Strip debug symbols
echo "2) strip ..."
strip "$BINARY"

# Show size
SIZE=$(du -h "$BINARY" | cut -f1)
echo "   Binary: $BINARY"
echo "   Size:   $SIZE"

# Inject into .app if path provided
if [[ -n "${1:-}" ]]; then
    APP_PATH="$1"
    if [[ ! -d "$APP_PATH" ]]; then
        echo "ERROR: App bundle not found: $APP_PATH"
        exit 1
    fi

    RESOURCES_DIR="$APP_PATH/Contents/Resources"
    mkdir -p "$RESOURCES_DIR"
    cp -f "$BINARY" "$RESOURCES_DIR/$BINARY_NAME"
    chmod +x "$RESOURCES_DIR/$BINARY_NAME"
    echo "3) Injected into: $RESOURCES_DIR/$BINARY_NAME"
fi

echo ""
echo "=== Done ==="
