/*
*****************************************************************************
	*
	* Copyright (C) 2024 Infineon Technologies AG. All rights reserved.
	*
	* Infineon Technologies AG (Infineon) is supplying this software for use with
	* Infineon's microcontrollers. This file can be freely distributed within
	* development tools that are supporting such microcontrollers.
	*
	* THIS SOFTWARE IS PROVIDED "AS IS". NO WARRANTIES, WHETHER EXPRESS, IMPLIED
	* OR STATUTORY, INCLUDING, BUT NOT LIMITED TO, IMPLIED WARRANTIES OF
	* MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE APPLY TO THIS SOFTWARE.
	* INFINEON SHALL NOT, IN ANY CIRCUMSTANCES, BE LIABLE FOR SPECIAL, INCIDENTAL,
	* OR CONSEQUENTIAL DAMAGES, FOR ANY REASON WHATSOEVER.
	*
	******************************************************************************
*/
#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"Hardware Security Module"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsm(pub(super) *mut u8);
unsafe impl core::marker::Send for Hsm {}
unsafe impl core::marker::Sync for Hsm {}
impl Hsm {
    #[doc = "Debug Window Base Address\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dbgbase(&self) -> crate::common::Reg<self::Dbgbase_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4112usize)) }
    }

    #[doc = "HSM to Host Flags\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hsm2htf(&self) -> crate::common::Reg<self::Hsm2Htf_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }

    #[doc = "HSM to Host Interrupt Enable\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hsm2htie(&self) -> crate::common::Reg<self::Hsm2Htie_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }

    #[doc = "HSM to Host Interrupt Select\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hsm2htis(&self) -> crate::common::Reg<self::Hsm2Htis_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }

    #[doc = "HSM to Host Status\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hsm2hts(&self) -> crate::common::Reg<self::Hsm2Hts_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "HSM Control\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hsmctrl(&self) -> crate::common::Reg<self::Hsmctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4096usize)) }
    }

    #[doc = "OCDS Suspend and Trigger Bus Control\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hsmotgb(&self) -> crate::common::Reg<self::Hsmotgb_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4128usize)) }
    }

    #[doc = "Host to HSM Flags\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ht2hsmf(&self) -> crate::common::Reg<self::Ht2Hsmf_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Host to HSM Status\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ht2hsms(&self) -> crate::common::Reg<self::Ht2Hsms_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }

    #[doc = "HSM Identifier Register\n resetvalue={Application Reset:0x0BEC002}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgbase_SPEC;
impl crate::sealed::RegSpec for Dbgbase_SPEC {
    type DataType = u32;
}
#[doc = "Debug Window Base Address\n resetvalue={Application Reset:0x0}"]
pub type Dbgbase = crate::RegValueT<Dbgbase_SPEC>;

impl Dbgbase {
    #[doc = "Base Address. Base address for debug memory window. The effective address of a debug        access is HSM DBGBASE 133DBGBASE . HSM DBGBASE.BASE 135BASE  offset of access address to base of the memory window."]
    #[inline(always)]
    pub fn base(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Dbgbase_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Dbgbase_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dbgbase {
    #[inline(always)]
    fn default() -> Dbgbase {
        <crate::RegValueT<Dbgbase_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsm2Htf_SPEC;
impl crate::sealed::RegSpec for Hsm2Htf_SPEC {
    type DataType = u32;
}
#[doc = "HSM to Host Flags\n resetvalue={Application Reset:0x0}"]
pub type Hsm2Htf = crate::RegValueT<Hsm2Htf_SPEC>;

impl Hsm2Htf {
    #[doc = "HSM to Host Flags. Each flag can be individually set by the HSM to generate an host        interrupt request if the corresponding interrupt enable flag is set. The        host can reset the flags individually by writing   8216 1  8217 . If a simultaneous        set and reset occurs  the value of the flag is   8216 1  8217 ."]
    #[inline(always)]
    pub fn hsm2htf(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Hsm2Htf_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Hsm2Htf_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Hsm2Htf {
    #[inline(always)]
    fn default() -> Hsm2Htf {
        <crate::RegValueT<Hsm2Htf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsm2Htie_SPEC;
impl crate::sealed::RegSpec for Hsm2Htie_SPEC {
    type DataType = u32;
}
#[doc = "HSM to Host Interrupt Enable\n resetvalue={Application Reset:0x0}"]
pub type Hsm2Htie = crate::RegValueT<Hsm2Htie_SPEC>;

impl Hsm2Htie {
    #[doc = "HSM to Host Interrupt Enable. Each set bit enables the interrupt for the corresponding flags in        register HSM HSM2HTF 104HSM2HTF .        If cleared bit disables the interrupt."]
    #[inline(always)]
    pub fn hsm2htie(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Hsm2Htie_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Hsm2Htie_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Hsm2Htie {
    #[inline(always)]
    fn default() -> Hsm2Htie {
        <crate::RegValueT<Hsm2Htie_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsm2Htis_SPEC;
impl crate::sealed::RegSpec for Hsm2Htis_SPEC {
    type DataType = u32;
}
#[doc = "HSM to Host Interrupt Select\n resetvalue={Application Reset:0x0}"]
pub type Hsm2Htis = crate::RegValueT<Hsm2Htis_SPEC>;

impl Hsm2Htis {
    #[doc = "HSM to Host Interrupt Select. If a bit is set the corresponding interrupt controlled by registers HSM HSM2HTF 104HSM2HTF and HSM2HTIE is mapped to host interrupt 1  otherwise to host interrupt 0."]
    #[inline(always)]
    pub fn hsm2htis(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Hsm2Htis_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Hsm2Htis_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Hsm2Htis {
    #[inline(always)]
    fn default() -> Hsm2Htis {
        <crate::RegValueT<Hsm2Htis_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsm2Hts_SPEC;
impl crate::sealed::RegSpec for Hsm2Hts_SPEC {
    type DataType = u32;
}
#[doc = "HSM to Host Status\n resetvalue={Application Reset:0x0}"]
pub type Hsm2Hts = crate::RegValueT<Hsm2Hts_SPEC>;

impl Hsm2Hts {
    #[doc = "HSM to Host Status. 32 bits of information to signal HSM internal status to the host."]
    #[inline(always)]
    pub fn hsm2hts(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Hsm2Hts_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Hsm2Hts_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Hsm2Hts {
    #[inline(always)]
    fn default() -> Hsm2Hts {
        <crate::RegValueT<Hsm2Hts_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsmctrl_SPEC;
impl crate::sealed::RegSpec for Hsmctrl_SPEC {
    type DataType = u32;
}
#[doc = "HSM Control\n resetvalue={Application Reset:0x0}"]
pub type Hsmctrl = crate::RegValueT<Hsmctrl_SPEC>;

impl Hsmctrl {
    #[doc = "End of Memory BIST test.. For security reasons this functionality is encoded redundantly."]
    #[inline(always)]
    pub fn eombt(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, hsmctrl::Eombt, Hsmctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,hsmctrl::Eombt, Hsmctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Hsmctrl {
    #[inline(always)]
    fn default() -> Hsmctrl {
        <crate::RegValueT<Hsmctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod hsmctrl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Eombt_SPEC;
    pub type Eombt = crate::EnumBitfieldStruct<u8, Eombt_SPEC>;
    impl Eombt {
        #[doc = "MBIST mode activated."]
        pub const MON_0: Self = Self::new(0);
        #[doc = "MBIST mode deactivated. See below."]
        pub const MOFF_11: Self = Self::new(1);
        #[doc = "MBIST mode deactivated. See below."]
        pub const MOFF_22: Self = Self::new(2);
        #[doc = "MBIST mode deactivated. Start of HSM subsystem. Cannot be reset by software"]
        pub const MOFF_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsmotgb_SPEC;
impl crate::sealed::RegSpec for Hsmotgb_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Suspend and Trigger Bus Control\n resetvalue={Application Reset:0x0}"]
pub type Hsmotgb = crate::RegValueT<Hsmotgb_SPEC>;

impl Hsmotgb {
    #[doc = "Trigger Set for OTGB0 1. This module supports only one Trigger Set. The following values are        defined"]
    #[inline(always)]
    pub fn tgs(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, hsmotgb::Tgs, Hsmotgb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,hsmotgb::Tgs, Hsmotgb_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OTGB0 1 Bus Select"]
    #[inline(always)]
    pub fn tgb(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, hsmotgb::Tgb, Hsmotgb_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,hsmotgb::Tgb, Hsmotgb_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TGS  TGB Write Protection. TGS and TGB are only overwritten when TG P is set at the same access         otherwise unchanged. Reads as 0."]
    #[inline(always)]
    pub fn tg_p(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, hsmotgb::TgP, Hsmotgb_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<3,0x1,1,0,hsmotgb::TgP, Hsmotgb_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Hsmotgb {
    #[inline(always)]
    fn default() -> Hsmotgb {
        <crate::RegValueT<Hsmotgb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod hsmotgb {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tgs_SPEC;
    pub type Tgs = crate::EnumBitfieldStruct<u8, Tgs_SPEC>;
    impl Tgs {
        #[doc = "No Trigger Set output."]
        pub const NOTS_0: Self = Self::new(0);
        #[doc = "Trigger Set 1."]
        pub const TS_11: Self = Self::new(1);
        #[doc = "No Trigger Set output."]
        pub const TS_22: Self = Self::new(2);
        #[doc = "Trigger Set 1."]
        pub const TS_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tgb_SPEC;
    pub type Tgb = crate::EnumBitfieldStruct<u8, Tgb_SPEC>;
    impl Tgb {
        #[doc = "Trigger Set is output on OTGB0."]
        pub const BUS_00: Self = Self::new(0);
        #[doc = "Trigger Set is output on OTGB1."]
        pub const BUS_11: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct TgP_SPEC;
    pub type TgP = crate::EnumBitfieldStruct<u8, TgP_SPEC>;
    impl TgP {
        #[doc = "No change."]
        pub const NC_0: Self = Self::new(0);
        #[doc = "Write enable for the current access."]
        pub const WE_1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ht2Hsmf_SPEC;
impl crate::sealed::RegSpec for Ht2Hsmf_SPEC {
    type DataType = u32;
}
#[doc = "Host to HSM Flags\n resetvalue={Application Reset:0x0}"]
pub type Ht2Hsmf = crate::RegValueT<Ht2Hsmf_SPEC>;

impl Ht2Hsmf {
    #[doc = "Host to HSM Flags. Each flag can be individually set by the host to generate an HSM        interrupt request if the corresponding interrupt enable flag is set. The        HSM can clear the flags individually. If a simultaneous the host        performs a set and HSM a clear  the value of the flag is   8216 1  8217 ."]
    #[inline(always)]
    pub fn ht2hsmf(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Ht2Hsmf_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Ht2Hsmf_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ht2Hsmf {
    #[inline(always)]
    fn default() -> Ht2Hsmf {
        <crate::RegValueT<Ht2Hsmf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ht2Hsms_SPEC;
impl crate::sealed::RegSpec for Ht2Hsms_SPEC {
    type DataType = u32;
}
#[doc = "Host to HSM Status\n resetvalue={Application Reset:0x0}"]
pub type Ht2Hsms = crate::RegValueT<Ht2Hsms_SPEC>;

impl Ht2Hsms {
    #[doc = "Host to HSM Status. 32 bits of status information to signal the host software status to the        HSM."]
    #[inline(always)]
    pub fn ht2hsms(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Ht2Hsms_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Ht2Hsms_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ht2Hsms {
    #[inline(always)]
    fn default() -> Ht2Hsms {
        <crate::RegValueT<Ht2Hsms_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "HSM Identifier Register\n resetvalue={Application Reset:0x0BEC002}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number. This bit field defines the module revision number. The value of a module        revision starts with 01  first revision ."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type. The bit field is set to C0 which defines        the module as a 32 bit module."]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value. This bit field defines a module identification number. The value for the        HSM module is 00BE ."]
    #[inline(always)]
    pub fn mod_number(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Id_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Id {
    #[inline(always)]
    fn default() -> Id {
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(12500994)
    }
}
