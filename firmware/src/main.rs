mod thunderborg;
mod encoder;
mod pid;
mod motors;

extern crate gpredict;

#[macro_use]
extern crate text_io;

use rppal::system::DeviceInfo;
use rppal::gpio::Gpio;
use encoder::Encoder;
use pid::Pid;
use motors::Motors;
use std::sync::Arc;
use std::sync::atomic::{ AtomicBool, AtomicI16, Ordering };
use std::thread;
use gpredict::{ Predict, Location, Tle };

const DRIVING_ALTITUDE_GEAR_TEETH: f64 = 7.0;
const MAIN_ALTITUDE_GEAR_TEETH: f64 = 32.0;
const DRIVING_AZIMUTH_GEAR_TEETH: f64 = 7.0;
const MAIN_AZIMUTH_GEAR_TEETH: f64 = 32.0;
const ALTITUDE_GEAR_RATIO: f64 = MAIN_ALTITUDE_GEAR_TEETH / DRIVING_ALTITUDE_GEAR_TEETH;
const AZIMUTH_GEAR_RATIO: f64 = MAIN_AZIMUTH_GEAR_TEETH / DRIVING_AZIMUTH_GEAR_TEETH;

const ALTITUDE_ENCODER_STEPS_PER_REVOLUTION: f64 = 897.96;
const AZIMUTH_ENCODER_STEPS_PER_REVOLUTION: f64 = 897.96;

// Convert an altitude angle in degrees into a number of revolutions of the driving motor.
fn altitude_angle_to_driving_revs(altitude_angle: f64) -> f64 {
    -(altitude_angle / 360.0) * ALTITUDE_GEAR_RATIO
}

// Convert an azimuth angle in degrees into a number of revolutions of the driving motor.
fn azimuth_angle_to_driving_revs(azimuth_angle: f64) -> f64 {
    -(azimuth_angle / 360.0) * AZIMUTH_GEAR_RATIO
}

fn main() {
    println!("Running on a {}.", DeviceInfo::new().unwrap().model());
   
    let finish = Arc::new(AtomicBool::new(false));
    let go_home = Arc::new(AtomicBool::new(false));
    // let target_altitude = Arc::new(AtomicI16::new(90));
    let target_altitude = Arc::new(AtomicI16::new(0));
    let target_azimuth = Arc::new(AtomicI16::new(0));

    let gpio = Arc::new(Gpio::new().unwrap());

    let mut motors = Motors::new(Arc::clone(&gpio), 4, 17, 18, 23);

    let finish_ref = Arc::clone(&finish);
    let go_home_ref = Arc::clone(&go_home);
    let mut control_c_presses: u8 = 0;
    ctrlc::set_handler(move || {
        control_c_presses += 1;
        if control_c_presses == 1 {
            println!("\rControl-C pressed once, returning to home position. Press again for emergency stop.");
	        go_home_ref.store(true, Ordering::Relaxed);
        }
        else {
            println!("\rControl-C pressed twice! Stopping motors and exiting.");
            finish_ref.store(true, Ordering::Relaxed);
        }
    }).expect("Failed to set Control-C handler!");

    let target_altitude_ref = Arc::clone(&target_altitude);
    let target_azimuth_ref = Arc::clone(&target_azimuth);
    thread::spawn(move || {
        //*(altitude_encoder.steps.lock().unwrap()) = (altitude_angle_to_driving_revs(90.0) * ALTITUDE_ENCODER_STEPS_PER_REVOLUTION) as i64;

        let mut azimuth_pid = Pid::new(-2.0, -0.005, -20.0, -1.0, 1.0);
        let mut altitude_pid = Pid::new(-2.0, -0.005, -20.0, -1.0, 1.0);
        //azimuth_pid.set_logfile("azimuth_encoder.csv");
        //altitude_pid.set_logfile("altitude_encoder.csv");

        while !finish.load(Ordering::Relaxed) {
            let altitude_revs: f64 = motors.get_revs_1();
            let azimuth_revs: f64 = motors.get_revs_2();

            let target_revs_driving_altitude = altitude_angle_to_driving_revs(target_altitude_ref.load(Ordering::Relaxed) as f64);
            println!("target_revs_driving_altitude: {}", target_revs_driving_altitude);
            let altitude_motor_target_speed: f64 = altitude_pid.compute(altitude_revs, target_revs_driving_altitude);

            let target_revs_driving_azimuth = azimuth_angle_to_driving_revs(target_azimuth_ref.load(Ordering::Relaxed) as f64);
            let azimuth_motor_target_speed: f64 = azimuth_pid.compute(azimuth_revs, target_revs_driving_azimuth);

            motors.set_target_speed_1(altitude_motor_target_speed);
            motors.set_target_speed_2(azimuth_motor_target_speed);

            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        motors.finish();
        std::process::exit(0);
    });

    //let tle: Tle = Tle::from_file("JUGNU", "jugnu.tle").unwrap();
    let tle: Tle = Tle::from_file("ISS (ZARYA)", "iss.tle").unwrap();
    //let tle: Tle = Tle::from_file("LUSAT (LO-19)", "amateur.tle").unwrap();
    // HOME:
    let location: Location = Location { lat_deg: 37.649250, lon_deg: -121.875070, alt_m: 105.0 };
    // HILL:
    //let location: Location = Location { lat_deg: 37.650444, lon_deg: -121.866836, alt_m: 171.0 };
    let mut predict: Predict = Predict::new(&tle, &location);

    //predict.update(None);
    //println!("{:#?}", predict);

    loop {
        println!("Target altitude?");
        let target_altitude_input: i16 = read!();
        target_altitude.store(target_altitude_input, Ordering::Relaxed);
        println!("Target azimuth?");
        let target_azimuth_input: i16 = read!();
        target_azimuth.store(target_azimuth_input, Ordering::Relaxed);
        /*
        if go_home.load(Ordering::Relaxed) {
            target_altitude.store(90, Ordering::Relaxed);
	    target_azimuth.store(0, Ordering::Relaxed);
        }
        else {
            predict.update(None);
            println!("Altitude: {}", predict.sat.el_deg);
            println!("Azimuth: {}", predict.sat.az_deg);
            println!("Range rate: {} km/s", predict.sat.range_rate_km_sec);
            let frequency_shifted: f64 = (145.800 * 1.0e6) + ((predict.sat.range_rate_km_sec * 1000.0) / 2.998e8) * (145.800 * 1.0e6);
            println!("Doppler-shifted frequency: {} MHz", frequency_shifted / 1.0e6);
            target_altitude.store(predict.sat.el_deg as i16, Ordering::Relaxed);
            target_azimuth.store(predict.sat.az_deg as i16, Ordering::Relaxed);
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
        */
    }
}
