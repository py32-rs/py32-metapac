#![allow(clippy::missing_safety_doc)]
                #![allow(clippy::identity_op)]
                #![allow(clippy::unnecessary_cast)]
                #![allow(clippy::erasing_op)]

# [doc = "Flash."]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Flash { ptr : * mut u8 } unsafe impl Send for Flash { } unsafe impl Sync for Flash { } impl Flash { # [inline (always)]
pub const unsafe fn from_ptr (ptr : * mut ()) -> Self { Self { ptr : ptr as _ , } } # [inline (always)]
pub const fn as_ptr (& self) -> * mut () { self . ptr as _ } # [doc = "Access control register."]
# [inline (always)]
pub const fn acr (self) -> crate :: common :: Reg < regs :: Acr , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x0usize) as _) } } # [doc = "Flash key register."]
# [inline (always)]
pub const fn keyr (self) -> crate :: common :: Reg < u32 , crate :: common :: W > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x08usize) as _) } } # [doc = "Option byte key register."]
# [inline (always)]
pub const fn optkeyr (self) -> crate :: common :: Reg < u32 , crate :: common :: W > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x0cusize) as _) } } # [doc = "Status register."]
# [inline (always)]
pub const fn sr (self) -> crate :: common :: Reg < regs :: Sr , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x10usize) as _) } } # [doc = "Flash control register."]
# [inline (always)]
pub const fn cr (self) -> crate :: common :: Reg < regs :: Cr , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x14usize) as _) } } # [doc = "Flash option register."]
# [inline (always)]
pub const fn optr (self) -> crate :: common :: Reg < regs :: Optr , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x20usize) as _) } } # [doc = "Flash SDK address. register."]
# [inline (always)]
pub const fn sdkr (self) -> crate :: common :: Reg < regs :: Sdkr , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x24usize) as _) } } # [doc = "Flash WRP address. register."]
# [inline (always)]
pub const fn wrpr (self) -> crate :: common :: Reg < regs :: Wrpr , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x2cusize) as _) } } # [doc = "Flash sleep time config. register."]
# [inline (always)]
pub const fn stcr (self) -> crate :: common :: Reg < regs :: Stcr , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x90usize) as _) } } # [doc = "Flash TS0. register."]
# [inline (always)]
pub const fn ts0 (self) -> crate :: common :: Reg < regs :: Ts0 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x0100usize) as _) } } # [doc = "Flash TS1. register."]
# [inline (always)]
pub const fn ts1 (self) -> crate :: common :: Reg < regs :: Ts1 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x0104usize) as _) } } # [doc = "Flash TS2P. register."]
# [inline (always)]
pub const fn ts2p (self) -> crate :: common :: Reg < regs :: Ts2p , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x0108usize) as _) } } # [doc = "Flash TPS3. register."]
# [inline (always)]
pub const fn tps3 (self) -> crate :: common :: Reg < regs :: Tps3 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x010cusize) as _) } } # [doc = "Flash TS3. register."]
# [inline (always)]
pub const fn ts3 (self) -> crate :: common :: Reg < regs :: Ts3 , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x0110usize) as _) } } # [doc = "Flash PERTPE. register."]
# [inline (always)]
pub const fn pertpe (self) -> crate :: common :: Reg < regs :: Pertpe , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x0114usize) as _) } } # [doc = "Flash SMERTPE. register."]
# [inline (always)]
pub const fn smertpe (self) -> crate :: common :: Reg < regs :: Smertpe , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x0118usize) as _) } } # [doc = "Flash PRGTPE. register."]
# [inline (always)]
pub const fn prgtpe (self) -> crate :: common :: Reg < regs :: Prgtpe , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x011cusize) as _) } } # [doc = "Flash PRETPE. register."]
# [inline (always)]
pub const fn pretpe (self) -> crate :: common :: Reg < regs :: Pretpe , crate :: common :: RW > { unsafe { crate :: common :: Reg :: from_ptr (self . ptr . add (0x0120usize) as _) } } } pub mod regs { # [doc = "Access control register."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Acr (pub u32) ; impl Acr { # [doc = "Latency."]
# [inline (always)]
pub const fn latency (& self) -> bool { let val = (self . 0 >> 0usize) & 0x01 ; val != 0 } # [doc = "Latency."]
# [inline (always)]
pub fn set_latency (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize) ; } } impl Default for Acr { # [inline (always)]
fn default () -> Acr { Acr (0) } } # [doc = "Flash control register."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Cr (pub u32) ; impl Cr { # [doc = "Programming."]
# [inline (always)]
pub const fn pg (& self) -> bool { let val = (self . 0 >> 0usize) & 0x01 ; val != 0 } # [doc = "Programming."]
# [inline (always)]
pub fn set_pg (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize) ; } # [doc = "Page erase."]
# [inline (always)]
pub const fn per (& self) -> bool { let val = (self . 0 >> 1usize) & 0x01 ; val != 0 } # [doc = "Page erase."]
# [inline (always)]
pub fn set_per (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize) ; } # [doc = "Mass erase."]
# [inline (always)]
pub const fn mer (& self) -> bool { let val = (self . 0 >> 2usize) & 0x01 ; val != 0 } # [doc = "Mass erase."]
# [inline (always)]
pub fn set_mer (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize) ; } # [doc = "Sector erase."]
# [inline (always)]
pub const fn ser (& self) -> bool { let val = (self . 0 >> 11usize) & 0x01 ; val != 0 } # [doc = "Sector erase."]
# [inline (always)]
pub fn set_ser (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize) ; } # [doc = "Option byte program start."]
# [inline (always)]
pub const fn optstrt (& self) -> bool { let val = (self . 0 >> 17usize) & 0x01 ; val != 0 } # [doc = "Option byte program start."]
# [inline (always)]
pub fn set_optstrt (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize) ; } # [doc = "Flash main memory program start."]
# [inline (always)]
pub const fn pgstrt (& self) -> bool { let val = (self . 0 >> 19usize) & 0x01 ; val != 0 } # [doc = "Flash main memory program start."]
# [inline (always)]
pub fn set_pgstrt (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize) ; } # [doc = "End of operation interrupt. enable."]
# [inline (always)]
pub const fn eopie (& self) -> bool { let val = (self . 0 >> 24usize) & 0x01 ; val != 0 } # [doc = "End of operation interrupt. enable."]
# [inline (always)]
pub fn set_eopie (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize) ; } # [doc = "Error interrupt enable."]
# [inline (always)]
pub const fn errie (& self) -> bool { let val = (self . 0 >> 25usize) & 0x01 ; val != 0 } # [doc = "Error interrupt enable."]
# [inline (always)]
pub fn set_errie (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize) ; } # [doc = "Force the option byte. loading."]
# [inline (always)]
pub const fn obl_launch (& self) -> bool { let val = (self . 0 >> 27usize) & 0x01 ; val != 0 } # [doc = "Force the option byte. loading."]
# [inline (always)]
pub fn set_obl_launch (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize) ; } # [doc = "Options Lock."]
# [inline (always)]
pub const fn optlock (& self) -> bool { let val = (self . 0 >> 30usize) & 0x01 ; val != 0 } # [doc = "Options Lock."]
# [inline (always)]
pub fn set_optlock (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize) ; } # [doc = "FLASH_CR Lock."]
# [inline (always)]
pub const fn lock (& self) -> bool { let val = (self . 0 >> 31usize) & 0x01 ; val != 0 } # [doc = "FLASH_CR Lock."]
# [inline (always)]
pub fn set_lock (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize) ; } } impl Default for Cr { # [inline (always)]
fn default () -> Cr { Cr (0) } } # [doc = "Flash option register."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Optr (pub u32) ; impl Optr { # [doc = "Read Protection."]
# [inline (always)]
pub const fn rdp (& self) -> u8 { let val = (self . 0 >> 0usize) & 0xff ; val as u8 } # [doc = "Read Protection."]
# [inline (always)]
pub fn set_rdp (& mut self , val : u8) { self . 0 = (self . 0 & ! (0xff << 0usize)) | (((val as u32) & 0xff) << 0usize) ; } # [doc = "BOR reset Level."]
# [inline (always)]
pub const fn boren (& self) -> bool { let val = (self . 0 >> 8usize) & 0x01 ; val != 0 } # [doc = "BOR reset Level."]
# [inline (always)]
pub fn set_boren (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize) ; } # [doc = "These bits contain the VDD supply level. threshold that activates the reset."]
# [inline (always)]
pub const fn borf_lev (& self) -> u8 { let val = (self . 0 >> 9usize) & 0x07 ; val as u8 } # [doc = "These bits contain the VDD supply level. threshold that activates the reset."]
# [inline (always)]
pub fn set_borf_lev (& mut self , val : u8) { self . 0 = (self . 0 & ! (0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize) ; } # [doc = "Independent watchdog. selection."]
# [inline (always)]
pub const fn idwg_sw (& self) -> bool { let val = (self . 0 >> 12usize) & 0x01 ; val != 0 } # [doc = "Independent watchdog. selection."]
# [inline (always)]
pub fn set_idwg_sw (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize) ; } # [doc = "Window watchdog selection."]
# [inline (always)]
pub const fn wwdg_sw (& self) -> bool { let val = (self . 0 >> 13usize) & 0x01 ; val != 0 } # [doc = "Window watchdog selection."]
# [inline (always)]
pub fn set_wwdg_sw (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize) ; } # [doc = "NRST_MODE."]
# [inline (always)]
pub const fn nrst_mode (& self) -> bool { let val = (self . 0 >> 14usize) & 0x01 ; val != 0 } # [doc = "NRST_MODE."]
# [inline (always)]
pub fn set_nrst_mode (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize) ; } # [doc = "Boot configuration."]
# [inline (always)]
pub const fn n_boot1 (& self) -> bool { let val = (self . 0 >> 15usize) & 0x01 ; val != 0 } # [doc = "Boot configuration."]
# [inline (always)]
pub fn set_n_boot1 (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize) ; } } impl Default for Optr { # [inline (always)]
fn default () -> Optr { Optr (0) } } # [doc = "Flash PERTPE. register."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Pertpe (pub u32) ; impl Pertpe { # [doc = "FLash PERTPE register."]
# [inline (always)]
pub const fn pertpe (& self) -> u32 { let val = (self . 0 >> 0usize) & 0x0001_ffff ; val as u32 } # [doc = "FLash PERTPE register."]
# [inline (always)]
pub fn set_pertpe (& mut self , val : u32) { self . 0 = (self . 0 & ! (0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize) ; } } impl Default for Pertpe { # [inline (always)]
fn default () -> Pertpe { Pertpe (0) } } # [doc = "Flash PRETPE. register."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Pretpe (pub u32) ; impl Pretpe { # [doc = "FLash PRETPE register."]
# [inline (always)]
pub const fn pretpe (& self) -> u16 { let val = (self . 0 >> 0usize) & 0x1fff ; val as u16 } # [doc = "FLash PRETPE register."]
# [inline (always)]
pub fn set_pretpe (& mut self , val : u16) { self . 0 = (self . 0 & ! (0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize) ; } } impl Default for Pretpe { # [inline (always)]
fn default () -> Pretpe { Pretpe (0) } } # [doc = "Flash PRGTPE. register."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Prgtpe (pub u32) ; impl Prgtpe { # [doc = "FLash PRGTPE register."]
# [inline (always)]
pub const fn prgtpe (& self) -> u16 { let val = (self . 0 >> 0usize) & 0xffff ; val as u16 } # [doc = "FLash PRGTPE register."]
# [inline (always)]
pub fn set_prgtpe (& mut self , val : u16) { self . 0 = (self . 0 & ! (0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize) ; } } impl Default for Prgtpe { # [inline (always)]
fn default () -> Prgtpe { Prgtpe (0) } } # [doc = "Flash SDK address. register."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Sdkr (pub u32) ; impl Sdkr { # [doc = "SDK area start address."]
# [inline (always)]
pub const fn sdk_strt (& self) -> u8 { let val = (self . 0 >> 0usize) & 0x1f ; val as u8 } # [doc = "SDK area start address."]
# [inline (always)]
pub fn set_sdk_strt (& mut self , val : u8) { self . 0 = (self . 0 & ! (0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize) ; } # [doc = "SDK area end address."]
# [inline (always)]
pub const fn sdk_end (& self) -> u8 { let val = (self . 0 >> 8usize) & 0x1f ; val as u8 } # [doc = "SDK area end address."]
# [inline (always)]
pub fn set_sdk_end (& mut self , val : u8) { self . 0 = (self . 0 & ! (0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize) ; } } impl Default for Sdkr { # [inline (always)]
fn default () -> Sdkr { Sdkr (0) } } # [doc = "Flash SMERTPE. register."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Smertpe (pub u32) ; impl Smertpe { # [doc = "FLash SMERTPE register."]
# [inline (always)]
pub const fn smertpe (& self) -> u32 { let val = (self . 0 >> 0usize) & 0x0001_ffff ; val as u32 } # [doc = "FLash SMERTPE register."]
# [inline (always)]
pub fn set_smertpe (& mut self , val : u32) { self . 0 = (self . 0 & ! (0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize) ; } } impl Default for Smertpe { # [inline (always)]
fn default () -> Smertpe { Smertpe (0) } } # [doc = "Status register."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Sr (pub u32) ; impl Sr { # [doc = "End of operation."]
# [inline (always)]
pub const fn eop (& self) -> bool { let val = (self . 0 >> 0usize) & 0x01 ; val != 0 } # [doc = "End of operation."]
# [inline (always)]
pub fn set_eop (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize) ; } # [doc = "Write protected error."]
# [inline (always)]
pub const fn wrperr (& self) -> bool { let val = (self . 0 >> 4usize) & 0x01 ; val != 0 } # [doc = "Write protected error."]
# [inline (always)]
pub fn set_wrperr (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize) ; } # [doc = "Option and Engineering bits loading. validity error."]
# [inline (always)]
pub const fn optverr (& self) -> bool { let val = (self . 0 >> 15usize) & 0x01 ; val != 0 } # [doc = "Option and Engineering bits loading. validity error."]
# [inline (always)]
pub fn set_optverr (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize) ; } # [doc = "Busy."]
# [inline (always)]
pub const fn bsy (& self) -> bool { let val = (self . 0 >> 16usize) & 0x01 ; val != 0 } # [doc = "Busy."]
# [inline (always)]
pub fn set_bsy (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize) ; } } impl Default for Sr { # [inline (always)]
fn default () -> Sr { Sr (0) } } # [doc = "Flash sleep time config. register."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Stcr (pub u32) ; impl Stcr { # [doc = "FLash sleep enable."]
# [inline (always)]
pub const fn sleep_en (& self) -> bool { let val = (self . 0 >> 0usize) & 0x01 ; val != 0 } # [doc = "FLash sleep enable."]
# [inline (always)]
pub fn set_sleep_en (& mut self , val : bool) { self . 0 = (self . 0 & ! (0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize) ; } # [doc = "FLash sleep time configuration(counter based on HSI_10M)."]
# [inline (always)]
pub const fn sleep_time (& self) -> u8 { let val = (self . 0 >> 8usize) & 0xff ; val as u8 } # [doc = "FLash sleep time configuration(counter based on HSI_10M)."]
# [inline (always)]
pub fn set_sleep_time (& mut self , val : u8) { self . 0 = (self . 0 & ! (0xff << 8usize)) | (((val as u32) & 0xff) << 8usize) ; } } impl Default for Stcr { # [inline (always)]
fn default () -> Stcr { Stcr (0) } } # [doc = "Flash TPS3. register."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Tps3 (pub u32) ; impl Tps3 { # [doc = "FLash TPS3 register."]
# [inline (always)]
pub const fn tps3 (& self) -> u16 { let val = (self . 0 >> 0usize) & 0x07ff ; val as u16 } # [doc = "FLash TPS3 register."]
# [inline (always)]
pub fn set_tps3 (& mut self , val : u16) { self . 0 = (self . 0 & ! (0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize) ; } } impl Default for Tps3 { # [inline (always)]
fn default () -> Tps3 { Tps3 (0) } } # [doc = "Flash TS0. register."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Ts0 (pub u32) ; impl Ts0 { # [doc = "FLash TS0 register."]
# [inline (always)]
pub const fn ts0 (& self) -> u8 { let val = (self . 0 >> 0usize) & 0xff ; val as u8 } # [doc = "FLash TS0 register."]
# [inline (always)]
pub fn set_ts0 (& mut self , val : u8) { self . 0 = (self . 0 & ! (0xff << 0usize)) | (((val as u32) & 0xff) << 0usize) ; } } impl Default for Ts0 { # [inline (always)]
fn default () -> Ts0 { Ts0 (0) } } # [doc = "Flash TS1. register."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Ts1 (pub u32) ; impl Ts1 { # [doc = "FLash TS1 register."]
# [inline (always)]
pub const fn ts1 (& self) -> u16 { let val = (self . 0 >> 0usize) & 0x01ff ; val as u16 } # [doc = "FLash TS1 register."]
# [inline (always)]
pub fn set_ts1 (& mut self , val : u16) { self . 0 = (self . 0 & ! (0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize) ; } } impl Default for Ts1 { # [inline (always)]
fn default () -> Ts1 { Ts1 (0) } } # [doc = "Flash TS2P. register."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Ts2p (pub u32) ; impl Ts2p { # [doc = "FLash TS2P register."]
# [inline (always)]
pub const fn ts2p (& self) -> u8 { let val = (self . 0 >> 0usize) & 0xff ; val as u8 } # [doc = "FLash TS2P register."]
# [inline (always)]
pub fn set_ts2p (& mut self , val : u8) { self . 0 = (self . 0 & ! (0xff << 0usize)) | (((val as u32) & 0xff) << 0usize) ; } } impl Default for Ts2p { # [inline (always)]
fn default () -> Ts2p { Ts2p (0) } } # [doc = "Flash TS3. register."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Ts3 (pub u32) ; impl Ts3 { # [doc = "FLash TS3 register."]
# [inline (always)]
pub const fn ts3 (& self) -> u8 { let val = (self . 0 >> 0usize) & 0xff ; val as u8 } # [doc = "FLash TS3 register."]
# [inline (always)]
pub fn set_ts3 (& mut self , val : u8) { self . 0 = (self . 0 & ! (0xff << 0usize)) | (((val as u32) & 0xff) << 0usize) ; } } impl Default for Ts3 { # [inline (always)]
fn default () -> Ts3 { Ts3 (0) } } # [doc = "Flash WRP address. register."]
# [repr (transparent)]
# [derive (Copy , Clone , Eq , PartialEq)]
pub struct Wrpr (pub u32) ; impl Wrpr { # [doc = "WRP address."]
# [inline (always)]
pub const fn wrp (& self) -> u16 { let val = (self . 0 >> 0usize) & 0xffff ; val as u16 } # [doc = "WRP address."]
# [inline (always)]
pub fn set_wrp (& mut self , val : u16) { self . 0 = (self . 0 & ! (0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize) ; } } impl Default for Wrpr { # [inline (always)]
fn default () -> Wrpr { Wrpr (0) } } }