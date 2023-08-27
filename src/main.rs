#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod board;
mod hid;
mod usb;

use defmt::*;
use defmt_rtt as _;
use embassy_time::{Duration, Timer};
use panic_probe as _;
use usbd_hid::descriptor::SerializedDescriptor;

use crate::hid::JoystickReport;

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let board = board::Board::init();
    info!("Initalised Board!");

    info!("Desc is: {:x}", JoystickReport::desc());

    let (mut writer, usb) = usb::setup_usb(board.usb_driver);
    unwrap!(spawner.spawn(usb::usb_task(usb)));

    info!("Started Application!");

    let mut y: i8 = 50;
    let mut btn: u8 = 0;

    loop {
        Timer::after(Duration::from_millis(500)).await;

        y = -y;
        btn = (btn +1)%8;
        let report = JoystickReport {
            throttle: 200,
            rudder: y,
            x: 0,
            y,
            buttons: 1 << btn,
        };
        match writer.write_serialize(&report).await {
            Ok(()) => {}
            Err(e) => warn!("Failed to send report: {:?}", e),
        }
    }
}
