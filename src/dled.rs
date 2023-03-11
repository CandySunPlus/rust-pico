#![no_std]
#![no_main]

use core::ops::Range;

use cortex_m::delay::Delay;
use defmt::*;
use embedded_hal::PwmPin;
use rp_pico::hal::pwm::{self, Slice};
use rp_pico::hal::{clocks, pac, Clock, Sio, Watchdog};
use rp_pico::{entry, Pins, XOSC_CRYSTAL_FREQ};
use {defmt_rtt as _, panic_probe as _};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = clocks::init_clocks_and_plls(
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

    let sio = Sio::new(pac.SIO);

    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let mut pwm_slices = pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    let pwm = &mut pwm_slices.pwm7;
    pwm.set_ph_correct();
    pwm.enable();

    pwm.channel_a.output_to(pins.gpio14);
    pwm.channel_a.enable();
    pwm.channel_b.output_to(pins.gpio15);
    pwm.channel_b.enable();

    let colors = [0xff00, 0x00ff, 0x0ff0, 0xf00f];
    info!("start colors");
    loop {
        for color in colors {
            set_color(color, pwm, &mut delay);
            delay.delay_ms(500);
        }
    }
}

fn set_color(color: u16, pwm: &mut Slice<pwm::Pwm7, pwm::FreeRunning>, delay: &mut Delay) {
    let mut red = color >> 8;
    let mut green = color & 0x00ff;

    red = pwm_map(red, 0..255, 0..65535);
    green = pwm_map(green, 0..255, 0..65535);

    pwm.channel_a.set_duty(red);
    delay.delay_us(8);
    pwm.channel_b.set_duty(green);
    delay.delay_us(8);
}

fn pwm_map(v: u16, in_range: Range<u16>, out_range: Range<u16>) -> u16 {
    (v - in_range.start) * out_range.len() as u16 / in_range.len() as u16 + out_range.start
}
