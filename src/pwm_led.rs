#![no_std]
#![no_main]

use defmt::*;
use embedded_hal::PwmPin;
use rp_pico::hal::{self, Clock};
use rp_pico::{entry, pac, XOSC_CRYSTAL_FREQ};
use {defmt_rtt as _, panic_probe as _};

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    let clocks = hal::clocks::init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = hal::Sio::new(pac.SIO);

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    let pwm = &mut pwm_slices.pwm4;
    pwm.set_ph_correct();
    pwm.enable();

    let channel = &mut pwm.channel_b;
    channel.output_to(pins.gpio25);

    loop {
        for i in 0..25000 {
            delay.delay_us(20);
            channel.set_duty(i);
        }

        for i in (0..25000).rev() {
            delay.delay_us(20);
            channel.set_duty(i);
        }

        delay.delay_ms(500);
    }
}
