

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
pub enum Interrupt { # [doc = "17 - LPTIM1_DAC"]
LPTIM1_DAC = 17 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { extern "C" { fn LPTIM1_DAC () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [link_section = ".vector_table.interrupts"]
# [no_mangle]
pub static __INTERRUPTS : [Vector ; 18]
= [Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : LPTIM1_DAC } ,]
; } pub const IWDG : iwdg :: Iwdg = unsafe { iwdg :: Iwdg :: from_ptr (0x4000_3000usize as _) } ; pub const CRC : crc :: Crc = unsafe { crc :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const GPIOA : gpio :: Gpio = unsafe { gpio :: Gpio :: from_ptr (0x5000_0000usize as _) } ; # [doc = r" Number available in the NVIC for configuring priority"]
# [cfg (feature = "rt")]
pub const NVIC_PRIO_BITS : u8 = 2 ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;pub fn GPIO(n: usize) -> gpio::Gpio {
            unsafe { gpio::Gpio::from_ptr((1342177280 + 1024*n) as _) }
        }#[path="../../peripherals/crc_v1.rs"] pub mod crc;
#[path="../../peripherals/gpio_v1.rs"] pub mod gpio;
#[path="../../peripherals/iwdg_v1.rs"] pub mod iwdg;
pub const CORE_INDEX: usize = 0;
pub const FLASH_BASE: usize = 134217728;
pub const FLASH_SIZE: usize = 24576;
pub const PAGE_SIZE: usize = 128;
pub const SECTOR_SIZE: usize = 4096;
