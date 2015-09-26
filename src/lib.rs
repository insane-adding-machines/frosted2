#![feature(lang_items, no_std)]
#![feature(no_std)]
#![no_std]
#![feature(start)]



#[lang = "eh_personality"]
pub fn eh_personality() {}

#[lang = "stack_exhausted"]
pub fn stack_exhausted() {}

#[lang = "panic_fmt"]
pub fn panic_fmt() -> !
{
    /* This is the kernel panic function. */
    loop {}

}


fn frosted_init()
{

}

fn frosted_kernel()
{
    loop {

    }
}

/* OS entry point */
fn main()
{
    frosted_init();
    frosted_kernel();
}

