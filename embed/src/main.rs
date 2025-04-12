#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Input, Level, Output, Pull};
use {defmt_rtt as _, panic_probe as _};

extern crate pkgcore;

const FRAME_TIME: u64 = 100;
const DEADZONE: f32 = 0.1;
const CALIBRATE_MILLIS: u64 = 500;
const CALIBRATE_COUNT: usize = 20;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let mut board_led = Output::new(peripherals.PIN_16, Level::Low);

    let mut output_a = Output::new(peripherals.PIN_15, Level::Low); // GPIO15; PIN 20
    let mut output_b = Output::new(peripherals.PIN_26, Level::Low); // GPIO26; PIN 31

    let endstop_input = Input::new(peripherals.PIN_7, Pull::Up); // GPIO7; PIN 10

    let _hx711_sck = Output::new(peripherals.PIN_8, Level::Low); // GPIO8; PIN 11
    let _hx711_dt = Input::new(peripherals.PIN_14, Pull::None); // GPIO14; PIN 19

    // calibrate
    let mut calibrate_getter = || 0.0;
    let calibrate_sleep = || async {
        Timer::after_millis(CALIBRATE_MILLIS).await;
    };
    let load_min =
        pkgcore::calibrate_min_sleep(&mut calibrate_getter, &calibrate_sleep, CALIBRATE_COUNT)
            .await;

    loop {
        board_led.set_low();
        output_a.set_low();
        output_b.set_low();

        // send output when endpoint is clicked
        if endstop_input.is_high() {
            board_led.set_high();
            output_a.set_high();
        }

        // TODO: need to get the value from the adc but how? what to do with dt/sck?
        let num = 0.0;
        if pkgcore::is_num_over(num, load_min, DEADZONE) {
            board_led.set_high();
            output_b.set_high();
        }

        // wait for the frame time to go through
        Timer::after_millis(FRAME_TIME).await;
    }
}
