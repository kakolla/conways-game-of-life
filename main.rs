use macroquad::prelude::*;

#[macroquad::main("Life")]
async fn main() {
    println!("Running");

    let x: f32 = screen_width() / 2.0;
    let y: f32 = screen_height() / 2.0;

    let w: f32 = 300.0;
    let h: f32 = 300.0;

    let mut grid: Vec<Vec<i32>> = vec![vec![0; w as usize]; h as usize];

    loop {
        clear_background(BLACK);

        draw_text("Hello there", 20.0, 20.0, 30.0, DARKGRAY);
        draw_rectangle(x, y, w, h, RED);
        next_frame().await
    }
}
