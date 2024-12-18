#![allow(clippy::missing_safety_doc)]
                #![allow(clippy::identity_op)]
                #![allow(clippy::unnecessary_cast)]
                #![allow(clippy::erasing_op)]

# [doc = "Analog-to-digital converter"]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Adc { ptr : * mut u8 } unsafe impl Send for Adc { } unsafe impl Sync for Adc { } impl Adc { # [inline (always)]
pub const unsafe fn from_ptr (ptr : * mut ()) -> Self { Self { ptr : ptr as _ , } } # [inline (always)]
pub const fn as_ptr (& self) -> * mut () { self . ptr as _ } # [doc = "interrupt and status register"]
# [inline (always)]
pub const fn isr (self) -> crate :: common :: Reg < regs :: Isr , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x0usize) as _) } } # [doc = "interrupt enable register"]
# [inline (always)]
pub const fn ier (self) -> crate :: common :: Reg < regs :: Ier , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x04usize) as _) } } # [doc = "control register"]
# [inline (always)]
pub const fn cr (self) -> crate :: common :: Reg < regs :: Cr , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x08usize) as _) } } # [doc = "configuration register 1"]
# [inline (always)]
pub const fn cfgr1 (self) -> crate :: common :: Reg < regs :: Cfgr1 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x0cusize) as _) } } # [doc = "configuration register 2"]
# [inline (always)]
pub const fn cfgr2 (self) -> crate :: common :: Reg < regs :: Cfgr2 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x10usize) as _) } } # [doc = "sampling time register"]
# [inline (always)]
pub const fn smpr (self) -> crate :: common :: Reg < regs :: Smpr , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x14usize) as _) } } # [doc = "watchdog threshold register"]
# [inline (always)]
pub const fn tr (self) -> crate :: common :: Reg < regs :: Tr , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x20usize) as _) } } # [doc = "channel selection register"]
# [inline (always)]
pub const fn chselr (self) -> crate :: common :: Reg < regs :: Chselr , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x28usize) as _) } } # [doc = "data register"]
# [inline (always)]
pub const fn dr (self) -> crate :: common :: Reg < regs :: Dr , crate :: common :: R > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x40usize) as _) } } # [doc = "ADC calibration configuration and status register."]
# [inline (always)]
pub const fn ccsr (self) -> crate :: common :: Reg < regs :: Ccsr , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x44usize) as _) } } # [doc = "ADC calibration result register 1."]
# [inline (always)]
pub const fn calrr1 (self) -> crate :: common :: Reg < regs :: Calrr1 , crate :: common :: R > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x48usize) as _) } } # [doc = "ADC calibration result register 2."]
# [inline (always)]
pub const fn calrr2 (self) -> crate :: common :: Reg < regs :: Calrr2 , crate :: common :: R > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x4cusize) as _) } } # [doc = "ADC calibration factor input register 1."]
# [inline (always)]
pub const fn calfir1 (self) -> crate :: common :: Reg < regs :: Calfir1 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x50usize) as _) } } # [doc = "ADC calibration factor input register 2."]
# [inline (always)]
pub const fn calfir2 (self) -> crate :: common :: Reg < regs :: Calfir2 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x54usize) as _) } } # [doc = "common configuration register"]
# [inline (always)]
pub const fn ccr (self) -> crate :: common :: Reg < regs :: Ccr , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x0308usize) as _) } } } pub mod regs { # [doc = "ADC calibration factor input register 1."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Calfir1 (pub u32) ; impl Calfir1 { # [doc = "Calibration C4 factor input."]
# [inline (always)]
pub const fn calc4io (& self) -> u8 { let val = (self . 0 >> 0usize) & 0xff ; val as u8 } # [doc = "Calibration C4 factor input."]
# [inline (always)]
pub fn set_calc4io (& mut self , val : u8) { self . 0 = (self . 0 & ! (0xff << 0usize)) | (((val as u32) & 0xff) << 0usize) ; } # [doc = "Calibration C5 factor input."]
# [inline (always)]
pub const fn calc5io (& self) -> u8 { let val = (self . 0 >> 8usize) & 0xff ; val as u8 } # [doc = "Calibration C5 factor input."]
# [inline (always)]
pub fn set_calc5io (& mut self , val : u8) { self . 0 = (self . 0 & ! (0xff << 8usize)) | (((val as u32) & 0xff) << 8usize) ; } # [doc = "Calibration offset factor input."]
# [inline (always)]
pub const fn calbio (& self) -> u8 { let val = (self . 0 >> 16usize) & 0x7f ; val as u8 } # [doc = "Calibration offset factor input."]
# [inline (always)]
pub fn set_calbio (& mut self , val : u8) { self . 0 = (self . 0 & ! (0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize) ; } } impl Default for Calfir1 { # [inline (always)]
fn default () -> Calfir1 { Calfir1 (0) } } # [doc = "ADC calibration factor input register 2."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Calfir2 (pub u32) ; impl Calfir2 { # [doc = "Calibration C0 factor input."]
# [inline (always)]
pub const fn calc0io (& self) -> u8 { let val = (self . 0 >> 0usize) & 0xff ; val as u8 } # [doc = "Calibration C0 factor input."]
# [inline (always)]
pub fn set_calc0io (& mut self , val : u8) { self . 0 = (self . 0 & ! (0xff << 0usize)) | (((val as u32) & 0xff) << 0usize) ; } # [doc = "Calibration C1 factor input."]
# [inline (always)]
pub const fn calc1io (& self) -> u8 { let val = (self . 0 >> 8usize) & 0xff ; val as u8 } # [doc = "Calibration C1 factor input."]
# [inline (always)]
pub fn set_calc1io (& mut self , val : u8) { self . 0 = (self . 0 & ! (0xff << 8usize)) | (((val as u32) & 0xff) << 8usize) ; } # [doc = "Calibration C2 factor input."]
# [inline (always)]
pub const fn calc2io (& self) -> u8 { let val = (self . 0 >> 16usize) & 0xff ; val as u8 } # [doc = "Calibration C2 factor input."]
# [inline (always)]
pub fn set_calc2io (& mut self , val : u8) { self . 0 = (self . 0 & ! (0xff << 16usize)) | (((val as u32) & 0xff) << 16usize) ; } # [doc = "Calibration C3 factor input."]
# [inline (always)]
pub const fn calc3io (& self) -> u8 { let val = (self . 0 >> 24usize) & 0xff ; val as u8 } # [doc = "Calibration C3 factor input."]
# [inline (always)]
pub fn set_calc3io (& mut self , val : u8) { self . 0 = (self . 0 & ! (0xff << 24usize)) | (((val as u32) & 0xff) << 24usize) ; } } impl Default for Calfir2 { # [inline (always)]
fn default () -> Calfir2 { Calfir2 (0) } } # [doc = "ADC calibration result register 1."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Calrr1 (pub u32) ; impl Calrr1 { # [doc = "C4 result."]
# [inline (always)]
pub const fn calc4out (& self) -> u8 { let val = (self . 0 >> 0usize) & 0xff ; val as u8 } # [doc = "C4 result."]
# [inline (always)]
pub fn set_calc4out (& mut self , val : u8) { self . 0 = (self . 0 & ! (0xff << 0usize)) | (((val as u32) & 0xff) << 0usize) ; } # [doc = "C5 result."]
# [inline (always)]
pub const fn calc5out (& self) -> u8 { let val = (self . 0 >> 8usize) & 0xff ; val as u8 } # [doc = "C5 result."]
# [inline (always)]
pub fn set_calc5out (& mut self , val : u8) { self . 0 = (self . 0 & ! (0xff << 8usize)) | (((val as u32) & 0xff) << 8usize) ; } # [doc = "offset result."]
# [inline (always)]
pub const fn calbout (& self) -> u8 { let val = (self . 0 >> 16usize) & 0x7f ; val as u8 } # [doc = "offset result."]
# [inline (always)]
pub fn set_calbout (& mut self , val : u8) { self . 0 = (self . 0 & ! (0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize) ; } } impl Default for Calrr1 { # [inline (always)]
fn default () -> Calrr1 { Calrr1 (0) } } # [doc = "ADC calibration result register 2."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Calrr2 (pub u32) ; impl Calrr2 { # [doc = "C0 result."]
# [inline (always)]
pub const fn calc0out (& self) -> u8 { let val = (self . 0 >> 0usize) & 0xff ; val as u8 } # [doc = "C0 result."]
# [inline (always)]
pub fn set_calc0out (& mut self , val : u8) { self . 0 = (self . 0 & ! (0xff << 0usize)) | (((val as u32) & 0xff) << 0usize) ; } # [doc = "C1 result."]
# [inline (always)]
pub const fn calc1out (& self) -> u8 { let val = (self . 0 >> 8usize) & 0xff ; val as u8 } # [doc = "C1 result."]
# [inline (always)]
pub fn set_calc1out (& mut self , val : u8) { self . 0 = (self . 0 & ! (0xff << 8usize)) | (((val as u32) & 0xff) << 8usize) ; } # [doc = "C2 result."]
# [inline (always)]
pub const fn calc2out (& self) -> u8 { let val = (self . 0 >> 16usize) & 0xff ; val as u8 } # [doc = "C2 result."]
# [inline (always)]
pub fn set_calc2out (& mut self , val : u8) { self . 0 = (self . 0 & ! (0xff << 16usize)) | (((val as u32) & 0xff) << 16usize) ; } # [doc = "C3 result."]
# [inline (always)]
pub const fn calc3out (& self) -> u8 { let val = (self . 0 >> 24usize) & 0xff ; val as u8 } # [doc = "C3 result."]
# [inline (always)]
pub fn set_calc3out (& mut self , val : u8) { self . 0 = (self . 0 & ! (0xff << 24usize)) | (((val as u32) & 0xff) << 24usize) ; } } impl Default for Calrr2 { # [inline (always)]
fn default () -> Calrr2 { Calrr2 (0) } } # [doc = "common configuration register"]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Ccr (pub u32) ; impl Ccr { # [doc = "Temperature sensor and VREFINT enable"]
# [inline (always)]
pub const fn vrefen (& self) -> bool { let val = (self . 0 >> 22usize) & 0x01 ; val != 0 } # [doc = "Temperature sensor and VREFINT enable"]
# [inline (always)]
pub fn set_vrefen (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize) ; } # [doc = "Temperature sensor enable"]
# [inline (always)]
pub const fn tsen (& self) -> bool { let val = (self . 0 >> 23usize) & 0x01 ; val != 0 } # [doc = "Temperature sensor enable"]
# [inline (always)]
pub fn set_tsen (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize) ; } } impl Default for Ccr { # [inline (always)]
fn default () -> Ccr { Ccr (0) } } # [doc = "ADC calibration configuration and status register."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Ccsr (pub u32) ; impl Ccsr { # [doc = "Calibration contents selection."]
# [inline (always)]
pub const fn calsel (& self) -> bool { let val = (self . 0 >> 11usize) & 0x01 ; val != 0 } # [doc = "Calibration contents selection."]
# [inline (always)]
pub fn set_calsel (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize) ; } # [doc = "Calibration sample time selection."]
# [inline (always)]
pub const fn calsmp (& self) -> u8 { let val = (self . 0 >> 12usize) & 0x03 ; val as u8 } # [doc = "Calibration sample time selection."]
# [inline (always)]
pub fn set_calsmp (& mut self , val : u8) { self . 0 = (self . 0 & ! (0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize) ; } # [doc = "Calibration factor selection."]
# [inline (always)]
pub const fn calset (& self) -> bool { let val = (self . 0 >> 15usize) & 0x01 ; val != 0 } # [doc = "Calibration factor selection."]
# [inline (always)]
pub fn set_calset (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize) ; } # [doc = "Calibration fail flag."]
# [inline (always)]
pub const fn calfail (& self) -> bool { let val = (self . 0 >> 30usize) & 0x01 ; val != 0 } # [doc = "Calibration fail flag."]
# [inline (always)]
pub fn set_calfail (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize) ; } # [doc = "Calibration flag."]
# [inline (always)]
pub const fn calon (& self) -> bool { let val = (self . 0 >> 31usize) & 0x01 ; val != 0 } # [doc = "Calibration flag."]
# [inline (always)]
pub fn set_calon (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize) ; } } impl Default for Ccsr { # [inline (always)]
fn default () -> Ccsr { Ccsr (0) } } # [doc = "configuration register 1"]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Cfgr1 (pub u32) ; impl Cfgr1 { # [doc = "Direct memory access enable"]
# [inline (always)]
pub const fn dmaen (& self) -> bool { let val = (self . 0 >> 0usize) & 0x01 ; val != 0 } # [doc = "Direct memory access enable"]
# [inline (always)]
pub fn set_dmaen (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize) ; } # [doc = "Direct memory access configuration"]
# [inline (always)]
pub const fn dmacfg (& self) -> super :: vals :: Dmacfg { let val = (self . 0 >> 1usize) & 0x01 ; super :: vals :: Dmacfg :: from_bits (val as u8) } # [doc = "Direct memory access configuration"]
# [inline (always)]
pub fn set_dmacfg (& mut self , val : super :: vals :: Dmacfg) { self . 0 = (self . 0 & ! (0x01 << 1usize)) | (((val . to_bits () as u32) & 0x01) << 1usize) ; } # [doc = "Scan sequence direction"]
# [inline (always)]
pub const fn scandir (& self) -> super :: vals :: Scandir { let val = (self . 0 >> 2usize) & 0x01 ; super :: vals :: Scandir :: from_bits (val as u8) } # [doc = "Scan sequence direction"]
# [inline (always)]
pub fn set_scandir (& mut self , val : super :: vals :: Scandir) { self . 0 = (self . 0 & ! (0x01 << 2usize)) | (((val . to_bits () as u32) & 0x01) << 2usize) ; } # [doc = "Data resolution"]
# [inline (always)]
pub const fn res (& self) -> super :: vals :: Res { let val = (self . 0 >> 3usize) & 0x03 ; super :: vals :: Res :: from_bits (val as u8) } # [doc = "Data resolution"]
# [inline (always)]
pub fn set_res (& mut self , val : super :: vals :: Res) { self . 0 = (self . 0 & ! (0x03 << 3usize)) | (((val . to_bits () as u32) & 0x03) << 3usize) ; } # [doc = "Data alignment"]
# [inline (always)]
pub const fn align (& self) -> super :: vals :: Align { let val = (self . 0 >> 5usize) & 0x01 ; super :: vals :: Align :: from_bits (val as u8) } # [doc = "Data alignment"]
# [inline (always)]
pub fn set_align (& mut self , val : super :: vals :: Align) { self . 0 = (self . 0 & ! (0x01 << 5usize)) | (((val . to_bits () as u32) & 0x01) << 5usize) ; } # [doc = "External trigger selection"]
# [inline (always)]
pub const fn extsel (& self) -> u8 { let val = (self . 0 >> 6usize) & 0x07 ; val as u8 } # [doc = "External trigger selection"]
# [inline (always)]
pub fn set_extsel (& mut self , val : u8) { self . 0 = (self . 0 & ! (0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize) ; } # [doc = "External trigger enable and polarity selection"]
# [inline (always)]
pub const fn exten (& self) -> super :: vals :: Exten { let val = (self . 0 >> 10usize) & 0x03 ; super :: vals :: Exten :: from_bits (val as u8) } # [doc = "External trigger enable and polarity selection"]
# [inline (always)]
pub fn set_exten (& mut self , val : super :: vals :: Exten) { self . 0 = (self . 0 & ! (0x03 << 10usize)) | (((val . to_bits () as u32) & 0x03) << 10usize) ; } # [doc = "Overrun management mode"]
# [inline (always)]
pub const fn ovrmod (& self) -> super :: vals :: Ovrmod { let val = (self . 0 >> 12usize) & 0x01 ; super :: vals :: Ovrmod :: from_bits (val as u8) } # [doc = "Overrun management mode"]
# [inline (always)]
pub fn set_ovrmod (& mut self , val : super :: vals :: Ovrmod) { self . 0 = (self . 0 & ! (0x01 << 12usize)) | (((val . to_bits () as u32) & 0x01) << 12usize) ; } # [doc = "Continuous conversion"]
# [inline (always)]
pub const fn cont (& self) -> bool { let val = (self . 0 >> 13usize) & 0x01 ; val != 0 } # [doc = "Continuous conversion"]
# [inline (always)]
pub fn set_cont (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize) ; } # [doc = "Wait conversion mode"]
# [inline (always)]
pub const fn wait (& self) -> bool { let val = (self . 0 >> 14usize) & 0x01 ; val != 0 } # [doc = "Wait conversion mode"]
# [inline (always)]
pub fn set_wait (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize) ; } # [doc = "Discontinuous mode"]
# [inline (always)]
pub const fn discen (& self) -> bool { let val = (self . 0 >> 16usize) & 0x01 ; val != 0 } # [doc = "Discontinuous mode"]
# [inline (always)]
pub fn set_discen (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize) ; } # [doc = "Enable the watchdog on a single channel or on all channels"]
# [inline (always)]
pub const fn awdsgl (& self) -> super :: vals :: Awdsgl { let val = (self . 0 >> 22usize) & 0x01 ; super :: vals :: Awdsgl :: from_bits (val as u8) } # [doc = "Enable the watchdog on a single channel or on all channels"]
# [inline (always)]
pub fn set_awdsgl (& mut self , val : super :: vals :: Awdsgl) { self . 0 = (self . 0 & ! (0x01 << 22usize)) | (((val . to_bits () as u32) & 0x01) << 22usize) ; } # [doc = "Analog watchdog enable"]
# [inline (always)]
pub const fn awden (& self) -> bool { let val = (self . 0 >> 23usize) & 0x01 ; val != 0 } # [doc = "Analog watchdog enable"]
# [inline (always)]
pub fn set_awden (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize) ; } # [doc = "Analog watchdog channel selection"]
# [inline (always)]
pub const fn awdch (& self) -> u8 { let val = (self . 0 >> 26usize) & 0x0f ; val as u8 } # [doc = "Analog watchdog channel selection"]
# [inline (always)]
pub fn set_awdch (& mut self , val : u8) { self . 0 = (self . 0 & ! (0x0f << 26usize)) | (((val as u32) & 0x0f) << 26usize) ; } } impl Default for Cfgr1 { # [inline (always)]
fn default () -> Cfgr1 { Cfgr1 (0) } } # [doc = "configuration register 2"]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Cfgr2 (pub u32) ; impl Cfgr2 { # [doc = "ADC clock mode"]
# [inline (always)]
pub const fn ckmode (& self) -> super :: vals :: Ckmode { let val = (self . 0 >> 18usize) & 0x0f ; super :: vals :: Ckmode :: from_bits (val as u8) } # [doc = "ADC clock mode"]
# [inline (always)]
pub fn set_ckmode (& mut self , val : super :: vals :: Ckmode) { self . 0 = (self . 0 & ! (0x0f << 18usize)) | (((val . to_bits () as u32) & 0x0f) << 18usize) ; } } impl Default for Cfgr2 { # [inline (always)]
fn default () -> Cfgr2 { Cfgr2 (0) } } # [doc = "channel selection register"]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Chselr (pub u32) ; impl Chselr { # [doc = "Channel-x selection"]
# [inline (always)]
pub const fn chselx (& self , n : usize) -> bool { assert ! (n < 13usize) ; let offs = 0usize + n * 1usize ; let val = (self . 0 >> offs) & 0x01 ; val != 0 } # [doc = "Channel-x selection"]
# [inline (always)]
pub fn set_chselx (& mut self , n : usize , val : bool) { assert ! (n < 13usize) ; let offs = 0usize + n * 1usize ; self . 0 = (self . 0 & ! (0x01 << offs)) | (((val as u32) & 0x01) << offs) ; } } impl Default for Chselr { # [inline (always)]
fn default () -> Chselr { Chselr (0) } } # [doc = "control register"]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Cr (pub u32) ; impl Cr { # [doc = "ADC enable command"]
# [inline (always)]
pub const fn aden (& self) -> bool { let val = (self . 0 >> 0usize) & 0x01 ; val != 0 } # [doc = "ADC enable command"]
# [inline (always)]
pub fn set_aden (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize) ; } # [doc = "ADC start conversion command"]
# [inline (always)]
pub const fn adstart (& self) -> bool { let val = (self . 0 >> 2usize) & 0x01 ; val != 0 } # [doc = "ADC start conversion command"]
# [inline (always)]
pub fn set_adstart (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize) ; } # [doc = "ADC stop conversion command"]
# [inline (always)]
pub const fn adstp (& self) -> bool { let val = (self . 0 >> 4usize) & 0x01 ; val != 0 } # [doc = "ADC stop conversion command"]
# [inline (always)]
pub fn set_adstp (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize) ; } # [doc = "ADC calibration"]
# [inline (always)]
pub const fn adcal (& self) -> bool { let val = (self . 0 >> 31usize) & 0x01 ; val != 0 } # [doc = "ADC calibration"]
# [inline (always)]
pub fn set_adcal (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize) ; } } impl Default for Cr { # [inline (always)]
fn default () -> Cr { Cr (0) } } # [doc = "data register"]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Dr (pub u32) ; impl Dr { # [doc = "Converted data"]
# [inline (always)]
pub const fn data (& self) -> u16 { let val = (self . 0 >> 0usize) & 0xffff ; val as u16 } # [doc = "Converted data"]
# [inline (always)]
pub fn set_data (& mut self , val : u16) { self . 0 = (self . 0 & ! (0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize) ; } } impl Default for Dr { # [inline (always)]
fn default () -> Dr { Dr (0) } } # [doc = "interrupt enable register"]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Ier (pub u32) ; impl Ier { # [doc = "End of sampling flag interrupt enable"]
# [inline (always)]
pub const fn eosmpie (& self) -> bool { let val = (self . 0 >> 1usize) & 0x01 ; val != 0 } # [doc = "End of sampling flag interrupt enable"]
# [inline (always)]
pub fn set_eosmpie (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize) ; } # [doc = "End of conversion interrupt enable"]
# [inline (always)]
pub const fn eocie (& self) -> bool { let val = (self . 0 >> 2usize) & 0x01 ; val != 0 } # [doc = "End of conversion interrupt enable"]
# [inline (always)]
pub fn set_eocie (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize) ; } # [doc = "End of conversion sequence interrupt enable"]
# [inline (always)]
pub const fn eoseqie (& self) -> bool { let val = (self . 0 >> 3usize) & 0x01 ; val != 0 } # [doc = "End of conversion sequence interrupt enable"]
# [inline (always)]
pub fn set_eoseqie (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize) ; } # [doc = "Overrun interrupt enable"]
# [inline (always)]
pub const fn ovrie (& self) -> bool { let val = (self . 0 >> 4usize) & 0x01 ; val != 0 } # [doc = "Overrun interrupt enable"]
# [inline (always)]
pub fn set_ovrie (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize) ; } # [doc = "Analog watchdog interrupt enable"]
# [inline (always)]
pub const fn awdie (& self) -> bool { let val = (self . 0 >> 7usize) & 0x01 ; val != 0 } # [doc = "Analog watchdog interrupt enable"]
# [inline (always)]
pub fn set_awdie (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize) ; } } impl Default for Ier { # [inline (always)]
fn default () -> Ier { Ier (0) } } # [doc = "interrupt and status register"]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Isr (pub u32) ; impl Isr { # [doc = "End of sampling flag"]
# [inline (always)]
pub const fn eosmp (& self) -> bool { let val = (self . 0 >> 1usize) & 0x01 ; val != 0 } # [doc = "End of sampling flag"]
# [inline (always)]
pub fn set_eosmp (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize) ; } # [doc = "End of conversion flag"]
# [inline (always)]
pub const fn eoc (& self) -> bool { let val = (self . 0 >> 2usize) & 0x01 ; val != 0 } # [doc = "End of conversion flag"]
# [inline (always)]
pub fn set_eoc (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize) ; } # [doc = "End of sequence flag"]
# [inline (always)]
pub const fn eoseq (& self) -> bool { let val = (self . 0 >> 3usize) & 0x01 ; val != 0 } # [doc = "End of sequence flag"]
# [inline (always)]
pub fn set_eoseq (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize) ; } # [doc = "ADC overrun"]
# [inline (always)]
pub const fn ovr (& self) -> bool { let val = (self . 0 >> 4usize) & 0x01 ; val != 0 } # [doc = "ADC overrun"]
# [inline (always)]
pub fn set_ovr (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize) ; } # [doc = "Analog watchdog flag"]
# [inline (always)]
pub const fn awd (& self) -> bool { let val = (self . 0 >> 7usize) & 0x01 ; val != 0 } # [doc = "Analog watchdog flag"]
# [inline (always)]
pub fn set_awd (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize) ; } } impl Default for Isr { # [inline (always)]
fn default () -> Isr { Isr (0) } } # [doc = "sampling time register"]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Smpr (pub u32) ; impl Smpr { # [doc = "Sampling time selection"]
# [inline (always)]
pub const fn smp (& self) -> super :: vals :: SampleTime { let val = (self . 0 >> 0usize) & 0x07 ; super :: vals :: SampleTime :: from_bits (val as u8) } # [doc = "Sampling time selection"]
# [inline (always)]
pub fn set_smp (& mut self , val : super :: vals :: SampleTime) { self . 0 = (self . 0 & ! (0x07 << 0usize)) | (((val . to_bits () as u32) & 0x07) << 0usize) ; } } impl Default for Smpr { # [inline (always)]
fn default () -> Smpr { Smpr (0) } } # [doc = "watchdog threshold register"]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Tr (pub u32) ; impl Tr { # [doc = "Analog watchdog lower threshold"]
# [inline (always)]
pub const fn lt (& self) -> u16 { let val = (self . 0 >> 0usize) & 0x0fff ; val as u16 } # [doc = "Analog watchdog lower threshold"]
# [inline (always)]
pub fn set_lt (& mut self , val : u16) { self . 0 = (self . 0 & ! (0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize) ; } # [doc = "Analog watchdog higher threshold"]
# [inline (always)]
pub const fn ht (& self) -> u16 { let val = (self . 0 >> 16usize) & 0x0fff ; val as u16 } # [doc = "Analog watchdog higher threshold"]
# [inline (always)]
pub fn set_ht (& mut self , val : u16) { self . 0 = (self . 0 & ! (0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize) ; } } impl Default for Tr { # [inline (always)]
fn default () -> Tr { Tr (0) } } } pub mod vals { # [repr (u8)]
# [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd)]
pub enum Align { # [doc = "Right alignment"]
RIGHT = 0x0 , # [doc = "Left alignment"]
LEFT = 0x01 , } impl Align { # [inline (always)]
pub const fn from_bits (val : u8) -> Align { unsafe { core :: mem :: transmute (val & 0x01) } } # [inline (always)]
pub const fn to_bits (self) -> u8 { unsafe { core :: mem :: transmute (self) } } } impl From < u8 > for Align { # [inline (always)]
fn from (val : u8) -> Align { Align :: from_bits (val) } } impl From < Align > for u8 { # [inline (always)]
fn from (val : Align) -> u8 { Align :: to_bits (val) } } # [repr (u8)]
# [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd)]
pub enum Awdsgl { # [doc = "Analog watchdog enabled on all channels"]
ALLCHANNELS = 0x0 , # [doc = "Analog watchdog enabled on a single channel"]
SINGLECHANNEL = 0x01 , } impl Awdsgl { # [inline (always)]
pub const fn from_bits (val : u8) -> Awdsgl { unsafe { core :: mem :: transmute (val & 0x01) } } # [inline (always)]
pub const fn to_bits (self) -> u8 { unsafe { core :: mem :: transmute (self) } } } impl From < u8 > for Awdsgl { # [inline (always)]
fn from (val : u8) -> Awdsgl { Awdsgl :: from_bits (val) } } impl From < Awdsgl > for u8 { # [inline (always)]
fn from (val : Awdsgl) -> u8 { Awdsgl :: to_bits (val) } } # [repr (u8)]
# [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd)]
pub enum Ckmode { # [doc = "Synchronous clock mode"]
PCLK = 0x0 , # [doc = "Synchronous clock mode (PCLK/2)"]
PCLK_DIV2 = 0x01 , # [doc = "Sychronous clock mode (PCLK/4)"]
PCLK_DIV4 = 0x02 , # [doc = "Sychronous clock mode (PCLK/8)"]
PCLK_DIV8 = 0x03 , # [doc = "Sychronous clock mode (PCLK/16)"]
PCLK_DIV16 = 0x04 , # [doc = "Sychronous clock mode (PCLK/32)"]
PCLK_DIV32 = 0x05 , # [doc = "Sychronous clock mode (PCLK/64)"]
PCLK_DIV64 = 0x06 , _RESERVED_7 = 0x07 , # [doc = "Asynchronous clock mode"]
HSI = 0x08 , # [doc = "Asynchronous clock mode (HSI/2)"]
HSI_DIV2 = 0x09 , # [doc = "Asynchronous clock mode (HSI/4)"]
HSI_DIV4 = 0x0a , # [doc = "Asynchronous clock mode (HSI/8)"]
HSI_DIV8 = 0x0b , # [doc = "Asynchronous clock mode (HSI/16)"]
HSI_DIV16 = 0x0c , # [doc = "Asynchronous clock mode (HSI/32)"]
HSI_DIV32 = 0x0d , # [doc = "Asynchronous clock mode (HSI/64)"]
HSI_DIV64 = 0x0e , _RESERVED_f = 0x0f , } impl Ckmode { # [inline (always)]
pub const fn from_bits (val : u8) -> Ckmode { unsafe { core :: mem :: transmute (val & 0x0f) } } # [inline (always)]
pub const fn to_bits (self) -> u8 { unsafe { core :: mem :: transmute (self) } } } impl From < u8 > for Ckmode { # [inline (always)]
fn from (val : u8) -> Ckmode { Ckmode :: from_bits (val) } } impl From < Ckmode > for u8 { # [inline (always)]
fn from (val : Ckmode) -> u8 { Ckmode :: to_bits (val) } } # [repr (u8)]
# [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd)]
pub enum Dmacfg { # [doc = "DMA One Shot mode selected"]
ONESHOT = 0x0 , # [doc = "DMA Circular mode selected"]
CIRCULAR = 0x01 , } impl Dmacfg { # [inline (always)]
pub const fn from_bits (val : u8) -> Dmacfg { unsafe { core :: mem :: transmute (val & 0x01) } } # [inline (always)]
pub const fn to_bits (self) -> u8 { unsafe { core :: mem :: transmute (self) } } } impl From < u8 > for Dmacfg { # [inline (always)]
fn from (val : u8) -> Dmacfg { Dmacfg :: from_bits (val) } } impl From < Dmacfg > for u8 { # [inline (always)]
fn from (val : Dmacfg) -> u8 { Dmacfg :: to_bits (val) } } # [repr (u8)]
# [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd)]
pub enum Exten { # [doc = "Trigger detection disabled"]
DISABLED = 0x0 , # [doc = "Trigger detection on the rising edge"]
RISINGEDGE = 0x01 , # [doc = "Trigger detection on the falling edge"]
FALLINGEDGE = 0x02 , # [doc = "Trigger detection on both the rising and falling edges"]
BOTHEDGES = 0x03 , } impl Exten { # [inline (always)]
pub const fn from_bits (val : u8) -> Exten { unsafe { core :: mem :: transmute (val & 0x03) } } # [inline (always)]
pub const fn to_bits (self) -> u8 { unsafe { core :: mem :: transmute (self) } } } impl From < u8 > for Exten { # [inline (always)]
fn from (val : u8) -> Exten { Exten :: from_bits (val) } } impl From < Exten > for u8 { # [inline (always)]
fn from (val : Exten) -> u8 { Exten :: to_bits (val) } } # [repr (u8)]
# [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd)]
pub enum Ovrmod { # [doc = "ADC_DR register is preserved with the old data when an overrun is detected"]
PRESERVED = 0x0 , # [doc = "ADC_DR register is overwritten with the last conversion result when an overrun is detected"]
OVERWRITTEN = 0x01 , } impl Ovrmod { # [inline (always)]
pub const fn from_bits (val : u8) -> Ovrmod { unsafe { core :: mem :: transmute (val & 0x01) } } # [inline (always)]
pub const fn to_bits (self) -> u8 { unsafe { core :: mem :: transmute (self) } } } impl From < u8 > for Ovrmod { # [inline (always)]
fn from (val : u8) -> Ovrmod { Ovrmod :: from_bits (val) } } impl From < Ovrmod > for u8 { # [inline (always)]
fn from (val : Ovrmod) -> u8 { Ovrmod :: to_bits (val) } } # [repr (u8)]
# [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd)]
pub enum Res { # [doc = "12-bit (14 ADCCLK cycles)"]
BITS12 = 0x0 , # [doc = "10-bit (13 ADCCLK cycles)"]
BITS10 = 0x01 , # [doc = "8-bit (11 ADCCLK cycles)"]
BITS8 = 0x02 , # [doc = "6-bit (9 ADCCLK cycles)"]
BITS6 = 0x03 , } impl Res { # [inline (always)]
pub const fn from_bits (val : u8) -> Res { unsafe { core :: mem :: transmute (val & 0x03) } } # [inline (always)]
pub const fn to_bits (self) -> u8 { unsafe { core :: mem :: transmute (self) } } } impl From < u8 > for Res { # [inline (always)]
fn from (val : u8) -> Res { Res :: from_bits (val) } } impl From < Res > for u8 { # [inline (always)]
fn from (val : Res) -> u8 { Res :: to_bits (val) } } # [repr (u8)]
# [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd)]
pub enum SampleTime { # [doc = "3.5 ADC clock cycles"]
CYCLES3_5 = 0x0 , # [doc = "5.5 ADC clock cycles"]
CYCLES5_5 = 0x01 , # [doc = "7.5 ADC clock cycles"]
CYCLES7_5 = 0x02 , # [doc = "13.5 ADC clock cycles"]
CYCLES13_5 = 0x03 , # [doc = "28.5 ADC clock cycles"]
CYCLES28_5 = 0x04 , # [doc = "41.5 ADC clock cycles"]
CYCLES41_5 = 0x05 , # [doc = "71.5 ADC clock cycles"]
CYCLES71_5 = 0x06 , # [doc = "239.5 ADC clock cycles"]
CYCLES239_5 = 0x07 , } impl SampleTime { # [inline (always)]
pub const fn from_bits (val : u8) -> SampleTime { unsafe { core :: mem :: transmute (val & 0x07) } } # [inline (always)]
pub const fn to_bits (self) -> u8 { unsafe { core :: mem :: transmute (self) } } } impl From < u8 > for SampleTime { # [inline (always)]
fn from (val : u8) -> SampleTime { SampleTime :: from_bits (val) } } impl From < SampleTime > for u8 { # [inline (always)]
fn from (val : SampleTime) -> u8 { SampleTime :: to_bits (val) } } # [repr (u8)]
# [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd)]
pub enum Scandir { # [doc = "Upward scan (from CHSEL0 to CHSEL18)"]
UPWARD = 0x0 , # [doc = "Backward scan (from CHSEL18 to CHSEL0)"]
BACKWARD = 0x01 , } impl Scandir { # [inline (always)]
pub const fn from_bits (val : u8) -> Scandir { unsafe { core :: mem :: transmute (val & 0x01) } } # [inline (always)]
pub const fn to_bits (self) -> u8 { unsafe { core :: mem :: transmute (self) } } } impl From < u8 > for Scandir { # [inline (always)]
fn from (val : u8) -> Scandir { Scandir :: from_bits (val) } } impl From < Scandir > for u8 { # [inline (always)]
fn from (val : Scandir) -> u8 { Scandir :: to_bits (val) } } }