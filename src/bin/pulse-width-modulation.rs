/*!
 * Read three filtered (RGB) light sensors and try to recreate the sensor input
 * into the RGB led
 * 
 */
#![no_std]
#![no_main]

use ufmt;
use arduino_hal::hal::port::{PC0, PC1, PC2};
use arduino_hal::port::mode::Analog;
use panic_halt as _;
use arduino_hal::prelude::*;

use arduino_hal::Adc;
use arduino_hal;
use arduino_hal::port::{mode, Pin};

struct SensorRead(u16, u16, u16);

struct RGBSensor{
    red_sensor: Pin<Analog, PC0>,
    green_sensor: Pin<Analog, PC1>,
    blue_sensor: Pin<Analog, PC2>
}

impl RGBSensor  {
    fn read(&self, adc: &mut Adc) -> SensorRead {
        let r = self.red_sensor.analog_read(adc);
        arduino_hal::delay_ms(5);
        let g = self.green_sensor.analog_read(adc);
        arduino_hal::delay_ms(5);
        let b = self.blue_sensor.analog_read(adc);
        arduino_hal::delay_ms(5);
        
        SensorRead(r, g, b)
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let mut pwm_output: [Pin<mode::Output>; 3] = [
        pins.d11.into_output().downgrade(),
        pins.d10.into_output().downgrade(),
        pins.d9.into_output().downgrade()
    ];

    let sensor = RGBSensor {
        red_sensor: pins.a0.into_analog_input(&mut adc),
        green_sensor: pins.a1.into_analog_input(&mut adc),
        blue_sensor: pins.a2.into_analog_input(&mut adc)
    };

    loop {
        let rgb_read = sensor.read(&mut adc);
        ufmt::uwriteln!(&mut serial, "Current red: {}", rgb_read.0).void_unwrap();
        ufmt::uwriteln!(&mut serial, "Current green: {}", rgb_read.1).void_unwrap();
        ufmt::uwriteln!(&mut serial, "Current blue: {}", rgb_read.2).void_unwrap();
        ufmt::uwriteln!(&mut serial, "").void_unwrap();
        arduino_hal::delay_ms(500);
    }
}
