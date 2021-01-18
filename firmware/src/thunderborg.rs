extern crate rppal;

use rppal::i2c::I2c;

pub struct Thunderborg {
    i2c: I2c
}

impl Thunderborg {
    const COMMAND_GET_ID: u8 = 0x99;
    const I2C_MAX_LEN: usize = 6;
    const COMMAND_SET_LED_BATT_MON: u8 = 6;
    const COMMAND_VALUE_ON: u8 = 1;
    const COMMAND_VALUE_OFF: u8 = 0;
    const PWM_MAX: u8 = 255;
    const COMMAND_SET_LED1: u8 = 1;
    const COMMAND_SET_I2C_ADD: u8 = 0xAA;
    const I2C_ID_THUNDERBORG: u8 = 0x15;
    const COMMAND_SET_B_REV: u8 = 12;
    const COMMAND_SET_B_FWD: u8 = 11;
    const COMMAND_SET_A_REV: u8 = 9;
    const COMMAND_SET_A_FWD: u8 = 8;

    pub fn new(address: u16) -> Thunderborg {
        let mut i2c_bus: I2c = I2c::new().unwrap();
        println!("Thunderborg: Opened I2C bus {} with clock speed {} Hz.", i2c_bus.bus(), i2c_bus.clock_speed().unwrap());
        i2c_bus.set_slave_address(address).unwrap();
        let mut buf: [u8; Thunderborg::I2C_MAX_LEN] = [0; Thunderborg::I2C_MAX_LEN];
        i2c_bus.write(&Thunderborg::COMMAND_GET_ID.to_ne_bytes()).unwrap();
        i2c_bus.read(&mut buf).unwrap();
        if buf[1] == Thunderborg::I2C_ID_THUNDERBORG {
            println!("Thunderborg: Found Thunderborg device.");
        }
        else {
            println!("Thunderborg: ERROR, failed to find Thunderborg device on I2C bus.");
        }
        Thunderborg { i2c: i2c_bus }
    }

    pub fn set_led_show_battery(&mut self, state: bool) {
        let buf: [u8; 2] = if state {
                               [ Thunderborg::COMMAND_SET_LED_BATT_MON, Thunderborg::COMMAND_VALUE_ON ]
                           }
                           else {
                               [ Thunderborg::COMMAND_SET_LED_BATT_MON, Thunderborg::COMMAND_VALUE_OFF ]
                           };
        self.i2c.write(&buf).unwrap();
    }
    
    pub fn set_led_1(&mut self, r: f32, g: f32, b: f32) {
        let r_int: u8 = (r * (Thunderborg::PWM_MAX as f32)) as u8;
        let g_int: u8 = (g * (Thunderborg::PWM_MAX as f32)) as u8;
        let b_int: u8 = (b * (Thunderborg::PWM_MAX as f32)) as u8;

        let buf: [u8; 4] = [Thunderborg::COMMAND_SET_LED1, r_int, g_int, b_int];

        self.i2c.write(&buf).unwrap();
    }

    pub fn set_new_address(&mut self, new_address: u8) {
        if new_address < 0x03 || new_address > 0x77 {
            println!("Thunderborg: ERROR, I2C addresses below 0x03 and above 0x77 are reserved.");
        }
        else {
            let buf: [u8; 2] = [Thunderborg::COMMAND_SET_I2C_ADD, new_address];
            self.i2c.write(&buf).unwrap();
            self.i2c.set_slave_address(new_address.into()).unwrap();

            let mut read_buf: [u8; Thunderborg::I2C_MAX_LEN] = [0; Thunderborg::I2C_MAX_LEN];
            self.i2c.write(&Thunderborg::COMMAND_GET_ID.to_ne_bytes()).unwrap();
            self.i2c.read(&mut read_buf).unwrap();
            if read_buf[1] == Thunderborg::I2C_ID_THUNDERBORG {
                println!("Thunderborg: Successfully changed Thunderborg I2C address to 0x{:x?}.", new_address);
            }
            else {
                println!("Thunderborg: ERROR, failed to change Thunderborg I2C address to 0x{:x?}.", new_address);
            }
        }
    }

    pub fn set_motor_1(&mut self, power: f32) {
        let command: u8;
        let mut pwm: u8;
        if power < 0.0 {
            command = Thunderborg::COMMAND_SET_A_REV;
            pwm = -(power * (Thunderborg::PWM_MAX as f32)) as u8;
        }
        else {
            command = Thunderborg::COMMAND_SET_A_FWD;
            pwm = (power * (Thunderborg::PWM_MAX as f32)) as u8;
        }
        if pwm > Thunderborg::PWM_MAX {
            pwm = Thunderborg::PWM_MAX;
        }

        let buf: [u8; 2] = [command, pwm];
        self.i2c.write(&buf).unwrap();
    }

    pub fn set_motor_2(&mut self, power: f32) {
        let command: u8;
        let mut pwm: u8;
        if power < 0.0 {
            command = Thunderborg::COMMAND_SET_B_REV;
            pwm = -(power * (Thunderborg::PWM_MAX as f32)) as u8;
        }
        else {
            command = Thunderborg::COMMAND_SET_B_FWD;
            pwm = (power * (Thunderborg::PWM_MAX as f32)) as u8;
        }
        if pwm > Thunderborg::PWM_MAX {
            pwm = Thunderborg::PWM_MAX;
        }

        let buf: [u8; 2] = [command, pwm];
        self.i2c.write(&buf).unwrap();
    }
}
