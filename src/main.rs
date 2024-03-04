#![no_std]
#![no_main]

use arduino_hal::delay_ms;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // set PWM-10 as output and high.
    let mut led = pins.d10.into_output();
    led.set_high();

    loop {
        // basic turn off and turn on LCD
        delay_ms(300);
        led.set_low();
        delay_ms(300);
        led.set_high();
        // led.toggle();
    }
}
