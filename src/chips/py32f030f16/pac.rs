

# [derive (Copy , Clone , Debug , PartialEq , Eq)]
pub enum Interrupt { # [doc = "0 - WWDG"]
WWDG = 0 , # [doc = "1 - PVD"]
PVD = 1 , # [doc = "2 - RTC"]
RTC = 2 , # [doc = "3 - FLASH"]
FLASH = 3 , # [doc = "4 - RCC"]
RCC = 4 , # [doc = "5 - EXTI0_1"]
EXTI0_1 = 5 , # [doc = "6 - EXTI2_3"]
EXTI2_3 = 6 , # [doc = "7 - EXTI4_15"]
EXTI4_15 = 7 , # [doc = "9 - DMA_CHANNEL1"]
DMA_CHANNEL1 = 9 , # [doc = "10 - DMA_CHANNEL2_3"]
DMA_CHANNEL2_3 = 10 , # [doc = "12 - ADC_COMP"]
ADC_COMP = 12 , # [doc = "13 - TIM1_BRK_UP_TRG_COM"]
TIM1_BRK_UP_TRG_COM = 13 , # [doc = "14 - TIM1_CC"]
TIM1_CC = 14 , # [doc = "16 - TIM3"]
TIM3 = 16 , # [doc = "19 - TIM14"]
TIM14 = 19 , # [doc = "21 - TIM16"]
TIM16 = 21 , # [doc = "22 - TIM17"]
TIM17 = 22 , # [doc = "23 - I2C1"]
I2C1 = 23 , # [doc = "25 - SPI1"]
SPI1 = 25 , # [doc = "26 - SPI2"]
SPI2 = 26 , # [doc = "27 - USART1"]
USART1 = 27 , # [doc = "28 - USART2"]
USART2 = 28 , # [doc = "30 - LED"]
LED = 30 , } unsafe impl cortex_m :: interrupt :: InterruptNumber for Interrupt { # [inline (always)]
fn number (self) -> u16 { self as u16 } } # [cfg (feature = "rt")]
mod _vectors { extern "C" { fn WWDG () ; fn PVD () ; fn RTC () ; fn FLASH () ; fn RCC () ; fn EXTI0_1 () ; fn EXTI2_3 () ; fn EXTI4_15 () ; fn DMA_CHANNEL1 () ; fn DMA_CHANNEL2_3 () ; fn ADC_COMP () ; fn TIM1_BRK_UP_TRG_COM () ; fn TIM1_CC () ; fn TIM3 () ; fn TIM14 () ; fn TIM16 () ; fn TIM17 () ; fn I2C1 () ; fn SPI1 () ; fn SPI2 () ; fn USART1 () ; fn USART2 () ; fn LED () ; } pub union Vector { _handler : unsafe extern "C" fn () , _reserved : u32 , } # [link_section = ".vector_table.interrupts"]
# [no_mangle]
pub static __INTERRUPTS : [Vector ; 31]
= [Vector { _handler : WWDG } , Vector { _handler : PVD } , Vector { _handler : RTC } , Vector { _handler : FLASH } , Vector { _handler : RCC } , Vector { _handler : EXTI0_1 } , Vector { _handler : EXTI2_3 } , Vector { _handler : EXTI4_15 } , Vector { _reserved : 0 } , Vector { _handler : DMA_CHANNEL1 } , Vector { _handler : DMA_CHANNEL2_3 } , Vector { _reserved : 0 } , Vector { _handler : ADC_COMP } , Vector { _handler : TIM1_BRK_UP_TRG_COM } , Vector { _handler : TIM1_CC } , Vector { _reserved : 0 } , Vector { _handler : TIM3 } , Vector { _reserved : 0 } , Vector { _reserved : 0 } , Vector { _handler : TIM14 } , Vector { _reserved : 0 } , Vector { _handler : TIM16 } , Vector { _handler : TIM17 } , Vector { _handler : I2C1 } , Vector { _reserved : 0 } , Vector { _handler : SPI1 } , Vector { _handler : SPI2 } , Vector { _handler : USART1 } , Vector { _handler : USART2 } , Vector { _reserved : 0 } , Vector { _handler : LED } ,]
; } pub const UID : uid :: Uid = unsafe { uid :: Uid :: from_ptr (0x1fff_0e00usize as _) } ; pub const CONFIGBYTES : configbytes :: Configbytes = unsafe { configbytes :: Configbytes :: from_ptr (0x1fff_0f00usize as _) } ; pub const TIM3 : timer :: TimGp16 = unsafe { timer :: TimGp16 :: from_ptr (0x4000_0400usize as _) } ; pub const TIM14 : timer :: Tim1ch = unsafe { timer :: Tim1ch :: from_ptr (0x4000_2000usize as _) } ; pub const SPI2 : spi :: Spi = unsafe { spi :: Spi :: from_ptr (0x4000_3800usize as _) } ; pub const USART2 : usart :: Usart = unsafe { usart :: Usart :: from_ptr (0x4000_4400usize as _) } ; pub const I2C1 : i2c :: I2c = unsafe { i2c :: I2c :: from_ptr (0x4000_5400usize as _) } ; pub const SYSCFG : syscfg :: Syscfg = unsafe { syscfg :: Syscfg :: from_ptr (0x4001_0000usize as _) } ; pub const ADC1 : adc :: Adc = unsafe { adc :: Adc :: from_ptr (0x4001_2400usize as _) } ; pub const TIM1 : timer :: TimAdv = unsafe { timer :: TimAdv :: from_ptr (0x4001_2c00usize as _) } ; pub const SPI1 : spi :: Spi = unsafe { spi :: Spi :: from_ptr (0x4001_3000usize as _) } ; pub const USART1 : usart :: Usart = unsafe { usart :: Usart :: from_ptr (0x4001_3800usize as _) } ; pub const TIM16 : timer :: Tim1chCmp = unsafe { timer :: Tim1chCmp :: from_ptr (0x4001_4400usize as _) } ; pub const TIM17 : timer :: Tim1chCmp = unsafe { timer :: Tim1chCmp :: from_ptr (0x4001_4800usize as _) } ; pub const DBGMCU : dbgmcu :: Dbgmcu = unsafe { dbgmcu :: Dbgmcu :: from_ptr (0x4001_5800usize as _) } ; pub const DMA1 : dma :: Dma = unsafe { dma :: Dma :: from_ptr (0x4002_0000usize as _) } ; pub const RCC : rcc :: Rcc = unsafe { rcc :: Rcc :: from_ptr (0x4002_1000usize as _) } ; pub const EXTI : exti :: Exti = unsafe { exti :: Exti :: from_ptr (0x4002_1800usize as _) } ; pub const FLASH : flash :: Flash = unsafe { flash :: Flash :: from_ptr (0x4002_2000usize as _) } ; pub const GPIOA : gpio :: Gpio = unsafe { gpio :: Gpio :: from_ptr (0x5000_0000usize as _) } ; pub const GPIOB : gpio :: Gpio = unsafe { gpio :: Gpio :: from_ptr (0x5000_0400usize as _) } ; pub const GPIOF : gpio :: Gpio = unsafe { gpio :: Gpio :: from_ptr (0x5000_1400usize as _) } ; # [doc = r" Number available in the NVIC for configuring priority"]
# [cfg (feature = "rt")]
pub const NVIC_PRIO_BITS : u8 = 2 ; # [cfg (feature = "rt")]
pub use cortex_m_rt :: interrupt ; # [cfg (feature = "rt")]
pub use Interrupt as interrupt ;pub fn GPIO(n: usize) -> gpio::Gpio {
            unsafe { gpio::Gpio::from_ptr((1342177280 + 1024*n) as _) }
        }#[path="../../peripherals/adc_v1.rs"] pub mod adc;
#[path="../../peripherals/configbytes_f030.rs"] pub mod configbytes;
#[path="../../peripherals/dbgmcu_f030.rs"] pub mod dbgmcu;
#[path="../../peripherals/dma_f030.rs"] pub mod dma;
#[path="../../peripherals/exti_v1.rs"] pub mod exti;
#[path="../../peripherals/flash_f030.rs"] pub mod flash;
#[path="../../peripherals/gpio_v1.rs"] pub mod gpio;
#[path="../../peripherals/i2c_v1.rs"] pub mod i2c;
#[path="../../peripherals/rcc_f030.rs"] pub mod rcc;
#[path="../../peripherals/spi_v1.rs"] pub mod spi;
#[path="../../peripherals/syscfg_f030.rs"] pub mod syscfg;
#[path="../../peripherals/timer_v1.rs"] pub mod timer;
#[path="../../peripherals/uid_v1.rs"] pub mod uid;
#[path="../../peripherals/usart_v1.rs"] pub mod usart;
pub const CORE_INDEX: usize = 0;
pub const FLASH_BASE: usize = 134217728;
pub const FLASH_SIZE: usize = 32768;
pub const PAGE_SIZE: usize = 128;
pub const SECTOR_SIZE: usize = 4096;
