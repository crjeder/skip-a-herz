# skip-a-herz
This is a submission for the [Hackaday 1Hz Challenge](https://hackaday.io/contest/203248ference) that explores the just noticeable difference (JND) in human perception of time gaps between tones. It plays tone pairs with varying delays and records user responses to estimate perceptual thresholds in real time.

## 🧠 Concept

This project investigates how small a difference in timing between two tones must be for a person to notice it. Participants listen to tone pairs with slightly varying intervals and respond when they perceive a change. The system estimates the perceptual threshold in real time.

## 🔧 Hardware

- Raspberry Pi Pico
- Active piezo buzzer
- Button (for user input)
- Optional: OLED display or serial monitor

## 🛠️ Software

- Language: Rust
- HAL: `rp-hal`
- Randomness: [`rand`](https://crates.io/crates/rand Clone this repo
2. Flash the firmware to your Raspberry Pi Pico
3. Connect the buzzer and button as described in `src/main.rs`
4. Power up and start testing your auditory perception!

## 📊 Future Features
- Data logging over USB serial
- Web-based interface for results

## 📄 License

MIT License

---

## 🤝 Contributing

Pull requests are welcome! For major changes, please open an issue first to discuss what you’d like to change.


