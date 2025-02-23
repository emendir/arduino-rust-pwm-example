use arduino_hal::port::mode::Floating;
use arduino_hal::port::mode::Input;
use arduino_hal::port::mode::Output;
use arduino_hal::port::Pin;
use arduino_hal::port::Pins;
use atmega_hal::port::Dynamic;
use core::convert::Infallible;
use embedded_hal::digital::OutputPin;

use atmega_hal::Atmega;
use atmega_hal::Usart;
use avr_device::atmega328p::USART0;
use avr_device::atmega328p::{TC0, TC1, TC2};

// Wrapper around the Pin struct that additionally contains the pin number
pub struct NumberedPin<Mode> {
    pin: Pin<Mode, Dynamic>,
    number: u8,
}

impl<Mode> NumberedPin<Mode> {
    pub fn new(pin: Pin<Mode, Dynamic>, number: u8) -> Self {
        Self { pin, number }
    }

    pub fn pin_number(&self) -> u8 {
        self.number
    }
}

// Implement deref to allow using all Pin methods directly
impl<Mode> core::ops::Deref for NumberedPin<Mode> {
    type Target = Pin<Mode, Dynamic>;

    fn deref(&self) -> &Self::Target {
        &self.pin
    }
}

impl<Mode> core::ops::DerefMut for NumberedPin<Mode> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pin
    }
}


// PWM (Pulse Width Modulation) functionality
pub struct PWM {
    pub timer0: TC0, // Controls D5, D6
    pub timer1: TC1, // Controls D9, D10
    pub timer2: TC2, // Controls D3, D11
}

impl PWM {
    // Constructor for PWM that sets up the timers using the timer clocks
    pub fn new(tc0: arduino_hal::pac::TC0, tc1: arduino_hal::pac::TC1, tc2: arduino_hal::pac::TC2) -> Self {
        tc0.tccr0a.write(|w| {
            w.wgm0()
                .bits(0b01)
                .com0a()
                .match_clear()
                .com0b()
                .match_clear()
        });
        tc0.tccr0b
            .write(|w| w.wgm02().clear_bit().cs0().prescale_64()); // WGM02 in TCCR0B

        tc1.tccr1a.write(|w| {
            w.wgm1()
                .bits(0b01)
                .com1a()
                .match_clear()
                .com1b()
                .match_clear()
        });
        tc1.tccr1b
            .write(|w| w.wgm1().bits(0b01).cs1().prescale_64()); // Timer1 uses wgm1() directly

        tc2.tccr2a.write(|w| {
            w.wgm2()
                .bits(0b01)
                .com2a()
                .match_clear()
                .com2b()
                .match_clear()
        });
        tc2.tccr2b
            .write(|w| w.wgm22().clear_bit().cs2().prescale_64()); // WGM22 in TCCR2B

        PWM { timer0: tc0, timer1: tc1, timer2: tc2 }
    }

    pub fn write(&mut self, pin: &NumberedPin<Output>, duty: u8) {
        self.write_to_pin_number(pin.pin_number(), duty)
    }
    pub fn write_to_pin_number(&mut self, pin: u8, duty: u8) {
        match pin {
            3 => self.timer2.ocr2b.write(|w| unsafe { w.bits(duty) }),
            5 => self.timer0.ocr0b.write(|w| unsafe { w.bits(duty) }),
            6 => self.timer0.ocr0a.write(|w| unsafe { w.bits(duty) }),
            9 => self.timer1.ocr1a.write(|w| unsafe { w.bits(duty as u16) }),
            10 => self.timer1.ocr1b.write(|w| unsafe { w.bits(duty as u16) }),
            11 => self.timer2.ocr2a.write(|w| unsafe { w.bits(duty) }),
            _ => {} // Ignore invalid pins
        }
    }
}

// Our boards pins
pub struct PinMap {
    pub pin_2: NumberedPin<Output>,
    pub pin_3: NumberedPin<Output>,
    pub pin_4: NumberedPin<Output>,
    pub pin_5: NumberedPin<Output>,
    pub pin_6: NumberedPin<Output>,
    pub pin_7: NumberedPin<Output>,
    pub pin_8: NumberedPin<Output>,
    pub pin_9: NumberedPin<Output>,
    pub pin_10: NumberedPin<Output>,
    pub pin_11: NumberedPin<Output>,
    pub pin_12: NumberedPin<Output>,
    pub pin_13: NumberedPin<Output>,
}

// Update the setup_pins function to use the new PWM constructor
pub fn setup_pins() -> (PinMap, PWM) {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let pwm = PWM::new(dp.TC0, dp.TC1, dp.TC2); // Create PWM instance using the timer clocks

    (
        PinMap {
            pin_2: NumberedPin::new(pins.d2.into_output().downgrade(), 2),
            pin_3: NumberedPin::new(pins.d3.into_output().downgrade(), 3),
            pin_4: NumberedPin::new(pins.d4.into_output().downgrade(), 4),
            pin_5: NumberedPin::new(pins.d5.into_output().downgrade(), 5),
            pin_6: NumberedPin::new(pins.d6.into_output().downgrade(), 6),
            pin_7: NumberedPin::new(pins.d7.into_output().downgrade(), 7),
            pin_8: NumberedPin::new(pins.d8.into_output().downgrade(), 8),
            pin_9: NumberedPin::new(pins.d9.into_output().downgrade(), 9),
            pin_10: NumberedPin::new(pins.d10.into_output().downgrade(), 10),
            pin_11: NumberedPin::new(pins.d11.into_output().downgrade(), 11),
            pin_12: NumberedPin::new(pins.d12.into_output().downgrade(), 12),
            pin_13: NumberedPin::new(pins.d13.into_output().downgrade(), 13),
        },
        pwm,
    )
}


