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

/// Represents a single speed-controlled motor.
struct Motor {
    /// Quadrature encoder for this motor.
    encoder: Encoder,
    /// PID used to control the speed of this motor.
    pid: Pid,
    /// Number of encoder steps from the previous time that update() was called. Used to calculate speed.
    prev_steps: i64,
    /// Time since Unix epoch (in microseconds) since the last time that update() was called. Used to calculate speed.
    prev_time: u128,
    /// Buffer of steps_changed (number of steps between calls to update()) values. Used to smooth out noise introduced by quantization error.
    steps_changed_buffer: [i64; STEPS_CHANGED_BUFFER_SIZE],
    /// Index to write to in steps_changed_buffer. Circles around back to 0 when it reaches STEPS_CHANGED_BUFFER_SIZE so that the buffer is "circular."
    steps_changed_buffer_index: usize
}

impl Motor {
    /// Create a motor.
    ///
    /// # Arguments
    ///
    /// * `gpio` - Arc pointing to a RPPAL Gpio object.
    ///
    /// * `encoder_channel_a_pin` - GPIO pin number for channel A of the quadrature encoder.
    ///
    /// * `encoder_channel_b_pin` - GPIO pin number for channel B of the quadrature encoder.
    pub fn new(gpio: Arc::<Gpio>, encoder_channel_a_pin: u8, encoder_channel_b_pin: u8) -> Motor {
        let encoder = Encoder::new(gpio, encoder_channel_a_pin, encoder_channel_b_pin);
        let mut pid = Pid::new(-2.0, -0.025, -1.8, -1.0, 1.0);

        Motor { encoder: encoder, pid: pid, prev_steps: 0, prev_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros(), steps_changed_buffer: [0_i64; STEPS_CHANGED_BUFFER_SIZE], steps_changed_buffer_index: 0 }
    }

    /// Update the state of the motor.
    ///
    /// # Arguments
    ///
    /// * `target_speed` - The target speed for the motor PID, given in revolutions per second.
    ///
    /// Returns a tuple of two floats. The first float is the power level (from -1.0 to 1.0) that
    /// should be sent to the motor. The second float is the current motor speed in revolutions per
    /// second.
    pub fn update(&mut self, target_speed: f64) -> (f64, f64) {
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

        let power: f64 = self.pid.compute(speed, target_speed);
        
        let revs: f64 = steps as f64 / STEPS_PER_REVOLUTION;

        (power, revs)
    }
}

/// Represents two speed-controlled motors. Does NOT handle position control, only speed. This struct's job is to keep the two motors running as close to their target speeds as possible.
pub struct Motors {
    /// Set this to true to stop the motors.
    finish: Arc::<AtomicBool>,
    /// Target speed (in revolutions per second) for motor 1.
    target_speed_1: Arc::<AtomicF64>,
    /// Target speed (in revolutions per second) for motor 2.
    target_speed_2: Arc::<AtomicF64>,
    /// Total number of revolutions done by motor 1 (decreases if the motor turns backwards, increases if it turns forwards).
    revs_1: Arc::<AtomicF64>,
    /// Total number of revolutions done by motor 2 (decreases if the motor turns backwards, increases if it turns forwards).
    revs_2: Arc::<AtomicF64>,
    /// Handle for the thread that runs the motor speed PIDs.
    control_thread: thread::JoinHandle::<()>
}

impl Motors {
    /// Create a Motors structure.
    ///
    /// # Arguments
    ///
    /// * `gpio` - Arc pointing to a RPPAL Gpio object.
    ///
    /// * `encoder_1_channel_a_pin` - GPIO pin number for channel A of the encoder for motor 1.
    ///
    /// * `encoder_1_channel_b_pin` - GPIO pin number for channel B of the encoder for motor 1.
    ///
    /// * `encoder_2_channel_a_pin` - GPIO pin number for channel A of the encoder for motor 2.
    ///
    /// * `encoder_2_channel_b_pin` - GPIO pin number for channel B of the encoder for motor 2.
    pub fn new(gpio: Arc::<Gpio>, encoder_1_channel_a_pin: u8, encoder_1_channel_b_pin: u8, encoder_2_channel_a_pin: u8, encoder_2_channel_b_pin: u8) -> Motors {
        let finish = Arc::new(AtomicBool::new(false));
        let target_speed_1 = Arc::new(AtomicF64::new(0.0));
        let target_speed_2 = Arc::new(AtomicF64::new(0.0));
        let revs_1 = Arc::new(AtomicF64::new(0.0));
        let revs_2 = Arc::new(AtomicF64::new(0.0));

        let finish_ref = Arc::clone(&finish);
        let target_speed_1_ref = Arc::clone(&target_speed_1);
        let target_speed_2_ref = Arc::clone(&target_speed_2);
        let revs_1_ref = Arc::clone(&revs_1);
        let revs_2_ref = Arc::clone(&revs_2);
        let control_thread = thread::spawn(move || {
            let mut thunderborg = Thunderborg::new(0x19);
            let mut motor_1 = Motor::new(Arc::clone(&gpio), encoder_1_channel_a_pin, encoder_1_channel_b_pin);
            let mut motor_2 = Motor::new(Arc::clone(&gpio), encoder_2_channel_a_pin, encoder_2_channel_b_pin);

            while !finish_ref.load(Ordering::Relaxed) {
                let (power_1, revs_1) = motor_1.update(target_speed_1_ref.load(Ordering::Relaxed));
                let (power_2, revs_2) = motor_2.update(target_speed_2_ref.load(Ordering::Relaxed));

                thunderborg.set_motor_1(power_1);
                thunderborg.set_motor_2(power_2);
                revs_1_ref.store(revs_1, Ordering::Relaxed);
                revs_2_ref.store(revs_2, Ordering::Relaxed);

                std::thread::sleep(std::time::Duration::from_millis(5));
            }

            thunderborg.set_motor_1(0.0);
            thunderborg.set_motor_2(0.0);
        });

        Motors { finish: finish, target_speed_1: target_speed_1, target_speed_2: target_speed_2, revs_1: revs_1, revs_2: revs_2, control_thread: control_thread }
    }

    /// Set the target speed (in revolutions per second) for the speed PID of motor 1.
    pub fn set_target_speed_1(&mut self, target_speed_1: f64) {
        self.target_speed_1.store(target_speed_1, Ordering::Relaxed);
    }

    /// Set the target speed (in revolutions per second) for the speed PID of motor 2.
    pub fn set_target_speed_2(&mut self, target_speed_2: f64) {
        self.target_speed_2.store(target_speed_2, Ordering::Relaxed);
    }

    /// Get the total number of revolutions done by motor 1 (increases for forward rotation, decreases for backward).
    pub fn get_revs_1(&mut self) -> f64 {
        return self.revs_1.load(Ordering::Relaxed);
    }

    /// Get the total number of revolutions done by motor 2 (increases for forward rotation, decreases for backward).
    pub fn get_revs_2(&mut self) -> f64 {
        return self.revs_2.load(Ordering::Relaxed);
    }

    /// Stop the motors and end the motor control thread.
    pub fn finish(self) {
        self.finish.store(true, Ordering::Relaxed);
        self.control_thread.join().expect("Failed to join motor control thread!");
    }
}
