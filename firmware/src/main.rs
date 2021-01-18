mod thunderborg;
mod encoder;

use rppal::system::DeviceInfo;
use rppal::gpio::Gpio;
use thunderborg::Thunderborg;
use encoder::Encoder;

use std::time::Duration;

fn main() {
    println!("Running on a {}.", DeviceInfo::new().unwrap().model());
    let mut thunderborg = Thunderborg::new(0x15);
    thunderborg.set_led_show_battery(false);
    thunderborg.set_led_1(0.0, 0.0, 0.0);
    thunderborg.set_motor_1(-1.0);
    thunderborg.set_motor_2(1.0);
    let gpio = Gpio::new().unwrap();
    let left_encoder = Encoder::new(&gpio, 26, 16);
    let right_encoder = Encoder::new(&gpio, 23, 24);
    loop {
        let left_steps: i64 = *left_encoder.steps.lock().unwrap();
        let right_steps: i64 = *right_encoder.steps.lock().unwrap();
        let left_revs: f32 = left_steps as f32 / 897.96;
        let right_revs: f32 = right_steps as f32 / 897.96;
        if left_revs >= 10.0 {
            break;
        }
    }
    thunderborg.set_motor_1(0.0);
    thunderborg.set_motor_2(0.0);
}
