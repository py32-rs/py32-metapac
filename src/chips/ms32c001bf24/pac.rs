

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
pub enum Interrupt { # [doc = "0 - PVD"]
PVD = 0 , # [doc = "1 - FLASH"]
FLASH = 1 , # [doc = "2 - RCC"]
RCC = 2 , # [doc = "3 - EXTI0_1"]
EXTI0_1 = 3 , # [doc = "4 - EXTI2_3"]
EXTI2_3 = 4 , # [doc = "5 - EXTI4_7"]
EXTI4_7 = 5 , # [doc = "8 - ADC1"]
ADC1 = 8 , # [doc = "9 - TIM1_BRK_UP_TRG_COM"]
TIM1_BRK_UP_TRG_COM = 9 , # [doc = "10 - TIM1_CC"]
TIM1_CC = 10 , # [doc = "11 - LPTIM1"]
LPTIM1 = 11 , # [doc = "12 - TIM14"]
TIM14 = 12 , # [doc = "16 - PWM1"]
PWM1 = 16 , # [doc = "17 - UART1"]
UART1 = 17 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { extern "C" { fn PVD () ; fn FLASH () ; fn RCC () ; fn EXTI0_1 () ; fn EXTI2_3 () ; fn EXTI4_7 () ; fn ADC1 () ; fn TIM1_BRK_UP_TRG_COM () ; fn TIM1_CC () ; fn LPTIM1 () ; fn TIM14 () ; fn PWM1 () ; fn UART1 () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [link_section = ".vector_table.interrupts"]
# [no_mangle]
pub static __INTERRUPTS : [Vector ; 18]
= [Vector { _handler : PVD } , Vector { _handler : FLASH } , Vector { _handler : RCC } , Vector { _handler : EXTI0_1 } , Vector { _handler : EXTI2_3 } , Vector { _handler : EXTI4_7 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : ADC1 } , Vector { _handler : TIM1_BRK_UP_TRG_COM } , Vector { _handler : TIM1_CC } , Vector { _handler : LPTIM1 } , Vector { _handler : TIM14 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : PWM1 } , Vector { _handler : UART1 } ,]
; } pub const UID : uid :: Uid = unsafe { uid :: Uid :: from_ptr (0x1fff_0000usize as _) } ; pub const CONFIGBYTES : configbytes :: Configbytes = unsafe { configbytes :: Configbytes :: from_ptr (0x1fff_0040usize as _) } ; pub const TIM14 : timer :: Tim1ch = unsafe { timer :: Tim1ch :: from_ptr (0x4000_2000usize as _) } ; pub const PWM1 : pwm :: Pwm = unsafe { pwm :: Pwm :: from_ptr (0x4000_2800usize as _) } ; pub const UART1 : usart :: Uart = unsafe { usart :: Uart :: from_ptr (0x4000_4800usize as _) } ; pub const SYSCFG : syscfg :: Syscfg = unsafe { syscfg :: Syscfg :: from_ptr (0x4001_0000usize as _) } ; pub const VREF : vref :: Vref = unsafe { vref :: Vref :: from_ptr (0x4001_0100usize as _) } ; pub const ADC1 : adc :: Adc = unsafe { adc :: Adc :: from_ptr (0x4001_2400usize as _) } ; pub const TIM1 : timer :: TimAdv = unsafe { timer :: TimAdv :: from_ptr (0x4001_2c00usize as _) } ; pub const DBGMCU : dbgmcu :: Dbgmcu = unsafe { dbgmcu :: Dbgmcu :: from_ptr (0x4001_5800usize as _) } ; pub const RCC : rcc :: Rcc = unsafe { rcc :: Rcc :: from_ptr (0x4002_1000usize as _) } ; pub const EXTI : exti :: Exti = unsafe { exti :: Exti :: from_ptr (0x4002_1800usize as _) } ; pub const FLASH : flash :: Flash = unsafe { flash :: Flash :: from_ptr (0x4002_2000usize as _) } ; pub const GPIOA : gpio :: Gpio = unsafe { gpio :: Gpio :: from_ptr (0x5000_0000usize as _) } ; pub const GPIOB : gpio :: Gpio = unsafe { gpio :: Gpio :: from_ptr (0x5000_0400usize as _) } ; pub const GPIOC : gpio :: Gpio = unsafe { gpio :: Gpio :: from_ptr (0x5000_0800usize as _) } ; # [doc = r" Number available in the NVIC for configuring priority"]
# [cfg (feature = "rt")]
pub const NVIC_PRIO_BITS : u8 = 2 ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;pub fn GPIO(n: usize) -> gpio::Gpio {
            unsafe { gpio::Gpio::from_ptr((1342177280 + 1024*n) as _) }
        }#[path="../../peripherals/adc_v3.rs"] pub mod adc;
#[path="../../peripherals/configbytes_ms32c001b.rs"] pub mod configbytes;
#[path="../../peripherals/dbgmcu_f002b.rs"] pub mod dbgmcu;
#[path="../../peripherals/exti_v1b.rs"] pub mod exti;
#[path="../../peripherals/flash_ms32c001b.rs"] pub mod flash;
#[path="../../peripherals/gpio_v1b.rs"] pub mod gpio;
#[path="../../peripherals/pwm_v1.rs"] pub mod pwm;
#[path="../../peripherals/rcc_ms32c001b.rs"] pub mod rcc;
#[path="../../peripherals/syscfg_f002b.rs"] pub mod syscfg;
#[path="../../peripherals/timer_v1b.rs"] pub mod timer;
#[path="../../peripherals/uid_v1.rs"] pub mod uid;
#[path="../../peripherals/usart_v2.rs"] pub mod usart;
#[path="../../peripherals/vref_v1.rs"] pub mod vref;
pub const CORE_INDEX: usize = 0;
pub const FLASH_BASE: usize = 134217728;
pub const FLASH_SIZE: usize = 16384;
pub const PAGE_SIZE: usize = 64;
pub const SECTOR_SIZE: usize = 1024;
