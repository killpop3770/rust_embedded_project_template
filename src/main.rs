#![no_std]
#![no_main]
use cortex_m_rt::entry;
use defmt_rtt as _;
use panic_halt as _;
use stm32f1xx_hal::{
    flash::FlashExt, gpio::GpioExt, i2c::{BlockingI2c, DutyCycle, Mode}, pac::Peripherals, prelude::*, rcc::RccExt
};

#[entry]
fn main() -> ! {

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain();
    let clocks: stm32f1xx_hal::rcc::Clocks = rcc.cfgr.use_hse(8.MHz()).freeze(&mut flash.acr);
    let mut gpiob = dp.GPIOB.split();
    let mut delay = cp.SYST.delay(&clocks);

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000.Hz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        1000,
        10,
        1000,
        1000,
    );

    loop{}
}
