#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};
use panic_halt as _;

#[entry]
fn entry_point() -> ! {
    // Initialize the board, timer, and display only once
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    // Initialize the LED matrix
    let mut leds = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let row_len = leds.len();
    let col_len = leds[0].len();

    loop {
        // Call spiral_display, passing display and timer
        spiral_display(
            &mut display,
            &mut timer,
            &mut leds,
            0,
            row_len - 1,
            0,
            col_len - 1,
        );
    }
}

fn spiral_display(
    display: &mut Display, 
    timer: &mut Timer<microbit::hal::pac::TIMER0>,  // Pass Timer and Display by reference
    leds: &mut [[u8; 5]; 5],
    start_row: usize,
    end_row: usize,
    start_col: usize,
    end_col: usize,
) {
    if start_row > end_row || start_col > end_col {
        return;
    }

    // Top row (left to right)
    for i in start_col..=end_col {
        leds[start_row][i] = 1;
        display.show(timer, *leds, 30);
        leds[start_row][i] = 0;
    }

    // Right column (top to bottom)
    for i in (start_row + 1)..=end_row {
        leds[i][end_col] = 1;
        display.show(timer, *leds, 30);
        leds[i][end_col] = 0;
    }

    // Bottom row (right to left)
    if start_row != end_row {
        for i in (start_col..end_col).rev() {
            leds[end_row][i] = 1;
            display.show(timer, *leds, 30);
            leds[end_row][i] = 0;
        }
    }

    // Left column (bottom to top)
    if start_col != end_col {
        for i in (start_row + 1..end_row).rev() {
            leds[i][start_col] = 1;
            display.show(timer, *leds, 30);
            leds[i][start_col] = 0;
        }
    }

    // Recursively call the next inner layer of the spiral
    spiral_display(
        display,
        timer,
        leds,
        start_row + 1,
        end_row - 1,
        start_col + 1,
        end_col - 1,
    );
}
