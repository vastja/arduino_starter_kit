file_name="${1:-hello_arduino}"

cargo build
avr-objcopy -O ihex -R .eeprom target/avr-none/debug/$file_name.elf $file_name.hex
avrdude -c arduino -p m328p -P /dev/tty.usbmodem101 -b 115200 -U flash:w:$file_name.hex
