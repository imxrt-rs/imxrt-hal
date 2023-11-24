cargo objcopy --example=hal_trng --features=board/teensy4 --target=thumbv7em-none-eabihf -- -O ihex firmware.hex
teensy_loader_cli --mcu=TEENSY40 -wsv .\firmware.hex
