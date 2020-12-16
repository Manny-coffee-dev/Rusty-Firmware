// std and main are not available for bare metal software
#![no_std]
#![no_main]

use cortex_m::asm::delay;
use cortex_m_rt::entry; // The runtime
use embedded_hal::digital::v2::OutputPin; // the `set_high/low`function
#[allow(unused_imports)]
use panic_halt; // When a panic occurs, stop the microcontroller
use stm32g0::stm32g031;
use stm32g0xx_hal::prelude::*; // STM32G0 specific functions

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
    let gpioa = dp.GPIOA.split(&mut rcc);
    // This gives us an exclusive handle to the GPIOC peripheral. To get the
    // handle to a single pin, we need to configure the pin first. Pin A15
    // is usually connected to the Bluepills onboard LED.
    let mut led = gpioa.pa15.into_push_pull_output();

    // TIM1
    let pwm = dp.TIM1.pwm(10.khz(), &mut rcc);
    let mut pwm_ch1 = pwm.bind_pin(gpioa.pa8);
    let max = pwm_ch1.get_max_duty();
    pwm_ch1.set_duty(max / 2);
    pwm_ch1.enable();
    cortex_m::asm::bkpt();

    // Now, enjoy the light show and spinning!
    loop {
        led.set_high().ok();
        pwm_ch1.set_duty(max / 8);
        delay(12000000);
        cortex_m::asm::bkpt();
        led.set_low().ok();
        pwm_ch1.set_duty(max / 2);
        delay(12000000);
        cortex_m::asm::bkpt();
    }
}
