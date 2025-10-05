#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::i2c::{Error, I2c};
use embassy_stm32::{bind_interrupts, i2c, peripherals};
use embassy_time::Timer;
use libm::{atan2f, sqrtf};
use {defmt_rtt as _, panic_probe as _};

const MPU6050_ADDR: u8 = 0x68;
const WHO_AM_I: u8 = 0x75;
const PWR_MGMT_1: u8 = 0x6B;
const ACCEL_XOUT_H: u8 = 0x3B;
const GYRO_XOUT_H: u8 = 0x43;
const GYRO_CONFIG: u8 = 0x1B;
const ACCEL_CONFIG: u8 = 0x1C;

bind_interrupts!(struct Irqs {
    I2C2_EV => i2c::EventInterruptHandler<peripherals::I2C2>;
    I2C2_ER => i2c::ErrorInterruptHandler<peripherals::I2C2>;
});

// 顶层初始化函数：接受两个闭包用于 write 和 write_read，这样可以把 init 与具体 I2C 类型分离
fn mpu6050_init<F, E>(ops: F) -> Result<u8, E>
where
    F: FnOnce() -> Result<u8, E>,
{
    ops()
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello I2C Demo");
    let p = embassy_stm32::init(Default::default());

    let mut i2c = I2c::new(
        p.I2C2,
        p.PB10,
        p.PB11,
        Irqs,
        p.DMA1_CH4,
        p.DMA1_CH5,
        Default::default(),
    );
    // 使用通用的顶层初始化函数，传入两个闭包以调用具体的 i2c 方法
    let who = mpu6050_init(|| -> Result<u8, Error> {
        i2c.blocking_write(MPU6050_ADDR, &[PWR_MGMT_1, 0x00])?;
        i2c.blocking_write(MPU6050_ADDR, &[GYRO_CONFIG, 0x00])?;
        i2c.blocking_write(MPU6050_ADDR, &[ACCEL_CONFIG, 0x00])?;
        let mut who = [0u8; 1];
        i2c.blocking_write_read(MPU6050_ADDR, &[WHO_AM_I], &mut who)?;
        Ok(who[0])
    });

    match who {
        Ok(w) => info!("MPU6050 WHO_AM_I: 0x{:X}", w),
        Err(Error::Timeout) => error!("Operation timed out during init"),
        Err(e) => error!("I2C init error: {:?}", e),
    }

    info!("开始循环读取加速度计和陀螺仪数据...");

    // 循环读取加速度计和陀螺仪数据
    // complementary filter state
    let mut roll: f32 = 0.0;
    let mut pitch: f32 = 0.0;
    // accel-derived angles (updated each accel read)
    let mut roll_acc: f32 = 0.0;
    let mut pitch_acc: f32 = 0.0;
    let alpha: f32 = 0.98; // 融合系数
    let dt: f32 = 0.5; // 500 ms

    let mut first = true;

    loop {
        // 读取加速度计数据 (6字节: X_H, X_L, Y_H, Y_L, Z_H, Z_L)
        let mut accel_data = [0u8; 6];
        match i2c.blocking_write_read(MPU6050_ADDR, &[ACCEL_XOUT_H], &mut accel_data) {
            Ok(()) => {
                // 将高低字节组合成 16 位有符号整数
                let accel_x = i16::from_be_bytes([accel_data[0], accel_data[1]]);
                let accel_y = i16::from_be_bytes([accel_data[2], accel_data[3]]);
                let accel_z = i16::from_be_bytes([accel_data[4], accel_data[5]]);

                // 转换为 g (重力加速度，假设量程为 ±2g, LSB = 16384)
                let accel_x_g = accel_x as f32 / 16384.0;
                let accel_y_g = accel_y as f32 / 16384.0;
                let accel_z_g = accel_z as f32 / 16384.0;

                info!("加速度: X={}g, Y={}g, Z={}g", accel_x_g, accel_y_g, accel_z_g);

                // 使用加速度计计算初始 roll/pitch（以度为单位）
                // roll = atan2(Ay, Az)
                // pitch = atan2(-Ax, sqrt(Ay^2 + Az^2))
                roll_acc = atan2f(accel_y_g, accel_z_g) * 57.29577951308232_f32; // rad->deg
                pitch_acc = atan2f(-accel_x_g, sqrtf(accel_y_g * accel_y_g + accel_z_g * accel_z_g)) * 57.29577951308232_f32;

                if first {
                    roll = roll_acc;
                    pitch = pitch_acc;
                    first = false;
                }
            },
            Err(e) => error!("加速度计读取失败: {:?}", e),
        }

        // 读取陀螺仪数据
        let mut gyro_data = [0u8; 6];
        match i2c.blocking_write_read(MPU6050_ADDR, &[GYRO_XOUT_H], &mut gyro_data) {
            Ok(()) => {
                // 将高低字节组合成 16 位有符号整数
                let gyro_x = i16::from_be_bytes([gyro_data[0], gyro_data[1]]);
                let gyro_y = i16::from_be_bytes([gyro_data[2], gyro_data[3]]);
                let gyro_z = i16::from_be_bytes([gyro_data[4], gyro_data[5]]);

                // 转换为度/秒 (假设量程为 ±250°/s, LSB = 131)
                let gyro_x_dps = gyro_x as f32 / 131.0;
                let gyro_y_dps = gyro_y as f32 / 131.0;
                let gyro_z_dps = gyro_z as f32 / 131.0;

            info!("陀螺仪: X={}°/s, Y={}°/s, Z={}°/s",
                gyro_x_dps, gyro_y_dps, gyro_z_dps);

            // 用陀螺仪积分增量角度（注意：这里只是简单示例，未处理陀螺仪偏置）
            let delta_roll = gyro_x_dps * dt; // 500 ms
            let delta_pitch = gyro_y_dps * dt;

            // 互补滤波融合陀螺仪与加速度计
            roll = alpha * (roll + delta_roll) + (1.0 - alpha) * roll_acc;
            pitch = alpha * (pitch + delta_pitch) + (1.0 - alpha) * pitch_acc;

            // 打印姿态角
            let roll_scaled = (roll * 100.0) as i32;
            let pitch_scaled = (pitch * 100.0) as i32;
            info!("姿态: roll={}.{:02}°, pitch={}.{:02}°",
                roll_scaled / 100, (roll_scaled.abs() % 100),
                pitch_scaled / 100, (pitch_scaled.abs() % 100));
            },
            Err(e) => error!("陀螺仪读取失败: {:?}", e),
        }

        info!("---");

        // 等待 500ms 再读取下一次
        Timer::after_millis(500).await;
    }
}
