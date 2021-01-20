use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::time::{ SystemTime, UNIX_EPOCH };

pub struct Pid {
    p: f64,
    i: f64,
    d: f64,
    min: f64,
    max: f64,
    i_accumulator: f64,
    first_time: bool,
    previous_error: f64,
    logfile: Option<File>
}

impl Pid {
    pub fn new(p: f64, i: f64, d: f64, min: f64, max: f64) -> Self {
        Pid { p: p, i: i, d: d, min: min, max: max, i_accumulator: 0.0, first_time: true, previous_error: 0.0, logfile: None }
    }

    pub fn compute(&mut self, value: f64, target_value: f64) -> f64 {
        let error: f64 = target_value - value;

        if self.first_time {
            self.previous_error = error;
            self.i_accumulator = 0.0;
            self.first_time = false;
        }

        self.i_accumulator += self.i * error;

        let mut output: f64 = (self.p * error) + self.i_accumulator + (self.d * (error - self.previous_error));

        if output > self.max {
            output = self.max;
        }
        else if output < self.min {
            output = self.min;
        }

        match self.logfile {
            Some(ref mut logfile) => {
                writeln!(logfile, "{},{},{},{},{},{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(), error, self.p * error, self.i_accumulator, self.d * (error - self.previous_error), output).unwrap();
            }
            _ => {}
        }

        self.previous_error = error;

        output
    }

    pub fn set_logfile(&mut self, logfile_name: &str) {
        let logfile_path = Path::new(logfile_name);
        let mut file = File::create(&logfile_path).unwrap();
        writeln!(file, "Time,Error,P,I-Accumulator,D,Output").unwrap();
        self.logfile = Some(file);
    }
    
    pub fn reset(&mut self) {
        self.first_time = true;
    }
}
