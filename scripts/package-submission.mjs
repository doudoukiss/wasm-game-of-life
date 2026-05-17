import { spawnSync } from "node:child_process";
import { cp, mkdir, rm, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const projectRoot = path.resolve(fileURLToPath(new URL("..", import.meta.url)));
const submissionRoot = path.join(projectRoot, "submission", "生命演化数学游戏");

function run(command, args) {
  const result = spawnSync(command, args, {
    cwd: projectRoot,
    stdio: "inherit",
    shell: process.platform === "win32",
  });
  if (result.error) {
    console.error(result.error.message);
    process.exit(1);
  }
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

await rm(submissionRoot, { recursive: true, force: true });

run("npm", ["run", "build"]);

await mkdir(submissionRoot, { recursive: true });
await cp(path.join(projectRoot, "dist"), path.join(submissionRoot, "主版本-dist"), {
  recursive: true,
});
await cp(path.join(projectRoot, "fallback"), path.join(submissionRoot, "离线备用版"), {
  recursive: true,
});
await cp(path.join(projectRoot, "docs", "submission"), path.join(submissionRoot, "文档"), {
  recursive: true,
});
await cp(path.join(projectRoot, "README.md"), path.join(submissionRoot, "README.md"));

await mkdir(path.join(submissionRoot, "截图"), { recursive: true });
await mkdir(path.join(submissionRoot, "视频"), { recursive: true });
await mkdir(path.join(submissionRoot, "数据"), { recursive: true });

await writeFile(
  path.join(submissionRoot, "先打开我-运行说明.md"),
  `# 生命演化数学游戏运行说明

## 推荐打开顺序

1. **首选：主版本**
   - 如果有部署链接，优先用 Chrome、Edge 或 Safari 打开部署链接。
   - 如果使用本文件夹内的离线包，请把 \`主版本-dist\` 放到任意静态服务器目录中，通过 HTTP 地址打开。
   - 不建议直接双击 \`主版本-dist/index.html\`，浏览器可能限制 ES module、WASM 或资源加载。

2. **备用：离线 HTML 版**
   - 打开 \`离线备用版/生命演化数学游戏-离线版.html\`。
   - 这个版本不需要 Node、Rust、WebAssembly 或网络，可以直接双击运行。
   - 它保留核心玩法：选关卡、选投放道具、点击棋盘、观察下一帧重新计算。

3. **兜底：视频、截图和数据**
   - 如果现场电脑无法运行浏览器程序，请查看 \`视频\`、\`截图\` 和 \`数据\` 文件夹。
   - 视频用于展示完整互动流程；截图用于展示关键现象；CSV 数据用于证明曲线来自真实计算。

## 本地快速启动主版本

如果电脑已安装 Node.js，可以在项目根目录运行：

\`\`\`bash
npm install
npm run build
npm run preview
\`\`\`

然后打开终端显示的本地地址。若只拿到本提交包，不包含源代码和 node_modules，请使用任意静态文件服务器打开 \`主版本-dist\`，或直接使用离线 HTML 版。

## 浏览器建议

- 推荐：Chrome、Microsoft Edge、Safari 的较新版本。
- 主版本需要浏览器支持 WebAssembly 和 ES module。
- 离线 HTML 版只使用普通 HTML、CSS、Canvas 和 JavaScript，兼容性更稳。
`,
  "utf8",
);

await writeFile(
  path.join(submissionRoot, "截图", "请把关键截图放在这里.txt"),
  "建议截图：发射器投放障碍块、飞船赛道、脉冲星扰动、密度三连曲线、离线备用版运行画面。\n",
  "utf8",
);
await writeFile(
  path.join(submissionRoot, "视频", "请把3分钟以内玩法视频放在这里.txt"),
  "视频建议展示：打开主版本、点击画布投放道具、切换关卡、暂停观察、查看数学旁白；最后展示离线备用版可运行。\n",
  "utf8",
);
await writeFile(
  path.join(submissionRoot, "数据", "请把导出的CSV数据放在这里.txt"),
  "建议从主版本导出至少一份生命曲线 CSV，用于作品说明中的真实数据记录。\n",
  "utf8",
);

console.log(`Submission package created at ${submissionRoot}`);
