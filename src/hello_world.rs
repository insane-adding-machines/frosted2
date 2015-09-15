#![feature(no_std)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(raw)]
#![no_std]
#![crate_type="staticlib"]

use core::raw::Repr;

// Needed to satisfy the compiler.
#[lang="stack_exhausted"] 
extern fn stack_exhausted() {}

#[lang="eh_personality"] 
extern fn eh_personality() {}

#[lang="panic_fmt"]
pub fn panic_fmt(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> ! {
  loop { }
}

// Interrupt vector table.
// Only the reset handler for now
//------------------------------------------------------------------------------
#[link_section=".isr_vector"]
pub static ISRVECTORS: [unsafe extern "C" fn(); 1] = [
    main, 
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

// Program entry
//------------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn main() {
    let text = "hello world\n";
    
    let message : [u32; 3] = [
        2,               // write to stderr
        text.repr().data as u32,
        text.repr().len as u32
    ];

    loop { 
        semihosting(
            5,           // 5 is semihosting "write" command
            &message
        );
    }
}
