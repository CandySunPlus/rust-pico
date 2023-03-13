#![no_std]
#![no_main]

use defmt::*;
use embedded_hal::PwmPin;
use rp_pico::hal::{self, Clock};
use rp_pico::{entry, pac, XOSC_CRYSTAL_FREQ};
use smart_leds::RGB8;
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

    let pwm5 = &mut pwm_slices.pwm5;
    pwm5.set_ph_correct();
    pwm5.enable();

    let pwm6 = &mut pwm_slices.pwm6;
    pwm6.set_ph_correct();
    pwm6.enable();

    let pwm_red = &mut pwm5.channel_b;
    pwm_red.output_to(pins.gpio11);
    let pwm_green = &mut pwm6.channel_a;
    pwm_green.output_to(pins.gpio12);
    let pwm_blue = &mut pwm6.channel_b;
    pwm_blue.output_to(pins.gpio13);

    let colors: [RGB8; 6] = [
        (255, 0, 0).into(),
        (0, 255, 0).into(),
        (0, 0, 255).into(),
        (255, 255, 0).into(),
        (255, 0, 255).into(),
        (0, 255, 255).into(),
    ];

    loop {
        for color in &colors {
            let r = (color.r as f32 / 255.0 * 62500.0) as u16;
            let g = (color.g as f32 / 255.0 * 62500.0) as u16;
            let b = (color.b as f32 / 255.0 * 62500.0) as u16;
            info!("R:{} G:{} B:{}", r, g, b);
            pwm_red.set_duty(r);
            pwm_green.set_duty(g);
            pwm_blue.set_duty(b);
            delay.delay_ms(1000);
        }
    }
}
