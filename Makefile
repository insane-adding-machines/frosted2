PORT?=stm32f4
KERNEL_OBJ:=startup.o

all: image.bin

image.bin: kernel.elf
	arm-none-eabi-objcopy -O binary --pad-to=0x20000 $^ $@

%.o : src/%.rs
	rustc -g --target=thumbv7m-none-eabi -O -Z no-landing-pads $^ --emit obj -L libcore -o startup.o 

kernel.elf: $(KERNEL_OBJ)
	arm-none-eabi-gcc -g -ggdb $^ -Wl,--gc-sections -T$(PORT).ld -o $@ -Wl,-Map,startup.map -nostdlib
	arm-none-eabi-size kernel.elf

qemu: image.bin
	qemu-system-arm -semihosting -M lm3s6965evb --kernel image.bin -serial stdio -S -gdb tcp::3333

clean:
	rm -f image.bin kernel.elf $(KERNEL_OBJ)
 
