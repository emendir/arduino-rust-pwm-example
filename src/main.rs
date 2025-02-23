#![no_std]
#![no_main]

use arduino_hal::delay_ms;
use panic_halt as _;
use ufmt::uWrite;

mod arduino_io;
use arduino_io::*;

#[arduino_hal::entry]
fn main() -> ! {
    // Initialise board IO
    let (mut pin_map, mut pwm) = setup_pins();
    
    // Pin Output Examples:
    
    // ordinary digital output
    pin_map.pin_13.set_high();
    
    // PWM output 
    pwm.write(&pin_map.pin_5, 100);
    
    // PWM output given pin number, like Arduino's analogWrite
    pwm.write_to_pin_number(3, 150);
    delay_ms(2000);
    
    
    
    
    // let's do something more fancy, such as fading
    loop {
        pin_map.pin_13.set_high();
        delay_ms(100);
        pin_map.pin_13.set_low();
        delay_ms(100);
        
        for duty in 0u8..=255u8 {
            pwm.write(&pin_map.pin_3, duty);
            delay_ms(20);
        }

        pwm.write(&pin_map.pin_3, 0);
        for duty in 0u8..=255u8 {
            pwm.write(&pin_map.pin_5, duty);
            delay_ms(20);
        }
        pwm.write(&pin_map.pin_5, 0);
        for duty in 0u8..=255u8 {
            pwm.write(&pin_map.pin_6, duty);
            delay_ms(20);
        }
        pwm.write(&pin_map.pin_6, 0);
    }
}
