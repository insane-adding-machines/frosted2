#![feature(no_std)]
#![feature(asm)]
#![feature(lang_items)]
#![no_std]
#![crate_type="staticlib"]
#![allow(dead_code)]

use core::option::Option;
use core::option::Option::{Some, None};

// Needed to satisfy the compiler.
#[lang="stack_exhausted"] 
extern fn stack_exhausted() {}

#[lang="eh_personality"] 
extern fn eh_personality() {}

#[lang="panic_fmt"]
pub fn panic_fmt(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> ! {
  loop { }
}

extern {
    static mut _data: u32;
    static mut _edata: u32;
    static mut _etext: u32;
    static mut _bss: u8;
    static mut _ebss: u8;
}


// Interrupt vector table.
//------------------------------------------------------------------------------

#[allow(non_upper_case_globals)]
const ISRCount: usize = 60;

#[link_section=".isr_vector"]
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static ISRVectors: [Option<unsafe extern "C" fn()>; ISRCount] = [
    Some(__StackTop),
    Some(Reset_Handler),                          // The reset handler
    Some(NMI_Handler),                            // The NMI handler
    Some(HardFault_Handler),                      // The hard fault handler
    Some(MemManage_Handler),                      // The MPU fault handler
    Some(BusFault_Handler),                       // The bus fault handler
    Some(UsageFault_Handler),                     // The usage fault handler
    None,                                           // Reserved
    None,                                           // Reserved
    None,                                           // Reserved
    None,                                           // Reserved
    Some(SVC_Handler),                            // SVCall handler
    Some(DebugMon_Handler),                       // Debug monitor handler
    None,                                           // Reserved
    Some(PendSV_Handler),                         // The PendSV handler
    Some(SysTick_Handler),                        // The SysTick handler
    Some(IntDefaultHandler),                      // GPIO Port A
    Some(IntDefaultHandler),                      // GPIO Port B
    Some(IntDefaultHandler),                      // GPIO Port C
    Some(IntDefaultHandler),                      // GPIO Port D
    Some(IntDefaultHandler),                      // GPIO Port E
    Some(UART0_IRQHandler),                       // UART0
    Some(UART1_IRQHandler),                       // UART1
    Some(SSI0_IRQHandler),                        // SSI0
    Some(I2C0_IRQHandler),                        // I2C0
    Some(PWMFault_IRQHandler),                    // PWM Fault
    Some(PWM0_IRQHandler),                        // PWM Generator 0
    Some(PWM1_IRQHandler),                        // PWM Generator 1
    Some(PWM2_IRQHandler),                        // PWM Generator 2
    Some(IntDefaultHandler),                      // QEI0
    Some(ADC0_IRQHandler),                        // ADC Sequence 0
    Some(ADC1_IRQHandler),                        // ADC Sequence 1
    Some(ADC2_IRQHandler),                        // ADC Sequence 2
    Some(ADC3_IRQHandler),                        // ADC Sequence 3
    Some(WDT_IRQHandler),                         // Watchdog timer 0
    Some(TIMER0_IRQHandler),                      // Timer 0A
    Some(TIMER0_IRQHandler),                      // Timer 0B
    Some(TIMER1_IRQHandler),                      // Timer 1A
    Some(TIMER1_IRQHandler),                      // Timer 1B
    Some(TIMER2_IRQHandler),                      // Timer 2A
    Some(TIMER2_IRQHandler),                      // Timer 2B
    Some(IntDefaultHandler),                      // Analog Comparator 0
    Some(IntDefaultHandler),                      // Analog Comparator 1
    Some(IntDefaultHandler),                      // Reserved
    Some(IntDefaultHandler),                      // System Control
    Some(IntDefaultHandler),                      // Flash Memory Control
    Some(IntDefaultHandler),                      // GPIO Port F
    Some(IntDefaultHandler),                      // GPIO Port G
    Some(IntDefaultHandler),                      // Reserved
    Some(UART2_IRQHandler),                       // UART2
    Some(IntDefaultHandler),                      // Reserved
    Some(IntDefaultHandler),                      // Timer 3A
    Some(IntDefaultHandler),                      // Timer 3B
    Some(I2C1_IRQHandler),                        // I2C1
    Some(IntDefaultHandler),                      // QEI1
    Some(IntDefaultHandler),                      // Reserved
    Some(IntDefaultHandler),                      // Reserved
    None,                                           // Reserved
    Some(ENET_IRQHandler),                        // Ethernet
    Some(IntDefaultHandler),                      // Hibernate
];


// Inline assembly to invoke semihosting commands
//------------------------------------------------------------------------------
fn semihosting(command: u32, message: &[u32; 3]) {
    unsafe {
        asm!(
            "mov r0, $0;
             mov r1, $1;
             bkpt #0xAB"
            : 
            : "r"(command), "r"(message)
            : "r0", "r1"
        );
    }
}

// Kernel entry
//------------------------------------------------------------------------------
#[no_mangle]
pub fn main() {
    loop 
    {
    };
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn Reset_Handler() {
    let mut pulSrc: *mut u32 = &mut _etext; 
    let mut pulDest: *mut u32 = &mut _data;
    let mut bssDest: *mut u8;
 
    //
    // Copy the data segment initializers from flash to SRAM.
    //
    while pulDest < &mut _edata as *mut u32
    {
        *pulDest = *pulSrc;
        pulDest = ((pulDest as u32) + 4) as *mut u32;
        pulSrc = ((pulSrc as u32) + 4) as *mut u32;
    }

    //
    // Zero-init the BSS section
    //
    bssDest = &mut _bss;

    while bssDest < &mut _ebss as *mut u8
    {
        *bssDest = 0;
        bssDest = ((bssDest as u32) + 1) as *mut u8
    }
 
    //
    // Call the application's entry point.
    //
    main();
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn NMI_Handler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn HardFault_Handler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn MemManage_Handler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn BusFault_Handler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn UsageFault_Handler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn SVC_Handler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn DebugMon_Handler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn PendSV_Handler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn SysTick_Handler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn IntDefaultHandler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn UART0_IRQHandler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn UART1_IRQHandler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn SSI0_IRQHandler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn I2C0_IRQHandler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn PWMFault_IRQHandler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn PWM0_IRQHandler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn PWM1_IRQHandler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn PWM2_IRQHandler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn ADC0_IRQHandler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn ADC1_IRQHandler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn ADC2_IRQHandler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn ADC3_IRQHandler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn WDT_IRQHandler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn TIMER0_IRQHandler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn TIMER1_IRQHandler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn TIMER2_IRQHandler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn UART2_IRQHandler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn I2C1_IRQHandler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn ENET_IRQHandler() {
}

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn __StackTop() {
}

