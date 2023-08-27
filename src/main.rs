#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod board;
mod hid;
mod joystick_button_matrix;
mod usb;

use defmt::*;
use defmt_rtt as _;
use embassy_time::{Duration, Timer};
use panic_probe as _;
use usbd_hid::descriptor::SerializedDescriptor;

use crate::{hid::JoystickReport, joystick_button_matrix::JoystickButtonsMatrix};

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let board = board::Board::init();
    info!("Initalised Board!");

    info!("Desc is: {:x}", JoystickReport::desc());

    let (mut writer, usb) = usb::setup_usb(board.usb_driver);
    unwrap!(spawner.spawn(usb::usb_task(usb)));

    info!("Started Application!");

    let mut joystick_buttons = JoystickButtonsMatrix::new(board.joystick_button_matrix);

    let mut analog_inputs = board.analog_inputs;

    loop {
        Timer::after(Duration::from_millis(100)).await;

        let btn = joystick_buttons.check().await;
        defmt::info!("{:?}", btn);
        defmt::info!("x: {} y: {}", analog_inputs.x(), analog_inputs.y());

        let button_bits = (btn.fire as u8)
            | (btn.lock as u8) << 1
            | (btn.launch as u8) << 2
            | (btn.a as u8) << 3
            | (btn.b as u8) << 4
            | (btn.c as u8) << 5;

        let report = JoystickReport {
            throttle: 0,
            rudder: 0,
            x: (((analog_inputs.x() as i32) - 2048) / 16) as i8,
            y: (((analog_inputs.y() as i32) - 2048) / 16) as i8,
            buttons: button_bits,
        };
        match writer.write_serialize(&report).await {
            Ok(()) => {}
            Err(e) => warn!("Failed to send report: {:?}", e),
        }
    }
}
