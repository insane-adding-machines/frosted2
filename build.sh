
# Stop execution when an error occurs
set -e

# Compile
rustc --target=thumbv7m-none-eabi -O -Z no-landing-pads src/hello_world.rs --emit obj -L libcore -o hello_world.o

# Link .o file using the provided linker script to generate an executable
# binary that can be deployed to the board's flash memory.  Remove the
# .o file aftwards as it is no longer needed
arm-none-eabi-gcc hello_world.o -Wl,--gc-sections -Tstm32f4.ld -o hello_world.elf -Wl,-Map,hello_world.map

# Display how much flash memory will be consumed by the executable
arm-none-eabi-size hello_world.elf
