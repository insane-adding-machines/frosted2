 tar ext :3333
 monitor reset
 layout src
 mon arm semihosting enable
 file hello_world.elf
 mon reset
 mon halt
 stepi
 focus c
