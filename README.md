
# STM32F1 Rust Template & Embassy Async Examples

本项目是一个基于 Rust 的 STM32F1（Bluepill）单片机模板，适用于嵌入式开发、硬件学习和异步任务实验。

## 项目结构
- `src/bin/blinky.rs`：LED 闪烁示例
- `src/bin/i2c.rs`：I2C 设备通信示例（如 MPU6050）
- `Cargo.toml`：Rust 项目配置文件
- `build.rs`：构建脚本

## 环境准备
1. 安装 [Rust](https://www.rust-lang.org/)
2. 安装 [rustup](https://rustup.rs/)
3. 添加目标架构：
   ```powershell
   rustup target add thumbv7m-none-eabi
   ```
4. 安装调试/烧录工具：
   ```powershell
   cargo install probe-rs-cli
   ```
5. 推荐 VS Code + Cortex-Debug 插件，支持断点和单步调试。

## 构建方法
```powershell
cargo build --release --target thumbv7m-none-eabi
```

## 烧录方法
使用 probe-rs 或其他支持的工具将生成的固件烧录到 STM32F1 板卡。

示例（使用 probe-rs）：
```powershell
probe-rs download --chip STM32F103C8 --format elf target/thumbv7m-none-eabi/release/<your-binary>
```

## 示例说明

### Blinky 示例
文件：`src/bin/blinky.rs`
功能：板载 LED 闪烁，验证 GPIO 输出和异步定时器。

### I2C 示例（MPU6050）
文件：`src/bin/i2c.rs`
功能：通过 I2C 读取 MPU6050 的 WHO_AM_I 寄存器，验证 I2C 通信。
关键代码：
```rust
let mut whoami = [0u8; 1];
match i2c.blocking_write_read(0x68, &[0x75], &mut whoami) {
    Ok(()) => info!("MPU6050 WHO_AM_I: 0x{:x}", whoami[0]),
    Err(e) => error!("I2c Error: {:?}", e),
}
```

## Bluepill 硬件连接
- ST-Link 调试接口：SWDIO (PA13), SWCLK (PA14), NRST (复位)
- I2C 通信：SCL (PB10), SDA (PB11)，建议加 4.7k~10k 上拉电阻
- 板载 LED：PC13

## Embassy 框架简介
- [Embassy](https://embassy.dev/book/) 是 Rust 异步嵌入式开发框架，支持 async/await，硬件抽象和高效任务调度。
- 本项目使用 `embassy-stm32` 驱动 STM32F1 外设。

## 常见问题排查
- I2C Arbitration 错误：检查设备地址、接线、上拉电阻、模块电源。
- 无法烧录：确认 ST-Link 驱动和连接，或尝试更换 USB 线。
- RTT 无输出：确认 probe-rs rtt 命令和固件已烧录。

## 资料与学习
- [Embassy Book](https://embassy.dev/book/)
- [embassy-examples](https://github.com/embassy-rs/embassy/tree/main/examples)
- [Rust Embedded HAL](https://github.com/rust-embedded/embedded-hal)
- [STM32 Bluepill 原理图](https://wiki.stm32duino.com/index.php?title=Blue_Pill)

## 许可证
MIT
