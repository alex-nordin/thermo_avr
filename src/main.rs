#![no_std]
#![no_main]

use ag_lcd::*;
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
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

    // I'm using a KY-013 digital temp sensor, so need to translate an digital reading to analog
    let mut ad_convert = arduino_hal::Adc::new(dp.ADC, Default::default());

    // temp sensor is connected to the a0 pin
    let a0 = pins.a0.into_analog_input(&mut ad_convert);

    // inifinite loop. required as hinted from the return type of main
    loop {
        // clear the screen at the start of each loop
        lcd.clear();
        // get temp reading
        let reading = a0.analog_read(&mut ad_convert);
        // lcd.print can only take a &str (string slice), so we use the itoa crate to make
        // a stack allocated buffer to format our u16 int readig into a &str
        let mut buffer = itoa::Buffer::new();
        let print = buffer.format(reading);

        lcd.print(print);

        // delay for 5000 ms
        arduino_hal::delay_ms(5000);
    }
}
