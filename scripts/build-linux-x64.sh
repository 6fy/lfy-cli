#!/bin/bash
# 构建 linux-x64 二进制并复制到 packages/linux-x64/bin
#
# 说明：macOS 上交叉编译 linux 往往需要额外的工具链/系统库，因此这里使用 Docker 在容器内编译。

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
PACKAGE_DIR="$ROOT_DIR/packages/linux-x64"
BINARY_PATH="$PACKAGE_DIR/bin/lfy-cli"

echo "=== 构建 linux-x64 二进制（docker）==="

if ! command -v docker >/dev/null 2>&1; then
  echo "Error: docker 未安装或不可用"
  exit 1
fi

DOCKER_IMAGE="${DOCKER_IMAGE:-rust:1-bullseye}"

cd "$ROOT_DIR"

HOST_ARCH="$(uname -m)"
DOCKER_PLATFORM=""
if [ "$HOST_ARCH" = "arm64" ] || [ "$HOST_ARCH" = "aarch64" ]; then
  # 目标是 linux-x64：在 Apple Silicon 上需要拉起 amd64 容器（依赖 Docker 的多架构支持）
  DOCKER_PLATFORM="--platform linux/amd64"
fi

docker run --rm $DOCKER_PLATFORM \
  -v "$PWD":/work \
  -v "$HOME/.cargo":/root/.cargo \
  -w /work \
  "$DOCKER_IMAGE" \
  bash -lc "set -e; export PATH=/usr/local/cargo/bin:\$PATH; \
    export CARGO_HTTP_MULTIPLEXING=false; \
    export CARGO_HTTP_TIMEOUT=600; \
    export CARGO_NET_RETRY=10; \
    apt-get update -y >/dev/null 2>&1 || true; \
    apt-get install -y build-essential pkg-config ca-certificates >/dev/null 2>&1 || true; \
    cargo build --release --locked"

TARGET_BINARY="$ROOT_DIR/target/release/lfy-cli"
mkdir -p "$PACKAGE_DIR/bin"
cp "$TARGET_BINARY" "$BINARY_PATH"
chmod +x "$BINARY_PATH"

FILE_SIZE=$(stat -f%z "$BINARY_PATH" 2>/dev/null || stat -c%s "$BINARY_PATH" 2>/dev/null)
echo "=== Done ==="
echo "二进制文件: $BINARY_PATH"
echo "文件大小: $FILE_SIZE bytes"
