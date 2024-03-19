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
#[doc = r"EDSADC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edsadc(pub(super) *mut u8);
unsafe impl core::marker::Send for Edsadc {}
unsafe impl core::marker::Sync for Edsadc {}
impl Edsadc {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "Access Protection Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn accprot(&self) -> crate::common::Reg<self::Accprot_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }

    #[doc = "Carrier Generator Configuration Register\n resetvalue={Application Reset:0x7100000}"]
    #[inline(always)]
    pub const fn cgcfg(&self) -> crate::common::Reg<self::Cgcfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Event Flag Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn evflag(&self) -> crate::common::Reg<self::Evflag_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(224usize)) }
    }

    #[doc = "Event Flag Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn evflagclr(&self) -> crate::common::Reg<self::Evflagclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(228usize)) }
    }

    #[doc = "Global Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globcfg(&self) -> crate::common::Reg<self::Globcfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "Global Run Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globrc(&self) -> crate::common::Reg<self::Globrc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={PowerOn Reset:0x0C6C007}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }

    #[doc = "Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }

    #[doc = "OCDS Control and Status Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }
    #[doc = "CH"]
    #[inline(always)]
    pub fn ch(self) -> [self::Ch; 6] {
        unsafe {
            [
                self::Ch(self.0.add(0x100usize + 0x0usize)),
                self::Ch(self.0.add(0x100usize + 0x100usize)),
                self::Ch(self.0.add(0x100usize + 0x200usize)),
                self::Ch(self.0.add(0x100usize + 0x300usize)),
                self::Ch(self.0.add(0x100usize + 0x400usize)),
                self::Ch(self.0.add(0x100usize + 0x500usize)),
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
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
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
pub struct Accprot_SPEC;
impl crate::sealed::RegSpec for Accprot_SPEC {
    type DataType = u32;
}
#[doc = "Access Protection Register\n resetvalue={Application Reset:0x0}"]
pub type Accprot = crate::RegValueT<Accprot_SPEC>;

impl Accprot {
    #[doc = "Register Group 7   RG07"]
    #[inline(always)]
    pub fn rg00(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Accprot_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accprot_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Register Group 7   RG07"]
    #[inline(always)]
    pub fn rg01(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Accprot_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accprot_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Register Group 7   RG07"]
    #[inline(always)]
    pub fn rg02(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Accprot_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accprot_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Register Group 7   RG07"]
    #[inline(always)]
    pub fn rg03(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Accprot_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accprot_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Register Group 7   RG07"]
    #[inline(always)]
    pub fn rg04(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Accprot_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accprot_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Register Group 7   RG07"]
    #[inline(always)]
    pub fn rg05(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Accprot_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accprot_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Register Group 7   RG07"]
    #[inline(always)]
    pub fn rg06(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Accprot_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accprot_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Register Group 7   RG07"]
    #[inline(always)]
    pub fn rg07(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Accprot_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accprot_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Register Group 10   RG10"]
    #[inline(always)]
    pub fn rg10(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accprot_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accprot_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Register Group Global   RGG"]
    #[inline(always)]
    pub fn rgg(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Accprot_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Accprot_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Accprot {
    #[inline(always)]
    fn default() -> Accprot {
        <crate::RegValueT<Accprot_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cgcfg_SPEC;
impl crate::sealed::RegSpec for Cgcfg_SPEC {
    type DataType = u32;
}
#[doc = "Carrier Generator Configuration Register\n resetvalue={Application Reset:0x7100000}"]
pub type Cgcfg = crate::RegValueT<Cgcfg_SPEC>;

impl Cgcfg {
    #[doc = "Carrier Generator Operating Mode   CGMOD. Stopping the carrier generator  CGMOD  160    160  00          terminates the PWM output after completion of the current period         indicated by bit RUN  160    160 0 ."]
    #[inline(always)]
    pub fn cgmod(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Cgcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Cgcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bit Reverse PWM Generation   BREV"]
    #[inline(always)]
    pub fn brev(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cgcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cgcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Signal Polarity   SIGPOL"]
    #[inline(always)]
    pub fn sigpol(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Cgcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cgcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Divider Factor for the PWM Pattern Signal Generator   DIVCG. Defines the input frequency of the carrier signal generator  derived from the selected internal clock source  f CG   f ADC   CGP. The frequency of the carrier signal itself is f CG   1024."]
    #[inline(always)]
    pub fn divcg(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Cgcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Cgcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Run Indicator   RUN"]
    #[inline(always)]
    pub fn run(self) -> crate::common::RegisterFieldBool<15, 1, 0, Cgcfg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Cgcfg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Bit Counter   BITCOUNT. Counts the 32 cycles generated for each step"]
    #[inline(always)]
    pub fn bitcount(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Cgcfg_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Cgcfg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Step Counter   STEPCOUNT. Counts the 8 steps generated for each quadrant of the carrier signal        period"]
    #[inline(always)]
    pub fn stepcount(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Cgcfg_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Cgcfg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Step Counter Sign   STEPS. Indicates the sign of the step counter value"]
    #[inline(always)]
    pub fn steps(self) -> crate::common::RegisterFieldBool<28, 1, 0, Cgcfg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Cgcfg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Step Counter Direction   STEPD"]
    #[inline(always)]
    pub fn stepd(self) -> crate::common::RegisterFieldBool<29, 1, 0, Cgcfg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Cgcfg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Sign Signal from Carrier Generator   SGNCG"]
    #[inline(always)]
    pub fn sgncg(self) -> crate::common::RegisterFieldBool<30, 1, 0, Cgcfg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Cgcfg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Cgcfg {
    #[inline(always)]
    fn default() -> Cgcfg {
        <crate::RegValueT<Cgcfg_SPEC> as RegisterValue<_>>::new(118489088)
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
    #[doc = "Module Disable Status Bit   DISS"]
    #[inline(always)]
    pub fn diss(self) -> crate::common::RegisterFieldBool<1, 1, 0, Clc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Clc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Sleep Mode Enable Control   EDIS. Used to control the module  8217 s reaction to sleep mode."]
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
pub struct Evflag_SPEC;
impl crate::sealed::RegSpec for Evflag_SPEC {
    type DataType = u32;
}
#[doc = "Event Flag Register\n resetvalue={Application Reset:0x0}"]
pub type Evflag = crate::RegValueT<Evflag_SPEC>;

impl Evflag {
    #[doc = "Result Event   RESEV5. Bit RESEVx is cleared when result register RESMx is read  or when bit          RESECx is written with 1."]
    #[inline(always)]
    pub fn resev0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Evflag_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Evflag_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Result Event   RESEV5. Bit RESEVx is cleared when result register RESMx is read  or when bit          RESECx is written with 1."]
    #[inline(always)]
    pub fn resev1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Evflag_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Evflag_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Result Event   RESEV5. Bit RESEVx is cleared when result register RESMx is read  or when bit          RESECx is written with 1."]
    #[inline(always)]
    pub fn resev2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Evflag_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Evflag_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Result Event   RESEV5. Bit RESEVx is cleared when result register RESMx is read  or when bit          RESECx is written with 1."]
    #[inline(always)]
    pub fn resev3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Evflag_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Evflag_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Result Event   RESEV5. Bit RESEVx is cleared when result register RESMx is read  or when bit          RESECx is written with 1."]
    #[inline(always)]
    pub fn resev4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Evflag_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Evflag_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Result Event   RESEV5. Bit RESEVx is cleared when result register RESMx is read  or when bit          RESECx is written with 1."]
    #[inline(always)]
    pub fn resev5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Evflag_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Evflag_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm Event   ALEV5"]
    #[inline(always)]
    pub fn alev0(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Evflag_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Evflag_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm Event   ALEV5"]
    #[inline(always)]
    pub fn alev1(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Evflag_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Evflag_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm Event   ALEV5"]
    #[inline(always)]
    pub fn alev2(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Evflag_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Evflag_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm Event   ALEV5"]
    #[inline(always)]
    pub fn alev3(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Evflag_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Evflag_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm Event   ALEV5"]
    #[inline(always)]
    pub fn alev4(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Evflag_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Evflag_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm Event   ALEV5"]
    #[inline(always)]
    pub fn alev5(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Evflag_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Evflag_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Evflag {
    #[inline(always)]
    fn default() -> Evflag {
        <crate::RegValueT<Evflag_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evflagclr_SPEC;
impl crate::sealed::RegSpec for Evflagclr_SPEC {
    type DataType = u32;
}
#[doc = "Event Flag Clear Register\n resetvalue={Application Reset:0x0}"]
pub type Evflagclr = crate::RegValueT<Evflagclr_SPEC>;

impl Evflagclr {
    #[doc = "Result Event Clear   RESEC5"]
    #[inline(always)]
    pub fn resec0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Evflagclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Evflagclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Result Event Clear   RESEC5"]
    #[inline(always)]
    pub fn resec1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Evflagclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Evflagclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Result Event Clear   RESEC5"]
    #[inline(always)]
    pub fn resec2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Evflagclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Evflagclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Result Event Clear   RESEC5"]
    #[inline(always)]
    pub fn resec3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Evflagclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Evflagclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Result Event Clear   RESEC5"]
    #[inline(always)]
    pub fn resec4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Evflagclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Evflagclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Result Event Clear   RESEC5"]
    #[inline(always)]
    pub fn resec5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Evflagclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Evflagclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm Event Clear   ALEC5"]
    #[inline(always)]
    pub fn alec0(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Evflagclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16,1,0,Evflagclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Alarm Event Clear   ALEC5"]
    #[inline(always)]
    pub fn alec1(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Evflagclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17,1,0,Evflagclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Alarm Event Clear   ALEC5"]
    #[inline(always)]
    pub fn alec2(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Evflagclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18,1,0,Evflagclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Alarm Event Clear   ALEC5"]
    #[inline(always)]
    pub fn alec3(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Evflagclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19,1,0,Evflagclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Alarm Event Clear   ALEC5"]
    #[inline(always)]
    pub fn alec4(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Evflagclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20,1,0,Evflagclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Alarm Event Clear   ALEC5"]
    #[inline(always)]
    pub fn alec5(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Evflagclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21,1,0,Evflagclr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Evflagclr {
    #[inline(always)]
    fn default() -> Evflagclr {
        <crate::RegValueT<Evflagclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Globcfg_SPEC;
impl crate::sealed::RegSpec for Globcfg_SPEC {
    type DataType = u32;
}
#[doc = "Global Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Globcfg = crate::RegValueT<Globcfg_SPEC>;

impl Globcfg {
    #[doc = "Trimming Value for the Dithering Function   DITRIM. This trim value is used for all modulators of the device. Not listed combinations are reserved."]
    #[inline(always)]
    pub fn ditrim(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Globcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Globcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Unsynchronized Clock Generation   USC. Defines the way the modulator clock is generated."]
    #[inline(always)]
    pub fn usc(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Globcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Globcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Supply Voltage Level. Adjusts the analog circuitry to the supply voltage used in the        application system. Make sure to keep SUPLEV   00 B or 01 B in the case of a 5 V supply."]
    #[inline(always)]
    pub fn suplev(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, Globcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x3,1,0,u8, Globcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Write Control for Clock Parameters   CPWC"]
    #[inline(always)]
    pub fn cpwc(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Globcfg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Globcfg_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Supervision Channel Select. Defines the channel for which the supervision signal selected by SVSIG        is output. Not listed combinations are reserved."]
    #[inline(always)]
    pub fn svch(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Globcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Globcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Supervision Signal Select. Defines the supervision signal of the channel selected by SVCH to be        output."]
    #[inline(always)]
    pub fn svsig(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, Globcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, Globcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Write Control for Supervision Parameters"]
    #[inline(always)]
    pub fn svwc(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Globcfg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Globcfg_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Globcfg {
    #[inline(always)]
    fn default() -> Globcfg {
        <crate::RegValueT<Globcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Globrc_SPEC;
impl crate::sealed::RegSpec for Globrc_SPEC {
    type DataType = u32;
}
#[doc = "Global Run Control Register\n resetvalue={Application Reset:0x0}"]
pub type Globrc = crate::RegValueT<Globrc_SPEC>;

impl Globrc {
    #[doc = "Channel 5 Run Control   CH5RUN. Each bit  when set  enables the corresponding demodulator channel. When CHxRUN is set  all filter blocks are cleared and the FIFO is        flushed."]
    #[inline(always)]
    pub fn ch0run(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Globrc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Globrc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel 5 Run Control   CH5RUN. Each bit  when set  enables the corresponding demodulator channel. When CHxRUN is set  all filter blocks are cleared and the FIFO is        flushed."]
    #[inline(always)]
    pub fn ch1run(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Globrc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Globrc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel 5 Run Control   CH5RUN. Each bit  when set  enables the corresponding demodulator channel. When CHxRUN is set  all filter blocks are cleared and the FIFO is        flushed."]
    #[inline(always)]
    pub fn ch2run(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Globrc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Globrc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel 5 Run Control   CH5RUN. Each bit  when set  enables the corresponding demodulator channel. When CHxRUN is set  all filter blocks are cleared and the FIFO is        flushed."]
    #[inline(always)]
    pub fn ch3run(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Globrc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Globrc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel 5 Run Control   CH5RUN. Each bit  when set  enables the corresponding demodulator channel. When CHxRUN is set  all filter blocks are cleared and the FIFO is        flushed."]
    #[inline(always)]
    pub fn ch4run(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Globrc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Globrc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel 5 Run Control   CH5RUN. Each bit  when set  enables the corresponding demodulator channel. When CHxRUN is set  all filter blocks are cleared and the FIFO is        flushed."]
    #[inline(always)]
    pub fn ch5run(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Globrc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Globrc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Modulator 5 Run Control   M5RUN. Each bit  when set  enables the corresponding modulator."]
    #[inline(always)]
    pub fn m0run(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Globrc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Globrc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Modulator 5 Run Control   M5RUN. Each bit  when set  enables the corresponding modulator."]
    #[inline(always)]
    pub fn m1run(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Globrc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Globrc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Modulator 5 Run Control   M5RUN. Each bit  when set  enables the corresponding modulator."]
    #[inline(always)]
    pub fn m2run(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Globrc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Globrc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Modulator 5 Run Control   M5RUN. Each bit  when set  enables the corresponding modulator."]
    #[inline(always)]
    pub fn m3run(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Globrc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Globrc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Modulator 5 Run Control   M5RUN. Each bit  when set  enables the corresponding modulator."]
    #[inline(always)]
    pub fn m4run(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Globrc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Globrc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Modulator 5 Run Control   M5RUN. Each bit  when set  enables the corresponding modulator."]
    #[inline(always)]
    pub fn m5run(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Globrc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Globrc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Globrc {
    #[inline(always)]
    fn default() -> Globrc {
        <crate::RegValueT<Globrc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={PowerOn Reset:0x0C6C007}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision   MOD REV. Indicates the revision number of the implementation. This information        depends on the design step."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MOD TYPE. This internal marker is fixed to C0 ."]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number   MOD NUMBER. Indicates the module identification number   00C6   EDSADC"]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(13025287)
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
    #[doc = "Kernel Reset   RST. Request a kernel reset. The reset is executed if the reset bits of both        kernel reset registers are set. RST is cleared after the kernel reset was executed."]
    #[inline(always)]
    pub fn rst(self) -> crate::common::RegisterFieldBool<0, 1, 0, Krst0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Krst0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Kernel Reset Status   RSTSTAT. Indicates an executed kernel reset. RSTSTAT is set after the execution        of a kernel reset in the same clock cycle in which the reset bits are        cleared. Clear RSTSTAT by setting bit CLR in register KRSTCLR."]
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
    #[doc = "Kernel Reset   RST. Request a kernel reset. The reset is executed if the reset bits of both        kernel reset registers are set. RST is cleared after the kernel reset was executed."]
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
pub struct Ocs_SPEC;
impl crate::sealed::RegSpec for Ocs_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Control and Status Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
pub type Ocs = crate::RegValueT<Ocs_SPEC>;

impl Ocs {
    #[doc = "OCDS Suspend Control   SUS. Controls the sensitivity to the suspend signal coming from the OCDS        Trigger Switch  OTGS . In soft suspend mode  the respective channel is stopped after the next        result has been stored. For products with less channels the upper codes are reserved."]
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

#[doc = "CH"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch(pub(super) *mut u8);
unsafe impl core::marker::Send for Ch {}
unsafe impl core::marker::Sync for Ch {}
impl Ch {
    #[doc = "Boundary Select Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn boundselx(&self) -> crate::common::Reg<ch::BoundseLx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(120usize)) }
    }
    #[doc = "Carrier Generator Synchronization Reg. 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cgsyncx(&self) -> crate::common::Reg<ch::CgsynCx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }
    #[doc = "Demodulator Input Config. Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dicfgx(&self) -> crate::common::Reg<ch::DicfGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Auxiliary Filter Configuration Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcfgax(&self) -> crate::common::Reg<ch::FcfgAx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(112usize)) }
    }
    #[doc = "Filter Configuration Register 0  CIC Filter\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcfgcx(&self) -> crate::common::Reg<ch::FcfgCx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Filter Configuration Register 0  Main\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcfgmx(&self) -> crate::common::Reg<ch::FcfgMx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Filter Counter Register 0  CIC Filter\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcntcx(&self) -> crate::common::Reg<ch::FcntCx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Gain Calibration Register 0\n resetvalue={Application Reset:0x61A81000}"]
    #[inline(always)]
    pub const fn gaincalx(&self) -> crate::common::Reg<ch::GaincaLx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }
    #[doc = "Gain Correction Register 0\n resetvalue={Application Reset:0x1000}"]
    #[inline(always)]
    pub const fn gaincorrx(&self) -> crate::common::Reg<ch::GaincorRx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }
    #[doc = "Gain Control Register 0\n resetvalue={Application Reset:0x1000}"]
    #[inline(always)]
    pub const fn gainctrx(&self) -> crate::common::Reg<ch::GainctRx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }
    #[doc = "Intermediate Integration Value\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn iivalx(&self) -> crate::common::Reg<ch::IivaLx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Integrator Status Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn istatx(&self) -> crate::common::Reg<ch::IstaTx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }
    #[doc = "Integration Window Control Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn iwctrx(&self) -> crate::common::Reg<ch::IwctRx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "Modulator Configuration Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn modcfgx(&self) -> crate::common::Reg<ch::ModcfGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Offset Compensation Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn offcompx(&self) -> crate::common::Reg<ch::OffcomPx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }
    #[doc = "Overshoot Compensation Cfg. Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ovscfgx(&self) -> crate::common::Reg<ch::OvscfGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Rectification Configuration Register 0\n resetvalue={Application Reset:0x080000000}"]
    #[inline(always)]
    pub const fn rectcfgx(&self) -> crate::common::Reg<ch::RectcfGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(168usize)) }
    }
    #[doc = "Result Register 0 Auxiliary\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn resax(&self) -> crate::common::Reg<ch::ResAx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }
    #[doc = "Result Register 0 Main\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn resmx(&self) -> crate::common::Reg<ch::ResMx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }
    #[doc = "Result FIFO Control Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rfcx(&self) -> crate::common::Reg<ch::RfCx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }
    #[doc = "Time Stamp Counter 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tscntx(&self) -> crate::common::Reg<ch::TscnTx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(84usize)) }
    }
    #[doc = "Time Stamp Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tstmpx(&self) -> crate::common::Reg<ch::TstmPx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }
    #[doc = "Common Mode Voltage Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn vcmx(&self) -> crate::common::Reg<ch::VcMx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(176usize)) }
    }
}
pub mod ch {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BoundseLx_SPEC;
    impl crate::sealed::RegSpec for BoundseLx_SPEC {
        type DataType = u32;
    }
    #[doc = "Boundary Select Register 0\n resetvalue={Application Reset:0x0}"]
    pub type BoundseLx = crate::RegValueT<BoundseLx_SPEC>;

    impl BoundseLx {
        #[doc = "Lower Boundary Value for Limit Checking   BOUNDARYL. This  two  8217 s complement  value is compared to the upper bits of the CIC        filter results."]
        #[inline(always)]
        pub fn boundaryl(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, BoundseLx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, BoundseLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Upper Boundary Value for Limit Checking   BOUNDARYU. This  two  8217 s complement  value is compared to the upper bits of the CIC        filter results."]
        #[inline(always)]
        pub fn boundaryu(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, BoundseLx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, BoundseLx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for BoundseLx {
        #[inline(always)]
        fn default() -> BoundseLx {
            <crate::RegValueT<BoundseLx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CgsynCx_SPEC;
    impl crate::sealed::RegSpec for CgsynCx_SPEC {
        type DataType = u32;
    }
    #[doc = "Carrier Generator Synchronization Reg. 0\n resetvalue={Application Reset:0x0}"]
    pub type CgsynCx = crate::RegValueT<CgsynCx_SPEC>;

    impl CgsynCx {
        #[doc = "Sign Delay Counter   SDCOUNT. Counts the result values from the filter chain to delay the carrier sign        signal"]
        #[inline(always)]
        pub fn sdcount(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, CgsynCx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xff,1,0,u8, CgsynCx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Sign Delay Capture Value   SDCAP. Indicates the result values counted between the begin of the positive        halfwave of the carrier signal and the first received positive value."]
        #[inline(always)]
        pub fn sdcap(
            self,
        ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, CgsynCx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<8,0xff,1,0,u8, CgsynCx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Sign Delay Value for Positive Halfwave   SDPOS. Defines the content of SDCOUNT to generate a negative delayed sign        signal  SGND ."]
        #[inline(always)]
        pub fn sdpos(
            self,
        ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, CgsynCx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xff,1,0,u8, CgsynCx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Sign Delay Value for Negative Halfwave   SDNEG. Defines the content of SDCOUNT to generate a positive delayed sign        signal  SGND ."]
        #[inline(always)]
        pub fn sdneg(
            self,
        ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, CgsynCx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0xff,1,0,u8, CgsynCx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for CgsynCx {
        #[inline(always)]
        fn default() -> CgsynCx {
            <crate::RegValueT<CgsynCx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DicfGx_SPEC;
    impl crate::sealed::RegSpec for DicfGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Demodulator Input Config. Register 0\n resetvalue={Application Reset:0x0}"]
    pub type DicfGx = crate::RegValueT<DicfGx_SPEC>;

    impl DicfGx {
        #[doc = "Data Stream Select   DSS"]
        #[inline(always)]
        pub fn dss(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, DicfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,u8, DicfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Data Source for External Modulator   DSRCEX"]
        #[inline(always)]
        pub fn dsrcex(
            self,
        ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, DicfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x7,1,0,u8, DicfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Clock Source for External Modulator   CSRCEX"]
        #[inline(always)]
        pub fn csrcex(
            self,
        ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, DicfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x7,1,0,u8, DicfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Write Control for Data Stream Selection   DSWC"]
        #[inline(always)]
        pub fn dswc(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, DicfGx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<15,1,0,DicfGx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Trigger Select   TRSEL. Selects an input for the trigger signal used for the following features         see also CROSSREFERENCE    To avoid unintended triggers  select the        trigger source first before enabling the corresponding function. integrator control  timestamp  multiplexer control  modulator control         APC   service request gating. The product specific appendix details the connected trigger input        signals."]
        #[inline(always)]
        pub fn trsel(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, DicfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, DicfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Integrator Trigger Mode   ITRMODE. To ensure proper operation  ensure that bitfield ITRMODE is 00 before selecting any other trigger mode. The        integration trigger mode controls bit INTEN in register IWCTRx and hence the operation of the integrator  Bit INTEN is set when ITRMODE  160    160  11 or when the selected trigger signal transition occurs. Bit INTEN is cleared when ITRMODE  160    160  00          after REPVAL 1 integration cycles  IWS  160    160 0  or when the inverse trigger        signal transition occurs  IWS  160    160 1 ."]
        #[inline(always)]
        pub fn itrmode(
            self,
        ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, DicfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0x3,1,0,u8, DicfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Timestamp Trigger Mode   TSTRMODE. The timestamp trigger mode controls capturing the timestamp information        to register TSTMPx ."]
        #[inline(always)]
        pub fn tstrmode(
            self,
        ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, DicfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<22,0x3,1,0,u8, DicfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Data Read Mode   DRM. Selects the data that is returned when register RESMx is read  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn drm(
            self,
        ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, DicfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<26,0x3,1,0,u8, DicfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Time Stamp Mode   TSM. See CROSSREFERENCE ."]
        #[inline(always)]
        pub fn tsm(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, DicfGx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<28,1,0,DicfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Result Display Mode   RDM"]
        #[inline(always)]
        pub fn rdm(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, DicfGx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<29,1,0,DicfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Write Control for Mode Settings   MSWC"]
        #[inline(always)]
        pub fn mswc(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, DicfGx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<31,1,0,DicfGx_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for DicfGx {
        #[inline(always)]
        fn default() -> DicfGx {
            <crate::RegValueT<DicfGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FcfgAx_SPEC;
    impl crate::sealed::RegSpec for FcfgAx_SPEC {
        type DataType = u32;
    }
    #[doc = "Auxiliary Filter Configuration Register 0\n resetvalue={Application Reset:0x0}"]
    pub type FcfgAx = crate::RegValueT<FcfgAx_SPEC>;

    impl FcfgAx {
        #[doc = "CIC Filter  Auxiliary  Enable"]
        #[inline(always)]
        pub fn cfaen(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, FcfgAx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,FcfgAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "CIC Filter  Auxiliary  Decimation Factor"]
        #[inline(always)]
        pub fn cfadf(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, FcfgAx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,FcfgAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "CIC Filter  Auxiliary  Decimation Counter. The decimation counter counts the filter cycles until an output is        generated  i.e. the oversampling rate."]
        #[inline(always)]
        pub fn cfacnt(
            self,
        ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, FcfgAx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0x1f,1,0,u8, FcfgAx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for FcfgAx {
        #[inline(always)]
        fn default() -> FcfgAx {
            <crate::RegValueT<FcfgAx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FcfgCx_SPEC;
    impl crate::sealed::RegSpec for FcfgCx_SPEC {
        type DataType = u32;
    }
    #[doc = "Filter Configuration Register 0  CIC Filter\n resetvalue={Application Reset:0x0}"]
    pub type FcfgCx = crate::RegValueT<FcfgCx_SPEC>;

    impl FcfgCx {
        #[doc = "CIC Filter Decimation Factor   CFMDF. Defines the oversampling rate of the CIC filter  OSR   CFMDF  160    160 1. Valid values are 0 03 to 1FF  OSR   4 to 512 ."]
        #[inline(always)]
        pub fn cfmdf(
            self,
        ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, FcfgCx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1ff,1,0,u16, FcfgCx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "CIC Filter Start Value   CFMSV. The decimation counter begins counting at value CFMSV  when started or        restarted. Valid values are 003 to CFMDF  4 to        selected OSR . Start values above the selected        oversampling rate may lead to overflows in the CIC filter"]
        #[inline(always)]
        pub fn cfmsv(
            self,
        ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, FcfgCx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1ff,1,0,u16, FcfgCx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for FcfgCx {
        #[inline(always)]
        fn default() -> FcfgCx {
            <crate::RegValueT<FcfgCx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FcfgMx_SPEC;
    impl crate::sealed::RegSpec for FcfgMx_SPEC {
        type DataType = u32;
    }
    #[doc = "Filter Configuration Register 0  Main\n resetvalue={Application Reset:0x0}"]
    pub type FcfgMx = crate::RegValueT<FcfgMx_SPEC>;

    impl FcfgMx {
        #[doc = "FIR0 Filter Enable   FIR0EN"]
        #[inline(always)]
        pub fn fir0en(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, FcfgMx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,FcfgMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "FIR1 Filter Enable   FIR1EN"]
        #[inline(always)]
        pub fn fir1en(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, FcfgMx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,FcfgMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Overshoot Compensation Enable"]
        #[inline(always)]
        pub fn ovcen(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, FcfgMx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,FcfgMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "FIR1 Filter Decimation Rate   FIR1DEC"]
        #[inline(always)]
        pub fn fir1dec(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, FcfgMx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,FcfgMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "CIC Filter Mode   CICMOD"]
        #[inline(always)]
        pub fn cicmod(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, FcfgMx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,FcfgMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Prefilter Enable   PFEN"]
        #[inline(always)]
        pub fn pfen(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, FcfgMx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,FcfgMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Offset Compensation Filter Enable   OCEN"]
        #[inline(always)]
        pub fn ocen(
            self,
        ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, FcfgMx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x7,1,0,u8, FcfgMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Offset Protection   OFFPROT. Controls the influence of the calibration sequence on register OFFCOMP."]
        #[inline(always)]
        pub fn offprot(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, FcfgMx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<11,1,0,FcfgMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Write Control for Filter Modes   FMWC"]
        #[inline(always)]
        pub fn fmwc(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, FcfgMx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<15,1,0,FcfgMx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Service Request Generation for Main Service Request   SRGM"]
        #[inline(always)]
        pub fn srgm(
            self,
        ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, FcfgMx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x3,1,0,u8, FcfgMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Generation for Alternate Service Request   SRGA. SRGA  160    160  00 can be used for testing  If TEST.ICTEN  160    160  1            Reload request for test shift register"]
        #[inline(always)]
        pub fn srga(
            self,
        ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, FcfgMx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0x3,1,0,u8, FcfgMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Event Select   ESEL. Defines when a comparator event is generated."]
        #[inline(always)]
        pub fn esel(
            self,
        ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, FcfgMx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<22,0x3,1,0,u8, FcfgMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Event Gating. Defines if alarm events are coupled to the integration window."]
        #[inline(always)]
        pub fn egt(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, FcfgMx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<24,1,0,FcfgMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Calibration Trigger   CALIB"]
        #[inline(always)]
        pub fn calib(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, FcfgMx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<28,1,0,FcfgMx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Automatic Calibration Control   AUTOCAL"]
        #[inline(always)]
        pub fn autocal(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, FcfgMx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<29,1,0,FcfgMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Write Control for Calibration and Service Request Modes   CSRWC"]
        #[inline(always)]
        pub fn csrwc(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, FcfgMx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<31,1,0,FcfgMx_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for FcfgMx {
        #[inline(always)]
        fn default() -> FcfgMx {
            <crate::RegValueT<FcfgMx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FcntCx_SPEC;
    impl crate::sealed::RegSpec for FcntCx_SPEC {
        type DataType = u32;
    }
    #[doc = "Filter Counter Register 0  CIC Filter\n resetvalue={Application Reset:0x0}"]
    pub type FcntCx = crate::RegValueT<FcntCx_SPEC>;

    impl FcntCx {
        #[doc = "CIC Filter Decimation Counter   CFMDCNT. The decimation counter counts the filter cycles until an output is        generated  i.e. the oversampling rate. CFMDCNT counts down from the respective start value."]
        #[inline(always)]
        pub fn cfmdcnt(
            self,
        ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, FcntCx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0x1ff,1,0,u16, FcntCx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Calibration Status Flag   CAL. Bitfield CAL is set to 01 in the next          clock cycle after setting bit CALIB or after detecting the selected          trigger  if auto calibration is activated ."]
        #[inline(always)]
        pub fn cal(
            self,
        ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, FcntCx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<30,0x3,1,0,u8, FcntCx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for FcntCx {
        #[inline(always)]
        fn default() -> FcntCx {
            <crate::RegValueT<FcntCx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GaincaLx_SPEC;
    impl crate::sealed::RegSpec for GaincaLx_SPEC {
        type DataType = u32;
    }
    #[doc = "Gain Calibration Register 0\n resetvalue={Application Reset:0x61A81000}"]
    pub type GaincaLx = crate::RegValueT<GaincaLx_SPEC>;

    impl GaincaLx {
        #[doc = "Multiplication Factor for Gain Calibration   CALFACTOR. The resulting factor is   lt CALFACTOR gt    4  160 096  The initial value of 4  160 096   1000            corresponds to a factor of 1.000."]
        #[inline(always)]
        pub fn calfactor(
            self,
        ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, GaincaLx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1fff,1,0,u16, GaincaLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Target Value for Calibrated Fullscale   CALTARGET. Defines the target value for the calibration algorithm. The initial value of 25  160 000   61A8            corresponds to 0.2  160 mV per LSB."]
        #[inline(always)]
        pub fn caltarget(
            self,
        ) -> crate::common::RegisterField<16, 0x7fff, 1, 0, u16, GaincaLx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x7fff,1,0,u16, GaincaLx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GaincaLx {
        #[inline(always)]
        fn default() -> GaincaLx {
            <crate::RegValueT<GaincaLx_SPEC> as RegisterValue<_>>::new(1638404096)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GaincorRx_SPEC;
    impl crate::sealed::RegSpec for GaincorRx_SPEC {
        type DataType = u32;
    }
    #[doc = "Gain Correction Register 0\n resetvalue={Application Reset:0x1000}"]
    pub type GaincorRx = crate::RegValueT<GaincorRx_SPEC>;

    impl GaincorRx {
        #[doc = "Multiplication Factor for Gain Correction   GAINFACTOR. The resulting factor is   lt GAINFACTOR gt    4  160 096"]
        #[inline(always)]
        pub fn gainfactor(
            self,
        ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, GaincorRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1fff,1,0,u16, GaincorRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Position of the CIC Filter Output Shifter. Selects the valid outputs bits from the CIC filter  depending on the        chosen decimation factor  see formula above  1D   8230  1F are reserved."]
        #[inline(always)]
        pub fn cicshift(
            self,
        ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, GaincorRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1f,1,0,u8, GaincorRx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GaincorRx {
        #[inline(always)]
        fn default() -> GaincorRx {
            <crate::RegValueT<GaincorRx_SPEC> as RegisterValue<_>>::new(4096)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GainctRx_SPEC;
    impl crate::sealed::RegSpec for GainctRx_SPEC {
        type DataType = u32;
    }
    #[doc = "Gain Control Register 0\n resetvalue={Application Reset:0x1000}"]
    pub type GainctRx = crate::RegValueT<GainctRx_SPEC>;

    impl GainctRx {
        #[doc = "Multiplication Factor for Gain Correction During Calibration   GAINFACTOR. The resulting factor is   lt GAINFACTOR gt    4  160 096"]
        #[inline(always)]
        pub fn gainfactor(
            self,
        ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, GainctRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1fff,1,0,u16, GainctRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Position of the CIC Filter Output Shifter During Calibration   CICSHIFT. Selects the valid outputs bits from the CIC filter  depending on the chosen decimation factor  see data shifter formula   1D   x2026  1F are reserved."]
        #[inline(always)]
        pub fn cicshift(
            self,
        ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, GainctRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1f,1,0,u8, GainctRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Decimation Rate of the CIC Filter During Calibration   CICDEC. Factor   2    CICDEC   3"]
        #[inline(always)]
        pub fn cicdec(
            self,
        ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, GainctRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0x7,1,0,u8, GainctRx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GainctRx {
        #[inline(always)]
        fn default() -> GainctRx {
            <crate::RegValueT<GainctRx_SPEC> as RegisterValue<_>>::new(4096)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IivaLx_SPEC;
    impl crate::sealed::RegSpec for IivaLx_SPEC {
        type DataType = u32;
    }
    #[doc = "Intermediate Integration Value\n resetvalue={Application Reset:0x0}"]
    pub type IivaLx = crate::RegValueT<IivaLx_SPEC>;

    impl IivaLx {
        #[doc = "Result of most recent accumulation   IVAL"]
        #[inline(always)]
        pub fn ival(
            self,
        ) -> crate::common::RegisterField<0, 0x3ffffff, 1, 0, u32, IivaLx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0x3ffffff,1,0,u32, IivaLx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for IivaLx {
        #[inline(always)]
        fn default() -> IivaLx {
            <crate::RegValueT<IivaLx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IstaTx_SPEC;
    impl crate::sealed::RegSpec for IstaTx_SPEC {
        type DataType = u32;
    }
    #[doc = "Integrator Status Register 0\n resetvalue={Application Reset:0x0}"]
    pub type IstaTx = crate::RegValueT<IstaTx_SPEC>;

    impl IstaTx {
        #[doc = "Number of Values Counted   NVALCNT. Counts the number of integrated values until integration is started         NVALDIS  or completed  NVALINT"]
        #[inline(always)]
        pub fn nvalcnt(
            self,
        ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, IstaTx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0x3f,1,0,u8, IstaTx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Integration Cycle Counter   REPCNT. Counts the number of integration cycles if activated  IWS  160    160 0 . This number is selected via bitfield REPVAL."]
        #[inline(always)]
        pub fn repcnt(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, IstaTx_SPEC, crate::common::R> {
            crate::common::RegisterField::<8,0xf,1,0,u8, IstaTx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Integration Enable   INTEN. Indicates the activity of the integrator. For        the control of bit INTEN  see also bitfield ITRMODE in register DICFGx ."]
        #[inline(always)]
        pub fn inten(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, IstaTx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<15,1,0,IstaTx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for IstaTx {
        #[inline(always)]
        fn default() -> IstaTx {
            <crate::RegValueT<IstaTx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IwctRx_SPEC;
    impl crate::sealed::RegSpec for IwctRx_SPEC {
        type DataType = u32;
    }
    #[doc = "Integration Window Control Register 0\n resetvalue={Application Reset:0x0}"]
    pub type IwctRx = crate::RegValueT<IwctRx_SPEC>;

    impl IwctRx {
        #[doc = "Integrator Shift Control   ISC. Controls the data shifter after the integrator that selects the portion        of the integrator data for the result register. 110   8230  111 are reserved. ISC selects the respective bits in register IIVAL. The lowest selected          bit is used for rounding and is then removed."]
        #[inline(always)]
        pub fn isc(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, IwctRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,u8, IwctRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Integration Window Size   IWS"]
        #[inline(always)]
        pub fn iws(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, IwctRx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,IwctRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Filter Chain Restart Control"]
        #[inline(always)]
        pub fn frc(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, IwctRx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,IwctRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Number of Integration Cycles   REPVAL. Defines the number of integration cycles to be counted by REPCNT if        activated  IWS  160    160 0 . The number of cycles is REPVAL 1."]
        #[inline(always)]
        pub fn repval(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, IwctRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xf,1,0,u8, IwctRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Number of Values Discarded   NVALDIS. Start the integration cycle after NVALDIS values"]
        #[inline(always)]
        pub fn nvaldis(
            self,
        ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, IwctRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x3f,1,0,u8, IwctRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Number of Values to be Accumulated   NVALINT. Stop the integration cycle after NVALINT 1 values Use intervals of 2 minimum  so no data is lost due to the data shifter."]
        #[inline(always)]
        pub fn nvalint(
            self,
        ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, IwctRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0x3f,1,0,u8, IwctRx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for IwctRx {
        #[inline(always)]
        fn default() -> IwctRx {
            <crate::RegValueT<IwctRx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModcfGx_SPEC;
    impl crate::sealed::RegSpec for ModcfGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Modulator Configuration Register 0\n resetvalue={Application Reset:0x0}"]
    pub type ModcfGx = crate::RegValueT<ModcfGx_SPEC>;

    impl ModcfGx {
        #[doc = "Configuration of Positive Input Line   INCFGP. Defines the internal connection of the positive input."]
        #[inline(always)]
        pub fn incfgp(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, ModcfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3,1,0,u8, ModcfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration of Negative Input Line   INCFGN. Defines the internal connection of the negative input."]
        #[inline(always)]
        pub fn incfgn(
            self,
        ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, ModcfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x3,1,0,u8, ModcfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Gain Select of Analog Input Path   GAINSEL. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn gainsel(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, ModcfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0xf,1,0,u8, ModcfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Input Pin Selection   INSEL. Defines the initial or permanent setting for the input multiplexer         bitfield INMUX  depending on the selected operating mode  bitfield        INMODE ."]
        #[inline(always)]
        pub fn insel(
            self,
        ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, ModcfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x3,1,0,u8, ModcfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Input Multiplexer Setting   INMUX. Indicates the current setting of the input multiplexer connecting the        input pins to the buffer inputs. The product specific appendix details the available channels and their        inputs."]
        #[inline(always)]
        pub fn inmux(
            self,
        ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, ModcfGx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<10,0x3,1,0,u8, ModcfGx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Input Multiplexer Control Mode   INMODE. Defines the condition for a trigger event to control the input        multiplexer. Bitfield INMAC selects the action upon a trigger event."]
        #[inline(always)]
        pub fn inmode(
            self,
        ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, ModcfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0x3,1,0,u8, ModcfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Input Multiplexer Action Control   INMAC. Defines the mechanism by which the input multiplexer is controlled."]
        #[inline(always)]
        pub fn inmac(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, ModcfGx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,ModcfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Write Control for Input Parameters   INCWC"]
        #[inline(always)]
        pub fn incwc(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, ModcfGx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<15,1,0,ModcfGx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Modulator Clock Period   DIVM. Defines the period of the modulator clock  on chip external   derived from the peripheral clock  t MOD   t ADC   xd7  CP   f MOD   f ADC   CP   with CP   4   DIVM   xd7  2."]
        #[inline(always)]
        pub fn divm(
            self,
        ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, ModcfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x7,1,0,u8, ModcfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Analog Clock Synchronization Delay   ACSD. Defines the delay in clocks after the sync signal. Valid only if the phase synchronizer is selected  USC  160    160  0  ."]
        #[inline(always)]
        pub fn acsd(
            self,
        ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, ModcfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0x7,1,0,u8, ModcfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Dithering Function Enable   DITHEN. Controls the dithering function for each modulator separately."]
        #[inline(always)]
        pub fn dithen(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, ModcfGx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<26,1,0,ModcfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Integrator Reset Enable   IREN. Controls the modulator overdrive handling"]
        #[inline(always)]
        pub fn iren(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, ModcfGx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<27,1,0,ModcfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Automatic Power Control   APC"]
        #[inline(always)]
        pub fn apc(
            self,
        ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, ModcfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0x3,1,0,u8, ModcfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Write Control for Modulator Mode Settings   MMWC"]
        #[inline(always)]
        pub fn mmwc(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, ModcfGx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<31,1,0,ModcfGx_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for ModcfGx {
        #[inline(always)]
        fn default() -> ModcfGx {
            <crate::RegValueT<ModcfGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OffcomPx_SPEC;
    impl crate::sealed::RegSpec for OffcomPx_SPEC {
        type DataType = u32;
    }
    #[doc = "Offset Compensation Register 0\n resetvalue={Application Reset:0x0}"]
    pub type OffcomPx = crate::RegValueT<OffcomPx_SPEC>;

    impl OffcomPx {
        #[doc = "Offset Value   OFFSET. Half of this signed value is subtracted from each result produced by the        filter chain. Bit 0 represents 1 2 LSB. This increases the precision in case of          accumulated result values  e.g. in the integrator."]
        #[inline(always)]
        pub fn offset(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, OffcomPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, OffcomPx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for OffcomPx {
        #[inline(always)]
        fn default() -> OffcomPx {
            <crate::RegValueT<OffcomPx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OvscfGx_SPEC;
    impl crate::sealed::RegSpec for OvscfGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Overshoot Compensation Cfg. Register 0\n resetvalue={Application Reset:0x0}"]
    pub type OvscfGx = crate::RegValueT<OvscfGx_SPEC>;

    impl OvscfGx {
        #[doc = "Slew Rate Filter Strength. Defines the time constant for the slew rate filter."]
        #[inline(always)]
        pub fn srfs(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, OvscfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3,1,0,u8, OvscfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Slew Rate Filter Run Time. Defines the time constant for the slew rate filter."]
        #[inline(always)]
        pub fn srfrt(
            self,
        ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, OvscfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x3,1,0,u8, OvscfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Step Detection Mode. Defines when the slew rate filter is activated."]
        #[inline(always)]
        pub fn sdm(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, OvscfGx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,OvscfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Step Detection Threshold. Defines the threshold value  magnitude  used for step detection. The        threshold value is  lt SDTH gt    215  32"]
        #[inline(always)]
        pub fn sdth(
            self,
        ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, OvscfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x7ff,1,0,u16, OvscfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for OvscfGx {
        #[inline(always)]
        fn default() -> OvscfGx {
            <crate::RegValueT<OvscfGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RectcfGx_SPEC;
    impl crate::sealed::RegSpec for RectcfGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Rectification Configuration Register 0\n resetvalue={Application Reset:0x080000000}"]
    pub type RectcfGx = crate::RegValueT<RectcfGx_SPEC>;

    impl RectcfGx {
        #[doc = "Rectification Enable   RFEN. General control of the rectifier circuit. Rectification is only active while the integrator is active."]
        #[inline(always)]
        pub fn rfen(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, RectcfGx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,RectcfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Sign Source. Selects the sign signal that is to be delayed."]
        #[inline(always)]
        pub fn ssrc(
            self,
        ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, RectcfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x3,1,0,u8, RectcfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Sign Source Channel. Selects the channel providing the sign signal if SSRC   01 . Other products of the family may have less channels and  consequently         less valid SSCH codes. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn ssch(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, RectcfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xf,1,0,u8, RectcfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag   SDCVAL. Indicates a new value in bitfield SDCAP."]
        #[inline(always)]
        pub fn sdcval(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, RectcfGx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<15,1,0,RectcfGx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Selected Carrier Sign Signal   SGNCS"]
        #[inline(always)]
        pub fn sgncs(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, RectcfGx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<30,1,0,RectcfGx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Sign Signal Delayed   SGND"]
        #[inline(always)]
        pub fn sgnd(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, RectcfGx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<31,1,0,RectcfGx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for RectcfGx {
        #[inline(always)]
        fn default() -> RectcfGx {
            <crate::RegValueT<RectcfGx_SPEC> as RegisterValue<_>>::new(2147483648)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ResAx_SPEC;
    impl crate::sealed::RegSpec for ResAx_SPEC {
        type DataType = u32;
    }
    #[doc = "Result Register 0 Auxiliary\n resetvalue={Application Reset:0x0}"]
    pub type ResAx = crate::RegValueT<ResAx_SPEC>;

    impl ResAx {
        #[doc = "Most Recent Result of Auxiliary Filter"]
        #[inline(always)]
        pub fn result(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, ResAx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, ResAx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for ResAx {
        #[inline(always)]
        fn default() -> ResAx {
            <crate::RegValueT<ResAx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ResMx_SPEC;
    impl crate::sealed::RegSpec for ResMx_SPEC {
        type DataType = u32;
    }
    #[doc = "Result Register 0 Main\n resetvalue={Application Reset:0x0}"]
    pub type ResMx = crate::RegValueT<ResMx_SPEC>;

    impl ResMx {
        #[doc = "Result Value Lower Part   RESULTLO. Returns the next value from the result FIFO  Result or timestamp  see CROSSREFERENCE"]
        #[inline(always)]
        pub fn resultlo(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, ResMx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, ResMx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Result Value Higher Part   RESULTHI. Returns an additional value  Sign extension  result from FIFO  timestamp  or zero  see CROSSREFERENCE"]
        #[inline(always)]
        pub fn resulthi(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, ResMx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, ResMx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for ResMx {
        #[inline(always)]
        fn default() -> ResMx {
            <crate::RegValueT<ResMx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RfCx_SPEC;
    impl crate::sealed::RegSpec for RfCx_SPEC {
        type DataType = u32;
    }
    #[doc = "Result FIFO Control Register 0\n resetvalue={Application Reset:0x0}"]
    pub type RfCx = crate::RegValueT<RfCx_SPEC>;

    impl RfCx {
        #[doc = "Service Request FIFO Level"]
        #[inline(always)]
        pub fn srlvl(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, RfCx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<0,0x3,1,0,u8, RfCx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Read Error Flag Clear"]
        #[inline(always)]
        pub fn rdec(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, RfCx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<4, 1, 0, RfCx_SPEC, crate::common::W>::from_register(
                self, 0,
            )
        }
        #[doc = "Write Error Flag Clear"]
        #[inline(always)]
        pub fn wrec(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, RfCx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<5, 1, 0, RfCx_SPEC, crate::common::W>::from_register(
                self, 0,
            )
        }
        #[doc = "FIFO Flush"]
        #[inline(always)]
        pub fn fifl(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, RfCx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<6, 1, 0, RfCx_SPEC, crate::common::W>::from_register(
                self, 0,
            )
        }
        #[doc = "FIFO Fill Level. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn fill(
            self,
        ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, RfCx_SPEC, crate::common::R> {
            crate::common::RegisterField::<16,0x7,1,0,u8, RfCx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Read Error Flag"]
        #[inline(always)]
        pub fn rderr(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, RfCx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<20, 1, 0, RfCx_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Write Error Flag"]
        #[inline(always)]
        pub fn wrerr(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, RfCx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<21, 1, 0, RfCx_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
    }
    impl core::default::Default for RfCx {
        #[inline(always)]
        fn default() -> RfCx {
            <crate::RegValueT<RfCx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TscnTx_SPEC;
    impl crate::sealed::RegSpec for TscnTx_SPEC {
        type DataType = u32;
    }
    #[doc = "Time Stamp Counter 0\n resetvalue={Application Reset:0x0}"]
    pub type TscnTx = crate::RegValueT<TscnTx_SPEC>;

    impl TscnTx {
        #[doc = "Timestamp Counter Value   TSCOUNT. TSCOUNT is clocked with the modulator clock and is cleared when a new result value has been generated."]
        #[inline(always)]
        pub fn tscount(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, TscnTx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, TscnTx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Timestamp Counter Clock Selection   TSCLK"]
        #[inline(always)]
        pub fn tsclk(
            self,
        ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, TscnTx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x3,1,0,u8, TscnTx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Timestamp Counter Run Control   TSCRUN"]
        #[inline(always)]
        pub fn tscrun(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, TscnTx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<19,1,0,TscnTx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Analog MUX Setting Copy Enable   AMXCOPY. Allows copying of bitfield AMX into bitfield TIMESTAMP  in register TSTMPx  ."]
        #[inline(always)]
        pub fn amxcopy(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, TscnTx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<20,1,0,TscnTx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for TscnTx {
        #[inline(always)]
        fn default() -> TscnTx {
            <crate::RegValueT<TscnTx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TstmPx_SPEC;
    impl crate::sealed::RegSpec for TstmPx_SPEC {
        type DataType = u32;
    }
    #[doc = "Time Stamp Register 0\n resetvalue={Application Reset:0x0}"]
    pub type TstmPx = crate::RegValueT<TstmPx_SPEC>;

    impl TstmPx {
        #[doc = "The Most Recent Captured Timestamp Value   TIMESTAMP. This value is copied from the timestamp counter TSCOUNT If bit AMXCOPY in register TSCNTx is 1  bits TIMESTAMP 15 14  are replaced with a copy of bitfield AMX."]
        #[inline(always)]
        pub fn timestamp(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, TstmPx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, TstmPx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Analog Multiplexer Setting   AMX. This value is copied from bitfield INMUX in register MODCFGx"]
        #[inline(always)]
        pub fn amx(
            self,
        ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, TstmPx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0x3,1,0,u8, TstmPx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Timestamp Valid   TSVAL. Indicates valid timestamp information."]
        #[inline(always)]
        pub fn tsval(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, TstmPx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<31,1,0,TstmPx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for TstmPx {
        #[inline(always)]
        fn default() -> TstmPx {
            <crate::RegValueT<TstmPx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VcMx_SPEC;
    impl crate::sealed::RegSpec for VcMx_SPEC {
        type DataType = u32;
    }
    #[doc = "Common Mode Voltage Register 0\n resetvalue={Application Reset:0x0}"]
    pub type VcMx = crate::RegValueT<VcMx_SPEC>;

    impl VcMx {
        #[doc = "Fractional Reference Voltage Selection   VREFXSEL"]
        #[inline(always)]
        pub fn vrefxsel(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, VcMx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<0,0x3,1,0,u8, VcMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Fractional Reference Voltage Enable   VXON"]
        #[inline(always)]
        pub fn vxon(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, VcMx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2, 1, 0, VcMx_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Voltage Control of Positive Inputs 3 of CH0. Defines the connection of the respective positive input y to the common        mode voltage. y indicates the input of the analog multiplexers  if available ."]
        #[inline(always)]
        pub fn inpvc0(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, VcMx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<16,1,0,VcMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Voltage Control of Positive Inputs 3 of CH0. Defines the connection of the respective positive input y to the common        mode voltage. y indicates the input of the analog multiplexers  if available ."]
        #[inline(always)]
        pub fn inpvc1(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, VcMx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<17,1,0,VcMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Voltage Control of Positive Inputs 3 of CH0. Defines the connection of the respective positive input y to the common        mode voltage. y indicates the input of the analog multiplexers  if available ."]
        #[inline(always)]
        pub fn inpvc2(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, VcMx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<18,1,0,VcMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Voltage Control of Positive Inputs 3 of CH0. Defines the connection of the respective positive input y to the common        mode voltage. y indicates the input of the analog multiplexers  if available ."]
        #[inline(always)]
        pub fn inpvc3(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, VcMx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<19,1,0,VcMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Voltage Control of Negative Inputs 3 of CH0. Defines the connection of the respective negative input y to the common        mode voltage. y indicates the input of the analog multiplexers  if available ."]
        #[inline(always)]
        pub fn innvc0(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, VcMx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<20,1,0,VcMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Voltage Control of Negative Inputs 3 of CH0. Defines the connection of the respective negative input y to the common        mode voltage. y indicates the input of the analog multiplexers  if available ."]
        #[inline(always)]
        pub fn innvc1(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, VcMx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<21,1,0,VcMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Voltage Control of Negative Inputs 3 of CH0. Defines the connection of the respective negative input y to the common        mode voltage. y indicates the input of the analog multiplexers  if available ."]
        #[inline(always)]
        pub fn innvc2(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, VcMx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<22,1,0,VcMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Voltage Control of Negative Inputs 3 of CH0. Defines the connection of the respective negative input y to the common        mode voltage. y indicates the input of the analog multiplexers  if available ."]
        #[inline(always)]
        pub fn innvc3(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, VcMx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<23,1,0,VcMx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for VcMx {
        #[inline(always)]
        fn default() -> VcMx {
            <crate::RegValueT<VcMx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
