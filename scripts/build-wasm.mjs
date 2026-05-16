import { spawnSync } from "node:child_process";

const args = ["build", "--target", "web", "--out-dir", "pkg", "--release"];
const result = spawnSync("wasm-pack", args, {
  stdio: "inherit",
  shell: process.platform === "win32",
});

if (result.error?.code === "ENOENT") {
  console.error(
    [
      "wasm-pack was not found.",
      "Run `npm install` to use the project-local wasm-pack package, or install it manually:",
      "  cargo install wasm-pack",
      "Then rerun `npm run build:wasm`.",
    ].join("\n"),
  );
  process.exit(1);
}

if (result.error) {
  console.error(result.error.message);
  process.exit(1);
}

process.exit(result.status ?? 1);
