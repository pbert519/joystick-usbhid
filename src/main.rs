#![no_std]
#![no_main]

mod board;
mod dpad;
mod hid;
mod joystick_button_matrix;
mod throttle_button_matrix;
mod usb;

use defmt::*;
use defmt_rtt as _;
use embassy_time::{Duration, Timer};
use panic_probe as _;
use usbd_hid::descriptor::SerializedDescriptor;

use crate::{
    hid::JoystickReport, joystick_button_matrix::JoystickButtonsMatrix,
    throttle_button_matrix::ThrottleButtonsMatrix,
};

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let board = board::Board::init();
    info!("Initalised Board!");

    info!("Desc is: {:x}", JoystickReport::desc());

    let (mut writer, usb) = usb::setup_usb(board.usb_driver);
    unwrap!(spawner.spawn(usb::usb_task(usb)));

    info!("Started Application!");

    let mut joystick_buttons = JoystickButtonsMatrix::new(board.joystick_button_matrix);
    let mut throttle_buttons = ThrottleButtonsMatrix::new(board.throttle_button_matrix);
    let mut analog_inputs: board::AnalogInput = board.analog_inputs;

    loop {
        Timer::after(Duration::from_millis(100)).await;

        let joystick_btn = joystick_buttons.check().await;
        let throttle_btn = throttle_buttons.check().await;

        let report = JoystickReport {
            throttle: (analog_inputs.throttle() / 16) as u8,
            rudder: (((analog_inputs.rudder() as i32) - 2048) / 16) as i8,
            x: (((analog_inputs.x() as i32) - 2048) / 16) as i8,
            y: (((analog_inputs.y() as i32) - 2048) / 16) as i8,
            poti1: (analog_inputs.poti1() / 16) as u8,
            poti2: (analog_inputs.poti2() / 16) as u8,
            buttons: joystick_btn.as_bitfield() | (throttle_btn.as_bitfield() << 6),
            hat_switch_1: joystick_btn.dpad1.as_bitfield(),
            hat_switch_2: joystick_btn.dpad2.as_bitfield(),
            hat_switch_3: throttle_btn.dpad1.as_bitfield(),
            hat_switch_4: throttle_btn.dpad2.as_bitfield(),
        };
        info!("\n {} \n {} \n {}", joystick_btn, throttle_btn, report);

        match writer.write_serialize(&report).await {
            Ok(()) => {}
            Err(e) => warn!("Failed to send report: {:?}", e),
        }
    }
}
