#!/bin/bash
# 构建 win32-x64 二进制并复制到 packages 目录
#
# 依赖（在 macOS 上交叉编译 Windows GNU 目标）：
#   brew install mingw-w64
#   rustup target add x86_64-pc-windows-gnu

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
PACKAGE_DIR="$ROOT_DIR/packages/win32-x64"
BINARY_PATH="$PACKAGE_DIR/bin/lfy-cli.exe"

echo "=== 构建 Windows x64 二进制 ==="

# MinGW 工具链（aws-lc-sys / cc-rs 编译 C 代码必需）
MINGW_GCC="${MINGW_GCC:-x86_64-w64-mingw32-gcc}"
if ! command -v "$MINGW_GCC" >/dev/null 2>&1; then
    echo "Error: 未找到 $MINGW_GCC（MinGW-w64）"
    echo ""
    echo "在 macOS 上请安装："
    echo "  brew install mingw-w64"
    echo ""
    echo "然后确认 PATH 中能执行："
    echo "  $MINGW_GCC --version"
    exit 1
fi

# 供 aws-lc-sys / cc-rs 交叉编译 C 代码（与 CARGO_TARGET_* 命名一致）
export CC_x86_64_pc_windows_gnu="$MINGW_GCC"
export CXX_x86_64_pc_windows_gnu="${MINGW_GCC%gcc}g++"
export AR_x86_64_pc_windows_gnu="${MINGW_GCC%gcc}ar"

# 检查是否已安装 x86_64-pc-windows-gnu 目标
rustup target list | grep -q "x86_64-pc-windows-gnu (installed)" || {
    echo "安装交叉编译目标 x86_64-pc-windows-gnu ..."
    rustup target add x86_64-pc-windows-gnu
}

# 构建 Windows x64 二进制
echo "正在构建 Windows x64 二进制 ..."
cd "$ROOT_DIR"
cargo build --release --target x86_64-pc-windows-gnu

# 复制二进制到 packages 目录
TARGET_BINARY="$ROOT_DIR/target/x86_64-pc-windows-gnu/release/lfy-cli.exe"
mkdir -p "$PACKAGE_DIR/bin"
cp "$TARGET_BINARY" "$BINARY_PATH"

# 检查二进制大小
FILE_SIZE=$(stat -f%z "$BINARY_PATH" 2>/dev/null || stat -c%s "$BINARY_PATH" 2>/dev/null)
echo ""
echo "=== 构建完成 ==="
echo "二进制文件: $BINARY_PATH"
echo "文件大小: $FILE_SIZE bytes"
