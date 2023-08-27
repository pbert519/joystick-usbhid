use embassy_time::{Duration, Timer};

use crate::board;

#[derive(Debug,defmt::Format)]
pub enum DPadDirection {
    None,
    Up,
    RightUp,
    Right,
    RightDown,
    Down,
    LeftDown,
    Left,
    LeftUp,
}
impl Default for DPadDirection {
    fn default() -> Self {
        DPadDirection::None
    }
}

impl DPadDirection {
    pub fn from_pins(up: bool, right: bool, down: bool, left: bool) -> DPadDirection {
        if up && !right && !down && !left {
            DPadDirection::Up
        } else if !up && right && !down && !left {
            DPadDirection::Right
        } else if !up && !right && down && !left {
            DPadDirection::Down
        } else if !up && !right && !down && left {
            DPadDirection::Left
        } else if up && right && !down && !left {
            DPadDirection::RightUp
        } else if !up && right && down && !left {
            DPadDirection::RightDown
        } else if up && !right && !down && left {
            DPadDirection::LeftUp
        } else if !up && !right && down && left {
            DPadDirection::LeftDown
        } else {
            DPadDirection::None
        }
    }
}

#[derive(Default,Debug,defmt::Format)]
pub struct JoystickButtons {
    lock: bool,
    fire: bool,
    launch: bool,
    a: bool,
    b: bool,
    c: bool,
    dpad1: DPadDirection,
    dpad2: DPadDirection,
}

pub struct JoystickButtonsMatrix {
    pins: board::ButtonMatrixPins,
}

impl JoystickButtonsMatrix {
    pub fn new(pins: board::ButtonMatrixPins) -> Self {
        Self { pins }
    }

    // need to be called periodically to update everything
    pub async fn check(&mut self) -> JoystickButtons {
        let mut buttons = JoystickButtons::default();
        self.pins.fire_dpad_up_btn_c_select.set_high();
        Timer::after(Duration::from_millis(1)).await;
        buttons.fire = self.pins.fire_lock_input.get_level().into();
        let dpad1_up = self.pins.dpad1_input.get_level();
        let dpad2_up = self.pins.dpad2_input.get_level();
        buttons.c = self.pins.buttons_input.get_level().into();

        self.pins.fire_dpad_up_btn_c_select.set_low();
        self.pins.lock_dpad_right_btn_a_select.set_high();
        Timer::after(Duration::from_millis(1)).await;
        buttons.lock = self.pins.fire_lock_input.get_level().into();
        let dpad1_right = self.pins.dpad1_input.get_level();
        let dpad2_right = self.pins.dpad2_input.get_level();
        buttons.a = self.pins.buttons_input.get_level().into();

        self.pins.lock_dpad_right_btn_a_select.set_low();
        self.pins.dpad_down_btn_b_select.set_high();
        Timer::after(Duration::from_millis(1)).await;
        let dpad1_down = self.pins.dpad1_input.get_level();
        let dpad2_down = self.pins.dpad2_input.get_level();
        buttons.b = self.pins.buttons_input.get_level().into();

        self.pins.dpad_down_btn_b_select.set_low();
        self.pins.dpad_left_launch_select.set_high();
        Timer::after(Duration::from_millis(1)).await;
        let dpad1_left = self.pins.dpad1_input.get_level();
        let dpad2_left = self.pins.dpad2_input.get_level();
        buttons.launch = self.pins.buttons_input.get_level().into();

        buttons.dpad1 = DPadDirection::from_pins(
            dpad1_up.into(),
            dpad1_right.into(),
            dpad1_down.into(),
            dpad1_left.into(),
        );
        buttons.dpad1 = DPadDirection::from_pins(
            dpad2_up.into(),
            dpad2_right.into(),
            dpad2_down.into(),
            dpad2_left.into(),
        );

        return buttons;
    }
}
