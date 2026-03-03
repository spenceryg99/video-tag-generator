# Global Rule

- 默认使用中文沟通。
- 当前项目是桌面端 Tauri 应用：前端 Vue 3 + TypeScript + Pinia + Naive UI，后端 Rust。
- 修改时优先聚焦 `src/` 与 `src-tauri/src/`，不要误改图标资源文件（`app-icon.png`、`src-tauri/icons/**`）。

## 项目初始化记忆

### 1. 业务目标
- 输入本地视频文件，输出中文社媒文案和标签。
- 当前主流程：选视频 -> 上传 DashScope OSS -> 多模态分析 -> 解析结果 -> 复制文案/标签。

### 2. 关键技术与入口
- 前端入口：`src/App.vue`
- 前端状态：`src/stores/settings.ts`、`src/stores/processing.ts`
- Tauri 命令封装：`src/utils/commands.ts`
- Rust 入口：`src-tauri/src/lib.rs`
- 核心命令：`src-tauri/src/commands/video.rs`
- DashScope 上传：`src-tauri/src/dashscope/upload.rs`
- DashScope 分析与解析：`src-tauri/src/dashscope/client.rs`

### 3. 本地开发命令
- 前端开发：`npm run dev`
- 前端构建：`npm run build`
- 桌面联调：`npm run tauri dev`

### 4. 现状与约束
- `API Key` 由前端 store 持久化，并通过 Tauri 命令注入 Rust 进程内存。
- `fps` 目前仅前端保存/展示，尚未接入后端实际抽帧或采样流程。
- 结果解析依赖“文案/标签”关键词与分隔符，模型输出格式漂移时可能解析不稳定。

### 5. 后续改动优先级（建议）
- P1：将 `fps` 真正接入视频采样/抽帧逻辑。
- P1：将模型输出约束为更稳定的结构（优先 JSON），降低解析脆弱性。
- P1：加强密钥存储安全，减少明文持久化风险。
- P2：补充失败分类与重试策略（上传失败、限流、超时等）。
- P2：建立最小可用测试（解析逻辑与关键命令链路）。
