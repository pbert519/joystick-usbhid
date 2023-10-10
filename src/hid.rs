use usbd_hid::descriptor::gen_hid_descriptor;
use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::descriptor::SerializedDescriptor;

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = JOYSTICK) = {
        (collection = PHYSICAL, usage_page = GENERIC_DESKTOP, usage = POINTER) = {
            (usage = X,) = {
                #[item_settings data,variable,absolute] x=input;
            };
            (usage = Y,) = {
                #[item_settings data,variable,absolute] y=input;
            };
        };
        (usage_page = SIMULATION_CONTROLS, usage = 0xbb /*Throttle */,) = {
            #[item_settings data,variable,absolute] throttle=input;
        };
        (usage_page = SIMULATION_CONTROLS, usage = 0xba/*Rudder */,) = {
            #[item_settings data,variable,absolute] rudder=input;
        };
        (usage_page = BUTTON, usage_min = BUTTON_1, usage_max = BUTTON_8) = {
            #[packed_bits 8] #[item_settings data,variable,absolute] buttons=input;
        };
        (usage_page = GENERIC_DESKTOP, usage = 0x39 /* HAT Switch*/,) = {
            #[packed_bits 4] #[item_settings data,variable,absolute] hat_switch_1 = input;
        };
        (usage_page = GENERIC_DESKTOP, usage = 0x39 /* HAT Switch*/,) = {
            #[packed_bits 4] #[item_settings data,variable,absolute] hat_switch_2 = input;
        };
        (usage_page = GENERIC_DESKTOP, usage = 0x39 /* HAT Switch*/,) = {
            #[packed_bits 4] #[item_settings data,variable,absolute] hat_switch_3 = input;
        };
        (usage_page = GENERIC_DESKTOP, usage = 0x39 /* HAT Switch*/,) = {
            #[packed_bits 4] #[item_settings data,variable,absolute] hat_switch_4 = input;
        };
    }
)]
#[allow(dead_code)]
#[derive(defmt::Format)]
pub struct JoystickReport {
    pub throttle: u8,
    pub rudder: i8,
    pub poti1: u8,
    pub poti2: u8,
    pub x: i8,
    pub y: i8,
    pub buttons: u8,
    pub hat_switch_1: u8,
    pub hat_switch_2: u8,
    pub hat_switch_3: u8,
    pub hat_switch_4: u8,
}
