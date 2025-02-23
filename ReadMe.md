## Arduino Rust PWM Example

This repo demonstrates how to work with an Arduino's PWM outputs in a clean programmer-friendly way. 
It packages all the low-level interaction with timer clocks that underly PWM functionality, exposing simple functions like Arduino's analogWrite for rust.

Currently, this project is only an example to spare you the trouble of figuring out how to work with Arduino PWM in rust.
It would be good to eventually package this into a proper crate with support for as many Arduino boards as possible, but I don't have the time to do that myself, so feel free to take up such a project yourself.

### Code Example

This is what interacting with PWM outputs looks like when using this project's code:
```rs
// Initialise board IO
let (mut pin_map, mut pwm) = setup_pins();

// Pin Output Examples:

// PWM output 
pwm.write(&pin_map.pin_5, 100);

// PWM output given pin number, like Arduino's analogWrite
pwm.write_to_pin_number(3, 150);


// ordinary digital output
pin_map.pin_13.set_high();
```

### Usage

- Learn how to work with arduinos in rust: https://blog.logrocket.com/complete-guide-running-rust-arduino/
- Create a project for your specific arduino board (better than cloning this repo, ensures the config for your arduino board is correct)
- Replace your project's rust code in the `src` directory with this repository's.
- If using a board other than the Arduino nano, you may need to adjust the pin declarations and the PWM timer clocks and channels in `arduino_io.rs`.

