use embassy_stm32::{bind_interrupts, dma, gpio, peripherals, usart, usb_otg};
use static_cell::StaticCell;

static USB_EP_BUFFER: StaticCell<[u8; 256]> = StaticCell::new();

bind_interrupts!(struct Irqs {
    USART3 => usart::InterruptHandler<peripherals::USART3>;
    OTG_HS => usb_otg::InterruptHandler<peripherals::USB_OTG_HS>;
});

pub struct Board {
    /// onboard red led
    pub led_red: gpio::Output<'static, gpio::AnyPin>,
    /// onboard yellow led
    pub led_yellow: gpio::Output<'static, gpio::AnyPin>,
    /// onboard green led
    pub led_green: gpio::Output<'static, gpio::AnyPin>,
    /// onboard user button
    pub btn_user: gpio::Input<'static, gpio::AnyPin>,
    /// usart connecto to stlink
    pub stlink_usart: usart::Uart<'static, peripherals::USART3>,
    /// onboard full speed usb interface
    pub usb_driver: usb_otg::Driver<'static, peripherals::USB_OTG_HS>,
    /// pins used for the button matrix on primary joystick
    pub joystick_button_matrix: ButtonMatrixPins,
}

pub struct ButtonMatrixPins {
    pub dpad1_input: gpio::Input<'static, gpio::AnyPin>,
    pub dpad2_input: gpio::Input<'static, gpio::AnyPin>,
    pub buttons_input: gpio::Input<'static, gpio::AnyPin>,
    pub fire_lock_input: gpio::Input<'static, gpio::AnyPin>,
    pub lock_dpad_right_btn_a_select: gpio::Output<'static, gpio::AnyPin>,
    pub fire_dpad_up_btn_c_select: gpio::Output<'static, gpio::AnyPin>,
    pub dpad_left_launch_select: gpio::Output<'static, gpio::AnyPin>,
    pub dpad_down_btn_b_select: gpio::Output<'static, gpio::AnyPin>,
}

impl Board {
    pub fn init() -> Self {
        let config = embassy_stm32::Config::default();
        let p = embassy_stm32::init(config);

        let led_red = gpio::Output::new(p.PB14, gpio::Level::High, gpio::Speed::Low).degrade();
        let led_yellow = gpio::Output::new(p.PE1, gpio::Level::High, gpio::Speed::Low).degrade();
        let led_green = gpio::Output::new(p.PB0, gpio::Level::High, gpio::Speed::Low).degrade();
        let btn_user = gpio::Input::new(p.PC13, gpio::Pull::None).degrade();

        // TODO Select Pins
        let joystick_button_matrix = ButtonMatrixPins {
            dpad1_input: gpio::Input::new(p.PA0, gpio::Pull::Down).degrade(),
            dpad2_input: gpio::Input::new(p.PA1, gpio::Pull::Down).degrade(),
            buttons_input: gpio::Input::new(p.PA2, gpio::Pull::Down).degrade(),
            fire_lock_input: gpio::Input::new(p.PA3, gpio::Pull::Down).degrade(),
            lock_dpad_right_btn_a_select: gpio::Output::new(
                p.PA4,
                gpio::Level::Low,
                gpio::Speed::Low,
            )
            .degrade(),
            fire_dpad_up_btn_c_select: gpio::Output::new(p.PA5, gpio::Level::Low, gpio::Speed::Low)
                .degrade(),
            dpad_left_launch_select: gpio::Output::new(p.PA6, gpio::Level::Low, gpio::Speed::Low)
                .degrade(),
            dpad_down_btn_b_select: gpio::Output::new(p.PA7, gpio::Level::Low, gpio::Speed::Low)
                .degrade(),
        };

        let stlink_usart = usart::Uart::new(
            p.USART3,
            p.PD9,
            p.PD8,
            Irqs,
            dma::NoDma,
            dma::NoDma,
            usart::Config::default(),
        );

        // create usb driver
        let ep_out_buffer = USB_EP_BUFFER.init([0u8; 256]);

        let mut usb_driver_config = usb_otg::Config::default();
        usb_driver_config.vbus_detection = true;
        let usb_driver = usb_otg::Driver::new_fs(
            p.USB_OTG_HS,
            Irqs,
            p.PA12,
            p.PA11,
            ep_out_buffer,
            usb_driver_config,
        );

        Board {
            led_red,
            led_yellow,
            led_green,
            btn_user,
            stlink_usart,
            usb_driver,
            joystick_button_matrix,
        }
    }
}
