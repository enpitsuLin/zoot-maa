# ZOOT MAA 助手

<small>ZEROTH ORDER OIL TANK</small>

## 简介

是一个一个 MAA 助手的 GUI 实现，使用 [GPUI](https://github.com/longbridge/gpui) 框架构建，支持跨平台。

> [!NOTE]
> 该项目处于实验阶段，开发尚处于非常早期的阶段。
> GPUI 仍然与 [Zed 编辑器](https://zed.dev/)关联，因此其文档记录不全，而且 API 中经常发生重大变更，可能导致行为发生变化。尽管如此，我还是选择 GPUI 作为 GUI 框架，以便学习 GPUI 并在 Zed 以外的代码库中进行测试。而且，这是一个非常棒的框架，一定要尝试一下！

## 目录结构

```text
├── app/
│   ├─── auto_update # 自动更新相关
│   ├─── global      # 全局常量和一些状态管理
│   ├─── settings    # 设置相关
|   ├─── installer   # MAA 资源下载相关
│   ├─── zoot        # ZOOT MAA 助手主程序包
├─── crates/*        # MAA 的 rust 绑定相关库
├─── xtask           # 构建和发布相关的 xtask 任务
│
```

## 开发

### 运行 maa-sys 测试

运行 `maa-sys` crate 的测试需要设置以下环境变量：

| 环境变量 | 说明 | 示例 |
|---------|------|------|
| `MAA_LIB_PATH` | MaaCore 库文件所在目录或完整路径 | `maa-include/build/bin/Debug` |
| `MAA_RESOURCE_PATH` | 资源文件**父目录**（包含 `resource` 子目录的目录） | `maa-include` |

**注意事项：**
- `MAA_RESOURCE_PATH` 必须指向包含 `resource` 子目录的父目录，而不是直接指向 `resource` 目录本身
- 运行测试时还需要设置 `LD_LIBRARY_PATH` 以加载依赖库

**VS Code 配置示例：**

在项目根目录的 `.vscode/settings.json` 中添加：

```json
{
  "rust-analyzer.runnables.extraEnv": {
    "MAA_RESOURCE_PATH": "${workspaceFolder}/maa-include",
    "MAA_LIB_PATH": "${workspaceFolder}/maa-include/build/bin/Debug"
  }
}
```

**命令行运行测试：**

```bash
export MAA_RESOURCE_PATH="/path/to/maa-include"
export MAA_LIB_PATH="/path/to/maa-include/build/bin/Debug"
export LD_LIBRARY_PATH="/path/to/maa-include/src/MaaUtils/MaaDeps/vcpkg/installed/maa-x64-linux/lib:/path/to/maa-include/src/MaaUtils/MaaDeps/x-tools/x86_64-linux-gnu/x86_64-linux-gnu/sysroot/usr/lib:$LD_LIBRARY_PATH"

cargo test --package maa-sys
```

# License

MIT
