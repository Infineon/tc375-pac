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
#[doc = r"INT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int(pub(super) *mut u8);
unsafe impl core::marker::Send for Int {}
unsafe impl core::marker::Sync for Int {}
impl Int {
    #[doc = "Access Enable covering all INT ECRx and all SRCy 15 0   Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen_config0(
        &self,
    ) -> crate::common::Reg<self::AccenConfig0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(240usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0B9C000}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "OTGM IRQ Trace\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn oit(&self) -> crate::common::Reg<self::Oit_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }

    #[doc = "OTGM IRQ MUX Missed IRQ Select\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn oixms(&self) -> crate::common::Reg<self::Oixms_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(140usize)) }
    }

    #[doc = "OTGM IRQ MUX Select 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn oixs0(&self) -> crate::common::Reg<self::Oixs0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }

    #[doc = "OTGM IRQ MUX Select 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn oixs1(&self) -> crate::common::Reg<self::Oixs1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }

    #[doc = "OTGM IRQ MUX Trigger Set Select\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn oixts(&self) -> crate::common::Reg<self::Oixts_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }

    #[doc = "OTGM MCDS I F Sensitivity Negedge\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn omisn(&self) -> crate::common::Reg<self::Omisn_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(168usize)) }
    }

    #[doc = "OTGM MCDS I F Sensitivity Posedge\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn omisp(&self) -> crate::common::Reg<self::Omisp_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(164usize)) }
    }

    #[doc = "OTGM OTGB0 1 Status\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn oobs(&self) -> crate::common::Reg<self::Oobs_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "OTGM SSI Control\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ossic(&self) -> crate::common::Reg<self::Ossic_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }

    #[doc = "Service Request Broadcast Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn srb(&self) -> [crate::common::Reg<self::Srb_SPEC, crate::common::W>; 3] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x10usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x10usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x10usize + 0x8usize)),
            ]
        }
    }
    #[doc = "ACCEN"]
    #[inline(always)]
    pub fn accen(self) -> [self::Accen; 3] {
        unsafe {
            [
                self::Accen(self.0.add(0x100usize + 0x0usize)),
                self::Accen(self.0.add(0x100usize + 0x8usize)),
                self::Accen(self.0.add(0x100usize + 0x10usize)),
            ]
        }
    }
    #[doc = "ACCEN SRC"]
    #[inline(always)]
    pub fn accen_src(self) -> [self::AccenSrc; 4] {
        unsafe {
            [
                self::AccenSrc(self.0.add(0x180usize + 0x0usize)),
                self::AccenSrc(self.0.add(0x180usize + 0x8usize)),
                self::AccenSrc(self.0.add(0x180usize + 0x10usize)),
                self::AccenSrc(self.0.add(0x180usize + 0x18usize)),
            ]
        }
    }
    #[doc = "CH"]
    #[inline(always)]
    pub fn ch(self) -> [self::Ch; 4] {
        unsafe {
            [
                self::Ch(self.0.add(0x200usize + 0x0usize)),
                self::Ch(self.0.add(0x200usize + 0x10usize)),
                self::Ch(self.0.add(0x200usize + 0x20usize)),
                self::Ch(self.0.add(0x200usize + 0x30usize)),
            ]
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AccenConfig0_SPEC;
impl crate::sealed::RegSpec for AccenConfig0_SPEC {
    type DataType = u32;
}
#[doc = "Access Enable covering all INT ECRx and all SRCy 15 0   Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type AccenConfig0 = crate::RegValueT<AccenConfig0_SPEC>;

impl AccenConfig0 {
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en9(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, AccenConfig0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,AccenConfig0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for AccenConfig0 {
    #[inline(always)]
    fn default() -> AccenConfig0 {
        <crate::RegValueT<AccenConfig0_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0B9C000}"]
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
    #[doc = "Module Number Value. This bit field defines a module identification number. The value for the        Interrupt Router module is 009Bh."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(12173312)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oit_SPEC;
impl crate::sealed::RegSpec for Oit_SPEC {
    type DataType = u32;
}
#[doc = "OTGM IRQ Trace\n resetvalue={Application Reset:0x0}"]
pub type Oit = crate::RegValueT<Oit_SPEC>;

impl Oit {
    #[doc = "Type of Service for Observation on OTGB0. Trigger Set TS16 SP Family concept encoding  compatible with SRC.TOS In products where the CPUx is not implemented  the related TOS          encoding will not be used and treated as RESERVED."]
    #[inline(always)]
    pub fn tos0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Oit_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0x7, 1, 0, u8, Oit_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Output Enable for OTGB0"]
    #[inline(always)]
    pub fn oe0(self) -> crate::common::RegisterFieldBool<7, 1, 0, Oit_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Oit_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Type of Service for Observation on OTGB1. Trigger Set TS16 SP Family concept encoding  compatible with SRC.TOS In products where the CPUx is not implemented  the related TOS          encoding will not be used and treated as RESERVED."]
    #[inline(always)]
    pub fn tos1(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Oit_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8, 0x7, 1, 0, u8, Oit_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Output Enable for OTGB1"]
    #[inline(always)]
    pub fn oe1(self) -> crate::common::RegisterFieldBool<15, 1, 0, Oit_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Oit_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Oit {
    #[inline(always)]
    fn default() -> Oit {
        <crate::RegValueT<Oit_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oixms_SPEC;
impl crate::sealed::RegSpec for Oixms_SPEC {
    type DataType = u32;
}
#[doc = "OTGM IRQ MUX Missed IRQ Select\n resetvalue={Application Reset:0x0}"]
pub type Oixms = crate::RegValueT<Oixms_SPEC>;

impl Oixms {
    #[doc = "SRN Index for Missed Interrupt Trigger"]
    #[inline(always)]
    pub fn mirq(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Oixms_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Oixms_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Oixms {
    #[inline(always)]
    fn default() -> Oixms {
        <crate::RegValueT<Oixms_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oixs0_SPEC;
impl crate::sealed::RegSpec for Oixs0_SPEC {
    type DataType = u32;
}
#[doc = "OTGM IRQ MUX Select 0\n resetvalue={Application Reset:0x0}"]
pub type Oixs0 = crate::RegValueT<Oixs0_SPEC>;

impl Oixs0 {
    #[doc = "SRN Index for Interrupt Trigger 0"]
    #[inline(always)]
    pub fn irq0(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Oixs0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Oixs0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SRN Index for Interrupt Trigger 1"]
    #[inline(always)]
    pub fn irq1(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Oixs0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Oixs0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Oixs0 {
    #[inline(always)]
    fn default() -> Oixs0 {
        <crate::RegValueT<Oixs0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oixs1_SPEC;
impl crate::sealed::RegSpec for Oixs1_SPEC {
    type DataType = u32;
}
#[doc = "OTGM IRQ MUX Select 1\n resetvalue={Application Reset:0x0}"]
pub type Oixs1 = crate::RegValueT<Oixs1_SPEC>;

impl Oixs1 {
    #[doc = "SRN Index for Interrupt Trigger 2"]
    #[inline(always)]
    pub fn irq2(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Oixs1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Oixs1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SRN Index for Interrupt Trigger 3"]
    #[inline(always)]
    pub fn irq3(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Oixs1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Oixs1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Oixs1 {
    #[inline(always)]
    fn default() -> Oixs1 {
        <crate::RegValueT<Oixs1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oixts_SPEC;
impl crate::sealed::RegSpec for Oixts_SPEC {
    type DataType = u32;
}
#[doc = "OTGM IRQ MUX Trigger Set Select\n resetvalue={Application Reset:0x0}"]
pub type Oixts = crate::RegValueT<Oixts_SPEC>;

impl Oixts {
    #[doc = "Trigger Set Select for OTGB0 1 Overlay"]
    #[inline(always)]
    pub fn tgs(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Oixts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Oixts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Overlay Byte Select"]
    #[inline(always)]
    pub fn obs(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Oixts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Oixts_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Oixts {
    #[inline(always)]
    fn default() -> Oixts {
        <crate::RegValueT<Oixts_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Omisn_SPEC;
impl crate::sealed::RegSpec for Omisn_SPEC {
    type DataType = u32;
}
#[doc = "OTGM MCDS I F Sensitivity Negedge\n resetvalue={Application Reset:0x0}"]
pub type Omisn = crate::RegValueT<Omisn_SPEC>;

impl Omisn {
    #[doc = "Bitwise Negedge Sensitivity for OTGB0. If a bit is set an OTGB value will be written to MCDS on a falling edge        of the associated OTGB0 bit."]
    #[inline(always)]
    pub fn otgb0(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Omisn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Omisn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bitwise Negedge Sensitivity for OTGB1. If a bit is set an OTGB value will be written to MCDS on a falling edge        of the associated OTGB1 bit."]
    #[inline(always)]
    pub fn otgb1(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Omisn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Omisn_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Omisn {
    #[inline(always)]
    fn default() -> Omisn {
        <crate::RegValueT<Omisn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Omisp_SPEC;
impl crate::sealed::RegSpec for Omisp_SPEC {
    type DataType = u32;
}
#[doc = "OTGM MCDS I F Sensitivity Posedge\n resetvalue={Application Reset:0x0}"]
pub type Omisp = crate::RegValueT<Omisp_SPEC>;

impl Omisp {
    #[doc = "Bitwise Posedge Sensitivity for OTGB0. If a bit is set an OTGB value will be written to MCDS on a rising edge        of the associated OTGB0 bit."]
    #[inline(always)]
    pub fn otgb0(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Omisp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Omisp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bitwise Posedge Sensitivity for OTGB1. If a bit is set an OTGB value will be written to MCDS on a rising edge        of the associated OTGB1 bit."]
    #[inline(always)]
    pub fn otgb1(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Omisp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Omisp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Omisp {
    #[inline(always)]
    fn default() -> Omisp {
        <crate::RegValueT<Omisp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oobs_SPEC;
impl crate::sealed::RegSpec for Oobs_SPEC {
    type DataType = u32;
}
#[doc = "OTGM OTGB0 1 Status\n resetvalue={Application Reset:0x0}"]
pub type Oobs = crate::RegValueT<Oobs_SPEC>;

impl Oobs {
    #[doc = "Status of OTGB0"]
    #[inline(always)]
    pub fn otgb0(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Oobs_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Oobs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Status of OTGB1"]
    #[inline(always)]
    pub fn otgb1(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Oobs_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Oobs_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Oobs {
    #[inline(always)]
    fn default() -> Oobs {
        <crate::RegValueT<Oobs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ossic_SPEC;
impl crate::sealed::RegSpec for Ossic_SPEC {
    type DataType = u32;
}
#[doc = "OTGM SSI Control\n resetvalue={Application Reset:0x0}"]
pub type Ossic = crate::RegValueT<Ossic_SPEC>;

impl Ossic {
    #[doc = "Trigger Set for OTGB0 1"]
    #[inline(always)]
    pub fn tgs(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Ossic_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Ossic_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OTGB0 1 Bus Select"]
    #[inline(always)]
    pub fn tgb(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ossic_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ossic_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ossic {
    #[inline(always)]
    fn default() -> Ossic {
        <crate::RegValueT<Ossic_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srb_SPEC;
impl crate::sealed::RegSpec for Srb_SPEC {
    type DataType = u32;
}
#[doc = "Service Request Broadcast Register 0\n resetvalue={Application Reset:0x0}"]
pub type Srb = crate::RegValueT<Srb_SPEC>;

impl Srb {
    #[doc = "General Purpose Service Request Trigger 7. This bit is always read as 0."]
    #[inline(always)]
    pub fn trig0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Srb_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Srb_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "General Purpose Service Request Trigger 7. This bit is always read as 0."]
    #[inline(always)]
    pub fn trig1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Srb_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Srb_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "General Purpose Service Request Trigger 7. This bit is always read as 0."]
    #[inline(always)]
    pub fn trig2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Srb_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Srb_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "General Purpose Service Request Trigger 7. This bit is always read as 0."]
    #[inline(always)]
    pub fn trig3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Srb_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Srb_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "General Purpose Service Request Trigger 7. This bit is always read as 0."]
    #[inline(always)]
    pub fn trig4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Srb_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Srb_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "General Purpose Service Request Trigger 7. This bit is always read as 0."]
    #[inline(always)]
    pub fn trig5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Srb_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Srb_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "General Purpose Service Request Trigger 7. This bit is always read as 0."]
    #[inline(always)]
    pub fn trig6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Srb_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Srb_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "General Purpose Service Request Trigger 7. This bit is always read as 0."]
    #[inline(always)]
    pub fn trig7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Srb_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Srb_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Srb {
    #[inline(always)]
    fn default() -> Srb {
        <crate::RegValueT<Srb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc = "ACCEN"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accen(pub(super) *mut u8);
unsafe impl core::marker::Send for Accen {}
unsafe impl core::marker::Sync for Accen {}
impl Accen {
    #[doc = "Access Enable covering SRB0  Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen_srbx0(
        &self,
    ) -> crate::common::Reg<accen::AccenSrBx0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod accen {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AccenSrBx0_SPEC;
    impl crate::sealed::RegSpec for AccenSrBx0_SPEC {
        type DataType = u32;
    }
    #[doc = "Access Enable covering SRB0  Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type AccenSrBx0 = crate::RegValueT<AccenSrBx0_SPEC>;

    impl AccenSrBx0 {
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, AccenSrBx0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, AccenSrBx0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, AccenSrBx0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en3(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, AccenSrBx0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en4(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, AccenSrBx0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en5(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, AccenSrBx0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en6(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, AccenSrBx0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en7(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, AccenSrBx0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en8(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, AccenSrBx0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en9(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, AccenSrBx0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en10(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, AccenSrBx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<10,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en11(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, AccenSrBx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<11,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en12(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, AccenSrBx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<12,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en13(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, AccenSrBx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<13,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en14(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, AccenSrBx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<14,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en15(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, AccenSrBx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<15,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en16(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, AccenSrBx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<16,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en17(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, AccenSrBx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<17,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en18(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, AccenSrBx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<18,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en19(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, AccenSrBx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<19,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en20(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, AccenSrBx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<20,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en21(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, AccenSrBx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<21,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en22(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, AccenSrBx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<22,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en23(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, AccenSrBx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<23,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en24(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, AccenSrBx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<24,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en25(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, AccenSrBx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<25,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en26(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, AccenSrBx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<26,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en27(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, AccenSrBx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<27,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en28(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, AccenSrBx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<28,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en29(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, AccenSrBx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<29,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en30(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, AccenSrBx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<30,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en31(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, AccenSrBx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<31,1,0,AccenSrBx0_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for AccenSrBx0 {
        #[inline(always)]
        fn default() -> AccenSrBx0 {
            <crate::RegValueT<AccenSrBx0_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
}
#[doc = "ACCEN SRC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AccenSrc(pub(super) *mut u8);
unsafe impl core::marker::Send for AccenSrc {}
unsafe impl core::marker::Sync for AccenSrc {}
impl AccenSrc {
    #[doc = "Access Enable covering all SRCx 31 16  mapped to ICU0  Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen_src_tosx0(
        &self,
    ) -> crate::common::Reg<accen_src::AccenSrcToSx0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod accen_src {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AccenSrcToSx0_SPEC;
    impl crate::sealed::RegSpec for AccenSrcToSx0_SPEC {
        type DataType = u32;
    }
    #[doc = "Access Enable covering all SRCx 31 16  mapped to ICU0  Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type AccenSrcToSx0 = crate::RegValueT<AccenSrcToSx0_SPEC>;

    impl AccenSrcToSx0 {
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<0,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<1,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<2,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en3(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<3,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en4(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<4,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en5(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<5,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en6(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<6,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en7(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<7,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en8(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<8,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en9(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<9,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en10(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<10,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en11(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<11,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en12(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<12,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en13(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<13,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en14(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<14,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en15(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<15,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en16(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<16,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en17(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<17,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en18(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<18,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en19(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<19,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en20(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<20,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en21(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<21,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en22(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<22,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en23(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<23,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en24(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<24,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en25(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<25,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en26(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<26,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en27(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<27,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en28(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<28,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en29(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<29,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en30(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<30,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en31(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, AccenSrcToSx0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<31,1,0,AccenSrcToSx0_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for AccenSrcToSx0 {
        #[inline(always)]
        fn default() -> AccenSrcToSx0 {
            <crate::RegValueT<AccenSrcToSx0_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
}
#[doc = "CH"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch(pub(super) *mut u8);
unsafe impl core::marker::Send for Ch {}
unsafe impl core::marker::Sync for Ch {}
impl Ch {
    #[doc = "Error Capture Register 0  related to ICU0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ecrx(&self) -> crate::common::Reg<ch::EcRx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Last Acknowledged Service Request Register 0  related to ICU0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn lasrx(&self) -> crate::common::Reg<ch::LasRx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Latest Winning Service Request Register 0  related to ICU0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn lwsrx(&self) -> crate::common::Reg<ch::LwsRx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod ch {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EcRx_SPEC;
    impl crate::sealed::RegSpec for EcRx_SPEC {
        type DataType = u32;
    }
    #[doc = "Error Capture Register 0  related to ICU0\n resetvalue={Application Reset:0x0}"]
    pub type EcRx = crate::RegValueT<EcRx_SPEC>;

    impl EcRx {
        #[doc = "Service Request Priority Number. This bit field shows the priority number of the last service request        where an error was detected. Bit field can be modified by writing to it."]
        #[inline(always)]
        pub fn pn(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, EcRx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<0,0xff,1,0,u8, EcRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request ECC. This bit field shows the ECC of the last service request where an error        was detected. Bit field can be modified by writing to it. This bit field can be modified by  Writing to SRC 23 16   byte write  Writing to SRC 31 16   16 bit write"]
        #[inline(always)]
        pub fn ecc(
            self,
        ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, EcRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<10,0x1f,1,0,u8, EcRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node ID. This bit field shows the ID of the last service request where an error        was detected. Bit field can be modified by writing to it"]
        #[inline(always)]
        pub fn id(
            self,
        ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, EcRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x3ff,1,0,u16, EcRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Error Overflow Bit. The EOVCLR bit is used to clear the EOV bit. The EOV bit must be cleared        togehter with the STAT bit."]
        #[inline(always)]
        pub fn eovclr(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, EcRx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<28, 1, 0, EcRx_SPEC, crate::common::W>::from_register(
                self, 0,
            )
        }
        #[doc = "Error Status Bit. The STATCLR bit is used to clear the STAT bit."]
        #[inline(always)]
        pub fn statclr(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, EcRx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<29, 1, 0, EcRx_SPEC, crate::common::W>::from_register(
                self, 0,
            )
        }
        #[doc = "Error Overflow Bit. The bit is set if an ECC error was detected by the ICU while        ECR.STAT   180 1  180   Error Overflow situation ."]
        #[inline(always)]
        pub fn eov(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, EcRx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<30, 1, 0, EcRx_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Error Status Bit. The Error Status Bit is set whenever an ECC was detected by the ICU."]
        #[inline(always)]
        pub fn stat(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, EcRx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<31, 1, 0, EcRx_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
    }
    impl core::default::Default for EcRx {
        #[inline(always)]
        fn default() -> EcRx {
            <crate::RegValueT<EcRx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LasRx_SPEC;
    impl crate::sealed::RegSpec for LasRx_SPEC {
        type DataType = u32;
    }
    #[doc = "Last Acknowledged Service Request Register 0  related to ICU0\n resetvalue={Application Reset:0x0}"]
    pub type LasRx = crate::RegValueT<LasRx_SPEC>;

    impl LasRx {
        #[doc = "Last Acknowledged Service Request Priority Number. This bit field shows the Priority Number of the last acknowledged        service request"]
        #[inline(always)]
        pub fn pn(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, LasRx_SPEC, crate::common::R> {
            crate::common::RegisterField::<0,0xff,1,0,u8, LasRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Last Acknowledged Interrupt ECC. This bit field shows the ECC value of the last acknowledged service        request  as send by the service provider with acknowledge In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
        #[inline(always)]
        pub fn ecc(
            self,
        ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, LasRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<10,0x1f,1,0,u8, LasRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Last Acknowledged Interrupt SRN ID. This bit field shows the ID number of the last acknowledged service        request  as sent by the service provider with acknowledge"]
        #[inline(always)]
        pub fn id(
            self,
        ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, LasRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0x3ff,1,0,u16, LasRx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for LasRx {
        #[inline(always)]
        fn default() -> LasRx {
            <crate::RegValueT<LasRx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LwsRx_SPEC;
    impl crate::sealed::RegSpec for LwsRx_SPEC {
        type DataType = u32;
    }
    #[doc = "Latest Winning Service Request Register 0  related to ICU0\n resetvalue={Application Reset:0x0}"]
    pub type LwsRx = crate::RegValueT<LwsRx_SPEC>;

    impl LwsRx {
        #[doc = "Latest Winner Priority Number. This bit field shows the Priority Number of a pending service request        that won the last arbitration round. This bit field is only valid if        STAT is set to 1"]
        #[inline(always)]
        pub fn pn(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, LwsRx_SPEC, crate::common::R> {
            crate::common::RegisterField::<0,0xff,1,0,u8, LwsRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Latest Winner ECC. This bit field shows the ECC field  SRN.ECC  that was transferred from        the last winning SRN to the ICU. This bit field is only valid if STAT is        set to 1. In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
        #[inline(always)]
        pub fn ecc(
            self,
        ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, LwsRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<10,0x1f,1,0,u8, LwsRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Latest Winner Index Number. This bit field shows the ID number of the last winning SRN. This bit        field is only valid if STAT is set to 1"]
        #[inline(always)]
        pub fn id(
            self,
        ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, LwsRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0x3ff,1,0,u16, LwsRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "LWSR Register Status. The STAT register indicates if the PN  ECC and ID bit fields are still        valid. They are still valid if the interrupt from the SRN identified by        ID is still pending. If the ICU does not have an winner because no        interrupt is pending or not yet arbitrated then it clears the STAT bit."]
        #[inline(always)]
        pub fn stat(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, LwsRx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<31,1,0,LwsRx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for LwsRx {
        #[inline(always)]
        fn default() -> LwsRx {
            <crate::RegValueT<LwsRx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
