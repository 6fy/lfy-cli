# 构建 win32-x64 二进制
```bash
brew install mingw-w64
rustup target add x86_64-pc-windows-gnu
pnpm build
```