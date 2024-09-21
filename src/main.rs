#![deny(unsafe_code)]
//dont't use standard crate
#![no_std]
//don't use standard main interface
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use microbit::board::Board;
use microbit::hal::prelude::*;


#[entry]
fn entry_point() -> ! {
    let mut board = Board::take().unwrap();

    board.display_pins.col1.set_low().unwrap();
    board.display_pins.row1.set_high().unwrap();

    loop {}
}