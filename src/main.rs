#![no_std]
#![no_main]

use panic_rtt_target as _;
use rtt_target::rtt_init_print;
use cortex_m_rt::entry;
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};
use embedded_hal_nb::serial::Write;
mod serial_setup;
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    let data = "The quick brown fox jumps over the lazy dog.";
    for c in data.chars() {
        nb::block!(serial.write(c as u8)).unwrap();
        nb::block!(serial.flush()).unwrap();
    }

    loop {}
}