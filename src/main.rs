#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::digital::OutputPin;
use panic_halt as _;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rp_pico::hal::{pac, clocks::init_clocks_and_plls, gpio::Pins, sio::Sio, watchdog::Watchdog, Timer};
// use rp_pico::hal::prelude::*;
use defmt_rtt as _;
use debouncr::{debounce_stateful_3, Edge};

// config
const NR_BEEPS: u16 = 10; // Anzahl der Töne
const BEEP_DURATION: u16 = 100; // Dauer jedes Tons in Millisekunden
const ONE_HERZ: u16 = 1000; // 1 Hz entspricht 1000 ms
const MAX_RUNDEN: u16 = 10; // maximale Anzahl der Runden

enum YesNo {
    Yes,
    No,    
}

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
    let mut yes_button = pins.gpio14.into_pull_up_input();
    let mut no_button = pins.gpio17.into_pull_up_input();
    let mut timer = Timer::new(peripherals.TIMER, &mut peripherals.RESETS, &clocks);

    let mut rng = SmallRng::seed_from_u64(42); // Fester Seed für Reproduzierbarkeit
    let mut avg_pause: u32 = 500; // durchschnittliche Veränderung der Pause zwischen den Tönen in Millisekunden

    for runde in 1..MAX_RUNDEN {
        // Ausgabe der Runde
        defmt::info!("Runde {}/{} mit durchschnittlicher Pause: {} ms", runde, MAX_RUNDEN, avg_pause);

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

        let mut debouncer = debounce_stateful_3(false);
        let mut antwort: YesNo = None;

        // Warten auf Benutzerinteraktion
        loop {
            let button_pressed = yes_button.is_low().unwrap() || no_button.is_low().unwrap();

            if debouncer.update(yes_button.is_low().unwrap(), Edge::Rising) {
                defmt::info!("Benutzereingabe: ja.");
                antwort = YesNo::Yes;
                break; // Beenden der Warte-Schleife bei Benutzereingabe
            }
              if debouncer.update(no_button.is_low().unwrap(), Edge::Rising) {
                defmt::info!("Benutzereingabe: nen.");
                antwort = YesNo::No;
                break; // Beenden der Warte-Schleife bei Benutzereingabe
            }
            timer.delay_ms(100); // Kurze Pause, um CPU-Last zu reduzieren
        }
        defmt::info!("Antwort: {:?}", antwort);
        
        match antwort {
            YesNo::Yes => {
                // Differenz erkannt, avg_pause halbieren
                if avg_pause > 100 { // Mindestwert für avg_pause
                    avg_pause /= 2;
                }
                defmt::info!("Pause halbiert: {} ms", avg_pause);
            },
            YesNo::No => {
                // Keine Differenz erkannt, avg_pause erhöhen
                avg_pause = (avg_pause as f32 * 1.5) as u32; // Erhöhung um 50%
                defmt::info!("Pause erhöht: {} ms", avg_pause);
            },           
        } 
        // user input: Differenz erkannt oder nicht
        // wenn ja, dann avg_pause verringern (halbieren) 
        // wenn nein, dann avg_pause erhöhen (* 1,5)
    } // Ende der Runde
}
