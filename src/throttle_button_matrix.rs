use embassy_time::{Duration, Timer};

use crate::board;
use crate::dpad::*;

#[derive(Default, Debug, defmt::Format)]
pub struct ThrottleButtons {
    pub mode: u8,
    pub aux: u8,
    pub d: bool,
    pub select: bool,
    pub dpad1: DPadDirection,
    pub dpad2: DPadDirection,
}

impl ThrottleButtons {
    pub fn as_bitfield(&self) -> u8 {
        (self.d as u8) | (self.select as u8) << 1
    }
}

pub struct ThrottleButtonsMatrix {
    pins: board::ThrottleButtonMatrixPins,
}

impl ThrottleButtonsMatrix {
    pub fn new(pins: board::ThrottleButtonMatrixPins) -> Self {
        Self { pins }
    }

    // need to be called periodically to update everything
    pub async fn check(&mut self) -> ThrottleButtons {
        let mut buttons = ThrottleButtons::default();

        let mode_left = self.pins.mode_aux_input.get_level();

        self.pins.dpad_up_select.set_high();
        Timer::after(Duration::from_millis(1)).await;
        let dpad1_up = self.pins.dpad1_input.get_level();
        let dpad2_up = self.pins.dpad2_input.get_level();
        let aux_right = self.pins.mode_aux_input.get_level();

        self.pins.dpad_up_select.set_low();
        self.pins.dpad_right_select.set_high();
        Timer::after(Duration::from_millis(1)).await;
        let dpad1_right = self.pins.dpad1_input.get_level();
        let dpad2_right = self.pins.dpad2_input.get_level();
        let aux_left = self.pins.mode_aux_input.get_level();

        self.pins.dpad_right_select.set_low();
        self.pins.dpad_down_select.set_high();
        Timer::after(Duration::from_millis(1)).await;
        let dpad1_down = self.pins.dpad1_input.get_level();
        let dpad2_down = self.pins.dpad2_input.get_level();
        buttons.d = self.pins.input.get_level().into();
        let mode_right = self.pins.mode_aux_input.get_level();

        self.pins.dpad_down_select.set_low();
        self.pins.dpad_left_select_select.set_high();
        Timer::after(Duration::from_millis(1)).await;
        let dpad1_left = self.pins.dpad1_input.get_level();
        let dpad2_left = self.pins.dpad2_input.get_level();
        buttons.select = self.pins.input.get_level().into();

        buttons.dpad1 = DPadDirection::from_pins(
            dpad1_up.into(),
            dpad1_right.into(),
            dpad1_down.into(),
            dpad1_left.into(),
        );
        buttons.dpad2 = DPadDirection::from_pins(
            dpad2_up.into(),
            dpad2_right.into(),
            dpad2_down.into(),
            dpad2_left.into(),
        );
        self.pins.dpad_left_select_select.set_low();

        buttons.aux = {
            if aux_left.into() {
                0
            } else if aux_right.into() {
                2
            } else {
                1
            }
        };

        buttons.mode = {
            if mode_left.into() {
                0
            } else if mode_right.into() {
                2
            } else {
                1
            }
        };

        buttons
    }
}
