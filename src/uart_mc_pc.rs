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

use microbit_learn::serial_setup::UartePort;
use core::fmt::Write;

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

    write!(serial, "The quick brown fox jumps over the lazy dog.\r\n").unwrap();
    {
        use embedded_hal_nb::serial::Write;
        nb::block!(serial.flush()).unwrap();
    }

    loop {}
}