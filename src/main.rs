#![no_std]
#![no_main]

use ag_lcd::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    // initialize access to peripherals as dp according to convention
    let dp = arduino_hal::Peripherals::take().unwrap();
    // store the pins accessed from dp as a Pins struct
    let pins = arduino_hal::pins!(dp);
    // init a Delay struct to get easy access to a delay function for our board
    let delay = arduino_hal::Delay::new();

    //data pins
    let d4 = pins.d4.into_output().downgrade();
    let d5 = pins.d5.into_output().downgrade();
    let d6 = pins.d6.into_output().downgrade();
    let d7 = pins.d7.into_output().downgrade();

    //register select pin
    let rs = pins.d12.into_output().downgrade();
    //"enable" pin
    let en = pins.d11.into_output().downgrade();

    let mut lcd: LcdDisplay<_, _> = LcdDisplay::new(rs, en, delay)
        // set the display to 4 pin mode usgin "with_half_bus" function
        .with_half_bus(d4, d5, d6, d7)
        .with_display(Display::On)
        .build();

    // inifinite loop. required as hinted from the return type of main
    loop {
        lcd.print("TEST");
    }
}
