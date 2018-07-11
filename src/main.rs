extern crate pancurses;
extern crate termion;
extern crate twentyfortyeight;

use pancurses::*;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{stdin, stdout, Write};
use twentyfortyeight::*;

fn main() {
    let window = initscr();
    start_color();
    const SQUARE_WIDTH: i32 = 2 * SQUARE_HEIGHT;
    const SQUARE_HEIGHT: i32 = 4;
    let mut num_display = vec![vec!["    "; 4]; 4];
    let mut num_calculate = vec![vec![0; 4]; 4];
    let mut num_reverse = vec![vec![0; 4]; 4];
    let mut num_temp = vec![vec![0; 4]; 4];
    let mut counter: i8 = 0;
    num_calculate[2][1] = 4;
    num_calculate[2][2] = 4;
    init_pair(1, COLOR_WHITE, COLOR_WHITE);
    init_pair(2, COLOR_BLACK, 6);
    init_pair(3, COLOR_BLACK, 39);
    init_pair(4, COLOR_WHITE, 57);
    init_pair(5, COLOR_WHITE, 55);
    init_pair(6, COLOR_WHITE, 125);
    init_pair(7, COLOR_WHITE, 89);
    init_pair(8, COLOR_WHITE, 160);
    init_pair(9, COLOR_WHITE, 88);
    init_pair(10, COLOR_WHITE, 52);
    init_pair(11, COLOR_WHITE, 245);
    init_pair(12, COLOR_WHITE, 235);

    let stdin = stdin();
    window.attron(COLOR_PAIR(1) | A_BOLD);
    for x in 0..SQUARE_WIDTH * 8 {
        for y in 0..SQUARE_HEIGHT * 8 {
            window.mvaddstr(y, x, " ");
        }
    }

    for c in stdin.keys() {
        backup(&mut num_calculate, &mut num_temp);

        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('r') => reverse(&mut num_calculate, &mut num_reverse),
            Key::Left => {
                counter = left(&mut num_calculate);
            }
            Key::Right => {
                counter = right(&mut num_calculate);
            }
            Key::Up => {
                counter = up(&mut num_calculate);
            }
            Key::Down => {
                counter = down(&mut num_calculate);
            }
            _ => counter=0,
        }

        if counter != 0 {
            backup(&mut num_temp, &mut num_reverse);
            new_num(&mut num_calculate, counter);
        }

        convert(&mut num_display, &mut num_calculate, &window);
    }
    window.refresh();
    endwin();
}
