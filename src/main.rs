// std and main are not available for bare metal software
#![no_std]
#![no_main]

use cortex_m::asm::delay;
use cortex_m_rt::entry; // The runtime
use embedded_hal::digital::v2::OutputPin; // the `set_high/low`function
#[allow(unused_imports)]
use panic_halt;
use stm32g0::stm32g031;
use stm32g0xx_hal::prelude::*; // STM32F1 specific functions // When a panic occurs, stop the microcontroller

// This marks the entrypoint of our application. The cortex_m_rt creates some
// startup code before this, but we don't need to worry about this
#[entry]
fn main() -> ! {
    // Get handles to the hardware objects. These functions can only be called
    // once, so that the borrowchecker can ensure you don't reconfigure
    // something by accident.
    let dp = stm32g031::Peripherals::take().unwrap();
    // GPIO pins on the STM32F1 must be driven by the APB2 peripheral clock.
    // This must be enabled first. The HAL provides some abstractions for
    // us: First get a handle to the RCC peripheral:
    let mut rcc = dp.RCC.constrain();
    // Now we have access to the RCC's registers. The GPIOC can be enabled in
    // RCC_APB2ENR (Prog. Ref. Manual 8.3.7), therefore we must pass this
    // register to the `split` function.
    let mut gpioa = dp.GPIOA.split(&mut rcc);
    // This gives us an exclusive handle to the GPIOC peripheral. To get the
    // handle to a single pin, we need to configure the pin first. Pin A15
    // is usually connected to the Bluepills onboard LED.
    let mut led = gpioa.pa15.into_push_pull_output();

    // Now, enjoy the lightshow!
    loop {
        led.set_high().ok();
        delay(12000000);
        led.set_low().ok();
        delay(12000000);
    }
}
