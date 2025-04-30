# RR-OS

RR-OS 是一个完全从零开始编写的 RISC-V 架构实验性操作系统项目，使用 Rust 语言开发。本项目的目标不是构建一个生产级操作系统，而是通过自己实现所有核心组件，深入理解操作系统的工作原理与底层机制。

## 构建与运行

```bash
make -C app bin && make -C os
```

## 项目目标

- ✅ 从零实现 RISC-V 裸机启动流程
- ✅ 实现 SBI 基础调用
- ✅ 实现特权级切换
- ✅ 用户程序加载与执行
- ✅ 批处理任务调度
- ✅ 协作式任务调度
- 🚧 抢占式任务调度

## 参考资料

- [rCore-Tutorial-Book](https://rcore-os.cn/rCore-Tutorial-Book-v3)
