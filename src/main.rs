#![no_std]
#![no_main]
use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_halt as _;
use stm32f1xx_hal::*;

#[entry]
fn main() -> ! {
    loop{}
}
