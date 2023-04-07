use piston_window::types::Color;
use piston_window::{clear, Button, PistonWindow, PressEvent, UpdateEvent, WindowSettings};

mod snake_game;
mod snake_snake;
mod snake_window;

use crate::snake_game::game::Game;
use snake_window::draw::to_coord_u32;

/// 定义背景颜色
const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    // 定义窗口大小的参数
    let (width, height) = (30, 30);

    // 定义游戏窗口
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();

    // 创建游戏
    let mut game = Game::new(width, height);

    // 监听窗口输入内容
    while let Some(event) = window.next() {
        // 监听用户输入
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        // 清理当前窗口内容，并重新绘制游戏内容
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g)
        });

        // 更新游戏数据
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
