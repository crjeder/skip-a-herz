#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use panic_halt as _;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rp_pico::hal::{clocks::init_clocks_and_plls, pac, sio::Sio, watchdog::Watchdog, Timer, gpio::Pins};
use rp_pico::hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(peripherals.WATCHDOG);
    let clocks = init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        peripherals.XOSC,
        peripherals.CLOCKS,
        peripherals.PLL_SYS,
        peripherals.PLL_USB,
        &mut peripherals.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = Sio::new(peripherals.SIO);
    let pins = Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        sio.gpio_bank0,
        &mut peripherals.RESETS,
    );

    let mut buzzer = pins.gpio15.into_push_pull_output();
    let mut timer = Timer::new(peripherals.TIMER, &mut peripherals.RESETS, &clocks);

    let mut rng = SmallRng::seed_from_u64(42); // Fester Seed für Reproduzierbarkeit
    loop {// Ton erzeugen (einfach HIGH für 100 ms)
        buzzer.set_high().unwrap();
        timer.delay_ms(100);
        buzzer.set_low().unwrap();

        // Zufällige Pause zwischen 100 und 500 ms
        let pause = rng.gen_range(100..500);
        timer.delay_ms(pause);
    }
}
