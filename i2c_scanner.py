#!/usr/bin/env python3
"""
I2C MPU6050 WHO_AM_I 读取脚本
用于验证 I2C 通信和 WHO_AM_I 寄存器值
需要安装: pip install smbus2
"""

import time
import smbus2

# 配置参数
I2C_BUS = 1  # 树莓派默认 I2C 总线，Windows/Linux 可能需要调整
MPU6050_ADDR = 0x68
WHO_AM_I_REG = 0x75

def scan_i2c_devices(bus):
    """扫描 I2C 总线上的设备"""
    print("正在扫描 I2C 总线...")
    devices = []
    for addr in range(0x03, 0x78):  # 常见 I2C 地址范围
        try:
            bus.read_byte(addr)
            devices.append(addr)
            print(f"发现设备: 0x{addr:02X}")
        except:
            pass
    return devices

def read_who_am_i(bus, device_addr):
    """读取设备的 WHO_AM_I 寄存器"""
    try:
        # 写寄存器地址，然后读取数据
        who_am_i = bus.read_byte_data(device_addr, WHO_AM_I_REG)
        return who_am_i
    except Exception as e:
        print(f"读取失败: {e}")
        return None

def main():
    try:
        # 初始化 I2C 总线
        bus = smbus2.SMBus(I2C_BUS)
        print(f"已连接到 I2C 总线 {I2C_BUS}")

        # 扫描设备
        devices = scan_i2c_devices(bus)
        if not devices:
            print("未发现任何 I2C 设备")
            return

        print(f"\n发现 {len(devices)} 个设备")

        # 尝试读取 MPU6050
        if MPU6050_ADDR in devices:
            print(f"\n正在读取地址 0x{MPU6050_ADDR:02X} 的 WHO_AM_I...")
            who_am_i = read_who_am_i(bus, MPU6050_ADDR)
            if who_am_i is not None:
                print(f"WHO_AM_I: 0x{who_am_i:02X} ({who_am_i})")
                print(f"二进制: {who_am_i:08b}")

                # 分析结果
                if who_am_i == 0x68:
                    print("✓ 标准 MPU6050")
                elif who_am_i == 0x70:
                    print("! 可能是兼容芯片或不同批次")
                else:
                    print("? 未知设备或非 MPU6050")
        else:
            print(f"地址 0x{MPU6050_ADDR:02X} 未响应，尝试 0x69...")
            # 尝试另一个地址
            alt_addr = 0x69
            if alt_addr in devices:
                who_am_i = read_who_am_i(bus, alt_addr)
                if who_am_i is not None:
                    print(f"地址 0x{alt_addr:02X} - WHO_AM_I: 0x{who_am_i:02X}")

        # 读取所有发现的设备
        print(f"\n尝试读取所有设备的寄存器 0x{WHO_AM_I_REG:02X}:")
        for addr in devices:
            who_am_i = read_who_am_i(bus, addr)
            if who_am_i is not None:
                print(f"地址 0x{addr:02X}: 0x{who_am_i:02X} ({who_am_i:08b})")

        bus.close()

    except FileNotFoundError:
        print("错误: 无法访问 I2C 设备")
        print("请确认:")
        print("1. I2C 已启用")
        print("2. 有足够权限 (可能需要 sudo)")
        print("3. 设备已正确连接")
    except Exception as e:
        print(f"错误: {e}")

if __name__ == "__main__":
    main()
