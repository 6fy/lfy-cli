#!/bin/bash

echo "=================================="
echo "   lfy-cli Installer for macOS    "
echo "=================================="

MAX_RETRY=3

retry() {
    local count=1

    while true
    do
        "$@"
        local status=$?

        if [ $status -eq 0 ]; then
            return 0
        fi

        if [ $count -ge $MAX_RETRY ]; then
            echo
            echo "❌ 命令执行失败，已重试 ${MAX_RETRY} 次"
            echo "命令: $*"
            exit 1
        fi

        echo
        echo "⚠️ 执行失败，正在进行第 ${count} 次重试"

        count=$((count + 1))

        sleep 3
    done
}

# macOS 检测
if [[ "$(uname)" != "Darwin" ]]; then
    echo "❌ 当前系统不是 macOS"
    exit 1
fi

echo "✅ macOS 检测通过"

# sudo 权限
echo
echo "🔐 检查管理员权限..."

sudo -v

if [ $? -ne 0 ]; then
    echo "❌ sudo 权限获取失败"
    exit 1
fi

echo "✅ sudo 权限验证成功"

# Homebrew
if ! command -v brew >/dev/null 2>&1; then

    echo
    echo "📦 Homebrew 未安装，开始安装..."

    retry bash -c 'NONINTERACTIVE=1 /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)" </dev/null'

    # 初始化 brew 环境
    if [[ -f "/opt/homebrew/bin/brew" ]]; then
        eval "$(/opt/homebrew/bin/brew shellenv)"
    fi

    if [[ -f "/usr/local/bin/brew" ]]; then
        eval "$(/usr/local/bin/brew shellenv)"
    fi
fi

# 再次检查 brew
if ! command -v brew >/dev/null 2>&1; then
    echo "❌ Homebrew 安装失败"
    exit 1
fi

echo "✅ brew 版本: $(brew --version | head -n 1)"

# Node.js
if ! command -v node >/dev/null 2>&1; then

    echo
    echo "📦 Node.js 未安装，开始安装最新版 Node.js..."

    retry brew install node

    if [ $? -ne 0 ]; then
        echo "❌ Node.js 安装失败"
        exit 1
    fi
fi

echo "✅ Node.js 版本: $(node -v)"
echo "✅ npm 版本: $(npm -v)"

# 安装 CLI
echo
echo "📦 安装 @6fy/cli ..."

retry npm install -g @6fy/cli

if [ $? -ne 0 ]; then
    echo "❌ @6fy/cli 安装失败"
    exit 1
fi

# 动态获取 npm global 路径
NPM_PREFIX=$(npm prefix -g)
NPM_BIN="$NPM_PREFIX/bin"

# 当前 shell 生效
export PATH="$NPM_BIN:$PATH"

# 刷新 shell 缓存
hash -r

echo "✅ npm prefix: $NPM_PREFIX"
echo "✅ npm bin: $NPM_BIN"

# 永久写入 PATH
PROFILE_FILE="$HOME/.zprofile"
PATH_LINE="export PATH=\"$NPM_BIN:\$PATH\""

if ! grep -Fq "$PATH_LINE" "$PROFILE_FILE" 2>/dev/null; then

    echo "" >> "$PROFILE_FILE"
    echo "# lfy-cli" >> "$PROFILE_FILE"
    echo "$PATH_LINE" >> "$PROFILE_FILE"

    echo "✅ 已写入 PATH 到 $PROFILE_FILE"
fi

# 检测 lfy-cli
if ! command -v lfy-cli >/dev/null 2>&1; then

    echo
    echo "❌ 未找到 lfy-cli 命令"

    echo
    echo "当前 PATH:"
    echo "$PATH"

    echo
    echo "查找 lfy-cli:"
    find "$NPM_PREFIX" -name lfy-cli 2>/dev/null

    exit 1
fi

echo "✅ lfy-cli 安装成功"

# upgrade
echo
echo "⬆️ 执行 lfy-cli upgrade ..."

retry lfy-cli upgrade

# 安装 skills
echo
echo "🛠️ 安装 skills ..."

retry npx skills add https://gitee.com/lfy-team/lfy-cli.git -y -g

echo
echo "=================================="
echo "         安装完成"
echo "=================================="

echo
echo "📱 当前设备信息："
echo "----------------------------------"

retry lfy-cli status

echo "----------------------------------"
echo "🎉 初始化完成"

echo
echo "⚠️ 请重新打开终端后再使用 lfy-cli"
