#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use arduino_hal::simple_pwm::*;
use panic_halt as _;
use ufmt;

const MAX_ANGLE: u8 = 180;
const MAX_POT_VALUE: u16 = 1023;

// adaptation from https://www.arduino.cc/reference/en/language/functions/math/map/
fn scale_range(x: u16, in_min: u16, in_max: u16, out_min: u8, out_max: u8) -> u8 {
    // Perform the calculation with u32 to avoid overflow
    let x = x as u32;
    let in_min = in_min as u32;
    let in_max = in_max as u32;
    let out_min = out_min as u32;
    let out_max = out_max as u32;

    let scaled = (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;

    scaled as u8
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let pot_pin = pins.a0.into_analog_input(&mut adc);

    let timer = Timer1Pwm::new(dp.TC1, Prescaler::Prescale256);
    let mut servo_pin = pins.d9.into_output().into_pwm(&timer);
    servo_pin.enable();

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    loop {
        let pot_val = pot_pin.analog_read(&mut adc);
        let scaled_angle = scale_range(pot_val, 0, MAX_POT_VALUE, 1, MAX_ANGLE);
        ufmt::uwriteln!(
            &mut serial,
            "Current pot value: {}, scaled {}",
            pot_val,
            scaled_angle
        )
        .void_unwrap();

        servo_pin.set_duty(scaled_angle);
        arduino_hal::delay_ms(10);
    }
}
