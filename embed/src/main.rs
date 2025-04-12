#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::{Delay, Timer};
use gpio::{Input, Level, Output, Pull};
use loadcell::{LoadCell, hx711};
use {defmt_rtt as _, panic_probe as _};

// extern crate pkgcore;

const FRAME_TIME: u64 = 100;
const DEADZONE: f32 = 0.1;
const CALIBRATE_MIN: f32 = 0.0;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let mut board_led = Output::new(peripherals.PIN_16, Level::Low);

    let mut output_a = Output::new(peripherals.PIN_15, Level::Low); // GPIO15; PIN 20
    let mut output_b = Output::new(peripherals.PIN_26, Level::Low); // GPIO26; PIN 31

    let endstop_input = Input::new(peripherals.PIN_7, Pull::Up); // GPIO7; PIN 10

    let hx711_sck = Output::new(peripherals.PIN_8, Level::Low); // GPIO8; PIN 11
    let hx711_dt = Input::new(peripherals.PIN_14, Pull::None); // GPIO14; PIN 19

    // create the load sensor
    let delay = Delay {};
    let mut load_sensor = hx711::HX711::new(hx711_sck, hx711_dt, delay);
    // zero the readings
    load_sensor.tare(16);
    load_sensor.set_scale(1.0);

    loop {
        board_led.set_low();
        output_a.set_low();
        output_b.set_low();

        // send output when endpoint is clicked
        if endstop_input.is_high() {
            board_led.set_high();
            output_a.set_high();
        }

        // wait for the load sensor
        if load_sensor.is_ready() {
            let reading = load_sensor.read_scaled().unwrap_or(0.0);
            let is_over = reading - DEADZONE >= CALIBRATE_MIN;
            if is_over {
                board_led.set_high();
                output_b.set_high();
            }
        }

        // wait for the frame time to go through
        Timer::after_millis(FRAME_TIME).await;
    }
}
