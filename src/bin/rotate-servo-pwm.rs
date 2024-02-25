/*!
 * Use PWM to rotate the SM-S2309S servo in the Arduino kit 180 degrees and back.
 */
#![no_std]
#![no_main]

use arduino_hal::simple_pwm::*;
use panic_halt as _;


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let timer = Timer1Pwm::new(dp.TC1, Prescaler::Prescale256);
    let mut servo_pin = pins.d9.into_output().into_pwm(&timer);
    servo_pin.enable();

    let min_duty = 1;
    let max_duty = 254;

    loop {
        for duty in (min_duty..=max_duty).chain((min_duty..max_duty).rev()) {
            servo_pin.set_duty(duty);
            arduino_hal::delay_ms(10);
        }
    }
}
