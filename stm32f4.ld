/* Frosted - Linker script for stm32f4 */
/* Specify the memory areas */
MEMORY
{
    FLASH (rx)      : ORIGIN = 0x08000000, LENGTH = 128K /* entire flash, 1 MiB */
    RAM (xrw)       : ORIGIN = 0x20000000, LENGTH = 0x020000 /* 128 KiB */
}
 
/* define output sections */
SECTIONS
{
    /DISCARD/ :
    {
        *(.ARM.extab*)
        *(.ARM.exidx*)
    }
    
    /DISCARD/ :
    {
        *(.ARM.attributes*)
        *(.comment)
    }    

    /* The startup code goes first into FLASH */
    .isr_vector :
    {
        . = ALIGN(4);
        KEEP(*(.isr_vector)) /* Startup code */
        KEEP(*(.syscall_vector))
        . = ALIGN(4);
    } >FLASH
   
    /* The program code and other data goes into FLASH */
    .text :
    {
        . = ALIGN(4);
        *(.text)           
        *(.text*)          
        *(.rodata)         
        *(.rodata*)        
        . = ALIGN(4);
        _etext = .;        /* define a global symbol at end of code */
    } >FLASH

    /* Code can explicitly ask for data to be 
       placed in these higher RAM banks where
       they will be left uninitialized. 
    */
    .task_stack (NOLOAD):
    {
        task_stack_start = . ;
        *(.task)
        task_stack_end = . ;
    } > RAM
     
    /* used by the startup to initialize data */
    _sidata = LOADADDR(.data);
        
    /* This is the initialized data section
    The program executes knowing that the data is in the RAM
    but the loader puts the initial values in the FLASH (inidata).
    It is one task of the startup to copy the initial values from FLASH to RAM. */
    .data :
    {
        . = ALIGN(4);
        _sdata = .;
        _data = .;
        _ram_start = .;
        *(.data)
        *(.data*)
        . = ALIGN(4);
        _edata = .;    
    } >RAM AT> FLASH
            
    /* Uninitialized data section */
    .bss :
    {
        . = ALIGN(4);
        __bss_start__ = .;
        _sbss = .;
        _bss = .;
        *(.bss)
        *(.bss*)
        *(COMMON)

        . = ALIGN(4);
        /* define a global symbol at bss end; used by startup code and GC */
        __bss_end__ = .;
        _ebss = .;
    } >RAM

    /* this is to define the start of the heap, and make sure we have a minimum size */
    .heap :
    {
        . = ALIGN(4);
	    PROVIDE ( end = . );
	    PROVIDE ( _end = . );
    } >RAM

    /DISCARD/ :
    {
        libc.a ( * )
        libm.a ( * )
        libgcc.a ( * )
    }

    .ARM.attributes 0 : { *(.ARM.attributes) }
}
