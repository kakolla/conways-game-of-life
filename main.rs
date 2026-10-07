use macroquad::prelude::*;

#[macroquad::main("Life")]
async fn main() {
    println!("Running");

    let x: f32 = screen_width() / 2.0;
    let y: f32 = screen_height() / 2.0;

    let w: f32 = 300.0;
    let h: f32 = 300.0;

    let mut grid: Vec<Vec<i32>> = vec![vec![0; w as usize]; h as usize];
    grid[0][0] = 1;

    let mut iter: i32 = 0;
    let mut iter_text: String = String::from("Iterations: ");
    loop {
        clear_background(BLACK);

        draw_text("Game of life", x - 20.0, y - 40.0, 30.0, DARKGRAY);
        draw_text(&iter_text, x - 20.0, y - 20.0, 30.0, DARKGRAY);

        // update to screen

        // grid
        draw_rectangle(x, y, w, h, DARKGRAY);
        for i in 0..h as i32 {
            for j in 0..w as i32 {
                if grid[i as usize][j as usize] == 1 {
                    draw_rectangle(x + i as f32, y + j as f32, 1.0, 1.0, WHITE);
                } else {
                    // draw_rectangle(x + i as f32, y + j as f32, 1.0, 1.0, BLACK);
                }
            }
        }

        // process

        // encode rules
        iter += 1;
        iter_text = format!("Iterations: {}", iter);

        next_frame().await
    }
}
