#![no_std]
#![no_main]

use cortex_m_rt::entry;
use debouncr::{debounce_stateful_3, Edge};
use defmt_rtt as _;
use embedded_hal::{
    delay::DelayNs,
    digital::{InputPin, OutputPin},
};
use panic_halt as _;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rp_pico::hal::{
    clocks::init_clocks_and_plls, gpio::Pins, pac, sio::Sio, watchdog::Watchdog, Timer,
};

// config
const NR_BEEPS: u32 = 10; // Anzahl der Töne
const BEEP_DURATION: u32 = 100; // Dauer jedes Tons in Millisekunden
const ONE_HERZ: u32 = 1000; // 1 Hz entspricht 1000 ms
const MAX_RUNDEN: u32 = 10; // maximale Anzahl der Runden
const PAUSE_MIN: f32 = 100.0; // Mindestwert für die Pause zwischen den Tönen in Millisekunden
const DEBOUNCE_TRASHOLD: u32 = 50; // Debounce-Zeit in Millisekunden

// Enum für die Benutzerantwort
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum YesNo {
    Yes,
    No,
}

#[entry]
fn main() -> ! {
    let mut peripherals = pac::Peripherals::take().unwrap();
    //let core = pac::CorePeripherals::take().unwrap();

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
    let mut avg_pause: f32 = 500.0; // durchschnittliche Veränderung der Pause zwischen den Tönen in Millisekunden
    let mut ergebnis: f32 = ONE_HERZ as f32; // Ergebnis des Versuchs

    for runde in 1..MAX_RUNDEN {
        // Ausgabe der Runde
        defmt::info!(
            "Runde {}/{} mit durchschnittlicher Pause: {} ms",
            runde,
            MAX_RUNDEN,
            avg_pause
        );

        let mit_abweichung: bool = rng.random();
        let mut pause_diff: f32; // Initialisierung der Pause-Differenz

        for _ in 1..NR_BEEPS {
            // ersten Ton erzeugen (einfach HIGH für 100 ms)
            buzzer.set_high().unwrap();
            timer.delay_ms(BEEP_DURATION);
            buzzer.set_low().unwrap();

            // Zufällige Abweichung +- avg_pause
            if mit_abweichung {
                pause_diff = rng.random_range(-avg_pause..avg_pause);
            } else {
                pause_diff = 0.0;
            }
            timer.delay_ms(ONE_HERZ as u32 + pause_diff as u32);

            // zweiten Ton erzeugen
            buzzer.set_high().unwrap();
            timer.delay_ms(BEEP_DURATION);
            buzzer.set_low().unwrap();

            timer.delay_ms(ONE_HERZ - pause_diff as u32); // Pause zwischen den Tönen
        }
        // Ton für Rundenende erzeugen
        buzzer.set_high().unwrap();
        timer.delay_ms(4 * BEEP_DURATION);
        buzzer.set_low().unwrap();

        let mut debouncer = debounce_stateful_3(false);
        let mut antwort: Option<YesNo>;

        // Warten auf Benutzerinteraktion
        loop {
            //let button_pressed = yes_button.is_low().unwrap() || no_button.is_low().unwrap();

            if debouncer.update(yes_button.is_low().unwrap()) == Some(Edge::Rising) {
                defmt::info!("Benutzereingabe: ja.");
                antwort = Some(YesNo::Yes);
                break; // Beenden der Warte-Schleife bei Benutzereingabe
            }
            if debouncer.update(no_button.is_low().unwrap()) == Some(Edge::Rising) {
                defmt::info!("Benutzereingabe: nein.");
                antwort = Some(YesNo::No);
                break; // Beenden der Warte-Schleife bei Benutzereingabe
            }
            timer.delay_ms(DEBOUNCE_TRASHOLD); // Kurze Pause, um CPU-Last zu reduzieren
        }

        match antwort {
            Some(YesNo::Yes) => {
                defmt::info!("Antwort: Ja");
                // Differenz erkannt, avg_pause halbieren
                if mit_abweichung {
                    ergebnis = avg_pause / 2.0; // Ergebnis ist die Hälfte der durchschnittlichen Pause
                    if avg_pause > PAUSE_MIN {
                        // Mindestwert für avg_pause
                        avg_pause /= 2.0;
                        defmt::info!("Abweichung halbiert: {} ms", avg_pause);
                    }
                }
            }
            Some(YesNo::No) => {
                // Keine Differenz erkannt, avg_pause erhöhen
                defmt::info!("Antwort: Nein");
                avg_pause = avg_pause * 1.5; // Erhöhung um 50%
                defmt::info!("Pause erhöht: {} ms", avg_pause);
            }
            None => {
                defmt::warn!("Keine Antwort erhalten");
                // Standardverhalten: Keine Änderung an avg_pause
            }
        }
        defmt::info!("Kleinste erkannte Abweichung: {} ms", ergebnis);
        // user input: Differenz erkannt oder nicht
        // wenn ja, dann avg_pause verringern (halbieren)
        // wenn nein, dann avg_pause erhöhen (* 1,5)
    } // Ende der Runde
    loop {}
}
