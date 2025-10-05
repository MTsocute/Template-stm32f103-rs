# STM32F1 Rust Template

本项目是一个基于 Rust 的 STM32F1 单片机模板，适用于嵌入式开发和快速原型设计。

## 项目结构
- `src/bin/blinky.rs`：示例 Blinky 程序
- `Cargo.toml`：Rust 项目配置文件
- `build.rs`：构建脚本

## 环境准备
1. 安装 [Rust](https://www.rust-lang.org/)
2. 安装 [rustup](https://rustup.rs/)
3. 添加目标架构：
   ```powershell
   rustup target add thumbv7m-none-eabi
   ```
4. 推荐安装 [cargo-binutils](https://github.com/rust-embedded/cargo-binutils) 和 [probe-rs](https://probe.rs/)

## 构建方法
```powershell
cargo build --release --target thumbv7m-none-eabi
```

## 烧录方法
使用 probe-rs 或其他支持的工具将生成的固件烧录到 STM32F1 板卡。

示例（使用 probe-rs）：
```powershell
cargo install probe-rs-cli
probe-rs download --chip STM32F103C8 --format elf target/thumbv7m-none-eabi/release/<your-binary>
```

## 示例
Blinky 示例位于 `src/bin/blinky.rs`，用于演示 LED 闪烁。

## 许可证
MIT
