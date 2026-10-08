use macroquad::prelude::*;

fn gosper_gun(startx: i32, starty: i32, grid: &mut Vec<Vec<i32>>) -> () {
    let pts: Vec<(i32, i32)> = vec![
        (4, 0),
        (4, 1),
        (5, 0),
        (5, 1),
        (2, 12),
        (2, 13),
        (3, 11),
        (3, 15),
        (4, 10),
        (5, 10),
        (6, 10),
        (7, 11),
        (7, 15),
        (8, 12),
        (8, 13),
        (4, 16),
        (5, 14),
        (5, 16),
        (5, 17),
        (6, 16),
        (0, 24),
        (1, 22),
        (1, 24),
        (2, 20),
        (2, 21),
        (3, 20),
        (3, 21),
        (4, 20),
        (4, 21),
        (5, 22),
        (5, 24),
        (6, 24),
        (2, 34),
        (2, 35),
        (3, 34),
        (3, 35),
    ];
    for (dx, dy) in pts {
        let nx: i32 = dx + startx;
        let ny: i32 = dy + starty;
        grid[nx as usize][ny as usize] = 1;
    }
}

#[macroquad::main("Life")]
async fn main() {
    println!("Running");

    // let x: f32 = screen_width() / 2.0;
    // let y: f32 = screen_height() / 2.0;
    //

    let x = 0.0;
    let y = 0.0;
    let w: f32 = 1200.0;
    let h: f32 = 1200.0;

    let mut grid: Vec<Vec<i32>> = vec![vec![0; w as usize]; h as usize];
    // grid[150][150] = 1;
    // grid[149][150] = 1;
    // grid[151][150] = 1;
    // grid[152][150] = 1;
    // grid[153][150] = 1;
    // grid[154][150] = 1;
    // grid[155][150] = 1;

    // gosper glider gun

    gosper_gun(150, 150, &mut grid);

    let mut iter: i32 = 0;
    let mut iter_text: String = String::from("Iterations: ");

    // create dirs
    // let dirs: Vec<(i32, i32)> = vec![(0, 0, ];
    let mut dirs: Vec<(i32, i32)> = vec![];
    for i in -1..2 {
        for j in -1..2 {
            if (i, j) == (0, 0) {
                continue;
            }
            dirs.push((i, j));
        }
    }
    for i in 0..dirs.len() - 1 {
        println!("{} {} ", dirs[i as usize].0, dirs[i as usize].1);
    }

    let mut to_die: Vec<(i32, i32)> = vec![];

    let mut to_birth: Vec<(i32, i32)> = vec![];

    loop {
        clear_background(BLACK);

        draw_text("Game of life", x - 20.0, y - 40.0, 30.0, DARKGRAY);
        draw_text(&iter_text, x - 20.0, y - 20.0, 30.0, DARKGRAY);

        // process births or deaths first
        for (x, y) in &to_die {
            grid[*x as usize][*y as usize] = 0;
        }
        to_die = vec![];

        for (x, y) in &to_birth {
            grid[*x as usize][*y as usize] = 1;
        }
        to_birth = vec![];

        // update to screen

        // grid
        // draw_rectangle(x, y, w, h, DARKGRAY);
        for i in 0..h as i32 {
            for j in 0..w as i32 {
                if grid[i as usize][j as usize] == 1 {
                    draw_rectangle(x + (i as f32 * 4.0), y + (j as f32 * 4.0), 4.0, 4.0, WHITE);
                } else {
                    // draw_rectangle(x + i as f32, y + j as f32, 1.0, 1.0, BLACK);
                }
            }
        }

        // encode rules
        for i in 0..h as i32 {
            for j in 0..w as i32 {
                if grid[i as usize][j as usize] == 1 {
                    // alive
                    // under pop
                    let mut aliven = 0;
                    for (dx, dy) in &dirs {
                        let nx = i + dx;
                        let ny = j + dy;
                        if 0 <= nx
                            && nx < w as i32
                            && 0 <= ny
                            && ny < h as i32
                            && grid[nx as usize][ny as usize] == 1
                        {
                            aliven += 1;
                        }
                    }

                    if aliven < 2 || aliven > 3 {
                        // mark as to die , underpop or overpop
                        to_die.push((i, j));
                    }
                } else {
                    // dead
                    let mut aliven = 0;
                    for (dx, dy) in &dirs {
                        let nx = i + dx;
                        let ny = j + dy;
                        if 0 <= nx
                            && nx < w as i32
                            && 0 <= ny
                            && ny < h as i32
                            && grid[nx as usize][ny as usize] == 1
                        {
                            aliven += 1;
                        }
                    }

                    if aliven == 3 {
                        // reprod
                        to_birth.push((i, j));
                    }
                }
            }
        }

        // process
        iter += 1;
        iter_text = format!("Iterations: {}", iter);

        next_frame().await
    }
}
