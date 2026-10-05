# sssrs

sssrs 是 [sing-box](https://github.com/SagerNet/sing-box) 的 **Rust 重写**版本。

上游参照：<https://github.com/SagerNet/sing-box>（跟踪 `testing` 分支），
许可证沿用上游的 **GPL-3.0-or-later**。

> 状态：**P0 骨架**。当前仓库只有可编译的 workspace 骨架、最小可用的 CLI 与 CI，
> 协议、传输、路由等实现尚未开始。

## 当前状态

| 阶段 | 内容 | 状态 |
| --- | --- | --- |
| P0 | Rust workspace 骨架、crate 布局、CLI 占位、CI 全绿 | 进行中 |
| P1 | 配置模型与常量层 | 未开始 |
| P2 | 适配器 / 传输 / 协议层 | 未开始 |
| P3 | DNS / 路由 / 核心运行时 | 未开始 |
| P4 | 与 sing-box 配置的兼容性收敛 | 未开始 |

## 配置兼容性目标

ssssrs 的目标是**读取上游 sing-box 的 JSON 配置文件**，因此：

* 配置文件的字段命名、层级结构、默认值与上游 `testing` 分支保持一致；
* 解析失败时给出明确的错误信息（文件不存在 / JSON 语法错误 / 字段非法）；
* 不引入 sssrs 专有的配置方言，差异部分会在文档中显式标注。

## 构建方式

要求 Rust stable（见 `rust-toolchain.toml`）。

```bash
# 构建整个 workspace
cargo build --workspace

# 构建 release 版 CLI
cargo build --release -p sb-cli

# 运行 CLI
./target/release/ssssrs version
./target/release/ssssrs check -c config.json
./target/release/ssssrs format -c config.json
./target/release/ssssrs run -c config.json

# 格式化检查
cargo fmt --all -- --check
```

CLI 子命令当前均为占位实现：

* `version`：打印 sssrs 版本与构建所用 rustc 版本；
* `check`：检查配置文件是否存在并做 JSON 语法解析；
* `format`：同上，暂不写回文件；
* `run`：先校验配置文件，随后以 `not implemented yet` 退出（退出码 1）。

## crate 布局

| crate | 说明 | 依赖 |
| --- | --- | --- |
| `sb-constant` | 共享常量 | — |
| `sb-option` | 配置模型 | `sb-constant` |
| `sb-common` | 通用工具 | `sb-constant` |
| `sb-adapter` | 适配器 trait | `sb-constant` `sb-common` `sb-option` |
| `sb-transport` | 传输层 | `sb-constant` `sb-common` `sb-adapter` |
| `sb-protocol` | 协议层 | `sb-constant` `sb-common` `sb-adapter` `sb-option` `sb-transport` |
| `sb-v2ray` | V2Ray 兼容层 | `sb-constant` `sb-common` `sb-adapter` `sb-option` `sb-transport` |
| `sb-dns` | DNS 子系统 | `sb-constant` `sb-common` `sb-adapter` `sb-option` |
| `sb-route` | 路由引擎 | `sb-constant` `sb-common` `sb-adapter` `sb-option` `sb-dns` |
| `sb-experimental` | 实验特性 | `sb-constant` `sb-common` `sb-adapter` `sb-option` |
| `sb-core` | 核心运行时 | 以上全部 |
| `sb-cli` | 命令行入口（二进制 `ssssrs`） | `sb-core` `sb-option` `sb-constant` `sb-common` |

## CI

<!-- CI 状态徽章占位：仓库公开后替换为实际 workflow 名称 -->
![CI](https://github.com/Acheirial/ssssrs/actions/workflows/ci.yml/badge.svg)

CI 在 `ubuntu-latest` 上运行四个 job：`fmt`、`clippy`、`test`、`smoke`。
本地不执行编译与测试，所有编译/测试验证均以 GitHub Actions 结果为准。

## 许可证

GPL-3.0-or-later，见 [LICENSE](LICENSE)。上游 sing-box 的版权声明见该文件。
