extern crate rppal;

use std::sync::{ Arc, Mutex };
use rppal::gpio::{ Gpio, Trigger, InputPin };

pub struct Encoder {
    _channel_a_pin: InputPin,
    _channel_b_pin: Arc<Mutex<InputPin>>,
    pub steps: Arc<Mutex<i64>>
}

impl Encoder {
    pub fn new(gpio: &Gpio, channel_a_pin_number: u8, channel_b_pin_number: u8) -> Encoder {
        let mut channel_a_pin = gpio.get(channel_a_pin_number).unwrap().into_input();
        let channel_b_pin = Arc::new(Mutex::new(gpio.get(channel_b_pin_number).unwrap().into_input()));
        let steps: Arc<Mutex<i64>> = Arc::new(Mutex::new(0));

        let channel_b_pin_reference = Arc::clone(&channel_b_pin);
        let steps_reference = Arc::clone(&steps);

        &mut channel_a_pin.set_async_interrupt(Trigger::RisingEdge, move |_level| {
            let mut steps = steps_reference.lock().unwrap();
            if channel_b_pin_reference.lock().unwrap().is_high() {
                *steps += 1;
            }
            else {
                *steps -= 1;
            }
        }).unwrap();

        Encoder { _channel_a_pin: channel_a_pin, _channel_b_pin: channel_b_pin, steps: steps }
    }
}
