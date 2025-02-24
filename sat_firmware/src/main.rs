// #![no_std]
// #![no_main]

// use cortex_m_rt::entry;
// use panic_halt as _; // Half on panic

// use core::fmt::Write;
// use nb::block;
// use stm32l4xx_hal::{
//     flash::Parts,
//     gpio::{self, Edge, Input, PullUp},
//     pac::{self, exti, syscfg, EXTI, NVIC, SYSCFG, USART2, interrupt},
//     prelude::*,
//     pwr::Pwr,
//     rcc::{Clocks, Rcc},
//     serial::{Config, Rx, Serial, Tx},
//     time::Hertz,
// };
// use heapless::String;

// static mut TX: Option<Tx<USART2>> = None;

// #[entry]
// fn main() -> ! {
//     let mut dp = pac::Peripherals::take().unwrap(); // Take ownership of all device peripherals
//     let cp = cortex_m::Peripherals::take().unwrap();

//     // Configure the RCC clock with 80 MHz system core frequency.
//     let mut rcc: Rcc = dp.RCC.constrain(); // take ownership of RCC/constrain returns it in a rust-friendly way
//     let mut pwr: Pwr = dp.PWR.constrain(&mut rcc.apb1r1);
//     let mut flash: Parts = dp.FLASH.constrain(); // get flash parts then use for ACR
//     let sysclock_freq = Hertz::MHz(80);
//     let clocks: Clocks = rcc
//         .cfgr
//         .sysclk(sysclock_freq)
//         .freeze(&mut flash.acr, &mut pwr);

//     // Configure the pin for a button press. It's accessible through AHB2 bus.
//     let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);

//     let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
//     led.set_high();

//     // let mut moder = gpioa.moder; // mode of the pin
//     // let mut pupdr = gpioa.pupdr; // pull-up/pull-down state register

//     let tx_pin = gpioa
//         .pa2
//         .into_alternate(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
//     let rx_pin = gpioa
//         .pa3
//         .into_alternate(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);

//     let serial = Serial::usart2(
//         dp.USART2,
//         (tx_pin, rx_pin),
//         Config::default().baudrate(115_200.bps()),
//         clocks,
//         &mut rcc.apb1r1,
//     );

//     let (mut tx, _) = serial.split();

//     for byte in b"System starting...\r\n" {
//         block!(tx.write(*byte)).ok();
//     }

//     // the moder and pupdr will be modified as a result of this method call
//     // needed for the clear ownership/one modification at a time on one pin
//     // PA0 is configured to pull up res/we're connecting the pin to a HIGH voltage
//     // by default.
//     let mut button = gpioa
//         .pa0
//         .into_pull_up_input(&mut gpioa.moder, &mut gpioa.pupdr);

//     for byte in b"Configuring button interrupt...\r\n" {
//         block!(tx.write(*byte)).ok();
//     }

//     // Configure EXTI for button press.
//     let mut exti: EXTI = dp.EXTI;
//     // let mut syscfg: SYSCFG = dp.SYSCFG;
//     // SYSCFG clock must be enabled in order to do register writes
//     // button.make_interrupt_source(&mut syscfg, &mut rcc.apb2);
//     button.make_interrupt_source(&mut dp.SYSCFG, &mut rcc.apb2);
//     // generate interrupt on falling edge (HIGH -> LOW detects the button press event)
//     button.trigger_on_edge(&mut exti, Edge::Falling);
//     button.enable_interrupt(&mut exti);

//     // Enable NVIC Interrupt for EXTI0
//     unsafe {
//         NVIC::unmask(pac::Interrupt::EXTI0);
//     }

//     for byte in b"Interrupt configured, ready!\r\n" {
//         block!(tx.write(*byte)).ok();
//     }

//     // Why is unsafe used?
//     // Global static variables are unsafe in Rust because they don’t enforce 
//     // borrow checking like normal variables. If multiple parts of the code 
//     // access TX at the same time, data races could occur. In this case, we 
//     // ensure only one part of the code (EXTI0 interrupt handler) modifies 
//     // TX, so we manually use unsafe to acknowledge this.
//     unsafe {
//         TX = Some(tx);
//     }
//     // Blink LED to indicate we've reached the main loop
//     for _ in 0..5 {
//         led.toggle();
//         cortex_m::asm::delay(8_000_000); // Crude delay
//     }

//     // Turn off LED before entering sleep loop
//     led.set_low();

//     loop {
//         // Briefly flash LED so we know the device hasn't crashed
//         led.set_high();
//         cortex_m::asm::delay(100_000);
//         led.set_low();
//         cortex_m::asm::wfi(); // low power mode until an interrupt occurs
//     }
// }

// /**
//  * Interrupt handler for hte button press action to start downlink of satellite data.
//  */
// #[interrupt]
// fn EXTI0() {
//     let dp = unsafe { pac::Peripherals::steal() };
//     let exti = &dp.EXTI;

//     // clear EXTI0 interrupt flag by writing 1 to it; 
//     // we clear the flag to prevent repeated interrupts.
//     exti.pr1.write(|w| w.pr0().set_bit());

//     send_downlink_request(37.7749, -122.4194);
// }

// fn send_downlink_request(lat: f32, long: f32) {
//     unsafe {
//         if let Some(tx) = TX.as_mut() {
//             // initialize a fix-sized heapless string since
//             // we don't have access to heap allocation
//             let mut buffer = String::<64>::new();
//             write!(&mut buffer, "DOWNLINK:{:.4},{:.4}\n", lat, long).unwrap();
            
//             for byte in buffer.as_bytes() {
//                 block!(tx.write(*byte)).ok();
//             }
//         }
//     }
// }

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32l4xx_hal::{
    prelude::*,
    pac,
};

#[entry]
fn main() -> ! {
    // Get access to the device peripherals
    let dp = pac::Peripherals::take().unwrap();
    
    // Get access to the core peripherals
    let cp = cortex_m::Peripherals::take().unwrap();
    
    // Get access to the RCC (Reset and Clock Control)
    let mut rcc = dp.RCC.constrain();
    
    // Get access to GPIOA
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    
    // Configure PA5 as output (LED on most STM32L4 boards)
    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    
    // Simple blink loop
    loop {
        // Turn LED on
        let _ = led.set_high();
        
        // Delay
        cortex_m::asm::delay(8_000_000);
        
        // Turn LED off
        let _ = led.set_low();
        
        // Delay
        cortex_m::asm::delay(8_000_000);
    }
}
