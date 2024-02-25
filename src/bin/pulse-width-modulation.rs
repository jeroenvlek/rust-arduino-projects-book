/*!
 * Read three filtered (RGB) light sensors and try to recreate the sensor input
 * into the RGB led
 *
 */
#![no_std]
#![no_main]

use arduino_hal::hal::port::{PC0, PC1, PC2};
use arduino_hal::port::mode::Analog;
use arduino_hal::port::Pin;
use arduino_hal::prelude::*;
use panic_halt as _;
use ufmt;

use arduino_hal;
use arduino_hal::simple_pwm::*;
use arduino_hal::Adc;

struct SensorRead(u16, u16, u16);

// Should probably get the ADC as a member to avoid passing it twice
struct RGBSensor {
    red_sensor: Pin<Analog, PC0>,
    green_sensor: Pin<Analog, PC1>,
    blue_sensor: Pin<Analog, PC2>,
}

impl RGBSensor {
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

    // Note the pins on the rgb led are actually in order rbg
    let timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);
    let mut r_pin = pins.d11.into_output().into_pwm(&timer2);
    r_pin.enable();

    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
    let mut b_pin = pins.d10.into_output().into_pwm(&timer1);
    b_pin.enable();

    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let mut g_pin = pins.d6.into_output().into_pwm(&timer0);
    g_pin.enable();

    let sensor = RGBSensor {
        red_sensor: pins.a0.into_analog_input(&mut adc),
        green_sensor: pins.a1.into_analog_input(&mut adc),
        blue_sensor: pins.a2.into_analog_input(&mut adc),
    };

    loop {
        let rgb_read = sensor.read(&mut adc);
        ufmt::uwriteln!(&mut serial, "Current red: {}", rgb_read.0).void_unwrap();
        ufmt::uwriteln!(&mut serial, "Current green: {}", rgb_read.1).void_unwrap();
        ufmt::uwriteln!(&mut serial, "Current blue: {}", rgb_read.2).void_unwrap();
        ufmt::uwriteln!(&mut serial, "").void_unwrap();

        r_pin.set_duty(rgb_read.0 as u8);
        g_pin.set_duty(rgb_read.1 as u8);
        b_pin.set_duty(rgb_read.2 as u8);

        arduino_hal::delay_ms(15);
    }
}
