

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
pub enum Interrupt { # [doc = "0 - WWDG"]
WWDG = 0 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { extern "C" { fn WWDG () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [link_section = ".vector_table.interrupts"]
# [no_mangle]
pub static __INTERRUPTS : [Vector ; 1]
= [Vector { _handler : WWDG } ,]
; } pub const RTC : rtc :: Rtc = unsafe { rtc :: Rtc :: from_ptr (0x4000_2800usize as _) } ; pub const WWDG : wwdg :: Wwdg = unsafe { wwdg :: Wwdg :: from_ptr (0x4000_2c00usize as _) } ; pub const IWDG : iwdg :: Iwdg = unsafe { iwdg :: Iwdg :: from_ptr (0x4000_3000usize as _) } ; pub const CTC : ctc :: Ctc = unsafe { ctc :: Ctc :: from_ptr (0x4000_6c00usize as _) } ; pub const PWR : pwr :: Pwr = unsafe { pwr :: Pwr :: from_ptr (0x4000_7000usize as _) } ; pub const OPA : opa :: Opa = unsafe { opa :: Opa :: from_ptr (0x4001_0300usize as _) } ; pub const CRC : crc :: Crc = unsafe { crc :: Crc :: from_ptr (0x4002_3000usize as _) } ; pub const DIV : div :: Div = unsafe { div :: Div :: from_ptr (0x4002_3800usize as _) } ; pub const GPIOA : gpio :: Gpio = unsafe { gpio :: Gpio :: from_ptr (0x5000_0000usize as _) } ; # [doc = r" Number available in the NVIC for configuring priority"]
# [cfg (feature = "rt")]
pub const NVIC_PRIO_BITS : u8 = 2 ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;pub fn GPIO(n: usize) -> gpio::Gpio {
            unsafe { gpio::Gpio::from_ptr((1342177280 + 1024*n) as _) }
        }#[path="../../peripherals/crc_v1.rs"] pub mod crc;
#[path="../../peripherals/ctc_v1.rs"] pub mod ctc;
#[path="../../peripherals/div_v1.rs"] pub mod div;
#[path="../../peripherals/gpio_v1.rs"] pub mod gpio;
#[path="../../peripherals/iwdg_v1.rs"] pub mod iwdg;
#[path="../../peripherals/opa_v1.rs"] pub mod opa;
#[path="../../peripherals/pwr_f072.rs"] pub mod pwr;
#[path="../../peripherals/rtc_v1.rs"] pub mod rtc;
#[path="../../peripherals/wwdg_v1.rs"] pub mod wwdg;
pub const CORE_INDEX: usize = 0;
pub const FLASH_BASE: usize = 134217728;
pub const FLASH_SIZE: usize = 65536;
pub const PAGE_SIZE: usize = 256;
pub const SECTOR_SIZE: usize = 8192;
