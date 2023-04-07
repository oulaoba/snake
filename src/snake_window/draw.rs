use piston_window::rectangle::Shape;
use piston_window::types::Color;
use piston_window::{rectangle, Context, DrawState, G2d, Rectangle};

/// 定义块的大小
const BLOCK_SIZE: f64 = 20.0;

/// 将 i32 转为 f64
pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

/// 将 i32 转为 u32
pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

/// 块图形绘制
/// * shape : piston_window::rectangle::Shape
pub fn draw_block(color: Color, shape: Shape, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let rec = Rectangle::new(color).color(color).shape(shape);
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);
    let rectangle = [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE];
    rec.draw(rectangle, &DrawState::default(), con.transform, g)
}

/// 长方形区域绘制
pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);
    let width = to_coord(width);
    let height = to_coord(height);
    rectangle(color, [gui_x, gui_y, width, height], con.transform, g);
}
