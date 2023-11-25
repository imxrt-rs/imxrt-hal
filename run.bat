cargo objcopy --example=rtic_spi --features=board/spi,board/teensy4,async --target=thumbv7em-none-eabihf -- -O ihex firmware.hex
teensy_loader_cli --mcu=TEENSY40 -wsv .\firmware.hex
