// https://adventofcode.com/2019/day/13

mod intcode;

use {
    intcode::Program,
    std::{
        collections::HashSet,
        io::{self, Write},
        thread, time,
    },
};

fn clear_screen() {
    print!("\x1bc");
}

fn clear_line() {
    print!("\x1b[2K");
}

fn hide_cursor() {
    print!("\x1b[?25l");
}

fn show_cursor() {
    print!("\x1b[?25h");
}

fn move_cursor(x: i64, y: i64) {
    print!("\x1b[{};{}f", y, x);
}

fn main() {
    clear_screen();
    hide_cursor();

    let mut program = Program::new(include_str!("input/13"));
    program.write(0, 2);
    program.set_input(&[0]);

    let mut paddle_x = 0;
    let mut ball_x = 0;
    let mut game_playing = false;
    let mut blocks = HashSet::new();
    let mut initial_block_count = 0;
    loop {
        let x = program.next();
        let y = program.next();
        let id = program.next();

        if x.is_none() || y.is_none() || id.is_none() {
            break;
        }

        let x = x.unwrap();
        let y = y.unwrap();
        let id = id.unwrap();

        if x == -1 && y == 0 {
            if !game_playing {
                initial_block_count = blocks.len();
            }
            game_playing = true;

            move_cursor(4, 1);
            clear_line();
            print!("Score: {}", id);

            move_cursor(25, 1);
            print!(
                "Blocks: {}/{}",
                initial_block_count - blocks.len(),
                initial_block_count
            );
        } else {
            let mut update_input = false;

            move_cursor(x + 1, y + 2);
            match id {
                0 => {
                    print!(" ");
                    blocks.remove(&(x, y));
                }
                1 => print!("█"),
                2 => {
                    print!("▒");
                    blocks.insert((x, y));
                }
                3 => {
                    print!("═");
                    paddle_x = x;
                    update_input = true;
                }
                4 => {
                    print!("○");
                    ball_x = x;
                    update_input = true;
                }
                _ => panic!(),
            }

            if update_input {
                program.set_input(&[match (paddle_x, ball_x) {
                    (p, b) if p > b => -1,
                    (p, b) if p < b => 1,
                    _ => 0,
                }]);
            }
        }

        io::stdout().flush().unwrap();

        if game_playing {
            thread::sleep(time::Duration::from_millis(2));
        }
    }

    move_cursor(0, 26);
    show_cursor();
}
