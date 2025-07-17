# skip-a-herz

| <img src="skip-a-herz-logo.jpeg" alt="Skip-a-Herz Logo" width="300"/> |   This is a submission for the [Hackaday 1Hz Challenge](https://hackaday.io/contest/203248ference) that explores the just noticeable difference (JND) in human perception of time gaps between tones. It plays tone pairs with varying delays and records user responses to estimate perceptual thresholds. |
|--|:--|

## 🧠 Concept

This project investigates how small a difference in timing between two tones must be for a person to just notice it. Participants listen to tone pairs with slightly varying intervals and respond when they perceive a change. The system estimates the perceptual threshold in real time.

## 🔧 Hardware

- Raspberry Pi Pico
- Active piezo buzzer
- Button (for user input)
- 3D printed case
- Optional: OLED display or serial monitor

## 🛠️ Software

- Firmware: Rust
- Android app

## Design Decisions

I am using the pico because I'd like to have the option to use PIO if the normal GPIO intoduces to much jitter but it probably will work fine. The fear of jitter is also the reason for Rust as programming language. I think it produces conde whose timing is much more consitant (no garbage collector, ..)

## 📊 Future Features

- Data logging over USB serial
- Web-based interface for results

## 📄 License

MIT License

---

## 🤝 Contributing

Pull requests are welcome! For major changes, please open an issue first to discuss what you’d like to change.
