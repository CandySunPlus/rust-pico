//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use bsp::{
    entry,
    hal::{prelude::_rphal_pio_PIOExt, Timer},
};
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
use smart_leds::{colors, SmartLedsWrite, RGB};
use ws2812_pio::Ws2812;

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

    let clz = [
        colors::PINK,
        colors::LIGHT_PINK,
        colors::HOT_PINK,
        colors::DEEP_PINK,
        colors::PALE_VIOLET_RED,
        colors::MEDIUM_VIOLET_RED,
        colors::LIGHT_SALMON,
        colors::SALMON,
        colors::DARK_SALMON,
        colors::LIGHT_CORAL,
        colors::INDIAN_RED,
        colors::CRIMSON,
        colors::FIREBRICK,
        colors::DARK_RED,
        colors::ORANGE_RED,
        colors::TOMATO,
        colors::CORAL,
        colors::DARK_ORANGE,
        colors::ORANGE,
        colors::LIGHT_YELLOW,
        colors::LEMON_CHIFFON,
        colors::LIGHT_GOLDENROD_YELLOW,
        colors::PAPAYA_WHIP,
        colors::MOCCASIN,
        colors::PEACH_PUFF,
        colors::PALE_GOLDENROD,
        colors::KHAKI,
        colors::DARK_KHAKI,
        colors::GOLD,
        colors::CORNSILK,
        colors::BLANCHED_ALMOND,
        colors::BISQUE,
        colors::NAVAJO_WHITE,
        colors::WHEAT,
        colors::BURLYWOOD,
        colors::TAN,
        colors::ROSY_BROWN,
        colors::SANDY_BROWN,
        colors::GOLDENROD,
        colors::DARK_GOLDENROD,
        colors::PERU,
        colors::CHOCOLATE,
        colors::SADDLE_BROWN,
        colors::SIENNA,
        colors::BROWN,
        colors::DARK_OLIVE_GREEN,
        colors::OLIVE_DRAB,
        colors::YELLOW_GREEN,
        colors::LIME_GREEN,
        colors::LAWN_GREEN,
        colors::CHARTREUSE,
        colors::GREEN_YELLOW,
        colors::SPRING_GREEN,
        colors::MEDIUM_SPRING_GREEN,
        colors::LIGHT_GREEN,
        colors::PALE_GREEN,
        colors::DARK_SEA_GREEN,
        colors::MEDIUM_AQUAMARINE,
        colors::MEDIUM_SEA_GREEN,
        colors::SEA_GREEN,
        colors::FOREST_GREEN,
        colors::DARK_GREEN,
        colors::CYAN,
        colors::LIGHT_CYAN,
        colors::PALE_TURQUOISE,
        colors::AQUAMARINE,
        colors::TURQUOISE,
        colors::MEDIUM_TURQUOISE,
        colors::DARK_TURQUOISE,
        colors::LIGHT_SEA_GREEN,
        colors::CADET_BLUE,
        colors::DARK_CYAN,
        colors::LIGHT_STEEL_BLUE,
        colors::POWDER_BLUE,
        colors::LIGHT_BLUE,
        colors::SKY_BLUE,
        colors::LIGHT_SKY_BLUE,
        colors::DEEP_SKY_BLUE,
        colors::DODGER_BLUE,
        colors::CORNFLOWER_BLUE,
        colors::STEEL_BLUE,
        colors::ROYAL_BLUE,
        colors::MEDIUM_BLUE,
        colors::DARK_BLUE,
        colors::MIDNIGHT_BLUE,
        colors::LAVENDER,
        colors::THISTLE,
        colors::PLUM,
        colors::VIOLET,
        colors::ORCHID,
        colors::MAGENTA,
        colors::MEDIUM_ORCHID,
        colors::MEDIUM_PURPLE,
        colors::BLUE_VIOLET,
        colors::DARK_VIOLET,
        colors::DARK_ORCHID,
        colors::DARK_MAGENTA,
        colors::INDIGO,
        colors::DARK_SLATE_BLUE,
        colors::SLATE_BLUE,
        colors::MEDIUM_SLATE_BLUE,
        colors::SNOW,
        colors::HONEYDEW,
        colors::MINT_CREAM,
        colors::AZURE,
        colors::ALICE_BLUE,
        colors::GHOST_WHITE,
        colors::WHITE_SMOKE,
        colors::SEASHELL,
        colors::BEIGE,
        colors::OLD_LACE,
        colors::FLORAL_WHITE,
        colors::IVORY,
        colors::ANTINQUE_WHITE,
        colors::LINEN,
        colors::LAVENDER_BLUSH,
        colors::MISTY_ROSE,
        colors::GAINSBORO,
        colors::LIGHT_GRAY,
        colors::DARK_GRAY,
        colors::DIM_GRAY,
        colors::LIGHT_SLATE_GRAY,
        colors::SLATE_GRAY,
        colors::DARK_SLATE_GRAY,
    ];

    let mut leds: [RGB<u8>; 4] = (&clz[0..4]).try_into().unwrap();

    loop {
        let mut i = 0;

        info!("flash");
        for color in clz {
            if i >= leds.len() {
                i = 0;
            }
            leds[i] = color;
            i += 1;
            info!("writed");
            ws.write(leds.iter().copied()).unwrap();
            delay.delay_ms(50);
        }
    }
}
// End of file
