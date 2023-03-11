//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use bsp::entry;
use bsp::hal::clocks::{init_clocks_and_plls, Clock};
use bsp::hal::prelude::_rphal_pio_PIOExt;
use bsp::hal::sio::Sio;
use bsp::hal::watchdog::Watchdog;
use bsp::hal::{pac, rom_data, Timer};
use defmt::*;
// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;
use smart_leds::{
    brightness,
    hsv::{hsv2rgb, Hsv},
    SmartLedsWrite, RGB8,
};
use ws2812_pio::Ws2812;
use {defmt_rtt as _, panic_probe as _};

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let sin = rom_data::float_funcs::fsin::ptr();

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);

    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);

    let mut ws = Ws2812::new(
        pins.gpio2.into_mode(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    let mut leds: [RGB8; 4] = [(0, 0, 0).into(); 4];
    let mut t = 0.0;

    let strip_brightness = 64u8;
    let animation_speed = 0.1;

    loop {
        for (i, led) in leds.iter_mut().enumerate() {
            let hue_offs = match i % 3 {
                1 => 0.25,
                2 => 0.5,
                _ => 0.0,
            };

            let sin_11 = sin((t + hue_offs) * 2.0 * core::f32::consts::PI);

            let sin_01 = (sin_11 + 1.0) * 0.5;

            let hue = 360.0 as u8 * sin_01 as u8;
            let sat = 1.0 as u8;
            let val = 1.0 as u8;

            let rgb = hsv2rgb(Hsv { hue, sat, val });

            info!("R{}G{}B{}", rgb.r, rgb.g, rgb.b);

            *led = rgb.into();
        }

        ws.write(brightness(leds.iter().copied(), strip_brightness))
            .unwrap();

        delay.delay_ms(16);

        t += (16.0 / 1000.0) * animation_speed;

        while t > 1.0 {
            t -= 1.0;
        }
    }
}
