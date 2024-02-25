#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use arduino_hal::simple_pwm::*;

use panic_halt as _;

use ufmt;
use ufmt_float::uFmt_f32;

const MAX_ANGLE: u16 = 179;
const MAX_POT_VALUE: u16 = 1023;

// adaptation from https://www.arduino.cc/reference/en/language/functions/math/map/
fn scale_range(x: u16, in_min: u16, in_max: u16, out_min: u16, out_max: u16) -> u16 {
    return x * out_max / in_max;
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let pot_pin = pins.a0.into_analog_input(&mut adc);

    // Important because this sets the bit in the DDR register!
    pins.d9.into_output();

    // - TC1 runs off a 250kHz clock, with 5000 counts per overflow => 50 Hz signal.
    // - Each count increases the duty-cycle by 4us.
    // - Use OC1A which is connected to D9 of the Arduino Uno.
    let tc1 = dp.TC1;
    tc1.icr1.write(|w| unsafe { w.bits(4999) });
    tc1.tccr1a
        .write(|w| w.wgm1().bits(0b10).com1a().match_clear());
    tc1.tccr1b
        .write(|w| w.wgm1().bits(0b11).cs1().prescale_64());

    loop {
        // let pot_val = pot_pin.analog_read(&mut adc);
        // let scaled_angle = scale_range(pot_val, 0, MAX_POT_VALUE, 0, MAX_ANGLE);
        // ufmt::uwriteln!(&mut serial, "Current pot value: {}, scaled {}", pot_val, scaled_angle).void_unwrap();

        for scaled_angle in 0..=(179 as u8) {
            servo_pin.set_duty(scaled_angle);
            arduino_hal::delay_ms(100)
        }
    }
}
