#!/usr/bin/env node

import { execFileSync } from "node:child_process";
import { createRequire } from "node:module";
import os from "node:os";
import { join } from "node:path";

const require = createRequire(import.meta.url);

function getPlatformPackage() {
  const platform = os.platform();
  const arch = os.arch();

  const platformMap = {
    "darwin-arm64": "@6fy/cli-darwin-arm64",
    "darwin-x64": "@6fy/cli-darwin-x64",
    "linux-x64": "@6fy/cli-linux-x64",
    "win32-x64": "@6fy/cli-win32-x64",
  };

  const key = `${platform}-${arch}`;
  const pkg = platformMap[key];

  if (!pkg) {
    console.error(
      `Error: unsupported platform ${platform}-${arch}.\n` +
        `Supported platforms: ${Object.keys(platformMap).join(", ")}`,
    );
    process.exit(1);
  }

  return pkg;
}

function getBinaryPath() {
  const pkg = getPlatformPackage();
  const binaryName = os.platform() === "win32" ? "lfy-cli.exe" : "lfy-cli";

  try {
    const pkgDir = require.resolve(`${pkg}/package.json`);
    return join(pkgDir, "..", "bin", binaryName);
  } catch {
    console.error(
      `Error: cannot find @6fy/cli binary.\n` +
        `Please try reinstalling: npm install @6fy/cli\n\n` +
        `If the problem persists, check:\n` +
        `  1. Your npm config does not disable optional dependencies (--no-optional)\n` +
        `  2. Your platform (${os.platform()}-${os.arch()}) is supported`,
    );
    process.exit(1);
  }
}

const binaryPath = getBinaryPath();

try {
  execFileSync(binaryPath, process.argv.slice(2), {
    stdio: "inherit",
    env: process.env,
  });
} catch (error) {
  if (error.status != null) {
    process.exit(error.status);
  }
}
