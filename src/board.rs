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
}

impl Board {
    pub fn init() -> Self {
        let config = embassy_stm32::Config::default();
        let p = embassy_stm32::init(config);

        let led_red = gpio::Output::new(p.PB14, gpio::Level::High, gpio::Speed::Low).degrade();
        let led_yellow = gpio::Output::new(p.PE1, gpio::Level::High, gpio::Speed::Low).degrade();
        let led_green = gpio::Output::new(p.PB0, gpio::Level::High, gpio::Speed::Low).degrade();
        let btn_user = gpio::Input::new(p.PC13, gpio::Pull::None).degrade();

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
        }
    }
}
