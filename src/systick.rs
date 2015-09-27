#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(private_no_mangle_fns)]
/* 
#[lang="stack_exhausted"] 
extern {fn stack_exhausted();}
#[lang="eh_personality"] 
extern {fn eh_personality();}

#[lang="panic_fmt"]
extern {
    pub fn panic_fmt(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> !;
}
*/

static mut jiffies: u64 = 0;
static SYSTICK_LOAD_RELOAD_MSK : u32 = 0xFFFFFFu32;
static SYSTICK_CTRL_CLKSOURCE_MSK: u32 = 4;
static SYSTICK_CTRL_TICKINT_MSK: u32 = 2;
static SYSTICK_CTRL_ENABLE_MSK: u32 = 1;


struct SysTickRegisters {
    load: u32,
    val: u32,
    ctrl: u32,
}


#[allow(non_snake_case)]
#[allow(dead_code)]
#[no_mangle]
pub unsafe extern "C" fn SysTick_Handler() {
    jiffies = jiffies + 1;  
}

pub fn SysTick_Config(ticks: u32) -> bool
{

  let mut SysTick: SysTickRegisters;
  if (ticks - 1) > SYSTICK_LOAD_RELOAD_MSK { 
        return false; 
  } 

  SysTick.load  = ticks -1;                         /* set reload register */
  //NVIC_SetPriority (SysTick_IRQn, (1UL << __NVIC_PRIO_BITS) - 1UL); /* set Priority for Systick Interrupt */
  SysTick.val   = 0;                                              /* Load the SysTick Counter Value */
  SysTick.ctrl  =  SYSTICK_CTRL_CLKSOURCE_MSK |
                   SYSTICK_CTRL_TICKINT_MSK   |
                   SYSTICK_CTRL_ENABLE_MSK;                         /* Enable SysTick IRQ and SysTick Timer */

  /* TODO: Write SysTick -> SysTick_base registry */

  true                                                     /* Function successful */
}

pub fn systick_init() {
    /* TODO */
    SysTick_Config(100);
}
