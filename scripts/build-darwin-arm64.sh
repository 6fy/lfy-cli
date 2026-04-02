#!/bin/bash
# 构建 darwin-arm64 二进制并复制到 packages/darwin-arm64/bin

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
PACKAGE_DIR="$ROOT_DIR/packages/darwin-arm64"
BINARY_PATH="$PACKAGE_DIR/bin/lfy-cli"

echo "=== 构建 darwin-arm64 二进制 ==="

# 安装目标（只需一次）
rustup target list | grep -q "aarch64-apple-darwin (installed)" || {
  echo "安装 rust 目标：aarch64-apple-darwin ..."
  rustup target add aarch64-apple-darwin
}

cd "$ROOT_DIR"
cargo build --release --target aarch64-apple-darwin

TARGET_BINARY="$ROOT_DIR/target/aarch64-apple-darwin/release/lfy-cli"
mkdir -p "$PACKAGE_DIR/bin"
cp "$TARGET_BINARY" "$BINARY_PATH"
chmod +x "$BINARY_PATH"

FILE_SIZE=$(stat -f%z "$BINARY_PATH" 2>/dev/null || stat -c%s "$BINARY_PATH" 2>/dev/null)
echo "=== Done ==="
echo "二进制文件: $BINARY_PATH"
echo "文件大小: $FILE_SIZE bytes"

