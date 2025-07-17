#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use panic_halt as _;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rp_pico::hal::{pac, clocks::init_clocks_and_plls, gpio::Pins, sio::Sio, watchdog::Watchdog, Timer};
// use rp_pico::hal::prelude::*;
use defmt_rtt as _;

// config
const NR_BEEPS: u16 = 10; // Anzahl der Töne
const BEEP_DURATION: u16 = 100; // Dauer jedes Tons in Millisekunden
const ONE_HERZ: u16 = 1000; // 1 Hz entspricht 1000 ms
const MAX_RUNDEN: u16 = 10; // maximale Anzahl der Runden

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
    let mut avg_pause: u32 = 500; // durchschnittliche Veränderung der Pause zwischen den Tönen in Millisekunden

    for runde in 1..MAX_RUNDEN {
        // Ausgabe der Runde
        defmt::info!("Runde {}/{}", runde, MAX_RUNDEN);
        defmt::info!("Durchschnittliche Pause: {} ms", avg_pause);

        for count in 1..NR_BEEPS {
            // ersten Ton erzeugen (einfach HIGH für 100 ms)
            buzzer.set_high().unwrap();
            timer.delay_ms(BEEP_DURATION);
            buzzer.set_low().unwrap();

            // Zufällige Abweichung +- avg_pause
            let pause_diff = rng.gen_range(-avg_pause..avg_pause);
            timer.delay_ms(ONE_HERZ + pause_diff);

            // zweiten Ton erzeugen 
            buzzer.set_high().unwrap();
            timer.delay_ms(BEEP_DURATION);
            buzzer.set_low().unwrap();

            timer.delay_ms(ONE_HERZ - pause_diff);
        }
        // Ton für Rundenende erzeugen
        buzzer.set_high().unwrap();
        timer.delay_ms(4 * BEEP_DURATION);
        buzzer.set_low().unwrap();
        // user input: Differenz erkannt oder nicht
        // wenn ja, dann avg_pause verringern (halbieren) 
        // wenn nein, dann avg_pause erhöhen (* 1,5)
    }
}
