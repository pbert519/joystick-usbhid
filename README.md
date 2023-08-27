# joystick-usbhid

Migrating a Saitek X36F gamepeort joystick by replacing the orignal electronics with a stm32 nucelo board.

Written in Rust with the async runtime embassy-rs.

Implements a USB HID Device, which should work fine with the default linux and windows drivers.
