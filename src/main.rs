#![deny(unsafe_code)]
//dont't use standard crate
#![no_std]
//don't use standard main interface
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use microbit::board::Board;
use microbit::hal::timer::Timer;
use microbit::hal::prelude::*;


#[entry]
fn entry_point() -> ! {
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    board.display_pins.col1.set_low().unwrap();
    let mut row1 = board.display_pins.row1;

    loop {
        row1.set_low().unwrap();
        timer.delay_ms(1_000_u16);
        row1.set_high().unwrap();
        timer.delay_ms(1_000_u16);
    }
}