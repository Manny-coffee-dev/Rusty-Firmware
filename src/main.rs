#![no_std]
#![no_main]
#[macro_use(entry)]
extern crate cortex_m_rt as rt;
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
extern crate m;
extern crate stm32g0;
extern crate stm32g0xx_hal as hal;
extern crate cortex_m;

use hal::gpio::gpioa::PA15;
use hal::gpio::{Floating, Input, Output, PushPull};
use hal::prelude::*;

use stm32g0::stm32g031::Peripherals;

#[entry]
fn entry_wrap() -> ! {
    let p = Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();

    // splits the GPIOA peripheral into 16 independent pins + registers
    let mut gpioa = p.GPIOA.split(&mut rcc);
    // all pins start as floating inputs
    let pa15: PA15<Input<Floating>> = gpioa.pa15;

    // API available in the `Input<_>` state

    // configure the pin PA15 as an output
    // this operation consumes the original `pa15` value
    let mut pa15: PA15<Output<PushPull>> = pa15.into_push_pull_output();

    loop {
        pa15.set_low().unwrap();
    }
}
