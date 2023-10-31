#![no_std]
#![no_main]

use ag_lcd::*;
use libm::*;
use panic_halt as _;
#[arduino_hal::entry]
fn main() -> ! {
    // initialize access to peripherals as dp according to convention
    let dp = arduino_hal::Peripherals::take().unwrap();
    // store the pins accessed from dp as a Pins struct
    let pins = arduino_hal::pins!(dp);
    // init a Delay struct to get easy access to a delay function for our board
    let delay = arduino_hal::Delay::new();

    //data pins for lcd screen
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

    // coefficients for steinhart-hart equation to translate voltage to temp reading
    let coef_a = 0.001129148;
    let coef_b = 0.000234125;
    let coef_c = 0.0000000876741;

    // a value for on board resistance is required for steinhart-hart equation
    let on_board_r = 10_000f64;
    //R2 = R1 * (1023.0 / (float)Vo - 1.0)

    // inifinite loop. required as hinted from the return type of main
    loop {
        // clear the screen at the start of each loop
        lcd.clear();
        // get temp reading
        let reading = a0.analog_read(&mut ad_convert);
        // let num = libm::log(23f64);
        let resistance = on_board_r * (1023.0f64 / reading as f64 - 1.0f64);
        let log_of_res = log(resistance.into());
        // let log_of_res_cube = powf(log_of_res, 3.0);
        // let tmp_kelvin = 1.0 / coef_a + coef_b * log_of_res + coef_c * log_of_res_cube;
        // let temp_c = tmp_kelvin - 273.15;
        // let temper = temp_c as u32;

        // lcd.print can only take a &str (string slice), so we use the itoa crate to make
        // a stack allocated buffer to format our u16 int readig into a &str
        // let mut buffer = itoa::Buffer::new();
        // let print = buffer.format(temper);

        lcd.print("yayaya");

        // delay for 5000 ms
        arduino_hal::delay_ms(5000);
    }
}
