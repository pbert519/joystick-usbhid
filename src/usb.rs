use crate::hid::JoystickReport;
use embassy_usb::{class::hid, control};
use static_cell::StaticCell;
use usbd_hid::descriptor::SerializedDescriptor;

struct UsbStaticData {
    device_descriptor: [u8; 256],
    config_descriptor: [u8; 256],
    bos_descriptor: [u8; 256],
    control_buf: [u8; 64],
    state: hid::State<'static>,
    request_handler: UsbRequestHandler,
}
static USB_STATIC_DATA: StaticCell<UsbStaticData> = StaticCell::new();

pub fn setup_usb<D: embassy_usb::driver::Driver<'static>>(
    usb_driver: D,
) -> (
    hid::HidWriter<'static, D, 10>,
    embassy_usb::UsbDevice<'static, D>,
) {
    let mut usb_config = embassy_usb::Config::new(0xc0de, 0xcafe);
    usb_config.manufacturer = Some("Pbert");
    usb_config.product = Some("usb joystick");
    usb_config.serial_number = Some("12345678");

    let usb_static_data = USB_STATIC_DATA.init(UsbStaticData {
        device_descriptor: [0; 256],
        config_descriptor: [0; 256],
        bos_descriptor: [0; 256],
        control_buf: [0; 64],
        state: hid::State::new(),
        request_handler: UsbRequestHandler {},
    });

    let mut builder = embassy_usb::Builder::new(
        usb_driver,
        usb_config,
        &mut usb_static_data.device_descriptor,
        &mut usb_static_data.config_descriptor,
        &mut usb_static_data.bos_descriptor,
        &mut usb_static_data.control_buf,
    );

    let writer: hid::HidWriter<_, 10> = hid::HidWriter::new(
        &mut builder,
        &mut usb_static_data.state,
        hid::Config {
            report_descriptor: JoystickReport::desc(),
            request_handler: Some(&usb_static_data.request_handler),
            poll_ms: 60,
            max_packet_size: 8,
        },
    );

    let usb = builder.build();

    (writer, usb)
}

#[embassy_executor::task]
pub async fn usb_task(
    mut usb: embassy_usb::UsbDevice<
        'static,
        embassy_stm32::usb_otg::Driver<'static, embassy_stm32::peripherals::USB_OTG_HS>,
    >,
) -> ! {
    usb.run().await
}

struct UsbRequestHandler {}

impl hid::RequestHandler for UsbRequestHandler {
    fn get_report(&self, id: hid::ReportId, _buf: &mut [u8]) -> Option<usize> {
        defmt::info!("Get report for {:?}", id);
        None
    }

    fn set_report(&self, id: hid::ReportId, data: &[u8]) -> control::OutResponse {
        defmt::info!("Set report for {:?}: {=[u8]}", id, data);
        control::OutResponse::Accepted
    }

    fn set_idle_ms(&self, id: Option<hid::ReportId>, dur: u32) {
        defmt::info!("Set idle rate for {:?} to {:?}", id, dur);
    }

    fn get_idle_ms(&self, id: Option<hid::ReportId>) -> Option<u32> {
        defmt::info!("Get idle rate for {:?}", id);
        None
    }
}
