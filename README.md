# Rust 实现的贪吃蛇

## 简介
一个非常简单的小项目。

通过上下左右四个方向键进行控制。

按 P 暂停/启动游戏，按 R 重置游戏。

## 代码结构
```
·
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├──snake_game/
│   │  ├── game.rs
│   │  └── mod.rs
│   ├──snake_snake/
│   │   ├── snake.rs
│   │   └── mod.rs
│   └──snake_window/
│       ├──draw.rs
│       └── mod.rs
```
- game.rs 负责游戏的逻辑、控制、等内容。
- snake.rs 负责蛇本身的实现。
- draw.rs 负责图形化的二次封装。

## about me 
目前失业，在家学习 rust 。

我的 [bilibili](https://space.bilibili.com/259260787),我的 [博客园](https://www.cnblogs.com/SantiagoZhang/)。
