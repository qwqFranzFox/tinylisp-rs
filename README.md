# tinylisp-rs

基于 Rust 与树莓派 Pico2 的嵌入式 Lisp 解释器。

在资源受限的微控制器上实现一个可交互的 Lisp REPL，通过串口与用户交互，探索 `no_std` 环境下的语言运行时设计。

> 当前支持硬件：树莓派 Pico 2、ESP32-C6  
> 项目状态：个人项目，持续完善中

---

## ✨ 特性

- 完整的 REPL 交互循环，支持通过串口输入表达式并查看求值结果
- 独立实现的词法分析器、Eval-Apply元循环求值器
- 支持变量绑定、函数定义与调用、基础列表操作（如 `car`、`cdr`、`cons`）
- (Pico 2 Only)使用双核架构：一个核处理串口 I/O，另一个核执行 Eval-Apply 循环，避免输入输出阻塞解释器
- 模块化设计，核心解释器与硬件相关代码解耦，可在宿主机上单独测试
- 已成功迁移至 [ESP32-C6](https://github.com/qwqFranzFox/tinylisp-esp32c6)，验证了可移植性

> ESP32C6移植使用官方脚手架创建新项目，因此和主线不在同一仓库。
---

## 🛠 硬件支持

| 开发板 | 状态 | 备注 |
|--------|------|------|
| Raspberry Pi Pico 2 | ✅ 已运行 | 主要开发平台，使用 rpi-hal |
| [ESP32-C6](https://github.com/qwqFranzFox/tinylisp-esp32c6) | ✅ 已迁移 | 通过 esp-hal 适配，验证跨平台能力 |

---

## 环境要求

- Rust 工具链
- 目标平台支持：`thumbv8m.main-none-eabihf`（Pico 2）或 `riscv32imac-unknown-none-elf`（ESP32-C6）
- 串口工具，如 `minicom`、`screen` 或 `picocom`

## 路线图
- [ ] 增加更多内置函数（如 map、filter、apply）
- [ ] 支持简单的宏系统
- [x] 优化内存分配策略，支持更大的 Lisp 对象
- [x] 提供一个宿主机版本，方便调试和教学演示
- [ ] 添加更完整的错误报告和行号信息

## 致谢
本项目的研究动力来源于MIT的[SICP](https://web.mit.edu/6.001/6.037/)课程，以及我先前翻译的潘润宇老师的一篇论文 "Predictable Virtualization on Memory Protection Unit-based Microcontrollers"（[原文](https://ieeexplore.ieee.org/document/8430066)及[个人翻译的typst源代码](https://github.com/qwqFranzFox/pan18mpu-translate)）

核心部分的元循环求值器参考了[tinylisp](https://github.com/Robert-van-Engelen/tinylisp)。
