/*!
 * Read the temperature sensor via the Analog Digital Converter (ADC) and display
 * the readout via LED indicators. Based on Project #3 the Love-o-meter, but with more leds.
 * 
 * Note that determining the amount of leds to light up could be done without any floats, but 
 * I wanted to mess around with floats and I learned something interesting:
 * 
 * https://github.com/avr-rust/rust-legacy-fork/issues/149
 * 
 */
#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

use arduino_hal;
use arduino_hal::port::{mode, Pin};

use ufmt;
// use ufmt_float::uFmt_f32;

const BASELINE_TEMP : f32 = 20.0;
const MAX_TEMP : f32 = 30.0;
const VOLTAGE_SCALE : f32 = 5.0 / 1024.0;

fn sensor_read_to_temp(sensor_read : u16) -> f32 {
    let voltage = sensor_read as f32 * VOLTAGE_SCALE;
    let temp = (voltage - 0.5) * 100.0;
    return temp
}

fn compute_num_leds_on(temp: f32, total_leds: usize) -> usize {
    let safe_temp: f32 = if temp < BASELINE_TEMP { 
        BASELINE_TEMP 
    } 
    else if temp > MAX_TEMP {
        MAX_TEMP
    }
    else {
        temp
    };

    let num_leds_on = ((safe_temp - BASELINE_TEMP) / (MAX_TEMP - BASELINE_TEMP)) * (total_leds as f32);

    return (num_leds_on + 0.5) as usize; // with rounding
}


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let temp_sensor = pins.a0.into_analog_input(&mut adc);

    let mut leds: [Pin<mode::Output>; 6] = [
        pins.d3.into_output().downgrade(),
        pins.d4.into_output().downgrade(),
        pins.d5.into_output().downgrade(),
        pins.d6.into_output().downgrade(),
        pins.d7.into_output().downgrade(),
        pins.d8.into_output().downgrade(),
    ];

    loop {
        let current_read = temp_sensor.analog_read(&mut adc);
        let current_temp = sensor_read_to_temp(current_read);
        ufmt::uwrite!(&mut serial, "Current read: {}", current_read).void_unwrap();
        // This causes a linker error, see: https://github.com/avr-rust/rust-legacy-fork/issues/149
        // ufmt::uwrite!(&mut serial, "Current temp: {}", uFmt_f32::Two(current_temp)).void_unwrap();

        let num_leds_on = compute_num_leds_on(current_temp, leds.len());
        for (i, led) in leds.iter_mut().enumerate() {
            if i < num_leds_on {
                led.set_high();
            } 
            else {
                led.set_low();
            }
        }

        ufmt::uwriteln!(&mut serial, "").void_unwrap();
        arduino_hal::delay_ms(500);
    }
}
