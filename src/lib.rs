extern crate colored;
extern crate pancurses;
extern crate rand;
extern crate termion;

use rand::Rng;
use colored::*;
use pancurses::*;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{stdin, stdout, Write};

pub const SQUARE_WIDTH: usize = 2 * SQUARE_HEIGHT;
pub const SQUARE_HEIGHT: usize = 8;

pub fn left(mut num_calculate: &mut Vec<Vec<i16>>) -> i8 {
    let mut counter = 0;
    for noidea in 0..3 {
        for number in 1..4 {
            for num in 0..4 {
                if num_calculate[num][number - 1] == 0 {
                    if num_calculate[num][number] != 0 {
                        num_calculate[num][number - 1] = num_calculate[num][number];
                        num_calculate[num][number] = 0;
                        counter += 1;
                    }
                }
            }
        }
    }
    counter += melt_left(&mut num_calculate, counter);
    //new_num(&mut num_calculate, counter);
    return counter;
}

pub fn right(mut num_calculate: &mut Vec<Vec<i16>>) -> i8 {
    let mut counter = 0;
    for noidea in 0..3 {
        for number in (0..3).rev() {
            for num in 0..4 {
                if num_calculate[num][number + 1] == 0 {
                    if num_calculate[num][number] != 0 {
                        num_calculate[num][number + 1] = num_calculate[num][number];
                        num_calculate[num][number] = 0;
                        counter += 1;
                    }
                }
            }
        }
    }
    counter += melt_right(&mut num_calculate, counter);
    return counter;
}

pub fn up(mut num_calculate: &mut Vec<Vec<i16>>) -> i8 {
    let mut counter = 0;
    for _number in 0..3 {
        for number in 0..4 {
            for num in 1..4 {
                if num_calculate[num - 1][number] == 0 {
                    if num_calculate[num][number] != 0 {
                        num_calculate[num - 1][number] = num_calculate[num][number];
                        num_calculate[num][number] = 0;
                        counter += 1;
                    }
                }
            }
        }
    }
    counter += melt_up(&mut num_calculate, counter);
    return counter;
}

pub fn down(mut num_calculate: &mut Vec<Vec<i16>>) -> i8 {
    let mut counter = 0;
    for _number in 0..3 {
        for number in 0..4 {
            for num in (0..3).rev() {
                if num_calculate[num + 1][number] == 0 {
                    if num_calculate[num][number] != 0 {
                        num_calculate[num + 1][number] = num_calculate[num][number];
                        num_calculate[num][number] = 0;
                        counter += 1;
                    }
                }
            }
        }
    }
    counter += melt_down(&mut num_calculate, counter);
    return counter;
}

pub fn melt_left(mut num_calculate: &mut Vec<Vec<i16>>, mut counter: i8) -> i8 {
    for number in 1..4 {
        for num in 0..4 {
            if num_calculate[num][number - 1] == num_calculate[num][number] {
                if num_calculate[num][number] != 0 {
                    num_calculate[num][number - 1] *= 2;
                    num_calculate[num][number] = 0;
                    counter += 1;
                }
            }
        }
    }

    for _number in 0..3 {
        for number in 1..4 {
            for num in 0..4 {
                if num_calculate[num][number - 1] == 0 {
                    num_calculate[num][number - 1] = num_calculate[num][number];
                    num_calculate[num][number] = 0;
                }
            }
        }
    }
    return counter;
}

pub fn melt_right(mut num_calculate: &mut Vec<Vec<i16>>, mut counter: i8) -> i8 {
    for number in (0..3).rev() {
        for num in 0..4 {
            if num_calculate[num][number + 1] == num_calculate[num][number] {
                if num_calculate[num][number] != 0 {
                    num_calculate[num][number + 1] *= 2;
                    num_calculate[num][number] = 0;
                    counter += 1
                }
            }
        }
    }

    for _number in 0..3 {
        for number in (0..3).rev() {
            for num in 0..4 {
                if num_calculate[num][number + 1] == 0 {
                    num_calculate[num][number + 1] = num_calculate[num][number];
                    num_calculate[num][number] = 0;
                }
            }
        }
    }
    return counter;
}

pub fn melt_up(mut num_calculate: &mut Vec<Vec<i16>>, mut counter: i8) -> i8 {
    for number in 0..4 {
        for num in 1..4 {
            if num_calculate[num - 1][number] == num_calculate[num][number] {
                if num_calculate[num][number] != 0 {
                    num_calculate[num - 1][number] *= 2;
                    num_calculate[num][number] = 0;
                    counter += 1;
                }
            }
        }
    }
    for _number in 0..3 {
        for number in 0..4 {
            for num in 1..4 {
                if num_calculate[num - 1][number] == 0 {
                    num_calculate[num - 1][number] = num_calculate[num][number];
                    num_calculate[num][number] = 0;
                }
            }
        }
    }
    return counter;
}

pub fn melt_down(mut num_calculate: &mut Vec<Vec<i16>>, mut counter: i8) -> i8 {
    for number in 0..4 {
        for num in (0..3).rev() {
            if num_calculate[num + 1][number] == num_calculate[num][number] {
                if num_calculate[num][number] != 0 {
                    num_calculate[num + 1][number] *= 2;
                    num_calculate[num][number] = 0;
                    counter += 1;
                }
            }
        }
    }
    for _number in 0..3 {
        for number in 0..4 {
            for num in (0..3).rev() {
                if num_calculate[num + 1][number] == 0 {
                    num_calculate[num + 1][number] = num_calculate[num][number];
                    num_calculate[num][number] = 0;
                }
            }
        }
    }
    return counter;
}

pub fn convert(
    mut num_display: &mut Vec<Vec<&str>>,
    mut num_calculate: &mut Vec<Vec<i16>>,
    window: &Window,
) {
    let mut cp = 1;
    for number in 0..4 {
        for num in 0..4 {
            match num_calculate[num][number] {
                0 => {
                    num_display[num][number] = "    ";
                    cp = 1;
                }
                2 => {
                    num_display[num][number] = " 2  ";
                    cp = 2;
                }
                4 => {
                    num_display[num][number] = " 4  ";
                    cp = 3;
                }
                8 => {
                    num_display[num][number] = " 8  ";
                    cp = 4;
                }
                16 => {
                    num_display[num][number] = " 16 ";
                    cp = 5;
                }
                32 => {
                    num_display[num][number] = " 32 ";
                    cp = 6;
                }
                64 => {
                    num_display[num][number] = " 64 ";
                    cp = 7;
                }
                128 => {
                    num_display[num][number] = "128 ";
                    cp = 8;
                }
                256 => {
                    num_display[num][number] = "256 ";
                    cp = 9;
                }
                512 => {
                    num_display[num][number] = "512 ";
                    cp = 10;
                }
                1024 => {
                    num_display[num][number] = "1024";
                    cp = 11;
                }
                2048 => {
                    num_display[num][number] = "2048";
                    cp = 12;
                }
                _ => (),
            }
            draw_square(
                &window,
                num * SQUARE_HEIGHT,
                number * SQUARE_WIDTH,
                num_display[num][number],
                cp,
            );
        }
    }
    window.refresh();
}

pub fn new_num(mut num_calculate: &mut Vec<Vec<i16>>, mut counter: i8) {
    let random = rand::thread_rng().gen_range(1, 3);
    if counter != 0 {
        counter = 0;
        loop {
            let num = rand::thread_rng().gen_range(0, 4);
            let number = rand::thread_rng().gen_range(0, 4);
            if num_calculate[num][number] == 0 {
                num_calculate[num][number] = random * 2;
                return ();
            }
        }
    } else {
        for number in 0..4 {
            for num in 0..4 {
                if num_calculate[num][number] == 0 {
                    return ();
                }
            }
        }
        endwin();
        //println!("GAME OVER");
    }
}

pub fn draw_square(window: &Window, offset_y: usize, offset_x: usize, text: &str, color_pair: u32) {
    window.attron(COLOR_PAIR(color_pair));
    for x in 0..SQUARE_WIDTH - 1 {
        for y in 0..SQUARE_HEIGHT - 1 {
            window.mvaddstr((offset_y + y) as i32, (offset_x + x) as i32, " ");
        }
    }
    window.mvaddstr(
        (offset_y + (SQUARE_HEIGHT - 1) / 2) as i32,
        (offset_x + (SQUARE_WIDTH - 5) / 2) as i32,
        text,
    );
}

pub fn backup(mut num_calculate: &mut Vec<Vec<i16>>, mut num_reverse: &mut Vec<Vec<i16>>) {
    for number in 0..4 {
        for num in 0..4 {
            num_reverse[num][number] = num_calculate[num][number];
        }
    }
}

pub fn reverse(mut num_calculate: &mut Vec<Vec<i16>>, mut num_reverse: &mut Vec<Vec<i16>>) {
    for number in 0..4 {
        for num in 0..4 {
            num_calculate[num][number] = num_reverse[num][number];
        }
    }
}
