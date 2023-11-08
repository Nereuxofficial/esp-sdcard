# esp-sdcard
This is an example of how to use an SD Card using no-std Rust on an ESP32.

## Wiring
1. GND to GND
2. 5V to VCC
3. MISO to GPIO2
4. MOSI to GPIO15
5. SCK to GPIO14
6. CS to GPIO13
When flashing the ESP32, you need to disconnect the SD Card from the ESP32.

## Usage
1. Install [espup](https://github.com/esp-rs/espup)
2. Run `espup install` and follow its instructions
3. Install [espflash](https://github.com/esp-rs/espflash/
4. Run `cargo run`
Your output should be similar to this:
```
I (240) boot: Disabling RNG early entropy source...
SPI initialized. Initializing SD-Card...
Card size is 8069840896 bytes
Volume 0: Volume(SearchId(5000))
INFO - Logger is setup
Hello world!
Loop...
Loop...
```
