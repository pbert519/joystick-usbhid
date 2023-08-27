use usbd_hid::descriptor::gen_hid_descriptor;
use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::descriptor::SerializedDescriptor;

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = JOYSTICK) = {
        (usage_page = SIMULATION_CONTROLS, usage = 0xbb /*Throttle */,) = {
            #[item_settings data,variable,absolute] throttle=input;
        };
        (usage_page = SIMULATION_CONTROLS, usage = 0xba/*Rudder */,) = {
            #[item_settings data,variable,absolute] rudder=input;
        };
        (collection = PHYSICAL, usage_page = GENERIC_DESKTOP, usage = POINTER) = {
            (usage = X,) = {
                #[item_settings data,variable,absolute] x=input;
            };
            (usage = Y,) = {
                #[item_settings data,variable,absolute] y=input;
            };
        };
        (usage_page = BUTTON, usage_min = BUTTON_1, usage_max = 0x10 /*16 buttons */) = {
            #[item_settings data,variable,absolute] buttons=input;
        };
    }
)]
#[allow(dead_code)]
pub struct JoystickReport {
    pub throttle: u8,
    pub rudder: i8,
    pub x: i8,
    pub y: i8,
    pub buttons: u8,
}
