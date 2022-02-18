#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d3.into_output();
    led.set_high();

    loop {
        (0..20).map(|i| i * 100).for_each(|ms| {
            led.toggle();
            arduino_hal::delay_ms(ms as u16);
        });
        (20..0).map(|i| i * 100).for_each(|ms| {
            led.toggle();
            arduino_hal::delay_ms(ms as u16);
        });
    }
}
