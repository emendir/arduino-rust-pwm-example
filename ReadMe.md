## Arduino Rust PWM Example

This repo demonstrates how to work with an Arduino's PWM outputs in a clean programmer-friendly way. 
It packages all the low-level interaction with timer clocks that underly PWM functionality, exposing simple functions like Arduino's analogWrite for rust.

Currently, this project is only an example to spare you the trouble of figuring out how to work with Arduino PWM in rust.
It would be good to eventually package this into a proper crate with support for as many Arduino boards as possible, but I don't have the time to do that myself, so feel free to take up such a project yourself.
Also, I'd like to take what I developed here further than PWM. See [Structure](#structure) below as well as my [list of next steps](./ToDo.md).

### Code Example

This is what interacting with PWM outputs looks like when using this project's code:
```rs
// Initialise board IO
let (mut pin_map, mut pwm) = setup_pins();

// Pin Output Examples:

// PWM output 
pwm.write(&pin_map.pin_5, 100);     // set pin 5 to PWM level 100/255

// PWM output given pin number, like Arduino's analogWrite
pwm.write_to_pin_number(3, 150);    // set pin 3 to PWM level 100/255


// ordinary digital output
pin_map.pin_13.set_high();
```

### Usage

- Learn how to work with arduinos in rust: https://blog.logrocket.com/complete-guide-running-rust-arduino/
- Create a project for your specific arduino board (better than cloning this repo, ensures the config for your arduino board is correct)
- Replace your project's rust code in the `src` directory with this repository's.
- If using a board other than the Arduino nano, you may need to adjust the pin declarations and the PWM timer clocks and channels in `arduino_io.rs`.
- If you want to change output pins into intput pins, you will need to make the changes both in the `setup_pins` function and the `PinMap` struct.

### Structure
All the pin setup and configuration is in the `arduino_io.rs` file.
The PWM struct takes care of configuring the timer clocks needed for PWM and provides the programmer-friendly functions `PWM.write()` and `PWM.write_to_pin_number()`.
The pin objects are stored in the `PinMap` struct, and can be used when writing normal digital outputs as well as when writing PWM outputs.
The `setup_pins` function initialises both the `PWM` and `PinMap` structs.

I find this way of organising the pin objects very useful, even beyond working with PWM (in fact, I developed this structure before working with PWM).
I'd like to add similarly programmer-friendly methods for analog reading.
See my [list of next steps](./ToDo.md).
