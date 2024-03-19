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
#[doc = r"HSSL"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hssl0(pub(super) *mut u8);
unsafe impl core::marker::Send for Hssl0 {}
unsafe impl core::marker::Sync for Hssl0 {}
impl Hssl0 {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "Access Rules Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ar(&self) -> crate::common::Reg<self::Ar_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(224usize)) }
    }

    #[doc = "Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cfg(&self) -> crate::common::Reg<self::Cfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "CRC Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn crc(&self) -> crate::common::Reg<self::Crc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0CBC000}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(244usize)) }
    }

    #[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(240usize)) }
    }

    #[doc = "Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(236usize)) }
    }

    #[doc = "Miscellaneous Flags Register\n resetvalue={Application Reset:0x080000000}"]
    #[inline(always)]
    pub const fn mflags(&self) -> crate::common::Reg<self::Mflags_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "Miscellaneous Flags Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mflagscl(&self) -> crate::common::Reg<self::Mflagscl_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Flags Enable Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mflagsen(&self) -> crate::common::Reg<self::Mflagsen_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "Miscellaneous Flags Set Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mflagsset(&self) -> crate::common::Reg<self::Mflagsset_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Multi Slave Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mscr(&self) -> crate::common::Reg<self::Mscr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(156usize)) }
    }

    #[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(232usize)) }
    }

    #[doc = "Request Flags Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn qflags(&self) -> crate::common::Reg<self::Qflags_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Security Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sec(&self) -> crate::common::Reg<self::Sec_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(152usize)) }
    }

    #[doc = "Stream FIFOs Status Flags Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sfsflags(&self) -> crate::common::Reg<self::Sfsflags_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }

    #[doc = "Target ID Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tidadd(&self) -> crate::common::Reg<self::Tidadd_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }

    #[doc = "Target Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tstat(&self) -> crate::common::Reg<self::Tstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }
    #[doc = "AW"]
    #[inline(always)]
    pub fn aw(self) -> [self::Aw; 4] {
        unsafe {
            [
                self::Aw(self.0.add(0xc0usize + 0x0usize)),
                self::Aw(self.0.add(0xc0usize + 0x8usize)),
                self::Aw(self.0.add(0xc0usize + 0x10usize)),
                self::Aw(self.0.add(0xc0usize + 0x18usize)),
            ]
        }
    }
    #[doc = "IS"]
    #[inline(always)]
    pub fn is(self) -> self::Is {
        unsafe { self::Is(self.0.add(160usize)) }
    }
    #[doc = "I"]
    #[inline(always)]
    pub fn i(self) -> [self::I; 4] {
        unsafe {
            [
                self::I(self.0.add(0x30usize + 0x0usize)),
                self::I(self.0.add(0x30usize + 0x10usize)),
                self::I(self.0.add(0x30usize + 0x20usize)),
                self::I(self.0.add(0x30usize + 0x30usize)),
            ]
        }
    }
    #[doc = "TS"]
    #[inline(always)]
    pub fn ts(self) -> self::Ts {
        unsafe { self::Ts(self.0.add(176usize)) }
    }
    #[doc = "T"]
    #[inline(always)]
    pub fn t(self) -> [self::T; 4] {
        unsafe {
            [
                self::T(self.0.add(0x70usize + 0x0usize)),
                self::T(self.0.add(0x70usize + 0x8usize)),
                self::T(self.0.add(0x70usize + 0x10usize)),
                self::T(self.0.add(0x70usize + 0x18usize)),
            ]
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accen0_SPEC;
impl crate::sealed::RegSpec for Accen0_SPEC {
    type DataType = u32;
}
#[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accen0 = crate::RegValueT<Accen0_SPEC>;

impl Accen0 {
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Accen0 {
    #[inline(always)]
    fn default() -> Accen0 {
        <crate::RegValueT<Accen0_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ar_SPEC;
impl crate::sealed::RegSpec for Ar_SPEC {
    type DataType = u32;
}
#[doc = "Access Rules Register\n resetvalue={Application Reset:0x0}"]
pub type Ar = crate::RegValueT<Ar_SPEC>;

impl Ar {
    #[doc = "Access Rule for Window 3   ARW3"]
    #[inline(always)]
    pub fn arw0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Ar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0x3, 1, 0, u8, Ar_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Rule for Window 3   ARW3"]
    #[inline(always)]
    pub fn arw1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Ar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2, 0x3, 1, 0, u8, Ar_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Rule for Window 3   ARW3"]
    #[inline(always)]
    pub fn arw2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Ar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4, 0x3, 1, 0, u8, Ar_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Rule for Window 3   ARW3"]
    #[inline(always)]
    pub fn arw3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Ar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6, 0x3, 1, 0, u8, Ar_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Memory Access Violation Channel   MAVCH. This bit field shows the number of the latest channel that attempted a        not allowed access."]
    #[inline(always)]
    pub fn mavch(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Ar_SPEC, crate::common::R> {
        crate::common::RegisterField::<16, 0x3, 1, 0, u8, Ar_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ar {
    #[inline(always)]
    fn default() -> Ar {
        <crate::RegValueT<Ar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg_SPEC;
impl crate::sealed::RegSpec for Cfg_SPEC {
    type DataType = u32;
}
#[doc = "Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Cfg = crate::RegValueT<Cfg_SPEC>;

impl Cfg {
    #[doc = "Global Predivider   PREDIV. Defines the down scaled module clock to be used by all channel timeout        timers."]
    #[inline(always)]
    pub fn prediv(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Cfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Cfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Streaming Mode Transmitter   SMT"]
    #[inline(always)]
    pub fn smt(self) -> crate::common::RegisterFieldBool<16, 1, 0, Cfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Cfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Streaming Mode Receiver   SMR"]
    #[inline(always)]
    pub fn smr(self) -> crate::common::RegisterFieldBool<17, 1, 0, Cfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Cfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Streaming Channel Mode   SCM. Defines if the channel 2 is used in a streaming or command mode."]
    #[inline(always)]
    pub fn scm(self) -> crate::common::RegisterFieldBool<18, 1, 0, Cfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Cfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel Code Control   CCC. Defines the coding of the channel number in the HSSL header."]
    #[inline(always)]
    pub fn ccc(self) -> crate::common::RegisterFieldBool<19, 1, 0, Cfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Cfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        <crate::RegValueT<Cfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clc_SPEC;
impl crate::sealed::RegSpec for Clc_SPEC {
    type DataType = u32;
}
#[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
pub type Clc = crate::RegValueT<Clc_SPEC>;

impl Clc {
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control of the module."]
    #[inline(always)]
    pub fn disr(self) -> crate::common::RegisterFieldBool<0, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Disable Status Bit   DISS. Bit indicates the current status of the module."]
    #[inline(always)]
    pub fn diss(self) -> crate::common::RegisterFieldBool<1, 1, 0, Clc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Clc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Sleep Mode Enable Control   EDIS. Used to control module  8217 s sleep mode."]
    #[inline(always)]
    pub fn edis(self) -> crate::common::RegisterFieldBool<3, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Clc {
    #[inline(always)]
    fn default() -> Clc {
        <crate::RegValueT<Clc_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crc_SPEC;
impl crate::sealed::RegSpec for Crc_SPEC {
    type DataType = u32;
}
#[doc = "CRC Control Register\n resetvalue={Application Reset:0x0}"]
pub type Crc = crate::RegValueT<Crc_SPEC>;

impl Crc {
    #[doc = "Value to be XORed with the Calculated CRC   XORMASK. Used for error injection  160    160 test purposes."]
    #[inline(always)]
    pub fn xormask(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Crc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Crc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable the Error Injection via XORMASK   XEN"]
    #[inline(always)]
    pub fn xen(self) -> crate::common::RegisterFieldBool<16, 1, 0, Crc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Crc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Crc {
    #[inline(always)]
    fn default() -> Crc {
        <crate::RegValueT<Crc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0CBC000}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MODREV. MODREV defines the module revision number. The value of a module        revision starts with 01 H  first        revision ."]
    #[inline(always)]
    pub fn modrev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MODTYPE. This bit field is C0 H . It defines a        32 bit module."]
    #[inline(always)]
    pub fn modtype(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MODNUMBER. This bit field together with MODTYPE uniquely identifies a module."]
    #[inline(always)]
    pub fn modnumber(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Id_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Id {
    #[inline(always)]
    fn default() -> Id {
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(13352960)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krst0_SPEC;
impl crate::sealed::RegSpec for Krst0_SPEC {
    type DataType = u32;
}
#[doc = "Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
pub type Krst0 = crate::RegValueT<Krst0_SPEC>;

impl Krst0 {
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel        reset will be executed if the reset bits of both kernel registers are        set. The RST bit will be cleared  re set to   180 0  180   by the BPI FPI after the        kernel reset was executed."]
    #[inline(always)]
    pub fn rst(self) -> crate::common::RegisterFieldBool<0, 1, 0, Krst0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Krst0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Kernel Reset Status   RSTSTAT. This bit indicates wether a kernel reset was executed or not. This bit        is set by the BPI FPI after the execution of a kernel reset in the same        clock cycle both reset bits. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related KRSTCLR register."]
    #[inline(always)]
    pub fn rststat(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Krst0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Krst0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Krst0 {
    #[inline(always)]
    fn default() -> Krst0 {
        <crate::RegValueT<Krst0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krst1_SPEC;
impl crate::sealed::RegSpec for Krst1_SPEC {
    type DataType = u32;
}
#[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
pub type Krst1 = crate::RegValueT<Krst1_SPEC>;

impl Krst1 {
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel        reset will be executed if the reset bits of both kernel reset registers        is set. The RST bit will be cleared  re set to   180 0  180   by the BPI FPI after the        kernel reset was executed."]
    #[inline(always)]
    pub fn rst(self) -> crate::common::RegisterFieldBool<0, 1, 0, Krst1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Krst1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Krst1 {
    #[inline(always)]
    fn default() -> Krst1 {
        <crate::RegValueT<Krst1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krstclr_SPEC;
impl crate::sealed::RegSpec for Krstclr_SPEC {
    type DataType = u32;
}
#[doc = "Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
pub type Krstclr = crate::RegValueT<Krstclr_SPEC>;

impl Krstclr {
    #[doc = "Kernel Reset Status Clear   CLR. Read always as 0."]
    #[inline(always)]
    pub fn clr(self) -> crate::common::RegisterFieldBool<0, 1, 0, Krstclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Krstclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Krstclr {
    #[inline(always)]
    fn default() -> Krstclr {
        <crate::RegValueT<Krstclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mflags_SPEC;
impl crate::sealed::RegSpec for Mflags_SPEC {
    type DataType = u32;
}
#[doc = "Miscellaneous Flags Register\n resetvalue={Application Reset:0x080000000}"]
pub type Mflags = crate::RegValueT<Mflags_SPEC>;

impl Mflags {
    #[doc = "Not Acknowledge Error   Target Error   NACK. Indicates for each channel that a target error frame has been received."]
    #[inline(always)]
    pub fn nack(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Mflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Mflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transaction Tag Error   TTE. Indicates for each channel if a CRC correct acknowledge frame with an        unexpected transaction tag number has been received."]
    #[inline(always)]
    pub fn tte(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Mflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Mflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Timeout Error   TIMEOUT. Indicates for each channel if an timeout event has occurred."]
    #[inline(always)]
    pub fn timeout(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Mflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Mflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Unexpected Type of Frame Error   UNEXPECTED. Indicates for each channel if an unexpected or inappropriate response is        received. For example a NACK for a Trigger frame or DATA for WRITE frame."]
    #[inline(always)]
    pub fn unexpected(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Mflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Mflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Target Memory Block   TMB. Selects the currently active memory block used by the target as a target        for the streaming data  with its start address and frame counter.        Switching the active block in the middle of a block transfer is not        allowed."]
    #[inline(always)]
    pub fn tmb(self) -> crate::common::RegisterFieldBool<18, 1, 0, Mflags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Mflags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Initiator Memory Block   IMB. Selects the currently active memory block used by the initiator as a        source for the streaming data  with its start address and frame counter.        Switching the active block in the middle of a block transfer is not        allowed."]
    #[inline(always)]
    pub fn imb(self) -> crate::common::RegisterFieldBool<19, 1, 0, Mflags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Mflags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Initiator Stream Block Request   ISB. Indicates if stream block request is pending. Set by the software to        start a stream block transfer by using the MFLAGSSET.ISBS bit  clear by        the software possible  if needed  by using the MFLAGSCL.ISBC bit         cleared by hardware at the end of the current block transfer in single        mode  but not in continuous mode."]
    #[inline(always)]
    pub fn isb(self) -> crate::common::RegisterFieldBool<20, 1, 0, Mflags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Mflags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Memory Access Violation   MAV. Indicates a memory access violation."]
    #[inline(always)]
    pub fn mav(self) -> crate::common::RegisterFieldBool<21, 1, 0, Mflags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Mflags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SRI SPB Bus Access Error   SRIE. Indicates an error on the SRI bus   transaction ID  ECC error or error        acknowledge. Indicates an error on the SPB bus  error acknowledge or timeout."]
    #[inline(always)]
    pub fn srie(self) -> crate::common::RegisterFieldBool<22, 1, 0, Mflags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Mflags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PHY Inconsistency Error 1 Channel Number Code Error    PIE1. Indicates if HSCT to HSSL channel number code comparator has detected an        inconsistency error."]
    #[inline(always)]
    pub fn pie1(self) -> crate::common::RegisterFieldBool<23, 1, 0, Mflags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Mflags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PHY Inconsistency Error 2 Data Length Error    PIE2. Indicates if HSCT to HSSL data length comparator has detected an        inconsistency error."]
    #[inline(always)]
    pub fn pie2(self) -> crate::common::RegisterFieldBool<24, 1, 0, Mflags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Mflags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Error   CRCE. Indicates if CRC checker has detected a CRC error."]
    #[inline(always)]
    pub fn crce(self) -> crate::common::RegisterFieldBool<25, 1, 0, Mflags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Mflags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Target Stream Enable   TSE. Used by the hardware to handle the single and continuous streaming. In        single mode  cleared by hardware after the current block transfer ends.        The module ignores afterwards the incoming steam frames."]
    #[inline(always)]
    pub fn tse(self) -> crate::common::RegisterFieldBool<28, 1, 0, Mflags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Mflags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmit Enable Input   TEI. Indicates the state of the TEI input signal of the HSSL module  which is        driven by the CTS output signal of the HSCT module. Any edge on this        signal triggers an EXI interrupt. This low level signal stops the transmission of both command and        response frames."]
    #[inline(always)]
    pub fn tei(self) -> crate::common::RegisterFieldBool<29, 1, 0, Mflags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Mflags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmit Enable Output   TEO. Indicates the state of the TEO output signal of the HSSL module  which        drives thee CTS input signal of the HSCT module. This bit is cleared by        hardware at entering the INIT and Soft Suspend state."]
    #[inline(always)]
    pub fn teo(self) -> crate::common::RegisterFieldBool<30, 1, 0, Mflags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Mflags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Initialize Mode   INI. Indicates if the module is in the Initialize or Run mode."]
    #[inline(always)]
    pub fn ini(self) -> crate::common::RegisterFieldBool<31, 1, 0, Mflags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Mflags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mflags {
    #[inline(always)]
    fn default() -> Mflags {
        <crate::RegValueT<Mflags_SPEC> as RegisterValue<_>>::new(2147483648)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mflagscl_SPEC;
impl crate::sealed::RegSpec for Mflagscl_SPEC {
    type DataType = u32;
}
#[doc = "Miscellaneous Flags Clear Register\n resetvalue={Application Reset:0x0}"]
pub type Mflagscl = crate::RegValueT<Mflagscl_SPEC>;

impl Mflagscl {
    #[doc = "NACK Flags Clear   NACKC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn nackc(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Mflagscl_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Mflagscl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Transaction Tag Error Flags Clear   TTEC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn ttec(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Mflagscl_SPEC, crate::common::W> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Mflagscl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Timeout Error Flags Clear   TIMEOUTC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn timeoutc(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Mflagscl_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Mflagscl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Unexpected Error Flags Clear   UNEXPECTEDC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn unexpectedc(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Mflagscl_SPEC, crate::common::W> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Mflagscl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Target Memory Block Flag Clear   TMBC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn tmbc(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Mflagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Mflagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Initiator Memory Block Flag Clear   IMBC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn imbc(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Mflagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, Mflagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Initiator Stream Block Request Clear   ISBC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn isbc(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Mflagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, Mflagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "MAV Flag Clear   MAVC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect"]
    #[inline(always)]
    pub fn mavc(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Mflagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, Mflagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "SRI SPB Bus Access Error Flag Clear   SRIEC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect"]
    #[inline(always)]
    pub fn sriec(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Mflagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, Mflagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "PIE1 Error Flag Clear   PIE1C. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect"]
    #[inline(always)]
    pub fn pie1c(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Mflagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, Mflagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "PIE2 Error Flag Clear   PIE2C. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn pie2c(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Mflagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Mflagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Error Flag Clear   CRCEC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn crcec(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Mflagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Mflagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Target Stream Enable Flag Clear   TSEC. Writing 1 sets the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn tsec(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Mflagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Mflagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmit Enable Flag Clear   TEOC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn teoc(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Mflagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Mflagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Initialize Mode Flag Clear   INIC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn inic(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Mflagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Mflagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mflagscl {
    #[inline(always)]
    fn default() -> Mflagscl {
        <crate::RegValueT<Mflagscl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mflagsen_SPEC;
impl crate::sealed::RegSpec for Mflagsen_SPEC {
    type DataType = u32;
}
#[doc = "Flags Enable Register\n resetvalue={Application Reset:0x0}"]
pub type Mflagsen = crate::RegValueT<Mflagsen_SPEC>;

impl Mflagsen {
    #[doc = "Not Acknowledge Error Enable Bits   NACKEN. Used to enable the error interrupt associated to the corresponding bit        in the MFLAGS register."]
    #[inline(always)]
    pub fn nacken(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Mflagsen_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Mflagsen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transaction Tag Error Enable Bits   TTEEN. Used to enable the error interrupt associated to the corresponding bit        in the MFLAGS register."]
    #[inline(always)]
    pub fn tteen(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Mflagsen_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Mflagsen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timeout Error Enable Bits   TIMEOUTEN. Used to enable the error interrupt associated to the corresponding bit        in the MFLAGS register."]
    #[inline(always)]
    pub fn timeouten(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Mflagsen_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Mflagsen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Unexpected Error Enable Bits   UNEXPECTEDEN. Used to enable the error interrupt associated to the corresponding bit        in the MFLAGS register."]
    #[inline(always)]
    pub fn unexpecteden(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Mflagsen_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Mflagsen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MAV Enable Bit   MAVEN. Used to enable the interrupt associated to the corresponding bit in the        MFLAGS register."]
    #[inline(always)]
    pub fn maven(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Mflagsen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21,1,0,Mflagsen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SRI SPB Bus Access Error Enable Bit   SRIEEN. Used to enable the error interrupt associated to the corresponding bit        in the MFLAGS register."]
    #[inline(always)]
    pub fn srieen(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Mflagsen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,Mflagsen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PIE1 Error Enable Bit   PIE1EN. Used to enable the error interrupt associated to the corresponding bit        in the MFLAGS register."]
    #[inline(always)]
    pub fn pie1en(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Mflagsen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,Mflagsen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PIE2 Error Enable Bit   PIE2EN. Used to enable the error interrupt associated to the corresponding bit        in the MFLAGS register."]
    #[inline(always)]
    pub fn pie2en(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Mflagsen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,Mflagsen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CRC Error Enable Bit   CRCEEN. Used to enable the error interrupt associated to the corresponding bit        in the MFLAGS register."]
    #[inline(always)]
    pub fn crceen(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Mflagsen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,Mflagsen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TEI Enable Bit   TEIEN. Used to enable the interrupt associated to the corresponding bit in the        MFLAGS register."]
    #[inline(always)]
    pub fn teien(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Mflagsen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,Mflagsen_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Mflagsen {
    #[inline(always)]
    fn default() -> Mflagsen {
        <crate::RegValueT<Mflagsen_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mflagsset_SPEC;
impl crate::sealed::RegSpec for Mflagsset_SPEC {
    type DataType = u32;
}
#[doc = "Miscellaneous Flags Set Register\n resetvalue={Application Reset:0x0}"]
pub type Mflagsset = crate::RegValueT<Mflagsset_SPEC>;

impl Mflagsset {
    #[doc = "NACK Flags Set   NACKS. Writing 1 sets the corresponding bit in the MFLAGS register and triggers        the ERR interrupt for the corresponding channel  if enabled in the        MFLAGSEN register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn nacks(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Mflagsset_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Mflagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Transaction Tag Error Flags Set   TTES. Writing 1 sets the corresponding bit in the MFLAGS register and triggers        the ERR interrupt for the corresponding channel  if enabled in the        MFLAGSEN register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ttes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Mflagsset_SPEC, crate::common::W> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Mflagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Timeout Error Flags Set   TIMEOUTS. Writing 1 sets the corresponding bit in the MFLAGS register and triggers        the ERR interrupt for the corresponding channel  if enabled in the        MFLAGSEN register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn timeouts(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Mflagsset_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Mflagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Unexpected Error Flags Set   UNEXPECTEDS. Writing 1 sets the corresponding bit in the MFLAGS register and triggers        the ERR interrupt for the corresponding channel  if enabled in the        MFLAGSEN register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn unexpecteds(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Mflagsset_SPEC, crate::common::W> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Mflagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Target Memory Block Flag Set   TMBS. Writing 1 sets the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn tmbs(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Mflagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18,1,0,Mflagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Initiator Memory Block Flag Set   IMBS. Writing 1 sets the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn imbs(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Mflagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19,1,0,Mflagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Initiator Stream Block Request Set   ISBS. Writing 1 sets the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn isbs(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Mflagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20,1,0,Mflagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "MAV Flag Set   MAVS. Writing 1 sets the corresponding bit in the MFLAGS register  and        generates an EXI interrupt if enabled in the corresponding bit of the        MFLAGSEN register. Writing 0 has no effect"]
    #[inline(always)]
    pub fn mavs(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Mflagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21,1,0,Mflagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "SRI SPB Bus Access Error Flag Set   SRIES. Writing 1 sets the corresponding bit in the MFLAGS register  and        generates an EXI interrupt if enabled in the corresponding bit of the        MFLAGSEN register. Writing 0 has no effect"]
    #[inline(always)]
    pub fn sries(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Mflagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22,1,0,Mflagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "PIE1 Error Flag Set   PIE1S. Writing 1 sets the corresponding bit in the MFLAGS register  and        generates an EXI interrupt if enabled in the corresponding bit of the        MFLAGSEN register. Writing 0 has no effect"]
    #[inline(always)]
    pub fn pie1s(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Mflagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23,1,0,Mflagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "PIE2 Error Flag Set   PIE2S. Writing 1 sets the corresponding bit in the MFLAGS register  and        generates an EXI interrupt if enabled in the corresponding bit of the        MFLAGSEN register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn pie2s(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Mflagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24,1,0,Mflagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "CRC Error Flag Set   CRCES. Writing 1 sets the corresponding bit in the MFLAGS register  and        generates an EXI interrupt if enabled in the corresponding bit of the        MFLAGSEN register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn crces(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Mflagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25,1,0,Mflagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Target Stream Enable Flag Set   TSES. Writing 1 sets the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn tses(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Mflagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28,1,0,Mflagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Transmit Enable Flag Set   TEOS. Writing 1 sets the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn teos(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Mflagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30,1,0,Mflagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Initialize Mode Flag Set   INIS. Writing 1 sets the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn inis(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Mflagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31,1,0,Mflagsset_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Mflagsset {
    #[inline(always)]
    fn default() -> Mflagsset {
        <crate::RegValueT<Mflagsset_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mscr_SPEC;
impl crate::sealed::RegSpec for Mscr_SPEC {
    type DataType = u32;
}
#[doc = "Multi Slave Control Register\n resetvalue={Application Reset:0x0}"]
pub type Mscr = crate::RegValueT<Mscr_SPEC>;

impl Mscr {
    #[doc = "Multi Slave Mode Enable   EN. This bit enables the multi slave mode of operation of the HSSL module."]
    #[inline(always)]
    pub fn en(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mscr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mscr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Slave Tag   SLAVETAG. The master uses this bit field to define the current slave it sends to        and receives from. The slave uses this bit field to pass through only        the relevant incoming frames and to tag its own transmit frames. This        tag is injected in the header by the Slave Tag Translator block and used        for filtering in Slave Tag Filter Block."]
    #[inline(always)]
    pub fn slavetag(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, Mscr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3,1,0,u8, Mscr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Initiator Transmission Stop   ITXSTOP. This bit is intended to be set or cleared by the master in a multi slave        operation by using 16 bit write command frames. MSCR.ITXSTOP        functionality can be activated only if MSCR.EN  160    160 1  otherwise it is        ignored. Setting this bit stops the arbitration and transmission of new command        and stream frames of the initiator but does not stop the ongoing frames."]
    #[inline(always)]
    pub fn itxstop(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Mscr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Mscr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mscr {
    #[inline(always)]
    fn default() -> Mscr {
        <crate::RegValueT<Mscr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ocs_SPEC;
impl crate::sealed::RegSpec for Ocs_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0}"]
pub type Ocs = crate::RegValueT<Ocs_SPEC>;

impl Ocs {
    #[doc = "Trigger Set for OTGB0 1   TGS"]
    #[inline(always)]
    pub fn tgs(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Ocs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0x3, 1, 0, u8, Ocs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "OTGB0 1 Bus Select   TGB"]
    #[inline(always)]
    pub fn tgb(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ocs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ocs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "TGS  TGB Write Protection   TG P. TGS and TGB are only written when TG P is 1  otherwise unchanged. Read        as 0."]
    #[inline(always)]
    pub fn tg_p(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ocs_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ocs_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "OCDS Suspend Control   SUS. Controls the sensitivity to the suspend signal coming from the OCDS        Trigger Switch  OTGS"]
    #[inline(always)]
    pub fn sus(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Ocs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Ocs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SUS Write Protection   SUS P. SUS is only written when SUS P is 1  otherwise unchanged. Read as 0."]
    #[inline(always)]
    pub fn sus_p(self) -> crate::common::RegisterFieldBool<28, 1, 0, Ocs_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Ocs_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Suspend State   SUSSTA"]
    #[inline(always)]
    pub fn sussta(self) -> crate::common::RegisterFieldBool<29, 1, 0, Ocs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Ocs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ocs {
    #[inline(always)]
    fn default() -> Ocs {
        <crate::RegValueT<Ocs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qflags_SPEC;
impl crate::sealed::RegSpec for Qflags_SPEC {
    type DataType = u32;
}
#[doc = "Request Flags Register\n resetvalue={Application Reset:0x0}"]
pub type Qflags = crate::RegValueT<Qflags_SPEC>;

impl Qflags {
    #[doc = "Request Flags for Initiated Commands   I. These flags are set by the corresponding channel when a WRTS command is        initiated. The WRT commands are initiated via the SPB bus. The S tream         commands are initiated by the module internally  by the TXFIFO  except        for the first stream frame start  which is done via the SPB bus. See        MFLAGS.ISB and ISF flags."]
    #[inline(always)]
    pub fn i(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Qflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Qflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Flags for Commands Arrived at Target   T. These flags are set by the hardware according to the header information        of a frame arrived at the target without a CRC error. They are used by        the arbiter of the SRI  SPB        master and cleared when the appropriate command is executed."]
    #[inline(always)]
    pub fn t(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Qflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Qflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Flags for Response Frames at the Target   R. After a command has been executed by the SRI  SPB        master  an appropriate flag is being set which indicates that an        ACK NACK DATA is pending."]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Qflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Qflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Expect Flags for Activated Timeout Timer 0   E0. An appropriate two bit flag is set by the hardware when a timeout timer        for a channel is started. The hardware clears the appropriate flag when        any response frame arrives at the initiator. In case of an unexpected response the flag UNEXPECTED is additionally        set."]
    #[inline(always)]
    pub fn e0(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Qflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Qflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Expect Flags for Activated Timeout Timer 1   E1"]
    #[inline(always)]
    pub fn e1(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, Qflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<18,0x3,1,0,u8, Qflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Expect Flags for Activated Timeout Timer 2   E2"]
    #[inline(always)]
    pub fn e2(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Qflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Qflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Expect Flags for Activated Timeout Timer 3   E3"]
    #[inline(always)]
    pub fn e3(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, Qflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<22,0x3,1,0,u8, Qflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "I Flag for Stream Frames   IS. See the   8220 I  8221  flag description above."]
    #[inline(always)]
    pub fn is(self) -> crate::common::RegisterFieldBool<28, 1, 0, Qflags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Qflags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "R Flag for Stream Frames   RS. See the   8220 R  8221  flag description above."]
    #[inline(always)]
    pub fn rs(self) -> crate::common::RegisterFieldBool<29, 1, 0, Qflags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Qflags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "T Flag for Stream Frames   TS. See the   8220 T  8221  flag description above."]
    #[inline(always)]
    pub fn ts(self) -> crate::common::RegisterFieldBool<30, 1, 0, Qflags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Qflags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "E Flag for Stream Frames   ES. See the   8220 E  8221  flag description above."]
    #[inline(always)]
    pub fn es(self) -> crate::common::RegisterFieldBool<31, 1, 0, Qflags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Qflags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Qflags {
    #[inline(always)]
    fn default() -> Qflags {
        <crate::RegValueT<Qflags_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sec_SPEC;
impl crate::sealed::RegSpec for Sec_SPEC {
    type DataType = u32;
}
#[doc = "Security Control Register\n resetvalue={Application Reset:0x0}"]
pub type Sec = crate::RegValueT<Sec_SPEC>;

impl Sec {
    #[doc = "Lock the HSSL Module   LCK. Setting this bit field prevents the MFLAGS.INI bit to be cleared  that        is prevents to leave the INIT state and go to RUN mode. If INIT bit has        already been cleared  it is possible to set it  but not to clear it        afterwards. This bit can only be written by an access from the HSM master  TAG          000011B . A write operation performed by any other master is ignored and        the bit remains unchanged."]
    #[inline(always)]
    pub fn lck(self) -> crate::common::RegisterFieldBool<0, 1, 0, Sec_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Sec_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Lock the Address Windows Registers   LAW. This bit field shows if the Address Window Registers AWSTARTx  AWENDx         AR and TIDADD are locked or not. If locked  the properties of the        address windows cannot be changed any more. This bit can only be written by an access from the HSM master  TAG          000011B . A write operation performed by any other master is ignored and        the bit remains unchanged."]
    #[inline(always)]
    pub fn law(self) -> crate::common::RegisterFieldBool<1, 1, 0, Sec_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Sec_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Sec {
    #[inline(always)]
    fn default() -> Sec {
        <crate::RegValueT<Sec_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfsflags_SPEC;
impl crate::sealed::RegSpec for Sfsflags_SPEC {
    type DataType = u32;
}
#[doc = "Stream FIFOs Status Flags Register\n resetvalue={Application Reset:0x0}"]
pub type Sfsflags = crate::RegValueT<Sfsflags_SPEC>;

impl Sfsflags {
    #[doc = "Stream RxFIFO Filling Level   RXFL. Indicates the filling level of the FIFO with granularity of 32 bytes         one stream frame payload size ."]
    #[inline(always)]
    pub fn rxfl(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Sfsflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Sfsflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Stream TxFIFO Filling Level   TXFL. Indicates the filling level of the FIFO with granularity of 32 bytes         one stream frame payload size ."]
    #[inline(always)]
    pub fn txfl(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Sfsflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Sfsflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Stream Expect FIFO Filling Level   EXFL. Indicates the filling level of the FIFO with granularity of 32 bytes         one stream frame payload size ."]
    #[inline(always)]
    pub fn exfl(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Sfsflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Sfsflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Initiator Stream Frame Request   ISF. Indicates if stream TXFIFO request is pending. Set and cleared by the        TXFIFO."]
    #[inline(always)]
    pub fn isf(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Sfsflags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Sfsflags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Sfsflags {
    #[inline(always)]
    fn default() -> Sfsflags {
        <crate::RegValueT<Sfsflags_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tidadd_SPEC;
impl crate::sealed::RegSpec for Tidadd_SPEC {
    type DataType = u32;
}
#[doc = "Target ID Address Register\n resetvalue={Application Reset:0x0}"]
pub type Tidadd = crate::RegValueT<Tidadd_SPEC>;

impl Tidadd {
    #[doc = "Address Pointer   A. Address pointer containing the address of the memory location containing        the unique ID data."]
    #[inline(always)]
    pub fn a(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Tidadd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Tidadd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Tidadd {
    #[inline(always)]
    fn default() -> Tidadd {
        <crate::RegValueT<Tidadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tstat_SPEC;
impl crate::sealed::RegSpec for Tstat_SPEC {
    type DataType = u32;
}
#[doc = "Target Status Register\n resetvalue={Application Reset:0x0}"]
pub type Tstat = crate::RegValueT<Tstat_SPEC>;

impl Tstat {
    #[doc = "Last Command Code   LASTCC3. Indicates the latest command code from the header for the corresponding        channel."]
    #[inline(always)]
    pub fn lastcc0(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, Tstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, Tstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Last Transaction Tag   LASTTT3. Indicates the transaction tag of the latest command or stream frame for        the corresponding channel."]
    #[inline(always)]
    pub fn lasttt0(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, Tstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x7,1,0,u8, Tstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Last Command Code   LASTCC3. Indicates the latest command code from the header for the corresponding        channel."]
    #[inline(always)]
    pub fn lastcc1(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, Tstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, Tstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Last Transaction Tag   LASTTT3. Indicates the transaction tag of the latest command or stream frame for        the corresponding channel."]
    #[inline(always)]
    pub fn lasttt1(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, Tstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<13,0x7,1,0,u8, Tstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Last Command Code   LASTCC3. Indicates the latest command code from the header for the corresponding        channel."]
    #[inline(always)]
    pub fn lastcc2(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Tstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Tstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Last Transaction Tag   LASTTT3. Indicates the transaction tag of the latest command or stream frame for        the corresponding channel."]
    #[inline(always)]
    pub fn lasttt2(
        self,
    ) -> crate::common::RegisterField<21, 0x7, 1, 0, u8, Tstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<21,0x7,1,0,u8, Tstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Last Command Code   LASTCC3. Indicates the latest command code from the header for the corresponding        channel."]
    #[inline(always)]
    pub fn lastcc3(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, Tstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, Tstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Last Transaction Tag   LASTTT3. Indicates the transaction tag of the latest command or stream frame for        the corresponding channel."]
    #[inline(always)]
    pub fn lasttt3(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, Tstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<29,0x7,1,0,u8, Tstat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tstat {
    #[inline(always)]
    fn default() -> Tstat {
        <crate::RegValueT<Tstat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc = "AW"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aw(pub(super) *mut u8);
unsafe impl core::marker::Send for Aw {}
unsafe impl core::marker::Sync for Aw {}
impl Aw {
    #[doc = "Access Window End Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn awendi(&self) -> crate::common::Reg<aw::AwenDi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Access Window Start Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn awstarti(&self) -> crate::common::Reg<aw::AwstarTi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod aw {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AwenDi_SPEC;
    impl crate::sealed::RegSpec for AwenDi_SPEC {
        type DataType = u32;
    }
    #[doc = "Access Window End Register 0\n resetvalue={Application Reset:0x0}"]
    pub type AwenDi = crate::RegValueT<AwenDi_SPEC>;

    impl AwenDi {
        #[doc = "Access Window End Address   AWE. This bit field defines the upper 24 bits of the end address of the        corresponding access window. This results in a granularity of 256 bytes        for the end address."]
        #[inline(always)]
        pub fn awe(
            self,
        ) -> crate::common::RegisterField<8, 0xffffff, 1, 0, u32, AwenDi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xffffff,1,0,u32, AwenDi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for AwenDi {
        #[inline(always)]
        fn default() -> AwenDi {
            <crate::RegValueT<AwenDi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AwstarTi_SPEC;
    impl crate::sealed::RegSpec for AwstarTi_SPEC {
        type DataType = u32;
    }
    #[doc = "Access Window Start Register 0\n resetvalue={Application Reset:0x0}"]
    pub type AwstarTi = crate::RegValueT<AwstarTi_SPEC>;

    impl AwstarTi {
        #[doc = "Access Window Start Address   AWS. This bit field defines the upper 24 bits of the start address of the        corresponding access window. This results in a granularity of 256 bytes        for the start address."]
        #[inline(always)]
        pub fn aws(
            self,
        ) -> crate::common::RegisterField<8, 0xffffff, 1, 0, u32, AwstarTi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xffffff,1,0,u32, AwstarTi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for AwstarTi {
        #[inline(always)]
        fn default() -> AwstarTi {
            <crate::RegValueT<AwstarTi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "IS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Is(pub(super) *mut u8);
unsafe impl core::marker::Send for Is {}
unsafe impl core::marker::Sync for Is {}
impl Is {
    #[doc = "Initiator Stream Current Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn isca(&self) -> crate::common::Reg<is::Isca_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Initiator Stream Frame Count Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn isfc(&self) -> crate::common::Reg<is::Isfc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Initiator Stream Start Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn issax(&self) -> [crate::common::Reg<is::IssAx_SPEC, crate::common::RW>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
            ]
        }
    }
}
pub mod is {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isca_SPEC;
    impl crate::sealed::RegSpec for Isca_SPEC {
        type DataType = u32;
    }
    #[doc = "Initiator Stream Current Address Register\n resetvalue={Application Reset:0x0}"]
    pub type Isca = crate::RegValueT<Isca_SPEC>;

    impl Isca {
        #[doc = "Address of the Memory Location for the Current Transfer   CURR. Aligned on 256 bit limit  stream frame payload size ."]
        #[inline(always)]
        pub fn curr(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, Isca_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<5,0x7ffffff,1,0,u32, Isca_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for Isca {
        #[inline(always)]
        fn default() -> Isca {
            <crate::RegValueT<Isca_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isfc_SPEC;
    impl crate::sealed::RegSpec for Isfc_SPEC {
        type DataType = u32;
    }
    #[doc = "Initiator Stream Frame Count Register\n resetvalue={Application Reset:0x0}"]
    pub type Isfc = crate::RegValueT<Isfc_SPEC>;

    impl Isfc {
        #[doc = "Reload Count Number   RELCOUNT. Contains the number of frames to transfer per memory block. Bit field        length depends on application requirements."]
        #[inline(always)]
        pub fn relcount(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Isfc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, Isfc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Current Count Number   CURCOUNT. Displays the current count number  which is generated by down counting from the RELCOUNT value. Bit field length depends on application requirements."]
        #[inline(always)]
        pub fn curcount(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Isfc_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, Isfc_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for Isfc {
        #[inline(always)]
        fn default() -> Isfc {
            <crate::RegValueT<Isfc_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IssAx_SPEC;
    impl crate::sealed::RegSpec for IssAx_SPEC {
        type DataType = u32;
    }
    #[doc = "Initiator Stream Start Address Register\n resetvalue={Application Reset:0x0}"]
    pub type IssAx = crate::RegValueT<IssAx_SPEC>;

    impl IssAx {
        #[doc = "Start Address for the Memory Range   START. Aligned on 256 bit limit  stream frame payload size ."]
        #[inline(always)]
        pub fn start(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, IssAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7ffffff,1,0,u32, IssAx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for IssAx {
        #[inline(always)]
        fn default() -> IssAx {
            <crate::RegValueT<IssAx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "I"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I(pub(super) *mut u8);
unsafe impl core::marker::Send for I {}
unsafe impl core::marker::Sync for I {}
impl I {
    #[doc = "Initiator Control Data Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn iconx(&self) -> crate::common::Reg<i::IcoNx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Initiator Read Data Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn irdx(&self) -> crate::common::Reg<i::IrDx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Initiator Read Write Address Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn irwax(&self) -> crate::common::Reg<i::IrwAx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Initiator Write Data Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn iwdx(&self) -> crate::common::Reg<i::IwDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod i {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IcoNx_SPEC;
    impl crate::sealed::RegSpec for IcoNx_SPEC {
        type DataType = u32;
    }
    #[doc = "Initiator Control Data Register 0\n resetvalue={Application Reset:0x0}"]
    pub type IcoNx = crate::RegValueT<IcoNx_SPEC>;

    impl IcoNx {
        #[doc = "Read ID Request   IDQ. This bit provides the only way to request a read ID frame. Reads always 0. Write of 1 commences a request. In case of parallel write of 1 to TQ and IDQ  the IDQ request has higher priority."]
        #[inline(always)]
        pub fn idq(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, IcoNx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<0, 1, 0, IcoNx_SPEC, crate::common::W>::from_register(
                self, 0,
            )
        }
        #[doc = "Trigger Request   TQ. This bit provides an alternative way to request a trigger frame  without having to write to the channel address register. Reads always 0. Write of 1 commences a request. In case of parallel write of 1 to TQ and IDQ  the IDQ request has higher priority."]
        #[inline(always)]
        pub fn tq(self) -> crate::common::RegisterFieldBool<1, 1, 0, IcoNx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<1, 1, 0, IcoNx_SPEC, crate::common::W>::from_register(
                self, 0,
            )
        }
        #[doc = "Last Error Transaction Tag   LETT"]
        #[inline(always)]
        pub fn lett(
            self,
        ) -> crate::common::RegisterField<2, 0x7, 1, 0, u8, IcoNx_SPEC, crate::common::R> {
            crate::common::RegisterField::<2,0x7,1,0,u8, IcoNx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Currently Expected Transaction Tag   CETT"]
        #[inline(always)]
        pub fn cett(
            self,
        ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, IcoNx_SPEC, crate::common::R> {
            crate::common::RegisterField::<5,0x7,1,0,u8, IcoNx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Time Out Current Value   TOCV"]
        #[inline(always)]
        pub fn tocv(
            self,
        ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, IcoNx_SPEC, crate::common::R> {
            crate::common::RegisterField::<8,0xff,1,0,u8, IcoNx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Data Length   DATLEN. Defines the length of the data in bits of the write and read command."]
        #[inline(always)]
        pub fn datlen(
            self,
        ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, IcoNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x3,1,0,u8, IcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Read Write Trigger Command Type   RWT. Defines if the write to the IRWA register will trigger read  write or trigger request."]
        #[inline(always)]
        pub fn rwt(
            self,
        ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, IcoNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<18,0x3,1,0,u8, IcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Channel Busy   BSY"]
        #[inline(always)]
        pub fn bsy(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, IcoNx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<20,1,0,IcoNx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Initiator Transaction Tag   ITTAG. This bit displays the current value of the three bit counter generating the new transaction tag value."]
        #[inline(always)]
        pub fn ittag(
            self,
        ) -> crate::common::RegisterField<21, 0x7, 1, 0, u8, IcoNx_SPEC, crate::common::R> {
            crate::common::RegisterField::<21,0x7,1,0,u8, IcoNx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Time Out Reload Value   TOREL. Defines the duration of the timeout in units of prescaled clock periods. This parameter is valid both in command and stream mode of the channel 2."]
        #[inline(always)]
        pub fn torel(
            self,
        ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, IcoNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0xff,1,0,u8, IcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for IcoNx {
        #[inline(always)]
        fn default() -> IcoNx {
            <crate::RegValueT<IcoNx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrDx_SPEC;
    impl crate::sealed::RegSpec for IrDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Initiator Read Data Register 0\n resetvalue={Application Reset:0x0}"]
    pub type IrDx = crate::RegValueT<IrDx_SPEC>;

    impl IrDx {
        #[doc = "Data Delivered by a Read Response Frame   DATA"]
        #[inline(always)]
        pub fn data(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, IrDx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, IrDx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for IrDx {
        #[inline(always)]
        fn default() -> IrDx {
            <crate::RegValueT<IrDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrwAx_SPEC;
    impl crate::sealed::RegSpec for IrwAx_SPEC {
        type DataType = u32;
    }
    #[doc = "Initiator Read Write Address Register 0\n resetvalue={Application Reset:0x0}"]
    pub type IrwAx = crate::RegValueT<IrwAx_SPEC>;

    impl IrwAx {
        #[doc = "Address Part of the Payload of a Write Frame   ADDRESS. Writing to this registers triggers transmission of a Write Frame. The        address must be aligned according to the data width  byte addresses for        8 bit data  half word addresses for 16 bit data  word addresses for        32 bit data."]
        #[inline(always)]
        pub fn address(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, IrwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, IrwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for IrwAx {
        #[inline(always)]
        fn default() -> IrwAx {
            <crate::RegValueT<IrwAx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IwDx_SPEC;
    impl crate::sealed::RegSpec for IwDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Initiator Write Data Register 0\n resetvalue={Application Reset:0x0}"]
    pub type IwDx = crate::RegValueT<IwDx_SPEC>;

    impl IwDx {
        #[doc = "Data Part of the Payload of a Write Frame   DATA. For 8 bit and16 bit write command frames  the whole frame payload width of 32 bit is automatically filled with copies of the lower 8 bits or 16 bits of the register."]
        #[inline(always)]
        pub fn data(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, IwDx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, IwDx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for IwDx {
        #[inline(always)]
        fn default() -> IwDx {
            <crate::RegValueT<IwDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "TS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ts(pub(super) *mut u8);
unsafe impl core::marker::Send for Ts {}
unsafe impl core::marker::Sync for Ts {}
impl Ts {
    #[doc = "Target Stream Current Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tsca(&self) -> crate::common::Reg<ts::Tsca_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Target Stream Frame Count Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tsfc(&self) -> crate::common::Reg<ts::Tsfc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Target Stream Start Address Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tssax(&self) -> [crate::common::Reg<ts::TssAx_SPEC, crate::common::RW>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
            ]
        }
    }
}
pub mod ts {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tsca_SPEC;
    impl crate::sealed::RegSpec for Tsca_SPEC {
        type DataType = u32;
    }
    #[doc = "Target Stream Current Address Register\n resetvalue={Application Reset:0x0}"]
    pub type Tsca = crate::RegValueT<Tsca_SPEC>;

    impl Tsca {
        #[doc = "Address of the Memory Location for the Current Transfer   CURR. Aligned on 256 bit  or 32 byte  limit  stream frame payload size ."]
        #[inline(always)]
        pub fn curr(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, Tsca_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<5,0x7ffffff,1,0,u32, Tsca_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for Tsca {
        #[inline(always)]
        fn default() -> Tsca {
            <crate::RegValueT<Tsca_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tsfc_SPEC;
    impl crate::sealed::RegSpec for Tsfc_SPEC {
        type DataType = u32;
    }
    #[doc = "Target Stream Frame Count Register\n resetvalue={Application Reset:0x0}"]
    pub type Tsfc = crate::RegValueT<Tsfc_SPEC>;

    impl Tsfc {
        #[doc = "Reload Count Number   RELCOUNT. Contains the number of frames to transfer per memory block. Bit field        length depends on application requirements."]
        #[inline(always)]
        pub fn relcount(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Tsfc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, Tsfc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Current Count Number   CURCOUNT. Displays the current count number  which is generated by down counting        from the RELCOUNT value. Bit field length depends on application        requirements."]
        #[inline(always)]
        pub fn curcount(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Tsfc_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, Tsfc_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for Tsfc {
        #[inline(always)]
        fn default() -> Tsfc {
            <crate::RegValueT<Tsfc_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TssAx_SPEC;
    impl crate::sealed::RegSpec for TssAx_SPEC {
        type DataType = u32;
    }
    #[doc = "Target Stream Start Address Register 0\n resetvalue={Application Reset:0x0}"]
    pub type TssAx = crate::RegValueT<TssAx_SPEC>;

    impl TssAx {
        #[doc = "Start Address for the Memory Range   ADDR. Aligned on 256 bit  or 32 byte  limit  stream frame payload size ."]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, TssAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7ffffff,1,0,u32, TssAx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for TssAx {
        #[inline(always)]
        fn default() -> TssAx {
            <crate::RegValueT<TssAx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "T"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T(pub(super) *mut u8);
unsafe impl core::marker::Send for T {}
unsafe impl core::marker::Sync for T {}
impl T {
    #[doc = "Target Current Address Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tcai(&self) -> crate::common::Reg<t::TcAi_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Target Current Data Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tcdi(&self) -> crate::common::Reg<t::TcDi_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod t {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcAi_SPEC;
    impl crate::sealed::RegSpec for TcAi_SPEC {
        type DataType = u32;
    }
    #[doc = "Target Current Address Register 0\n resetvalue={Application Reset:0x0}"]
    pub type TcAi = crate::RegValueT<TcAi_SPEC>;

    impl TcAi {
        #[doc = "Address Part of the Payload of a Write Command Frame or a Read Command Frame or ID Frame   A"]
        #[inline(always)]
        pub fn a(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, TcAi_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, TcAi_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for TcAi {
        #[inline(always)]
        fn default() -> TcAi {
            <crate::RegValueT<TcAi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcDi_SPEC;
    impl crate::sealed::RegSpec for TcDi_SPEC {
        type DataType = u32;
    }
    #[doc = "Target Current Data Register 0\n resetvalue={Application Reset:0x0}"]
    pub type TcDi = crate::RegValueT<TcDi_SPEC>;

    impl TcDi {
        #[doc = "Data Part of the Payload of a Write Command Frame or Read Data of a Read Command Frame   D"]
        #[inline(always)]
        pub fn d(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, TcDi_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, TcDi_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for TcDi {
        #[inline(always)]
        fn default() -> TcDi {
            <crate::RegValueT<TcDi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
