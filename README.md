# `dark_instruments`

**Dark Instruments** is a Rust crate for Android automation, wrapping ADB (Android Debug Bridge) and Tesseract OCR. This crate provides tools for scripting and automating Android device interactions, enabling efficient execution of complex tasks such as UI navigation, text recognition, and device control.

### Features
- ADB binary wrapper for device control
- Tesseract OCR in toolkit for text recognition

### Prerequisites
- Android SDK Platform Tools (platform-tools/)
- tesseract-ocr

### Install
- [https://developer.android.com/tools#tools-platform](https://developer.android.com/tools#tools-platform)
- `sudo apt-get install tesseract-ocr`
```toml
[dependencies]
dark_instruments = { git = "https://github.com/michaelbentz/dark_instruments", tag = "0.0.2" }
```
### Usage
- TODO