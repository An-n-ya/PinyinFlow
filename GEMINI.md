## 项目描述
PinyinFlow 是一个跨平台的 text-to-voice 无障碍工具，旨在帮助发声障碍用户（如听障人士、喉癌患者、口吃患者）通过打字发出声音，实现顺畅交流。
该项目基于 Tauri 框架构建，包含前端界面和后端服务。

## 技术栈
- **前端**: React, TypeScript, Material UI (MUI)
- **后端**: Rust, Tauri
- **音频处理**: rodio (PCM 播放)
- **通信**: WebSocket (与本地语音合成服务通信), HTTP (与本地音调服务通信)

## 开发指令
```bash
./scripts/run.sh -s        # 启动本地语音合成服务
./scripts/run.sh -r        # 启动 Tauri 桌面应用
./scripts/log.sh -l        # 启动 lnav 日志查看器
pnpm format                # 格式化代码
pnpm test                  # 运行测试
```

## 项目架构
### 前端代码 (src/)
- `App.tsx`: 应用入口，配置 MUI 主题和布局。
- `components/chat/`: 聊天相关组件。
    - `Chat.tsx`: 核心业务逻辑，处理消息发送、播放状态及事件监听。
    - `InputArea.tsx`: 用户输入区域。
    - `ChatHistory.tsx`: 消息历史展示。
- `services/`: 前端服务，如 LLM 交互逻辑。

### 后端代码 (src-tauri/src/)
- `main.rs` & `lib.rs`: Tauri 应用入口及初始化配置。
- `commands.rs`: 定义前端可调用的 Tauri 命令（如 `play`, `split`, `tone`）。
- `device/`: 服务与硬件交互模块。
    - `audio.rs`: 音频播放服务，处理 PCM 字节流并使用 `rodio` 播放。
    - `websocket.rs`: WebSocket 客户端，管理与语音合成后端（通常在 localhost:8000）的连接，处理文本发送及二进制音频数据接收。
    - `frontend.rs`: 前端事件分发器，负责将后端事件（如 `TTSFinished`, `AudioPlayed`）推送到 React 界面。

## 开发约定
- **错误处理**: Rust 后端优先使用 `anyhow` 和 `anyhow-tauri` 进行错误包装。
- **状态同步**: 前端通过 `listen` 监听后端发送的音频播放状态事件，以同步 UI 表现。
- **代码风格**: 使用 `pnpm format` 进行代码格式化，且代码中不应包含注释。
