

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
pub enum Interrupt { # [doc = "0 - WWDG"]
WWDG = 0 , # [doc = "29 - AES_LPUART1"]
AES_LPUART1 = 29 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { extern "C" { fn WWDG () ; fn AES_LPUART1 () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [link_section = ".vector_table.interrupts"]
# [no_mangle]
pub static __INTERRUPTS : [Vector ; 30]
= [Vector { _handler : WWDG } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : AES_LPUART1 } ,]
; } pub const GPIOA : gpio :: Gpio = unsafe { gpio :: Gpio :: from_ptr (0x5000_0000usize as _) } ; # [doc = r" Number available in the NVIC for configuring priority"]
# [cfg (feature = "rt")]
pub const NVIC_PRIO_BITS : u8 = 2 ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;pub fn GPIO(n: usize) -> gpio::Gpio {
            unsafe { gpio::Gpio::from_ptr((1342177280 + 1024*n) as _) }
        }#[path="../../peripherals/gpio_v1.rs"] pub mod gpio;
pub const CORE_INDEX: usize = 0;
pub const FLASH_BASE: usize = 134217728;
pub const FLASH_SIZE: usize = 131072;
pub const PAGE_SIZE: usize = 256;
pub const SECTOR_SIZE: usize = 2048;
