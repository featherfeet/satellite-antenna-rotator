mod thunderborg;
mod encoder;
mod pid;

use rppal::system::DeviceInfo;
use rppal::gpio::Gpio;
use thunderborg::Thunderborg;
use encoder::Encoder;
use pid::Pid;

fn main() {
    println!("Running on a {}.", DeviceInfo::new().unwrap().model());
    let mut thunderborg = Thunderborg::new(0x15);
    thunderborg.set_led_show_battery(false);
    thunderborg.set_led_1(0.0, 0.0, 0.0);
    thunderborg.set_motor_1(0.0);
    thunderborg.set_motor_2(0.0);
    let gpio = Gpio::new().unwrap();
    let left_encoder = Encoder::new(&gpio, 26, 16);
    let right_encoder = Encoder::new(&gpio, 23, 24);
    let mut left_pid = Pid::new(2.0, 0.0, 0.0, 0.0, 1.0);
    left_pid.set_logfile("left_encoder.csv");
    loop {
        let left_steps: i64 = *left_encoder.steps.lock().unwrap();
        let right_steps: i64 = *right_encoder.steps.lock().unwrap();

        let left_revs: f64 = left_steps as f64 / 897.96;
        let right_revs: f64 = right_steps as f64 / 897.96;

        let left_motor_power: f64 = left_pid.compute(left_revs, 3.0);
        thunderborg.set_motor_2(left_motor_power as f32);
    }
//    thunderborg.set_motor_1(0.0);
//    thunderborg.set_motor_2(0.0);
}
