#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    starship_interface()
}

fn getting_started() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let mut led = pins.d8.into_output();

    loop {
        led.toggle();
        arduino_hal::delay_ms(250);
    }
}

fn starship_interface() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let input = pins.d2.into_pull_up_input();
    let mut green = pins.d3.into_output();
    let mut red_one = pins.d4.into_output();
    let mut red_two = pins.d5.into_output();

    loop {
        if input.is_low() {
            green.set_high();
            red_one.set_low();
            red_two.set_low();
        } else {
            green.set_low();
            red_one.set_low();
            red_two.set_high();

            arduino_hal::delay_ms(250);

            red_one.set_high();
            red_two.set_low();

            arduino_hal::delay_ms(250);
        }
    }
}
