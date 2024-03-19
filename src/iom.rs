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
#[doc = r"IOM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iom(pub(super) *mut u8);
unsafe impl core::marker::Send for Iom {}
unsafe impl core::marker::Sync for Iom {}
impl Iom {
    #[doc = "IOM Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }

    #[doc = "IOM Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "IOM Event Combiner Module Counter Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ecmccfg(&self) -> crate::common::Reg<self::Ecmccfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }

    #[doc = "IOM Event Combiner Module Event Trigger History Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ecmeth0(&self) -> crate::common::Reg<self::Ecmeth0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }

    #[doc = "IOM Event Combiner Module Event Trigger History Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ecmeth1(&self) -> crate::common::Reg<self::Ecmeth1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "IOM Event Combiner Module Global Event Selection Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ecmselr(&self) -> crate::common::Reg<self::Ecmselr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "IOM Filter and Prescaler Channel Control Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fpcctrk(&self) -> [crate::common::Reg<self::FpcctRk_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x3cusize)),
            ]
        }
    }

    #[doc = "IOM Filter and Prescaler Channels Rising   Falling Edge Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fpcesr(&self) -> crate::common::Reg<self::Fpcesr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(120usize)) }
    }

    #[doc = "IOM Filter and Prescaler Channel Timer Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fpctimk(&self) -> [crate::common::Reg<self::FpctiMk_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x3cusize)),
            ]
        }
    }

    #[doc = "IOM GTM Input EXOR Combiner Selection Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gtmexr(&self) -> crate::common::Reg<self::Gtmexr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "IOM Identification Register\n resetvalue={Application Reset:0x0CCC002}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "IOM Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "IOM Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "IOM Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "IOM Logic Analyzer Module Configuration Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn lamcfgm(&self) -> [crate::common::Reg<self::LamcfGm_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x3cusize)),
            ]
        }
    }

    #[doc = "IOM Logic Analyzer Module Event Window Count Status Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn lamewcm(&self) -> [crate::common::Reg<self::LamewCm_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x3cusize)),
            ]
        }
    }

    #[doc = "IOM Logic Analyzer Module Event Window Configuration Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn lamewsm(&self) -> [crate::common::Reg<self::LamewSm_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x3cusize)),
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
#[doc = "IOM Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accen0 = crate::RegValueT<Accen0_SPEC>;

impl Accen0 {
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
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
pub struct Clc_SPEC;
impl crate::sealed::RegSpec for Clc_SPEC {
    type DataType = u32;
}
#[doc = "IOM Clock Control Register\n resetvalue={Application Reset:0x3}"]
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
    #[doc = "8 bit Clock Divider Value in RUN Mode   RMC"]
    #[inline(always)]
    pub fn rmc(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Clc_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct Ecmccfg_SPEC;
impl crate::sealed::RegSpec for Ecmccfg_SPEC {
    type DataType = u32;
}
#[doc = "IOM Event Combiner Module Counter Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Ecmccfg = crate::RegValueT<Ecmccfg_SPEC>;

impl Ecmccfg {
    #[doc = "Event Channel Select. This bit field determines which channel event output is to be routed to counter C0."]
    #[inline(always)]
    pub fn selc0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Ecmccfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Ecmccfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel Event Counter Threshold. This bit field determines the threshold count value. Upon the counter meeting the threshold  the counter event output becomes active high for one clock cycle and the count is reset to zero."]
    #[inline(always)]
    pub fn thrc0(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Ecmccfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Ecmccfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Channel Select . This bit field determines which channel event output is to be routed to counter C1."]
    #[inline(always)]
    pub fn selc1(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Ecmccfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Ecmccfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel Event Counter Threshold . This bit field determines the threshold count value. Upon the counter meeting the threshold  the counter event output becomes active high for one clock cycle and the count is reset to zero."]
    #[inline(always)]
    pub fn thrc1(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Ecmccfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Ecmccfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Channel Select . This bit field determines which channel event output is to be routed to counter C2."]
    #[inline(always)]
    pub fn selc2(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Ecmccfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Ecmccfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel Event Counter Threshold. This bit field determines the threshold count value. Upon the counter meeting the threshold  the counter event output becomes active high for one clock cycle and the count is reset to zero."]
    #[inline(always)]
    pub fn thrc2(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Ecmccfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Ecmccfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Channel Select. This bit field determines which channel event output is to be routed to counter C3."]
    #[inline(always)]
    pub fn selc3(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Ecmccfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Ecmccfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel Event Counter Threshold. This bit field determines the threshold count value. Upon the counter meeting the threshold  the counter event output becomes active high for one clock cycle and the count is reset to zero."]
    #[inline(always)]
    pub fn thrc3(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Ecmccfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Ecmccfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ecmccfg {
    #[inline(always)]
    fn default() -> Ecmccfg {
        <crate::RegValueT<Ecmccfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecmeth0_SPEC;
impl crate::sealed::RegSpec for Ecmeth0_SPEC {
    type DataType = u32;
}
#[doc = "IOM Event Combiner Module Event Trigger History Register 0\n resetvalue={Application Reset:0x0}"]
pub type Ecmeth0 = crate::RegValueT<Ecmeth0_SPEC>;

impl Ecmeth0 {
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta9(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb0(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb1(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb2(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb3(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb4(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb5(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb6(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb7(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb8(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb9(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb10(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb11(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb12(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb13(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb14(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb15(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Ecmeth0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ecmeth0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ecmeth0 {
    #[inline(always)]
    fn default() -> Ecmeth0 {
        <crate::RegValueT<Ecmeth0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecmeth1_SPEC;
impl crate::sealed::RegSpec for Ecmeth1_SPEC {
    type DataType = u32;
}
#[doc = "IOM Event Combiner Module Event Trigger History Register 1\n resetvalue={Application Reset:0x0}"]
pub type Ecmeth1 = crate::RegValueT<Ecmeth1_SPEC>;

impl Ecmeth1 {
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc9(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd0(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd1(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd2(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd3(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd4(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd5(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd6(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd7(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd8(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd9(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd10(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd11(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd12(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd13(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd14(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd15(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Ecmeth1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ecmeth1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ecmeth1 {
    #[inline(always)]
    fn default() -> Ecmeth1 {
        <crate::RegValueT<Ecmeth1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecmselr_SPEC;
impl crate::sealed::RegSpec for Ecmselr_SPEC {
    type DataType = u32;
}
#[doc = "IOM Event Combiner Module Global Event Selection Register\n resetvalue={Application Reset:0x0}"]
pub type Ecmselr = crate::RegValueT<Ecmselr_SPEC>;

impl Ecmselr {
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Ecmselr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ecmselr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Ecmselr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ecmselr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Ecmselr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ecmselr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ecmselr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ecmselr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ecmselr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ecmselr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ecmselr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ecmselr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ecmselr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ecmselr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ecmselr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ecmselr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Ecmselr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ecmselr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces9(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ecmselr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ecmselr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ecmselr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ecmselr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ecmselr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ecmselr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Ecmselr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ecmselr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Ecmselr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ecmselr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Ecmselr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ecmselr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Ecmselr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ecmselr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated  Cou3ted  Event Combiner Selection   CTS3. The setting of individual bitfields determines the inclusion of the respective channel event counter output  1 of 4  in the generation of the global event."]
    #[inline(always)]
    pub fn cts0(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Ecmselr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ecmselr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated  Cou3ted  Event Combiner Selection   CTS3. The setting of individual bitfields determines the inclusion of the respective channel event counter output  1 of 4  in the generation of the global event."]
    #[inline(always)]
    pub fn cts1(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Ecmselr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ecmselr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated  Cou3ted  Event Combiner Selection   CTS3. The setting of individual bitfields determines the inclusion of the respective channel event counter output  1 of 4  in the generation of the global event."]
    #[inline(always)]
    pub fn cts2(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Ecmselr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ecmselr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated  Cou3ted  Event Combiner Selection   CTS3. The setting of individual bitfields determines the inclusion of the respective channel event counter output  1 of 4  in the generation of the global event."]
    #[inline(always)]
    pub fn cts3(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Ecmselr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ecmselr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ecmselr {
    #[inline(always)]
    fn default() -> Ecmselr {
        <crate::RegValueT<Ecmselr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FpcctRk_SPEC;
impl crate::sealed::RegSpec for FpcctRk_SPEC {
    type DataType = u32;
}
#[doc = "IOM Filter and Prescaler Channel Control Register 0\n resetvalue={Application Reset:0x0}"]
pub type FpcctRk = crate::RegValueT<FpcctRk_SPEC>;

impl FpcctRk {
    #[doc = "Threshold Value of Filter   Prescaler Channel k   CMP. CMP is the 16 bit threshold value that is compared with the 16 bit timer value FPCTIMk.TIM."]
    #[inline(always)]
    pub fn cmp(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, FpcctRk_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, FpcctRk_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Operation Mode Selection for Filter   Prescaler Channel k   MOD"]
    #[inline(always)]
    pub fn r#mod(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, FpcctRk_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8, FpcctRk_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Monitor Input Signal Selection for Filter   Prescaler Channel k   ISM. ISM selects the monitor signal that goes to the monitor input  MON  of        the Logic Analyzer Module."]
    #[inline(always)]
    pub fn ism(
        self,
    ) -> crate::common::RegisterField<19, 0x3, 1, 0, u8, FpcctRk_SPEC, crate::common::RW> {
        crate::common::RegisterField::<19,0x3,1,0,u8, FpcctRk_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reset Timer behavior for Filter   Prescaler Channel k on Glitch   RTG. This bit is effective in Delayed Debounce Filter Mode only."]
    #[inline(always)]
    pub fn rtg(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, FpcctRk_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, FpcctRk_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Reference Input Signal Selection for Filter   Prescaler Channel k   ISR. ISR select the reference signal that goes to the reference input  REF         of the Logic Analyzer Module."]
    #[inline(always)]
    pub fn isr(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, FpcctRk_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, FpcctRk_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for FpcctRk {
    #[inline(always)]
    fn default() -> FpcctRk {
        <crate::RegValueT<FpcctRk_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fpcesr_SPEC;
impl crate::sealed::RegSpec for Fpcesr_SPEC {
    type DataType = u32;
}
#[doc = "IOM Filter and Prescaler Channels Rising   Falling Edge Status Register\n resetvalue={Application Reset:0x0}"]
pub type Fpcesr = crate::RegValueT<Fpcesr_SPEC>;

impl Fpcesr {
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg0(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg1(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg2(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg3(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg4(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg5(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg6(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg7(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg8(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg9(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg10(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg11(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg12(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg13(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg14(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg15(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Fpcesr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Fpcesr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Fpcesr {
    #[inline(always)]
    fn default() -> Fpcesr {
        <crate::RegValueT<Fpcesr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FpctiMk_SPEC;
impl crate::sealed::RegSpec for FpctiMk_SPEC {
    type DataType = u32;
}
#[doc = "IOM Filter and Prescaler Channel Timer Register 0\n resetvalue={Application Reset:0x0}"]
pub type FpctiMk = crate::RegValueT<FpctiMk_SPEC>;

impl FpctiMk {
    #[doc = "Timer Value of Filter and Prescaler Channel k   TIM. This bit bitfield shows the values of the timer of the Filter and Prescaler Channel. Writing to TIM will result in its content being set to its reset value."]
    #[inline(always)]
    pub fn tim(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, FpctiMk_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, FpctiMk_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for FpctiMk {
    #[inline(always)]
    fn default() -> FpctiMk {
        <crate::RegValueT<FpctiMk_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtmexr_SPEC;
impl crate::sealed::RegSpec for Gtmexr_SPEC {
    type DataType = u32;
}
#[doc = "IOM GTM Input EXOR Combiner Selection Register\n resetvalue={Application Reset:0x0}"]
pub type Gtmexr = crate::RegValueT<Gtmexr_SPEC>;

impl Gtmexr {
    #[doc = "GTM input 0 selection for EXOR combiner   EN0"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gtmexr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gtmexr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GTM input 1 selection for EXOR combiner   EN1"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gtmexr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gtmexr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GTM input 2 selection for EXOR combiner   EN2"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gtmexr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gtmexr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GTM input 3 selection for EXOR combiner   EN3"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gtmexr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gtmexr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GTM input 4 selection for EXOR combiner   EN4"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gtmexr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gtmexr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GTM input 5 selection for EXOR combiner   EN5"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gtmexr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gtmexr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GTM input 6 selection for EXOR combiner   EN6"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gtmexr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gtmexr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GTM input 7 selection for EXOR combiner   EN7"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gtmexr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gtmexr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Gtmexr {
    #[inline(always)]
    fn default() -> Gtmexr {
        <crate::RegValueT<Gtmexr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "IOM Identification Register\n resetvalue={Application Reset:0x0CCC002}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MOD REV. MOD REV defines the Module revision number."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MOD TYPE. This bit field defines the module as a 32 bit module  C0 H"]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MOD NUM. This bit field defines the identification number for the IOM."]
    #[inline(always)]
    pub fn mod_num(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Id_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Id {
    #[inline(always)]
    fn default() -> Id {
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(13418498)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krst0_SPEC;
impl crate::sealed::RegSpec for Krst0_SPEC {
    type DataType = u32;
}
#[doc = "IOM Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
pub type Krst0 = crate::RegValueT<Krst0_SPEC>;

impl Krst0 {
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel registers are set. The RST bit will be cleared  re set to   xb4 0  xb4   by the BPI FPI after the kernel reset was executed."]
    #[inline(always)]
    pub fn rst(self) -> crate::common::RegisterFieldBool<0, 1, 0, Krst0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Krst0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Kernel Reset Status   RSTSTAT. This bit indicates wether a kernel reset was executed or not. This bit is set by the BPI FPI after the execution of a kernel reset in the same clock cycle both reset bits. This bit can be cleared by writing with   xb4 1  xb4  to the CLR bit in the related KRSTCLR register."]
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
#[doc = "IOM Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
pub type Krst1 = crate::RegValueT<Krst1_SPEC>;

impl Krst1 {
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel reset registers is set. The RST bit will be cleared  re set to   xb4 0  xb4   by the BPI FPI after the kernel reset was executed."]
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
#[doc = "IOM Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
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
pub struct LamcfGm_SPEC;
impl crate::sealed::RegSpec for LamcfGm_SPEC {
    type DataType = u32;
}
#[doc = "IOM Logic Analyzer Module Configuration Register 0\n resetvalue={Application Reset:0x0}"]
pub type LamcfGm = crate::RegValueT<LamcfGm_SPEC>;

impl LamcfGm {
    #[doc = "Invert Reference LAM block m   IVR. This bit field determines whether the reference signal from the FPC  reference channel  is inverted or not."]
    #[inline(always)]
    pub fn ivr(self) -> crate::common::RegisterFieldBool<0, 1, 0, LamcfGm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, LamcfGm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Invert Monitor LAM block m   IVM. This bit field determines whether the monitor signal from the FPC  monitor channel  is inverted or not."]
    #[inline(always)]
    pub fn ivm(self) -> crate::common::RegisterFieldBool<1, 1, 0, LamcfGm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, LamcfGm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Monitor Source Select LAM block m   MOS. This bit field determines whether the monitor signal from the FPC  monitor channel  is sourced directly or compared  EXOR  x2019 d  with the reference signal from the FPC reference channel  for the event compare."]
    #[inline(always)]
    pub fn mos(self) -> crate::common::RegisterFieldBool<2, 1, 0, LamcfGm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, LamcfGm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Runmode Select LAM block m   RMS. This bit field determines whether the event window generation is free running or gated with the monitor or reference."]
    #[inline(always)]
    pub fn rms(self) -> crate::common::RegisterFieldBool<3, 1, 0, LamcfGm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, LamcfGm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Event Window Select LAM block m   EWS. This bit field determines whether the event window generation is from the monitor or reference signal."]
    #[inline(always)]
    pub fn ews(self) -> crate::common::RegisterFieldBool<4, 1, 0, LamcfGm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, LamcfGm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable Events LAM block m   DISEV. This bit field allows to suppress alarm outputs from LAM block m to the ECM. Except for sending alarms to the ECM  all other effects of an alarm condition being detected  for instance  setting LAMEWCm.CNT to the value of the counter  will still take place inside LAM block m."]
    #[inline(always)]
    pub fn disev(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, LamcfGm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, LamcfGm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Event Window Active Edge Selection LAM block m   EDS. This bit field determines which active edges of the monitor and reference signals are used for the event window generation."]
    #[inline(always)]
    pub fn eds(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, LamcfGm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, LamcfGm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Invert Event Window LAM block m   IVW. This bit field determines whether the event window polarity is inverted or not."]
    #[inline(always)]
    pub fn ivw(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, LamcfGm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, LamcfGm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Monitor Input Signal Selection LAM block m   MCS. This bit field determines which FPC mux block k monitor output signal is to be used for LAM block m."]
    #[inline(always)]
    pub fn mcs(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, LamcfGm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, LamcfGm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reference Input Signal Selection LAM block m   RCS. This bit field determines which FPC mux block k reference output signal is to be used for LAM block m."]
    #[inline(always)]
    pub fn rcs(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, LamcfGm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, LamcfGm_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for LamcfGm {
    #[inline(always)]
    fn default() -> LamcfGm {
        <crate::RegValueT<LamcfGm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LamewCm_SPEC;
impl crate::sealed::RegSpec for LamewCm_SPEC {
    type DataType = u32;
}
#[doc = "IOM Logic Analyzer Module Event Window Count Status Register 0\n resetvalue={Application Reset:0x0}"]
pub type LamewCm = crate::RegValueT<LamewCm_SPEC>;

impl LamewCm {
    #[doc = "Event Window Count Value LAM block m   CNT. The count value of the event window attained coincident with an event occurring."]
    #[inline(always)]
    pub fn cnt(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, LamewCm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffff,1,0,u32, LamewCm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Count Overflow Flag LAM block m   CNTO. This bit field provides the information wether the count has reached its maximum value."]
    #[inline(always)]
    pub fn cnto(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, LamewCm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, LamewCm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for LamewCm {
    #[inline(always)]
    fn default() -> LamewCm {
        <crate::RegValueT<LamewCm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LamewSm_SPEC;
impl crate::sealed::RegSpec for LamewSm_SPEC {
    type DataType = u32;
}
#[doc = "IOM Logic Analyzer Module Event Window Configuration Register 0\n resetvalue={Application Reset:0x0}"]
pub type LamewSm = crate::RegValueT<LamewSm_SPEC>;

impl LamewSm {
    #[doc = "Event Window Count Threshold   THR. This bit field contains the value of the counter at which the event window becomes active  before optional inversion ."]
    #[inline(always)]
    pub fn thr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, LamewSm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffff,1,0,u32, LamewSm_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for LamewSm {
    #[inline(always)]
    fn default() -> LamewSm {
        <crate::RegValueT<LamewSm_SPEC> as RegisterValue<_>>::new(0)
    }
}
