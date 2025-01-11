#![no_std]
#![no_main]

use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};
use embedded_hal_nb::serial::{Read, Write};
use cortex_m_rt::entry;
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

use microbit_learn::serial_setup::UartePort;

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

    loop {
        // Read data from pc
        let byte_char: char = nb::block!(serial.read()).unwrap().into();
        rprintln!("{}", byte_char);

        {
            // send the same data back to pc
            use core::fmt::Write;
            write!(serial, "{}\r\n", byte_char).unwrap();
            nb::block!(serial.flush()).unwrap();
        }
    }
}