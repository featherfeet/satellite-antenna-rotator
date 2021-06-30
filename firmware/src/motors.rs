use std::thread;
use crate::Pid;
use crate::thunderborg::Thunderborg;
use crate::Encoder;
use atomicfloat::AtomicF64;
use std::sync::Arc;
use rppal::gpio::Gpio;
use std::time::{ SystemTime, UNIX_EPOCH };
use std::sync::atomic::{ AtomicBool, Ordering };

const STEPS_CHANGED_BUFFER_SIZE: usize = 5;
const STEPS_PER_REVOLUTION: f64 = 897.96;

struct Motor {
    encoder: Encoder,
    pid: Pid,
    prev_steps: i64,
    prev_time: u128,
    steps_changed_buffer: [i64; STEPS_CHANGED_BUFFER_SIZE],
    steps_changed_buffer_index: usize
}

impl Motor {
    pub fn new(gpio: Arc::<Gpio>, encoder_channel_a_pin: u8, encoder_channel_b_pin: u8) -> Motor {
        let encoder = Encoder::new(gpio, encoder_channel_a_pin, encoder_channel_b_pin);
        let mut pid = Pid::new(-2.0, -0.025, -1.8, -1.0, 1.0);

        Motor { encoder: encoder, pid: pid, prev_steps: 0, prev_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros(), steps_changed_buffer: [0_i64; STEPS_CHANGED_BUFFER_SIZE], steps_changed_buffer_index: 0 }
    }

    pub fn update_and_compute_power_level(&mut self, target_speed: f64) -> f64 {
        let time: u128 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();
        let time_elapsed: u128 = time - self.prev_time;

        let steps: i64 = *self.encoder.steps.lock().unwrap();
        let steps_changed: i64 = steps - self.prev_steps;
        self.steps_changed_buffer[self.steps_changed_buffer_index] = steps_changed;
        self.steps_changed_buffer_index += 1;
        if self.steps_changed_buffer_index == STEPS_CHANGED_BUFFER_SIZE {
            self.steps_changed_buffer_index = 0;
        }

        let mut steps_changed_averaged: f64 = 0.0;
        for i in 0..STEPS_CHANGED_BUFFER_SIZE {
            steps_changed_averaged += self.steps_changed_buffer[i] as f64;
        }
        steps_changed_averaged /= STEPS_CHANGED_BUFFER_SIZE as f64;

        let speed: f64 = (steps_changed_averaged / (time_elapsed as f64 / 1.0e6)) / STEPS_PER_REVOLUTION;

        self.prev_time = time;
        self.prev_steps = steps;

        self.pid.compute(speed, target_speed)
    }
}

pub struct Motors {
    finish: Arc::<AtomicBool>,
    target_speed_1: Arc::<AtomicF64>,
    target_speed_2: Arc::<AtomicF64>,
    control_thread: thread::JoinHandle::<()>
}

impl Motors {
    pub fn new(gpio: Arc::<Gpio>, encoder_1_channel_a_pin: u8, encoder_1_channel_b_pin: u8, encoder_2_channel_a_pin: u8, encoder_2_channel_b_pin: u8) -> Motors {
        let finish = Arc::new(AtomicBool::new(false));
        let target_speed_1 = Arc::new(AtomicF64::new(0.0));
        let target_speed_2 = Arc::new(AtomicF64::new(0.0));

        let finish_ref = Arc::clone(&finish);
        let target_speed_1_ref = Arc::clone(&target_speed_1);
        let target_speed_2_ref = Arc::clone(&target_speed_2);
        let control_thread = thread::spawn(move || {
            let mut thunderborg = Thunderborg::new(0x19);
            let mut motor_1 = Motor::new(Arc::clone(&gpio), encoder_1_channel_a_pin, encoder_1_channel_b_pin);
            let mut motor_2 = Motor::new(Arc::clone(&gpio), encoder_2_channel_a_pin, encoder_2_channel_b_pin);

            while !finish_ref.load(Ordering::Relaxed) {
                let power_1: f64 = motor_1.update_and_compute_power_level(target_speed_1_ref.load(Ordering::Relaxed));
                let power_2: f64 = motor_2.update_and_compute_power_level(target_speed_2_ref.load(Ordering::Relaxed));

                thunderborg.set_motor_1(power_1);
                thunderborg.set_motor_2(power_2);

                std::thread::sleep(std::time::Duration::from_millis(5));
            }

            thunderborg.set_motor_1(0.0);
            thunderborg.set_motor_2(0.0);
        });

        Motors { finish: finish, target_speed_1: target_speed_1, target_speed_2: target_speed_2, control_thread: control_thread }
    }

    pub fn set_target_speed_1(&mut self, target_speed_1: f64) {
        self.target_speed_1.store(target_speed_1, Ordering::Relaxed);
    }

    pub fn set_target_speed_2(&mut self, target_speed_2: f64) {
        self.target_speed_2.store(target_speed_2, Ordering::Relaxed);
    }

    pub fn finish(self) {
        self.finish.store(true, Ordering::Relaxed);
        self.control_thread.join().expect("Failed to join motor control thread!");
    }
}
