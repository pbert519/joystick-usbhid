use embassy_stm32::{adc, bind_interrupts, dma, gpio, peripherals, rcc, usart, usb_otg};
use embassy_time::Delay;
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
    pub joystick_button_matrix: JoystickButtonMatrixPins,
    /// pins used for the button matrix on throttle
    pub throttle_button_matrix: ThrottleButtonMatrixPins,
    /// analog inputs
    pub analog_inputs: AnalogInput,
}

pub struct JoystickButtonMatrixPins {
    pub dpad1_input: gpio::Input<'static, gpio::AnyPin>,
    pub dpad2_input: gpio::Input<'static, gpio::AnyPin>,
    pub buttons_input: gpio::Input<'static, gpio::AnyPin>,
    pub fire_lock_input: gpio::Input<'static, gpio::AnyPin>,
    pub lock_dpad_right_btn_a_select: gpio::Output<'static, gpio::AnyPin>,
    pub fire_dpad_up_btn_c_select: gpio::Output<'static, gpio::AnyPin>,
    pub dpad_left_launch_select: gpio::Output<'static, gpio::AnyPin>,
    pub dpad_down_btn_b_select: gpio::Output<'static, gpio::AnyPin>,
}

pub struct ThrottleButtonMatrixPins {
    pub dpad1_input: gpio::Input<'static, gpio::AnyPin>,
    pub dpad2_input: gpio::Input<'static, gpio::AnyPin>,
    pub mode_aux_input: gpio::Input<'static, gpio::AnyPin>,
    pub input: gpio::Input<'static, gpio::AnyPin>,
    pub dpad_right_select: gpio::Output<'static, gpio::AnyPin>,
    pub dpad_up_select: gpio::Output<'static, gpio::AnyPin>,
    pub dpad_left_select_select: gpio::Output<'static, gpio::AnyPin>,
    pub dpad_down_select: gpio::Output<'static, gpio::AnyPin>,
}

pub struct AnalogInput {
    adc: adc::Adc<'static, peripherals::ADC1>,
    adc2: adc::Adc<'static, peripherals::ADC2>,
    x: peripherals::PA3,          // A0 on board
    y: peripherals::PC0,          // A1 on board
    rudder: peripherals::PF11,    // A2 on board
    throttle: peripherals::PC2_C, // A3 on board
    poti1: peripherals::PB1,      // A4 on board
    poti2: peripherals::PC3_C,    // A5 on board
}
impl AnalogInput {
    pub fn x(&mut self) -> u16 {
        self.adc.read(&mut self.x)
    }
    pub fn y(&mut self) -> u16 {
        self.adc.read(&mut self.y)
    }
    pub fn rudder(&mut self) -> u16 {
        self.adc.read(&mut self.rudder)
    }
    pub fn throttle(&mut self) -> u16 {
        self.adc2.read(&mut self.throttle)
    }
    pub fn poti1(&mut self) -> u16 {
        self.adc.read(&mut self.poti1)
    }
    pub fn poti2(&mut self) -> u16 {
        self.adc2.read(&mut self.poti2)
    }
}

impl Board {
    pub fn init() -> Self {
        let mut config = embassy_stm32::Config::default();
        config.enable_debug_during_sleep = true;
        config.rcc.adc_clock_source = rcc::AdcClockSource::PER;
        let p = embassy_stm32::init(config);

        let led_red = gpio::Output::new(p.PB14, gpio::Level::High, gpio::Speed::Low).degrade();
        let led_yellow = gpio::Output::new(p.PE1, gpio::Level::High, gpio::Speed::Low).degrade();
        let led_green = gpio::Output::new(p.PB0, gpio::Level::High, gpio::Speed::Low).degrade();
        let btn_user = gpio::Input::new(p.PC13, gpio::Pull::None).degrade();

        let joystick_button_matrix = JoystickButtonMatrixPins {
            dpad1_input: gpio::Input::new(p.PE2, gpio::Pull::Down).degrade(), // black
            dpad2_input: gpio::Input::new(p.PF7, gpio::Pull::Down).degrade(), // brown
            buttons_input: gpio::Input::new(p.PD10, gpio::Pull::Down).degrade(), // lila
            fire_lock_input: gpio::Input::new(p.PE4, gpio::Pull::Down).degrade(), // red
            lock_dpad_right_btn_a_select: gpio::Output::new(
                p.PE6,
                gpio::Level::Low,
                gpio::Speed::Low,
            )
            .degrade(), // yellow
            fire_dpad_up_btn_c_select: gpio::Output::new(p.PE3, gpio::Level::Low, gpio::Speed::Low)
                .degrade(), // orange
            dpad_left_launch_select: gpio::Output::new(p.PF9, gpio::Level::Low, gpio::Speed::Low)
                .degrade(), // green
            dpad_down_btn_b_select: gpio::Output::new(p.PF8, gpio::Level::Low, gpio::Speed::Low)
                .degrade(), // blue
        };

        let throttle_button_matrix = ThrottleButtonMatrixPins {
            dpad1_input: gpio::Input::new(p.PC12, gpio::Pull::Down).degrade(), // Light green
            dpad2_input: gpio::Input::new(p.PG8, gpio::Pull::Down).degrade(),  // green
            mode_aux_input: gpio::Input::new(p.PG10, gpio::Pull::Down).degrade(), // BLK/WT
            input: gpio::Input::new(p.PD2, gpio::Pull::Down).degrade(),        // GREY
            dpad_right_select: gpio::Output::new(p.PC8, gpio::Level::Low, gpio::Speed::Low)
                .degrade(), // BLUE
            dpad_up_select: gpio::Output::new(p.PC9, gpio::Level::Low, gpio::Speed::Low).degrade(), // BLK
            dpad_left_select_select: gpio::Output::new(p.PC10, gpio::Level::Low, gpio::Speed::Low)
                .degrade(), // BRN/WT
            dpad_down_select: gpio::Output::new(p.PC11, gpio::Level::Low, gpio::Speed::Low)
                .degrade(), // PING
        };

        let stlink_usart = usart::Uart::new(
            p.USART3,
            p.PD9,
            p.PD8,
            Irqs,
            dma::NoDma,
            dma::NoDma,
            usart::Config::default(),
        )
        .unwrap();

        let mut adc2 = adc::Adc::new(p.ADC2, &mut Delay);
        adc2.set_sample_time(adc::SampleTime::Cycles32_5);
        adc2.set_resolution(adc::Resolution::TwelveBit);

        let mut adc = adc::Adc::new(p.ADC1, &mut Delay);
        adc.set_sample_time(adc::SampleTime::Cycles32_5);
        adc.set_resolution(adc::Resolution::TwelveBit);

        let analog_inputs = AnalogInput {
            adc,
            adc2,
            x: p.PA3,
            y: p.PC0,          // A1 on board
            rudder: p.PF11,    // A2 on board
            throttle: p.PC2_C, // A3 on board
            poti1: p.PB1,      // A4 on board
            poti2: p.PC3_C,    // A5 on board
        };

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
            throttle_button_matrix,
            analog_inputs,
        }
    }
}
