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
#[doc = r"CAN"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can0(pub(super) *mut u8);
unsafe impl core::marker::Send for Can0 {}
unsafe impl core::marker::Sync for Can0 {}
impl Can0 {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(33020usize)) }
    }

    #[doc = "Access Enable Register Control 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accenctr0(&self) -> crate::common::Reg<self::Accenctr0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32988usize)) }
    }

    #[doc = "Buffer receive address and transmit address\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn bufadr(&self) -> crate::common::Reg<self::Bufadr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32820usize)) }
    }

    #[doc = "CAN Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32768usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0B8C000}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32776usize)) }
    }

    #[doc = "Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(33012usize)) }
    }

    #[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(33008usize)) }
    }

    #[doc = "Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(33004usize)) }
    }

    #[doc = "Module Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mcr(&self) -> crate::common::Reg<self::Mcr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32816usize)) }
    }

    #[doc = "Measure Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mecr(&self) -> crate::common::Reg<self::Mecr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32832usize)) }
    }

    #[doc = "Measure Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mestat(&self) -> crate::common::Reg<self::Mestat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32836usize)) }
    }

    #[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(33000usize)) }
    }
    #[doc = "N"]
    #[inline(always)]
    pub fn n(self) -> [self::N; 4] {
        unsafe {
            [
                self::N(self.0.add(0x8100usize + 0x0usize)),
                self::N(self.0.add(0x8100usize + 0x400usize)),
                self::N(self.0.add(0x8100usize + 0x800usize)),
                self::N(self.0.add(0x8100usize + 0xc00usize)),
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
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
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
pub struct Accenctr0_SPEC;
impl crate::sealed::RegSpec for Accenctr0_SPEC {
    type DataType = u32;
}
#[doc = "Access Enable Register Control 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accenctr0 = crate::RegValueT<Accenctr0_SPEC>;

impl Accenctr0 {
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en9(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Accenctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,Accenctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Accenctr0 {
    #[inline(always)]
    fn default() -> Accenctr0 {
        <crate::RegValueT<Accenctr0_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bufadr_SPEC;
impl crate::sealed::RegSpec for Bufadr_SPEC {
    type DataType = u32;
}
#[doc = "Buffer receive address and transmit address\n resetvalue={Application Reset:0x0}"]
pub type Bufadr = crate::RegValueT<Bufadr_SPEC>;

impl Bufadr {
    #[doc = "Transmit Buffer start address   TXBUF. This is the start address of the first dedicated transmit buffer."]
    #[inline(always)]
    pub fn txbuf(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Bufadr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Bufadr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Buffer start address   RXBUF. This is the start address of the first dedicated receive buffer."]
    #[inline(always)]
    pub fn rxbuf(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, Bufadr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3fff,1,0,u16, Bufadr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Bufadr {
    #[inline(always)]
    fn default() -> Bufadr {
        <crate::RegValueT<Bufadr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clc_SPEC;
impl crate::sealed::RegSpec for Clc_SPEC {
    type DataType = u32;
}
#[doc = "CAN Clock Control Register\n resetvalue={Application Reset:0x3}"]
pub type Clc = crate::RegValueT<Clc_SPEC>;

impl Clc {
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control of the module. The synchronous and        asynchronous clock is switched on off Note that no register access is        possible to any register while module is disabled. A disable request is        granted  if the M CAN clock is disabled  or all M CAN nodes acknowledge the disable request."]
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
    #[doc = "Sleep Mode Disable Control   EDIS. Used to control module s sleep mode."]
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
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0B8C000}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MOD REV. MOD REV defines the revision number. The value of a module revision        starts with 01 H  first revision ."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MOD TYPE"]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MOD NUMBER. This bit field defines the MCMCAN module identification number 00B8 ."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(12107776)
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel        reset will be executed if the reset bits of both kernel registers are        set. The RST bit will be cleared  reset to   180 0  180   by the BPI FPI after the        kernel reset was executed."]
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel reset registers is set. The RST bit will be cleared  reset to  0   by the BPI FPI after the kernel reset was executed."]
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
pub struct Mcr_SPEC;
impl crate::sealed::RegSpec for Mcr_SPEC {
    type DataType = u32;
}
#[doc = "Module Control Register\n resetvalue={Application Reset:0x0}"]
pub type Mcr = crate::RegValueT<Mcr_SPEC>;

impl Mcr {
    #[doc = "Clock Select 0   CLKSEL0. This bitfield is MCR.CI and MCR.CCCE protected."]
    #[inline(always)]
    pub fn clksel0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Mcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0x3, 1, 0, u8, Mcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clock Select 1   CLKSEL1. This bitfield is MCR.CI and MCR.CCCE protected."]
    #[inline(always)]
    pub fn clksel1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Mcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2, 0x3, 1, 0, u8, Mcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clock Select 2   CLKSEL2. This bitfield is MCR.CI and MCR.CCCE protected."]
    #[inline(always)]
    pub fn clksel2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Mcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4, 0x3, 1, 0, u8, Mcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clock Select 3   CLKSEL3. This bitfield is MCR.CI and MCR.CCCE protected."]
    #[inline(always)]
    pub fn clksel3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Mcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6, 0x3, 1, 0, u8, Mcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Node   NODE. This bit field determines the CAN node i which is used for debug over        CAN. This bitfield only exists on CAN0."]
    #[inline(always)]
    pub fn node(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Mcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Mcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Debug Over CAN Messages Enable   DXCM. This bit enables the debug over serial connections between DAP and CAN0        module. If enabled the lowest receive transmit message buffer is reserved for        debugger communication. DXCM is described in detail in the OCDS chapter.        This bit only exists on CAN0."]
    #[inline(always)]
    pub fn dxcm(self) -> crate::common::RegisterFieldBool<27, 1, 0, Mcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Mcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "RAM BUSY   RBUSY. This bit is not implemented in TC39x A step silicon. This bit shows that the RAM Initialization is running. This bit is set back to 0b by hardware when the RAM intialization is completed."]
    #[inline(always)]
    pub fn rbusy(self) -> crate::common::RegisterFieldBool<28, 1, 0, Mcr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Mcr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RAM Init   RINIT. This bit is not implemented in TC39x A step silicon. This bit is MCR.CI and MCR.CCCE protected. This bit starts the initialization of the RAM block to all 0x0. The RAM initialization is started only when this bit is changed from 0b to 1b and also RBUSY is 0b."]
    #[inline(always)]
    pub fn rinit(self) -> crate::common::RegisterFieldBool<29, 1, 0, Mcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Mcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Change Init   CI. Needs to be set to enable and disable clocks."]
    #[inline(always)]
    pub fn ci(self) -> crate::common::RegisterFieldBool<30, 1, 0, Mcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Mcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clock and RAM Change Enable   CCCE. Needs to be set to enable and disable the clocks."]
    #[inline(always)]
    pub fn ccce(self) -> crate::common::RegisterFieldBool<31, 1, 0, Mcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Mcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mcr {
    #[inline(always)]
    fn default() -> Mcr {
        <crate::RegValueT<Mcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mecr_SPEC;
impl crate::sealed::RegSpec for Mecr_SPEC {
    type DataType = u32;
}
#[doc = "Measure Control Register\n resetvalue={Application Reset:0x0}"]
pub type Mecr = crate::RegValueT<Mecr_SPEC>;

impl Mecr {
    #[doc = "Threshold   TH. This bit field contains the threshold value for the measurement timer.        If TH   0000   the timer is stopped and        the capture function is disabled."]
    #[inline(always)]
    pub fn th(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Mecr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Mecr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer   INP. INP selects the interrupt output line INT Om  m   0 15  for a capture        event interrupt."]
    #[inline(always)]
    pub fn inp(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Mecr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Mecr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Node   NODE. This bit field determines the CAN node i whose input line RXDCANi is        used for start and capture of the measurement timer."]
    #[inline(always)]
    pub fn node(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, Mecr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x7,1,0,u8, Mecr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Any Edge   ANYED. This bit enables capture on any edge of CAN input line specified by NODE."]
    #[inline(always)]
    pub fn anyed(self) -> crate::common::RegisterFieldBool<24, 1, 0, Mecr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Mecr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture Event Interrupt Enable   CAPEIE. This bit enables the capture event interrupt. Bit field INP selects the interrupt output line which becomes activated at this type of interrupt."]
    #[inline(always)]
    pub fn capeie(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Mecr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Mecr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Digital Glitch Filter Depth   DEPTH. DEPTH determines the number of input samples clocked with f SYNi that are taken into account for the calculation of the floating average.        The higher DEPTH is chosen to be  the longer the glitches that are        suppressed and the longer the delay of the input signal introduced by        this filter."]
    #[inline(always)]
    pub fn depth(
        self,
    ) -> crate::common::RegisterField<27, 0x7, 1, 0, u8, Mecr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x7,1,0,u8, Mecr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Start Of Frame   SOF. This bit selects falling edge or any edge as measurement for start of        frame detection."]
    #[inline(always)]
    pub fn sof(self) -> crate::common::RegisterFieldBool<30, 1, 0, Mecr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Mecr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mecr {
    #[inline(always)]
    fn default() -> Mecr {
        <crate::RegValueT<Mecr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mestat_SPEC;
impl crate::sealed::RegSpec for Mestat_SPEC {
    type DataType = u32;
}
#[doc = "Measure Status Register\n resetvalue={Application Reset:0x0}"]
pub type Mestat = crate::RegValueT<Mestat_SPEC>;

impl Mestat {
    #[doc = "Captured Timer   CAPT. This bit field contains the captured measurement timer content. The timer itself is cleared and started by the first falling  dominant         edge of a CAN frame on the input line of the CAN node specified by        MECR.NODE. The timer is incremented by the module control clock f SYNi and will be stopped when FFFF is        reached. If MECR.TH   0000   the timer is        always stopped. A capture will take place if all the following conditions are met  MECR.TH  gt  0000 Timer is cleared and started by new frame Timer reaches MECR.TH This node is not sending and first edge  as specified by MECR.ANYED           after 3. occurs on input line Capture will be repeated for the following CAN frames until MECR.TH is        cleared."]
    #[inline(always)]
    pub fn capt(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Mestat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Mestat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Captured Rising Edge   CAPRED. This bit indicates the type of edge that caused the last capture event."]
    #[inline(always)]
    pub fn capred(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Mestat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Mestat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture Event   CAPE. This flag is set on a capture event. It must be reset by software. An interrupt request is generated if MECR.CAPEIE   1. If CAPE 1 then no further measurement results are posted to MESTAT.CAPT and MESTAT.CAPRED. CAPE bit has to be cleared to re enable update of MESTAT.CAPT and MESTAT.CAPRED."]
    #[inline(always)]
    pub fn cape(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Mestat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Mestat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mestat {
    #[inline(always)]
    fn default() -> Mestat {
        <crate::RegValueT<Mestat_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "TGS  TGB Write Protection   TG P. TGS and TGB are only written when TG P is 1  otherwise unchanged. Read as 0."]
    #[inline(always)]
    pub fn tg_p(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ocs_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ocs_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "OCDS Suspend Control   SUS. Controls the sensitivity to the suspend signal coming from the OCDS Trigger Switch  OTGS"]
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

#[doc = "N"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct N(pub(super) *mut u8);
unsafe impl core::marker::Send for N {}
unsafe impl core::marker::Sync for N {}
impl N {
    #[doc = "Access Enable Register CAN Node 0 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accennodei0(&self) -> crate::common::Reg<n::AccennodEi0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "CC Control Register 0\n resetvalue={Application Reset:0x1}"]
    #[inline(always)]
    pub const fn cccri(&self) -> crate::common::Reg<n::CccRi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(280usize)) }
    }
    #[doc = "Core Release Register 0\n resetvalue={Application Reset:0x32150320,Application Reset:0x32150323,Application Reset:0x32150320}"]
    #[inline(always)]
    pub const fn creli(&self) -> crate::common::Reg<n::CreLi_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
    }
    #[doc = "Data Bit Timing   Prescaler Register 0\n resetvalue={Application Reset:0x0A33}"]
    #[inline(always)]
    pub const fn dbtpi(&self) -> crate::common::Reg<n::DbtPi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(268usize)) }
    }
    #[doc = "Error Counter Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ecri(&self) -> crate::common::Reg<n::EcRi_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(320usize)) }
    }
    #[doc = "End Address Node 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn endadri(&self) -> crate::common::Reg<n::EndadRi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Endian Register 0\n resetvalue={Application Reset:0x087654321}"]
    #[inline(always)]
    pub const fn endni(&self) -> crate::common::Reg<n::EndNi_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }
    #[doc = "Global Filter Configuration 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gfci(&self) -> crate::common::Reg<n::GfCi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(384usize)) }
    }
    #[doc = "Interrupt routing for Groups 1 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn grint1i(&self) -> crate::common::Reg<n::Grint1I_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Interrupt routing for Groups 2 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn grint2i(&self) -> crate::common::Reg<n::Grint2I_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "High Priority Message Status 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hpmsi(&self) -> crate::common::Reg<n::HpmSi_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(404usize)) }
    }
    #[doc = "Interrupt Enable 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn iei(&self) -> crate::common::Reg<n::IEi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(340usize)) }
    }
    #[doc = "Interrupt Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn iri(&self) -> crate::common::Reg<n::IRi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(336usize)) }
    }
    #[doc = "Interrupt Signalling Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn isregi(&self) -> crate::common::Reg<n::IsreGi_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Nominal Bit Timing   Prescaler Register 0\n resetvalue={Application Reset:0x6000A03}"]
    #[inline(always)]
    pub const fn nbtpi(&self) -> crate::common::Reg<n::NbtPi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(284usize)) }
    }
    #[doc = "New Data 1 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ndat1i(&self) -> crate::common::Reg<n::Ndat1I_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(408usize)) }
    }
    #[doc = "New Data 2 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ndat2i(&self) -> crate::common::Reg<n::Ndat2I_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(412usize)) }
    }
    #[doc = "Node 0 Port Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn npcri(&self) -> crate::common::Reg<n::NpcRi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }
    #[doc = "Protocol Status Register 0\n resetvalue={Application Reset:0x707}"]
    #[inline(always)]
    pub const fn psri(&self) -> crate::common::Reg<n::PsRi_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(324usize)) }
    }
    #[doc = "RAM Watchdog 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rwdi(&self) -> crate::common::Reg<n::RwDi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(276usize)) }
    }
    #[doc = "Standard ID Filter Configuration 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sidfci(&self) -> crate::common::Reg<n::SidfCi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(388usize)) }
    }
    #[doc = "Start Address Node 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn startadri(&self) -> crate::common::Reg<n::StartadRi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Transmitter Delay Compensation Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tdcri(&self) -> crate::common::Reg<n::TdcRi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(328usize)) }
    }
    #[doc = "Test Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn testi(&self) -> crate::common::Reg<n::TesTi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(272usize)) }
    }
    #[doc = "Timeout Counter Configuration 0\n resetvalue={Application Reset:0x0FFFF0000}"]
    #[inline(always)]
    pub const fn tocci(&self) -> crate::common::Reg<n::TocCi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(296usize)) }
    }
    #[doc = "Timeout Counter Value 0\n resetvalue={Application Reset:0x0FFFF}"]
    #[inline(always)]
    pub const fn tocvi(&self) -> crate::common::Reg<n::TocVi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(300usize)) }
    }
    #[doc = "Timestamp Counter Configuration 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tscci(&self) -> crate::common::Reg<n::TscCi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(288usize)) }
    }
    #[doc = "Timestamp Counter Value 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tscvi(&self) -> crate::common::Reg<n::TscVi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(292usize)) }
    }
    #[doc = "Time Trigger Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ttcri(&self) -> crate::common::Reg<n::TtcRi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(240usize)) }
    }
    #[doc = "Extended ID AND Mask 0\n resetvalue={Application Reset:0x1FFFFFFF}"]
    #[inline(always)]
    pub const fn xidami(&self) -> crate::common::Reg<n::XidaMi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(400usize)) }
    }
    #[doc = "Extended ID Filter Configuration 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn xidfci(&self) -> crate::common::Reg<n::XidfCi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(392usize)) }
    }
    #[doc = "NT"]
    #[inline(always)]
    pub fn nt(self) -> n::Nt {
        unsafe { n::Nt(self.0.add(32usize)) }
    }
    #[doc = "RX"]
    #[inline(always)]
    pub fn rx(self) -> n::Rx {
        unsafe { n::Rx(self.0.add(416usize)) }
    }
    #[doc = "TT"]
    #[inline(always)]
    pub fn tt(self) -> n::Tt {
        unsafe { n::Tt(self.0.add(512usize)) }
    }
    #[doc = "TX"]
    #[inline(always)]
    pub fn tx(self) -> n::Tx {
        unsafe { n::Tx(self.0.add(448usize)) }
    }
}
pub mod n {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AccennodEi0_SPEC;
    impl crate::sealed::RegSpec for AccennodEi0_SPEC {
        type DataType = u32;
    }
    #[doc = "Access Enable Register CAN Node 0 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type AccennodEi0 = crate::RegValueT<AccennodEi0_SPEC>;

    impl AccennodEi0 {
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<0,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<1,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<2,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en3(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<3,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en4(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<4,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en5(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<5,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en6(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<6,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en7(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<7,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en8(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<8,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en9(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<9,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en10(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<10,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en11(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<11,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en12(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<12,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en13(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<13,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en14(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<14,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en15(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<15,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en16(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<16,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en17(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<17,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en18(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<18,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en19(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<19,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en20(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<20,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en21(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<21,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en22(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<22,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en23(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<23,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en24(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<24,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en25(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<25,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en26(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<26,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en27(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<27,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en28(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<28,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en29(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<29,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en30(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<30,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en31(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, AccennodEi0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<31,1,0,AccennodEi0_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for AccennodEi0 {
        #[inline(always)]
        fn default() -> AccennodEi0 {
            <crate::RegValueT<AccennodEi0_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CccRi_SPEC;
    impl crate::sealed::RegSpec for CccRi_SPEC {
        type DataType = u32;
    }
    #[doc = "CC Control Register 0\n resetvalue={Application Reset:0x1}"]
    pub type CccRi = crate::RegValueT<CccRi_SPEC>;

    impl CccRi {
        #[doc = "Initialization   INIT. Due to the synchronization mechanism between the two clock domains           there may be a delay until the value written to INIT can be read back.          Therefore the programmer has to assure that the previous value written          to INIT has been accepted by reading INIT before setting INIT to a new          value."]
        #[inline(always)]
        pub fn init(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, CccRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration Change Enable   CCE"]
        #[inline(always)]
        pub fn cce(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, CccRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Restricted Operation Mode   ASM. Bit ASM can only be set by the Host when both CCE and INIT are set to          8216 1  8217 . In can also be set by the M CAN .        The bit can be reset by the Host at any time. For a description of the        Restricted Operation Mode see paragraph Restricted Operation Mode."]
        #[inline(always)]
        pub fn asm(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, CccRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Clock Stop Acknowledge   CSA"]
        #[inline(always)]
        pub fn csa(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, CccRi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<3, 1, 0, CccRi_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Clock Stop Request   CSR"]
        #[inline(always)]
        pub fn csr(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, CccRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Bus Monitoring Mode   MON. Bit MON can only be set by the Host when both CCE and INIT are set to          8216 1  8217 . The bit can be reset by the Host at any time. The bus monitoring mode corresponds to the Analyzer Mode of the MultiCAN        module."]
        #[inline(always)]
        pub fn mon(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, CccRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Disable Automatic Retransmission   DAR. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set."]
        #[inline(always)]
        pub fn dar(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, CccRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Test Mode Enable   TEST. The TEST register can only be set  if CCE  INIT and TEST are set. Writes to test will only have effect  if all three bits are set."]
        #[inline(always)]
        pub fn test(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, CccRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "FD Operation Enable   FDOE. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set."]
        #[inline(always)]
        pub fn fdoe(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, CccRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Bit Rate Switch Enable   BRSE. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set."]
        #[inline(always)]
        pub fn brse(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, CccRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9,1,0,CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Protocol Exception Handling Disable   PXHD. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
        #[inline(always)]
        pub fn pxhd(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, CccRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12,1,0,CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Edge Filtering during Bus Integration   EFBI. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
        #[inline(always)]
        pub fn efbi(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, CccRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13,1,0,CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Transmit Pause   TXP. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. If this bit is set  the M CAN pauses for two CAN bit times before starting the next transmission after        itself has successfully transmitted a frame  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn txp(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, CccRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Non ISO Operation   NISO. If this bit is set  the M CAN uses the CAN FD frame format as specified        by the Bosch CAN FD Specification V1.0."]
        #[inline(always)]
        pub fn niso(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, CccRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for CccRi {
        #[inline(always)]
        fn default() -> CccRi {
            <crate::RegValueT<CccRi_SPEC> as RegisterValue<_>>::new(1)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CreLi_SPEC;
    impl crate::sealed::RegSpec for CreLi_SPEC {
        type DataType = u32;
    }
    #[doc = "Core Release Register 0\n resetvalue={Application Reset:0x32150320,Application Reset:0x32150323,Application Reset:0x32150320}"]
    pub type CreLi = crate::RegValueT<CreLi_SPEC>;

    impl CreLi {
        #[doc = "Time Stamp Day"]
        #[inline(always)]
        pub fn day(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, CreLi_SPEC, crate::common::R> {
            crate::common::RegisterField::<0,0xff,1,0,u8, CreLi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Time Stamp Month"]
        #[inline(always)]
        pub fn mon(
            self,
        ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, CreLi_SPEC, crate::common::R> {
            crate::common::RegisterField::<8,0xff,1,0,u8, CreLi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Time Stamp Year"]
        #[inline(always)]
        pub fn year(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, CreLi_SPEC, crate::common::R> {
            crate::common::RegisterField::<16,0xf,1,0,u8, CreLi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Sub step of Core Release   SUBSTEP. One digit  BCD coded."]
        #[inline(always)]
        pub fn substep(
            self,
        ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, CreLi_SPEC, crate::common::R> {
            crate::common::RegisterField::<20,0xf,1,0,u8, CreLi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Step of Core Release   STEP. One digit  BCD coded."]
        #[inline(always)]
        pub fn step(
            self,
        ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, CreLi_SPEC, crate::common::R> {
            crate::common::RegisterField::<24,0xf,1,0,u8, CreLi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Core Release   REL. One digit  BCD coded."]
        #[inline(always)]
        pub fn rel(
            self,
        ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, CreLi_SPEC, crate::common::R> {
            crate::common::RegisterField::<28,0xf,1,0,u8, CreLi_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for CreLi {
        #[inline(always)]
        fn default() -> CreLi {
            <crate::RegValueT<CreLi_SPEC> as RegisterValue<_>>::new(840237856)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbtPi_SPEC;
    impl crate::sealed::RegSpec for DbtPi_SPEC {
        type DataType = u32;
    }
    #[doc = "Data Bit Timing   Prescaler Register 0\n resetvalue={Application Reset:0x0A33}"]
    pub type DbtPi = crate::RegValueT<DbtPi_SPEC>;

    impl DbtPi {
        #[doc = "Data  Re  Synchronization Jump Width   DSJW. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Valid values are 0 to 15. The actual interpretation by the hardware of        this value is such that one more than the value programmed here is used."]
        #[inline(always)]
        pub fn dsjw(
            self,
        ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, DbtPi_SPEC, crate::common::RW> {
            crate::common::RegisterField::<0,0xf,1,0,u8, DbtPi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Data time segment after sample point   DTSEG2. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Valid values are 0 to 15. The actual interpretation by the hardware of        this value is such that one more than the programmed value is used."]
        #[inline(always)]
        pub fn dtseg2(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, DbtPi_SPEC, crate::common::RW> {
            crate::common::RegisterField::<4,0xf,1,0,u8, DbtPi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Data time segment before sample point   DTSEG1. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Valid values are 0 to 31. The actual interpretation by the hardware of        this value is such that one more than the programmed value is used."]
        #[inline(always)]
        pub fn dtseg1(
            self,
        ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, DbtPi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x1f,1,0,u8, DbtPi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Data Baud Rate Prescaler   DBRP. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. The value by which the oscillator frequency is divided for generating        the bit time quanta. The bit time is built up from a multiple of this        quanta. Valid values for the Baud Rate Prescaler are   8201 0 to 31. The        actual interpretation by the hardware of this value is such that one        more than the value programmed here is used."]
        #[inline(always)]
        pub fn dbrp(
            self,
        ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, DbtPi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1f,1,0,u8, DbtPi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Transmitter Delay Compensation   TDC. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
        #[inline(always)]
        pub fn tdc(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, DbtPi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<23,1,0,DbtPi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for DbtPi {
        #[inline(always)]
        fn default() -> DbtPi {
            <crate::RegValueT<DbtPi_SPEC> as RegisterValue<_>>::new(2611)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EcRi_SPEC;
    impl crate::sealed::RegSpec for EcRi_SPEC {
        type DataType = u32;
    }
    #[doc = "Error Counter Register 0\n resetvalue={Application Reset:0x0}"]
    pub type EcRi = crate::RegValueT<EcRi_SPEC>;

    impl EcRi {
        #[doc = "Transmit Error Counter   TEC. Actual state of the Transmit Error Counter  values between 0 and 255 When CCCR.ASM is set  the CAN protocol controller does not increment          TEC and REC when a CAN protocol error is detected  but CEL is still          incremented."]
        #[inline(always)]
        pub fn tec(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, EcRi_SPEC, crate::common::R> {
            crate::common::RegisterField::<0,0xff,1,0,u8, EcRi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Receive Error Counter   REC. Actual state of the Receive Error Counter  values between 0 and 127 When CCCR.ASM is set  the CAN protocol controller does not increment          TEC and REC when a CAN protocol error is detected  but CEL is still          incremented."]
        #[inline(always)]
        pub fn rec(
            self,
        ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, EcRi_SPEC, crate::common::R> {
            crate::common::RegisterField::<8,0x7f,1,0,u8, EcRi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Receive Error Passive   RP"]
        #[inline(always)]
        pub fn rp(self) -> crate::common::RegisterFieldBool<15, 1, 0, EcRi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<15, 1, 0, EcRi_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "CAN Error Logging   CEL. The counter is incremented each time when a CAN protocol error causes        the Transmit Error Counter or the Receive Error Counter to be        incremented. It is reset by read access to CEL. The counter stops at        0xFF  the next increment of TEC or REC sets interrupt flag IR.ELO. The counter is reset on read  if the bit NPCRi.DELE is set for the node."]
        #[inline(always)]
        pub fn cel(
            self,
        ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, EcRi_SPEC, crate::common::R> {
            crate::common::RegisterField::<16,0xff,1,0,u8, EcRi_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for EcRi {
        #[inline(always)]
        fn default() -> EcRi {
            <crate::RegValueT<EcRi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EndadRi_SPEC;
    impl crate::sealed::RegSpec for EndadRi_SPEC {
        type DataType = u32;
    }
    #[doc = "End Address Node 0\n resetvalue={Application Reset:0x0}"]
    pub type EndadRi = crate::RegValueT<EndadRi_SPEC>;

    impl EndadRi {
        #[doc = "Message RAM end   END. The address within the RAM area of the MCMCAN          of node i  where the message RAM to be protected ends"]
        #[inline(always)]
        pub fn end(
            self,
        ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, EndadRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x3fff,1,0,u16, EndadRi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for EndadRi {
        #[inline(always)]
        fn default() -> EndadRi {
            <crate::RegValueT<EndadRi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EndNi_SPEC;
    impl crate::sealed::RegSpec for EndNi_SPEC {
        type DataType = u32;
    }
    #[doc = "Endian Register 0\n resetvalue={Application Reset:0x087654321}"]
    pub type EndNi = crate::RegValueT<EndNi_SPEC>;

    impl EndNi {
        #[doc = "Endianness Test Value   ETV. The endianness test value is 0x87654321."]
        #[inline(always)]
        pub fn etv(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, EndNi_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, EndNi_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for EndNi {
        #[inline(always)]
        fn default() -> EndNi {
            <crate::RegValueT<EndNi_SPEC> as RegisterValue<_>>::new(2271560481)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfCi_SPEC;
    impl crate::sealed::RegSpec for GfCi_SPEC {
        type DataType = u32;
    }
    #[doc = "Global Filter Configuration 0\n resetvalue={Application Reset:0x0}"]
    pub type GfCi = crate::RegValueT<GfCi_SPEC>;

    impl GfCi {
        #[doc = "Reject Remote Frames Extended   RRFE. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set."]
        #[inline(always)]
        pub fn rrfe(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, GfCi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0, 1, 0, GfCi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Reject Remote Frames Standard   RRFS. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set."]
        #[inline(always)]
        pub fn rrfs(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, GfCi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1, 1, 0, GfCi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Accept Non matching Frames Extended   ANFE. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set. Defines how received messages with 29 bit IDs that do not match any element of the filter list are treated."]
        #[inline(always)]
        pub fn anfe(
            self,
        ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, GfCi_SPEC, crate::common::RW> {
            crate::common::RegisterField::<2,0x3,1,0,u8, GfCi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Accept Non matching Frames Standard   ANFS. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set. Defines how received messages with 11 bit IDs that do not match any element of the filter list are treated."]
        #[inline(always)]
        pub fn anfs(
            self,
        ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, GfCi_SPEC, crate::common::RW> {
            crate::common::RegisterField::<4,0x3,1,0,u8, GfCi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GfCi {
        #[inline(always)]
        fn default() -> GfCi {
            <crate::RegValueT<GfCi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Grint1I_SPEC;
    impl crate::sealed::RegSpec for Grint1I_SPEC {
        type DataType = u32;
    }
    #[doc = "Interrupt routing for Groups 1 0\n resetvalue={Application Reset:0x0}"]
    pub type Grint1I = crate::RegValueT<Grint1I_SPEC>;

    impl Grint1I {
        #[doc = "Transmit Event FIFO Incidents   TEFIFO. are mapped here. IR.TEFF  Transmit Event FIFO Full  and IR.TEFN         Transmit Event FIFO New Entry"]
        #[inline(always)]
        pub fn tefifo(
            self,
        ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Grint1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xf,1,0,u8, Grint1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "High Priority Events   HPE. are mapped here  giving IR.HPM an interrupt level"]
        #[inline(always)]
        pub fn hpe(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Grint1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0xf,1,0,u8, Grint1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Watermark interrupts   WATI. are mapped here  IR.TEFW  Transmit FIFO warning interrupt reached          IR.RF1W  Receive FIFO 1 warning interrupt reached . IR.RF0W  Receive        FIFO 0 warning interrupt reached"]
        #[inline(always)]
        pub fn wati(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Grint1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xf,1,0,u8, Grint1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "ALERTS   ALRT. All kind of alerts are mapped here. IR.EW  warning status   IR.EP  error        passive   IR.TSW  timestamp wrap around   IR.TEFL  Transmit Event FIFO        Element Lost   IR.RF0L  Receive FIFO 0 Message Lost   IR.RF1L  Receive        FIFO 1 Message Lost . The        following TTCAN error messages and warnings are also shown here  TTIR.        CER  Configuration Error   TTIR.AW Application Watchdog  TTIR.WT  Watch        Trigger   TTIR.IWT Initialization Watch Trigger  TTIR.ELC  Error Level        Changed   TTIR.SE2  Scheduling Error 2   TTIR.SE1  Scheduling Error          TTIR.TXO  Tx Count Overflow   TTIR.TXU  TX Count Underflow   TTIR.GTE         Global Time Error   TTIR.GTD  Global Time Discontinuity  and TTIR.GTW         Global Time Wrap"]
        #[inline(always)]
        pub fn alrt(
            self,
        ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Grint1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0xf,1,0,u8, Grint1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Module errors   MOER. IR.WDI  watchdog interrupt  and IR.MRAF  message RAM access failure  are        mapped here."]
        #[inline(always)]
        pub fn moer(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Grint1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, Grint1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Safety counter overflow   SAFE. The interrupt node for IR.ELO showing a safety counter overflow"]
        #[inline(always)]
        pub fn safe(
            self,
        ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Grint1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0xf,1,0,u8, Grint1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Bus Off has been reached   BOFF. Mapped to IRi.BO flag indication the change in Bus Off status. To get        out of bus off  the CCCRn.INIT bit has to be reset."]
        #[inline(always)]
        pub fn boff(
            self,
        ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Grint1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0xf,1,0,u8, Grint1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Last Error Interrupts   LOI. The interrupt sources IR.PED  Protocol Error in Data Phase  and IR.PEA         Protocol Error in Arbitration Phase  are signalled here."]
        #[inline(always)]
        pub fn loi(
            self,
        ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Grint1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0xf,1,0,u8, Grint1I_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Grint1I {
        #[inline(always)]
        fn default() -> Grint1I {
            <crate::RegValueT<Grint1I_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Grint2I_SPEC;
    impl crate::sealed::RegSpec for Grint2I_SPEC {
        type DataType = u32;
    }
    #[doc = "Interrupt routing for Groups 2 0\n resetvalue={Application Reset:0x0}"]
    pub type Grint2I = crate::RegValueT<Grint2I_SPEC>;

    impl Grint2I {
        #[doc = "Message stored in dedicated receive buffer interrupt  IR.DRX    REINT. is assigned to interrupt node."]
        #[inline(always)]
        pub fn reint(
            self,
        ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Grint2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xf,1,0,u8, Grint2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "IR.RF1F   RxF1F. Receive FIFO1 full interrupt assigned to an interrupt node"]
        #[inline(always)]
        pub fn rxf1f(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Grint2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0xf,1,0,u8, Grint2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "IR.RF0F   RxF0F. Receive FIFO0 full interrupt assigned to an interrupt node"]
        #[inline(always)]
        pub fn rxf0f(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Grint2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xf,1,0,u8, Grint2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "IR.RF1N   RxF1N. Receive FIFO1 new message assigned to an interrupt node"]
        #[inline(always)]
        pub fn rxf1n(
            self,
        ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Grint2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0xf,1,0,u8, Grint2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "IR.RF0N   RxF0N. Receive FIFO0 new message assigned to an interrupt node"]
        #[inline(always)]
        pub fn rxf0n(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Grint2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, Grint2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Receive Timeouts   RETI. can be assigned here. IR.TOO  time out event  and TE  Timer Event"]
        #[inline(always)]
        pub fn reti(
            self,
        ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Grint2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0xf,1,0,u8, Grint2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Transmission Queue Events   TRAQ. can be assigned here. IR.TFE Transmission FIFO Empty"]
        #[inline(always)]
        pub fn traq(
            self,
        ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Grint2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0xf,1,0,u8, Grint2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Interrupts of the transmission control   TRACO. can be assigned here. IR.TCF  Transmission Cancellation Finished  and        IR.TF  Transmission Completed . As        an additional information the copy of a local time event is shown here        with TTIR.SWT  Stop Watch Event . Further on the TTIR.TTMI Trigger Time        Event Internal  TTIR.RTMI  Register Time Mark   TTIR.SOG  Start of Gap          TTIR.CSM  Change of Synchronization Mode   TTIR.SMC  Start Matrix Cycle         and TTIR.SBC  Start of Basic Cycle  are shown here."]
        #[inline(always)]
        pub fn traco(
            self,
        ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Grint2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0xf,1,0,u8, Grint2I_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Grint2I {
        #[inline(always)]
        fn default() -> Grint2I {
            <crate::RegValueT<Grint2I_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpmSi_SPEC;
    impl crate::sealed::RegSpec for HpmSi_SPEC {
        type DataType = u32;
    }
    #[doc = "High Priority Message Status 0\n resetvalue={Application Reset:0x0}"]
    pub type HpmSi = crate::RegValueT<HpmSi_SPEC>;

    impl HpmSi {
        #[doc = "Buffer Index   BIDX. Index of Rx FIFO element to which the message was stored. Only valid when MSI 1     1 ."]
        #[inline(always)]
        pub fn bidx(
            self,
        ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, HpmSi_SPEC, crate::common::R> {
            crate::common::RegisterField::<0,0x3f,1,0,u8, HpmSi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Message Storage Indicator   MSI"]
        #[inline(always)]
        pub fn msi(
            self,
        ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, HpmSi_SPEC, crate::common::R> {
            crate::common::RegisterField::<6,0x3,1,0,u8, HpmSi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Filter Index   FIDX. Index of matching filter element. Range is 0 to SIDFC.LSS  160    160 1        resp. XIDFC.LSE  160    160 1."]
        #[inline(always)]
        pub fn fidx(
            self,
        ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, HpmSi_SPEC, crate::common::R> {
            crate::common::RegisterField::<8,0x7f,1,0,u8, HpmSi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Filter List   FLST. Indicates the filter list of the matching filter element."]
        #[inline(always)]
        pub fn flst(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpmSi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<15,1,0,HpmSi_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpmSi {
        #[inline(always)]
        fn default() -> HpmSi {
            <crate::RegValueT<HpmSi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IEi_SPEC;
    impl crate::sealed::RegSpec for IEi_SPEC {
        type DataType = u32;
    }
    #[doc = "Interrupt Enable 0\n resetvalue={Application Reset:0x0}"]
    pub type IEi = crate::RegValueT<IEi_SPEC>;

    impl IEi {
        #[doc = "Rx FIFO 0 New Message Interrupt Enable   RF0NE"]
        #[inline(always)]
        pub fn rf0ne(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Rx FIFO 0 Watermark Reached Interrupt Enable   RF0WE"]
        #[inline(always)]
        pub fn rf0we(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Rx FIFO 0 Full Interrupt Enable   RF0FE"]
        #[inline(always)]
        pub fn rf0fe(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Rx FIFO 0 Message Lost Interrupt Enable   RF0LE"]
        #[inline(always)]
        pub fn rf0le(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Rx FIFO 1 New Message Interrupt Enable   RF1NE"]
        #[inline(always)]
        pub fn rf1ne(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Rx FIFO 1 Watermark Reached Interrupt Enable   RF1WE"]
        #[inline(always)]
        pub fn rf1we(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Rx FIFO 1 Full Interrupt Enable   RF1FE"]
        #[inline(always)]
        pub fn rf1fe(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Rx FIFO 1 Message Lost Interrupt Enable   RF1LE"]
        #[inline(always)]
        pub fn rf1le(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "High Priority Message Interrupt Enable   HPME"]
        #[inline(always)]
        pub fn hpme(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Transmission Completed Interrupt Enable   TCE"]
        #[inline(always)]
        pub fn tce(self) -> crate::common::RegisterFieldBool<9, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Transmission Cancellation Finished Interrupt Enable   TCFE"]
        #[inline(always)]
        pub fn tcfe(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Tx FIFO Empty Interrupt Enable   TFEE"]
        #[inline(always)]
        pub fn tfee(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<11, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Tx Event FIFO New Entry Interrupt Enable   TEFNE"]
        #[inline(always)]
        pub fn tefne(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Tx Event FIFO Watermark Reached Interrupt Enable   TEFWE"]
        #[inline(always)]
        pub fn tefwe(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Tx Event FIFO Full Interrupt Enable   TEFFE"]
        #[inline(always)]
        pub fn teffe(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Tx Event FIFO Element Lost Interrupt Enable   TEFLE"]
        #[inline(always)]
        pub fn tefle(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Timestamp Wraparound Interrupt Enable   TSWE"]
        #[inline(always)]
        pub fn tswe(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<16, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Message RAM Access Failure Interrupt Enable   MRAFE"]
        #[inline(always)]
        pub fn mrafe(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<17, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Timeout Occurred Interrupt Enable   TOOE"]
        #[inline(always)]
        pub fn tooe(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<18, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Message stored to Dedicated Rx Buffer Interrupt Enable   DRXE"]
        #[inline(always)]
        pub fn drxe(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<19, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Error Logging Overflow Interrupt Enable   ELOE"]
        #[inline(always)]
        pub fn eloe(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<22, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Error Passive Interrupt Enable   EPE"]
        #[inline(always)]
        pub fn epe(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<23, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Warning Status Interrupt Enable   EWE"]
        #[inline(always)]
        pub fn ewe(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<24, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Bus Off Status Interrupt Enable   BOE"]
        #[inline(always)]
        pub fn boe(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<25, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Watchdog Interrupt Enable   WDIE"]
        #[inline(always)]
        pub fn wdie(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<26, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Protocol Error in Arbitration Phase Enable   PEAE"]
        #[inline(always)]
        pub fn peae(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<27, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Protocol Error in Data Phase Enable   PEDE"]
        #[inline(always)]
        pub fn pede(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, IEi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<28, 1, 0, IEi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
    }
    impl core::default::Default for IEi {
        #[inline(always)]
        fn default() -> IEi {
            <crate::RegValueT<IEi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IRi_SPEC;
    impl crate::sealed::RegSpec for IRi_SPEC {
        type DataType = u32;
    }
    #[doc = "Interrupt Register 0\n resetvalue={Application Reset:0x0}"]
    pub type IRi = crate::RegValueT<IRi_SPEC>;

    impl IRi {
        #[doc = "Rx FIFO 0 New Message   RF0N"]
        #[inline(always)]
        pub fn rf0n(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Rx FIFO 0 Watermark Reached   RF0W"]
        #[inline(always)]
        pub fn rf0w(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Rx FIFO 0 Full   RF0F"]
        #[inline(always)]
        pub fn rf0f(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Rx FIFO 0 Message Lost   RF0L"]
        #[inline(always)]
        pub fn rf0l(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Rx FIFO 1 New Message   RF1N"]
        #[inline(always)]
        pub fn rf1n(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Rx FIFO 1 Watermark Reached   RF1W"]
        #[inline(always)]
        pub fn rf1w(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Rx FIFO 1 Full   RF1F"]
        #[inline(always)]
        pub fn rf1f(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Rx FIFO 1 Message Lost   RF1L"]
        #[inline(always)]
        pub fn rf1l(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "High Priority Message   HPM"]
        #[inline(always)]
        pub fn hpm(self) -> crate::common::RegisterFieldBool<8, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Transmission Completed   TC"]
        #[inline(always)]
        pub fn tc(self) -> crate::common::RegisterFieldBool<9, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Transmission Cancellation Finished   TCF"]
        #[inline(always)]
        pub fn tcf(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Tx FIFO Empty   TFE"]
        #[inline(always)]
        pub fn tfe(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<11, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Tx Event FIFO New Entry   TEFN"]
        #[inline(always)]
        pub fn tefn(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Tx Event FIFO Watermark Reached   TEFW"]
        #[inline(always)]
        pub fn tefw(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Tx Event FIFO Full   TEFF"]
        #[inline(always)]
        pub fn teff(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Tx Event FIFO Element Lost   TEFL"]
        #[inline(always)]
        pub fn tefl(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Timestamp Wraparound   TSW"]
        #[inline(always)]
        pub fn tsw(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<16, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Message RAM Access Failure   MRAF. The flag is set  when the Rx Handler has not completed acceptance filtering or storage of an accepted          message until the arbitration field of the following message has been          received. In this case acceptance filtering or message storage is          aborted and the Rx Handler starts processing of the following message. was not able to write a message to the Message RAM. In this case          message storage is aborted. In both cases the FIFO put index is not updated resp. the New Data flag        for a dedicated Rx Buffer is not set  a partly stored message is        overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message        from the Message RAM in time. In this case message transmission is        aborted. In case of a Tx Handler access failure the M CAN is switched into Restricted Operation Mode. To leave Restricted        Operation Mode  the Host CPU has to reset CCCR.ASM."]
        #[inline(always)]
        pub fn mraf(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<17, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Timeout Occurred   TOO"]
        #[inline(always)]
        pub fn too(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<18, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Message stored to Dedicated Rx Buffer   DRX. The flag is set whenever a received message has been stored into a        dedicated Rx Buffer."]
        #[inline(always)]
        pub fn drx(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<19, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Error Logging Overflow   ELO"]
        #[inline(always)]
        pub fn elo(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<22, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Error Passive   EP"]
        #[inline(always)]
        pub fn ep(self) -> crate::common::RegisterFieldBool<23, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<23, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Warning Status   EW"]
        #[inline(always)]
        pub fn ew(self) -> crate::common::RegisterFieldBool<24, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<24, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Bus Off Status   BO"]
        #[inline(always)]
        pub fn bo(self) -> crate::common::RegisterFieldBool<25, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<25, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Watchdog Interrupt   WDI"]
        #[inline(always)]
        pub fn wdi(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<26, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Protocol Error in Arbitration Phase   PEA.  Nominal Bit Time is used"]
        #[inline(always)]
        pub fn pea(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<27, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Protocol Error in Data Phase   PED.  Data Bit Time is used"]
        #[inline(always)]
        pub fn ped(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, IRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<28, 1, 0, IRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
    }
    impl core::default::Default for IRi {
        #[inline(always)]
        fn default() -> IRi {
            <crate::RegValueT<IRi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IsreGi_SPEC;
    impl crate::sealed::RegSpec for IsreGi_SPEC {
        type DataType = u32;
    }
    #[doc = "Interrupt Signalling Register 0\n resetvalue={Application Reset:0x0}"]
    pub type IsreGi = crate::RegValueT<IsreGi_SPEC>;

    impl IsreGi {
        #[doc = "A message stored in a receive buffer interrupt   REINT"]
        #[inline(always)]
        pub fn reint(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Receive FIFO1 is full interrupt   RxF1F"]
        #[inline(always)]
        pub fn rxf1f(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<1,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Receive FIFO0 is full interrupt   RxF0F"]
        #[inline(always)]
        pub fn rxf0f(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<2,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Receive FIFO1 got a new message interrupt   RxF1N"]
        #[inline(always)]
        pub fn rxf1n(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<3,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Receive FIFO0 got a new message interrupt   RxF0N"]
        #[inline(always)]
        pub fn rxf0n(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<4,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "A receive timeout event interrupt   RETI"]
        #[inline(always)]
        pub fn reti(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<5,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "A transmission queue event interrupt   TRAQ"]
        #[inline(always)]
        pub fn traq(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<6,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "A transmission control event interrupt   TRACO"]
        #[inline(always)]
        pub fn traco(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<7,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "A Transmit Event FIFO Incident interrupt   TEFIFO"]
        #[inline(always)]
        pub fn tefifo(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<8,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "A high priority event interrupt   HPE"]
        #[inline(always)]
        pub fn hpe(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<9,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "A watermark interrupt has been reached   WATI"]
        #[inline(always)]
        pub fn wati(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<10,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "An alert interrupt   ALRT"]
        #[inline(always)]
        pub fn alrt(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<11,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Module error interrupt   MOER"]
        #[inline(always)]
        pub fn moer(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<12,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "The safety counter interrupt ELO   SAFE"]
        #[inline(always)]
        pub fn safe(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<13,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Bus Off Interrupt   BOFF"]
        #[inline(always)]
        pub fn boff(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<14,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Last Error Interrupt   LOI"]
        #[inline(always)]
        pub fn loi(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<15,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for IsreGi {
        #[inline(always)]
        fn default() -> IsreGi {
            <crate::RegValueT<IsreGi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NbtPi_SPEC;
    impl crate::sealed::RegSpec for NbtPi_SPEC {
        type DataType = u32;
    }
    #[doc = "Nominal Bit Timing   Prescaler Register 0\n resetvalue={Application Reset:0x6000A03}"]
    pub type NbtPi = crate::RegValueT<NbtPi_SPEC>;

    impl NbtPi {
        #[doc = "Nominal Time segment after sample point   NTSEG2. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Valid values are 1 to 127. The actual interpretation by the hardware of        this value is such that one more than the programmed value is used."]
        #[inline(always)]
        pub fn ntseg2(
            self,
        ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, NbtPi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7f,1,0,u8, NbtPi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Nominal Time segment before sample point   NTSEG1. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Valid values are 1 to 255. The actual interpretation by the hardware of        this value is such that one more than the programmed value is used."]
        #[inline(always)]
        pub fn ntseg1(
            self,
        ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, NbtPi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xff,1,0,u8, NbtPi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Baud Rate Prescaler   NBRP. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. The value by which the oscillator frequency is divided for generating        the bit time quanta. The bit time is built up from a multiple of this        quanta. Valid values for the Baud Rate Prescaler are 0 to 511. The        actual interpretation by the hardware of this value is such that one        more than the value programmed here is used."]
        #[inline(always)]
        pub fn nbrp(
            self,
        ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, NbtPi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1ff,1,0,u16, NbtPi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Re  Synchronization Jump Width   NSJW. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Valid values are 0 to 127. The actual interpretation by the hardware of        this value is such that one more than the value programmed here is used."]
        #[inline(always)]
        pub fn nsjw(
            self,
        ) -> crate::common::RegisterField<25, 0x7f, 1, 0, u8, NbtPi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<25,0x7f,1,0,u8, NbtPi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for NbtPi {
        #[inline(always)]
        fn default() -> NbtPi {
            <crate::RegValueT<NbtPi_SPEC> as RegisterValue<_>>::new(100665859)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ndat1I_SPEC;
    impl crate::sealed::RegSpec for Ndat1I_SPEC {
        type DataType = u32;
    }
    #[doc = "New Data 1 0\n resetvalue={Application Reset:0x0}"]
    pub type Ndat1I = crate::RegValueT<Ndat1I_SPEC>;

    impl Ndat1I {
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd3(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd4(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd5(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd6(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd7(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd8(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd9(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd10(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd11(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<11,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd12(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd13(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd14(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd15(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd16(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<16,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd17(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<17,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd18(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<18,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd19(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<19,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd20(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<20,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd21(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<21,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd22(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<22,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd23(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<23,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd24(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<24,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd25(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<25,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd26(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<26,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd27(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<27,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd28(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<28,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd29(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<29,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd30(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<30,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd31(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, Ndat1I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<31,1,0,Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Ndat1I {
        #[inline(always)]
        fn default() -> Ndat1I {
            <crate::RegValueT<Ndat1I_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ndat2I_SPEC;
    impl crate::sealed::RegSpec for Ndat2I_SPEC {
        type DataType = u32;
    }
    #[doc = "New Data 2 0\n resetvalue={Application Reset:0x0}"]
    pub type Ndat2I = crate::RegValueT<Ndat2I_SPEC>;

    impl Ndat2I {
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd32(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd33(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd34(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd35(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd36(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd37(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd38(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd39(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd40(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd41(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd42(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd43(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<11,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd44(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd45(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd46(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd47(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd48(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<16,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd49(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<17,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd50(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<18,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd51(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<19,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd52(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<20,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd53(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<21,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd54(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<22,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd55(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<23,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd56(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<24,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd57(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<25,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd58(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<26,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd59(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<27,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd60(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<28,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd61(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<29,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd62(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<30,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd63(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, Ndat2I_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<31,1,0,Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Ndat2I {
        #[inline(always)]
        fn default() -> Ndat2I {
            <crate::RegValueT<Ndat2I_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NpcRi_SPEC;
    impl crate::sealed::RegSpec for NpcRi_SPEC {
        type DataType = u32;
    }
    #[doc = "Node 0 Port Control Register\n resetvalue={Application Reset:0x0}"]
    pub type NpcRi = crate::RegValueT<NpcRi_SPEC>;

    impl NpcRi {
        #[doc = "Receive Select   RXSEL. RXSEL selects one out of 8 possible receive inputs. The CAN receive        signal is performed by the selected input.  see the device related        chapter for RXSEL"]
        #[inline(always)]
        pub fn rxsel(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, NpcRi_SPEC, crate::common::RW> {
            crate::common::RegisterField::<0,0x7,1,0,u8, NpcRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Loop Back Mode   LBM"]
        #[inline(always)]
        pub fn lbm(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, NpcRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,NpcRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Loop Back Mode Out   LOUT. This bit is not implemented in TC39x A step silicon. The loop back bus is switched to the external CAN bus of the node."]
        #[inline(always)]
        pub fn lout(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, NpcRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9,1,0,NpcRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable destructive read on ECR0.CEL   DELE. If this bit is set  the destructive read on ECRi.CEL and on the PSR        register takes place. Meaning  that with read access on ECRi  the CEL is        reset. The same is true for the PSR register  for the bits PXE  RFDF         RBRS  RESI  LEC and DLEC. After the destructive read it is advised to        reset the bit again."]
        #[inline(always)]
        pub fn dele(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, NpcRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,NpcRi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for NpcRi {
        #[inline(always)]
        fn default() -> NpcRi {
            <crate::RegValueT<NpcRi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PsRi_SPEC;
    impl crate::sealed::RegSpec for PsRi_SPEC {
        type DataType = u32;
    }
    #[doc = "Protocol Status Register 0\n resetvalue={Application Reset:0x707}"]
    pub type PsRi = crate::RegValueT<PsRi_SPEC>;

    impl PsRi {
        #[doc = "Last Error Code   LEC. The LEC indicates the type of the last error to occur on the CAN bus.        This field will be cleared to   8216 0  8217  when a message has been transferred         reception or transmission  without error. This bit field is set to 0x7        on read  if NPCRi.DELE is set. The Bus Off recovery sequence  see ISO11898 1  cannot be shortened by          setting or resetting CCCR.INIT. If the device goes Bus Off  it will          set CCCR.INIT of its own accord  stopping all bus activities. Once          CCCR.INIT has been cleared by the CPU  the device will then wait for          129 occurrences of Bus Idle  129   11 consecutive recessive bits           before resuming normal operation. At the end of the Bus Off recovery          sequence  the Error Management Counters will be reset. During the          waiting time after the resetting of CCCR.INIT  each time a sequence of          11 recessive bits has been monitored  a Bit0Error code is written to          PSR.LEC  enabling the CPU to readily check up whether the CAN bus is          stuck at dominant or continuously disturbed and to monitor the Bus Off          recovery sequence. ECR.REC is used to count these sequences."]
        #[inline(always)]
        pub fn lec(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, PsRi_SPEC, crate::common::R> {
            crate::common::RegisterField::<0,0x7,1,0,u8, PsRi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Activity   ACT. Monitors the module s CAN communication state. ACT is set to  00  by a Protocol Exception Event."]
        #[inline(always)]
        pub fn act(
            self,
        ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, PsRi_SPEC, crate::common::R> {
            crate::common::RegisterField::<3,0x3,1,0,u8, PsRi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Error Passive   EP"]
        #[inline(always)]
        pub fn ep(self) -> crate::common::RegisterFieldBool<5, 1, 0, PsRi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<5, 1, 0, PsRi_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Warning Status   EW"]
        #[inline(always)]
        pub fn ew(self) -> crate::common::RegisterFieldBool<6, 1, 0, PsRi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<6, 1, 0, PsRi_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Bus Off Status   BO"]
        #[inline(always)]
        pub fn bo(self) -> crate::common::RegisterFieldBool<7, 1, 0, PsRi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<7, 1, 0, PsRi_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Data Phase Last Error Code   DLEC. Type of last error that occurred in the data phase of a CAN FD format        frame with its BRS flag set. Coding is the same as for LEC. This field        will be cleared to zero when a CAN FD format frame with its BRS flag set        has been transferred  reception or transmission  without error. This bit        field is set to 0x7 on read  if NPCRi.DELE is set. When a frame in CAN FD format has reached the data phase with BRS flag          set  the next CAN event  error or valid frame  will be shown in DLEC          instead of LEC. An error in a fixed stuff bit of a CAN FD CRC sequence          will be shown as a Form Error  not Stuff Error."]
        #[inline(always)]
        pub fn dlec(
            self,
        ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, PsRi_SPEC, crate::common::R> {
            crate::common::RegisterField::<8,0x7,1,0,u8, PsRi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "ESI flag of last received CAN FD Message   RESI. This bit is set together with REDF  independent of acceptance filtering. This bit is reset after read access  if NPCRi.DELE is set."]
        #[inline(always)]
        pub fn resi(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, PsRi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<11, 1, 0, PsRi_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "BRS flag of last received CAN FD Message   RBRS. This bit is set together with REDF  independent of acceptance filtering. This bit is reset after read access  if NPCRi.DELE is set."]
        #[inline(always)]
        pub fn rbrs(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, PsRi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<12, 1, 0, PsRi_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Received a CAN FD Message   RFDF. This bit is set independent of acceptance filtering. This bit is reset after read access  if NPCRi.DELE is set."]
        #[inline(always)]
        pub fn rfdf(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, PsRi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<13, 1, 0, PsRi_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Protocol Exception Event   PXE. This bit is reset after read access  if NPCRi.DELE is set."]
        #[inline(always)]
        pub fn pxe(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, PsRi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<14, 1, 0, PsRi_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Transmitter Delay Compensation Value   TDCV. Position of the secondary sample point  defined by the sum of the        measured delay from TX to RX and TDCR.TDCO. The SSP position is  in the        data phase  the number of mtq between the start of the transmitted bit        and the secondary sample point. Valid values are 0 to 127 mtq."]
        #[inline(always)]
        pub fn tdcv(
            self,
        ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, PsRi_SPEC, crate::common::R> {
            crate::common::RegisterField::<16,0x7f,1,0,u8, PsRi_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for PsRi {
        #[inline(always)]
        fn default() -> PsRi {
            <crate::RegValueT<PsRi_SPEC> as RegisterValue<_>>::new(1799)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RwDi_SPEC;
    impl crate::sealed::RegSpec for RwDi_SPEC {
        type DataType = u32;
    }
    #[doc = "RAM Watchdog 0\n resetvalue={Application Reset:0x0}"]
    pub type RwDi = crate::RegValueT<RwDi_SPEC>;

    impl RwDi {
        #[doc = "Watchdog Configuration   WDC. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set. Start value of the Message RAM Watchdog Counter. With the reset value of  00  the counter is disabled."]
        #[inline(always)]
        pub fn wdc(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, RwDi_SPEC, crate::common::RW> {
            crate::common::RegisterField::<0,0xff,1,0,u8, RwDi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Watchdog Value   WDV. Actual Message RAM Watchdog Counter Value."]
        #[inline(always)]
        pub fn wdv(
            self,
        ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, RwDi_SPEC, crate::common::R> {
            crate::common::RegisterField::<8,0xff,1,0,u8, RwDi_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for RwDi {
        #[inline(always)]
        fn default() -> RwDi {
            <crate::RegValueT<RwDi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SidfCi_SPEC;
    impl crate::sealed::RegSpec for SidfCi_SPEC {
        type DataType = u32;
    }
    #[doc = "Standard ID Filter Configuration 0\n resetvalue={Application Reset:0x0}"]
    pub type SidfCi = crate::RegValueT<SidfCi_SPEC>;

    impl SidfCi {
        #[doc = "Filter List Standard Start Address   FLSSA. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Start address of standard Message ID filter list  32 bit word address ."]
        #[inline(always)]
        pub fn flssa(
            self,
        ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, SidfCi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x3fff,1,0,u16, SidfCi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "List Size Standard   LSS. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
        #[inline(always)]
        pub fn lss(
            self,
        ) -> crate::common::RegisterField<16, 0xff, 1, 0, sidfci::Lss, SidfCi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                16,
                0xff,
                1,
                0,
                sidfci::Lss,
                SidfCi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for SidfCi {
        #[inline(always)]
        fn default() -> SidfCi {
            <crate::RegValueT<SidfCi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod sidfci {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lss_SPEC;
        pub type Lss = crate::EnumBitfieldStruct<u8, Lss_SPEC>;
        impl Lss {
            #[doc = "128 Message ID filter elements"]
            pub const REST_255: Self = Self::new(255);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct StartadRi_SPEC;
    impl crate::sealed::RegSpec for StartadRi_SPEC {
        type DataType = u32;
    }
    #[doc = "Start Address Node 0\n resetvalue={Application Reset:0x0}"]
    pub type StartadRi = crate::RegValueT<StartadRi_SPEC>;

    impl StartadRi {
        #[doc = "Message RAM start   START. The address within the RAM area of the MCMCAN          of node i  where the message RAM to be protected starts"]
        #[inline(always)]
        pub fn start(
            self,
        ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, StartadRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x3fff,1,0,u16, StartadRi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for StartadRi {
        #[inline(always)]
        fn default() -> StartadRi {
            <crate::RegValueT<StartadRi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TdcRi_SPEC;
    impl crate::sealed::RegSpec for TdcRi_SPEC {
        type DataType = u32;
    }
    #[doc = "Transmitter Delay Compensation Register 0\n resetvalue={Application Reset:0x0}"]
    pub type TdcRi = crate::RegValueT<TdcRi_SPEC>;

    impl TdcRi {
        #[doc = "Transmitter Delay Compensation Filter Window Length   TDCF. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Defines the minimum value for the Secondary Sample        Point position  dominant edges on RX that would result in an earlier        Secondary Sample Point position are ignored for transmitter delay        measurement. This feature is enabled when TDCF is configured to a value        greater than TDCO. Valid values are from 0 to 127 mtq."]
        #[inline(always)]
        pub fn tdcf(
            self,
        ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, TdcRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7f,1,0,u8, TdcRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Transmitter Delay Compensation Offset   TDCO. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Offset value defining the distance between the        measured delay from TX to RX and the secondary sample point. Valid        values are 0 to 127 mtq. The duration of one mtq is equal to the fASYNi        clock period."]
        #[inline(always)]
        pub fn tdco(
            self,
        ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, TdcRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x7f,1,0,u8, TdcRi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for TdcRi {
        #[inline(always)]
        fn default() -> TdcRi {
            <crate::RegValueT<TdcRi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TesTi_SPEC;
    impl crate::sealed::RegSpec for TesTi_SPEC {
        type DataType = u32;
    }
    #[doc = "Test Register 0\n resetvalue={Application Reset:0x0}"]
    pub type TesTi = crate::RegValueT<TesTi_SPEC>;

    impl TesTi {
        #[doc = "Loop Back Mode   LBCK. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. This is the external loop back mode  visible on the outside."]
        #[inline(always)]
        pub fn lbck(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, TesTi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,TesTi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Control of Transmit Pin   TX. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
        #[inline(always)]
        pub fn tx(
            self,
        ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, TesTi_SPEC, crate::common::RW> {
            crate::common::RegisterField::<5,0x3,1,0,u8, TesTi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Receive Pin   RX. Monitors the actual value of RX pin."]
        #[inline(always)]
        pub fn rx(self) -> crate::common::RegisterFieldBool<7, 1, 0, TesTi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<7, 1, 0, TesTi_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
    }
    impl core::default::Default for TesTi {
        #[inline(always)]
        fn default() -> TesTi {
            <crate::RegValueT<TesTi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TocCi_SPEC;
    impl crate::sealed::RegSpec for TocCi_SPEC {
        type DataType = u32;
    }
    #[doc = "Timeout Counter Configuration 0\n resetvalue={Application Reset:0x0FFFF0000}"]
    pub type TocCi = crate::RegValueT<TocCi_SPEC>;

    impl TocCi {
        #[doc = "Enable Timeout Counter   ETOC. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. For use of timeout function with CAN FD see chapter Timeout Counter."]
        #[inline(always)]
        pub fn etoc(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, TocCi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,TocCi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Timeout Select   TOS. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set. When operating in Continuous mode  a write to TOCV presets the counter to the value configured by TOCC.TOP and continues down counting. When the Timeout Counter is controlled by one of the FIFOs  an empty FIFO presets the counter to the value configured by TOCC.TOP. Down counting is started when the first FIFO element is stored."]
        #[inline(always)]
        pub fn tos(
            self,
        ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, TocCi_SPEC, crate::common::RW> {
            crate::common::RegisterField::<1,0x3,1,0,u8, TocCi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Timeout Period   TOP. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set. Start value of the Timeout Counter  down counter . Configures the Timeout Period."]
        #[inline(always)]
        pub fn top(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, TocCi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, TocCi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for TocCi {
        #[inline(always)]
        fn default() -> TocCi {
            <crate::RegValueT<TocCi_SPEC> as RegisterValue<_>>::new(4294901760)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TocVi_SPEC;
    impl crate::sealed::RegSpec for TocVi_SPEC {
        type DataType = u32;
    }
    #[doc = "Timeout Counter Value 0\n resetvalue={Application Reset:0x0FFFF}"]
    pub type TocVi = crate::RegValueT<TocVi_SPEC>;

    impl TocVi {
        #[doc = "Timeout Counter   TOC. The Timeout Counter is decremented in multiples of CAN bit times    8201 1  8230 16  8201           depending on the configuration of TSCC.TCP. When decremented to zero         interrupt flag IR.TOO is set and the Timeout Counter is stopped. Start        and reset restart conditions are configured via TOCC.TOS. Any write access will lead to clearing of the counter."]
        #[inline(always)]
        pub fn toc(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, TocVi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, TocVi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for TocVi {
        #[inline(always)]
        fn default() -> TocVi {
            <crate::RegValueT<TocVi_SPEC> as RegisterValue<_>>::new(65535)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TscCi_SPEC;
    impl crate::sealed::RegSpec for TscCi_SPEC {
        type DataType = u32;
    }
    #[doc = "Timestamp Counter Configuration 0\n resetvalue={Application Reset:0x0}"]
    pub type TscCi = crate::RegValueT<TscCi_SPEC>;

    impl TscCi {
        #[doc = "Time segment before sample point   TSS. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
        #[inline(always)]
        pub fn tss(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, TscCi_SPEC, crate::common::RW> {
            crate::common::RegisterField::<0,0x3,1,0,u8, TscCi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Timestamp Counter Prescaler   TCP. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Configures the timestamp and timeout counters time unit in multiples of        CAN bit times    8201 1  8230 16  8201  . The actual interpretation by the hardware of        this value is such that one more than the value programmed here is used. With CAN FD an external counter is required for timestamp generation           TSS     8220 10  8221"]
        #[inline(always)]
        pub fn tcp(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, TscCi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, TscCi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for TscCi {
        #[inline(always)]
        fn default() -> TscCi {
            <crate::RegValueT<TscCi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TscVi_SPEC;
    impl crate::sealed::RegSpec for TscVi_SPEC {
        type DataType = u32;
    }
    #[doc = "Timestamp Counter Value 0\n resetvalue={Application Reset:0x0}"]
    pub type TscVi = crate::RegValueT<TscVi_SPEC>;

    impl TscVi {
        #[doc = "Timestamp Counter   TSC. The internal external Timestamp Counter value is captured on start of        frame  both Rx and Tx . When TSCC.TSS     8220 01  8221   the Timestamp Counter is        incremented in multiples of CAN bit times    8201 1  8230 16  8201   depending on the        configuration of TSCC.TCP. A wrap around sets interrupt flag IR.TSW. Write access resets the counter to zero. When TSCC.TSS  160    160   8220 10  8221   TSC reflects the external Timestamp Counter value.        A write access has no impact. A   8220 wrap around  8221  is a change of the Timestamp Counter value from          non zero to zero not caused by write access to TSCV."]
        #[inline(always)]
        pub fn tsc(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, TscVi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, TscVi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for TscVi {
        #[inline(always)]
        fn default() -> TscVi {
            <crate::RegValueT<TscVi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TtcRi_SPEC;
    impl crate::sealed::RegSpec for TtcRi_SPEC {
        type DataType = u32;
    }
    #[doc = "Time Trigger Control Register\n resetvalue={Application Reset:0x0}"]
    pub type TtcRi = crate::RegValueT<TtcRi_SPEC>;

    impl TtcRi {
        #[doc = "External Trigger Event Selection   ETESEL. This bit field defines the external trigger event that can be used to        trigger the transmission of the reference message. The event causes the        Event Trigger to be triggered. Control settings for this will not be        influenced."]
        #[inline(always)]
        pub fn etesel(
            self,
        ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, TtcRi_SPEC, crate::common::RW> {
            crate::common::RegisterField::<2,0x3,1,0,u8, TtcRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "External Trigger Source Selection   ETSSEL. This bit fields selects the input source for the external reference        message trigger."]
        #[inline(always)]
        pub fn etssel(
            self,
        ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, TtcRi_SPEC, crate::common::RW> {
            crate::common::RegisterField::<4,0x7,1,0,u8, TtcRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "TTCapture Time Trigger Source Select   TTCTSS. This bit selects the input source for the TT Capture Time  TTCPT         trigger. This register influences the stop watch event trigger"]
        #[inline(always)]
        pub fn ttctss(
            self,
        ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, TtcRi_SPEC, crate::common::RW> {
            crate::common::RegisterField::<9,0x7,1,0,u8, TtcRi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for TtcRi {
        #[inline(always)]
        fn default() -> TtcRi {
            <crate::RegValueT<TtcRi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct XidaMi_SPEC;
    impl crate::sealed::RegSpec for XidaMi_SPEC {
        type DataType = u32;
    }
    #[doc = "Extended ID AND Mask 0\n resetvalue={Application Reset:0x1FFFFFFF}"]
    pub type XidaMi = crate::RegValueT<XidaMi_SPEC>;

    impl XidaMi {
        #[doc = "Extended ID Mask   EIDM. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set. For acceptance filtering of extended frames the Extended ID AND Mask is ANDed with the Message ID of a received frame. Intended for masking of 29 bit IDs in SAE J1939. With the reset value of all bits set to one the mask is not active."]
        #[inline(always)]
        pub fn eidm(
            self,
        ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, XidaMi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1fffffff,1,0,u32, XidaMi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for XidaMi {
        #[inline(always)]
        fn default() -> XidaMi {
            <crate::RegValueT<XidaMi_SPEC> as RegisterValue<_>>::new(536870911)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct XidfCi_SPEC;
    impl crate::sealed::RegSpec for XidfCi_SPEC {
        type DataType = u32;
    }
    #[doc = "Extended ID Filter Configuration 0\n resetvalue={Application Reset:0x0}"]
    pub type XidfCi = crate::RegValueT<XidfCi_SPEC>;

    impl XidfCi {
        #[doc = "Filter List Extended Start Address   FLESA. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Start address of extended Message ID filter list  32 bit word addess ."]
        #[inline(always)]
        pub fn flesa(
            self,
        ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, XidfCi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x3fff,1,0,u16, XidfCi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "List Size Extended   LSE. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
        #[inline(always)]
        pub fn lse(
            self,
        ) -> crate::common::RegisterField<16, 0x7f, 1, 0, xidfci::Lse, XidfCi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                16,
                0x7f,
                1,
                0,
                xidfci::Lse,
                XidfCi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for XidfCi {
        #[inline(always)]
        fn default() -> XidfCi {
            <crate::RegValueT<XidfCi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod xidfci {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Lse_SPEC;
        pub type Lse = crate::EnumBitfieldStruct<u8, Lse_SPEC>;
        impl Lse {
            #[doc = "64 extended Message ID filter elements"]
            pub const REST_127: Self = Self::new(127);
        }
    }
    #[doc = "NT"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nt(pub(super) *mut u8);
    unsafe impl core::marker::Send for Nt {}
    unsafe impl core::marker::Sync for Nt {}
    impl Nt {
        #[doc = "Node 0 Timer A Transmit Trigger Register\n resetvalue={Application Reset:0x10000}"]
        #[inline(always)]
        pub const fn ntattri(&self) -> crate::common::Reg<nt::NtattRi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "Node 0 Timer B Transmit Trigger Register\n resetvalue={Application Reset:0x20000}"]
        #[inline(always)]
        pub const fn ntbttri(&self) -> crate::common::Reg<nt::NtbttRi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Node 0 Timer Clock Control Register\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ntccri(&self) -> crate::common::Reg<nt::NtccRi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Node 0 Timer C Transmit Trigger Register\n resetvalue={Application Reset:0x30000}"]
        #[inline(always)]
        pub const fn ntcttri(&self) -> crate::common::Reg<nt::NtcttRi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "Node 0 Timer Receive Timeout Register\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ntrtri(&self) -> crate::common::Reg<nt::NtrtRi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
        }
    }
    pub mod nt {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct NtattRi_SPEC;
        impl crate::sealed::RegSpec for NtattRi_SPEC {
            type DataType = u32;
        }
        #[doc = "Node 0 Timer A Transmit Trigger Register\n resetvalue={Application Reset:0x10000}"]
        pub type NtattRi = crate::RegValueT<NtattRi_SPEC>;

        impl NtattRi {
            #[doc = "Reload Value   RELOAD. This bit field contains the reload value for the timer. The timer will        restart when RELOAD is written."]
            #[inline(always)]
            pub fn reload(
                self,
            ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, NtattRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xffff,1,0,u16, NtattRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmit Message Object   TXMO. This transmit trigger is fixed to transmit buffer 1"]
            #[inline(always)]
            pub fn txmo(
                self,
            ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, NtattRi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<16,0xff,1,0,u8, NtattRi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Timer Start   STRT. This bit field controls the operation of the timer."]
            #[inline(always)]
            pub fn strt(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, NtattRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<24,1,0,NtattRi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for NtattRi {
            #[inline(always)]
            fn default() -> NtattRi {
                <crate::RegValueT<NtattRi_SPEC> as RegisterValue<_>>::new(65536)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct NtbttRi_SPEC;
        impl crate::sealed::RegSpec for NtbttRi_SPEC {
            type DataType = u32;
        }
        #[doc = "Node 0 Timer B Transmit Trigger Register\n resetvalue={Application Reset:0x20000}"]
        pub type NtbttRi = crate::RegValueT<NtbttRi_SPEC>;

        impl NtbttRi {
            #[doc = "Reload Value   RELOAD. This bit field contains the reload value for the timer. The timer will restart when RELOAD is written."]
            #[inline(always)]
            pub fn reload(
                self,
            ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, NtbttRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xffff,1,0,u16, NtbttRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmit Message Object   TXMO. This transmit object is fixed to transmit buffer 2"]
            #[inline(always)]
            pub fn txmo(
                self,
            ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, NtbttRi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<16,0xff,1,0,u8, NtbttRi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Timer Start   STRT. This bit field controls the operation of the timer."]
            #[inline(always)]
            pub fn strt(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, NtbttRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<24,1,0,NtbttRi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for NtbttRi {
            #[inline(always)]
            fn default() -> NtbttRi {
                <crate::RegValueT<NtbttRi_SPEC> as RegisterValue<_>>::new(131072)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct NtccRi_SPEC;
        impl crate::sealed::RegSpec for NtccRi_SPEC {
            type DataType = u32;
        }
        #[doc = "Node 0 Timer Clock Control Register\n resetvalue={Application Reset:0x0}"]
        pub type NtccRi = crate::RegValueT<NtccRi_SPEC>;

        impl NtccRi {
            #[doc = "Timer Prescaler   TPSC. The duration of one timer clock is given by  TPSC  160    160 1  CAN bit times for        all NTCCRi.TRIGSRC settings."]
            #[inline(always)]
            pub fn tpsc(
                self,
            ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, NtccRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<8,0xf,1,0,u8, NtccRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Stamping Reset   STRESET. This bit is not implemented in TC39x A step silicon. This bit gives the possibility to reset the time stamp for CAN FD messages."]
            #[inline(always)]
            pub fn streset(
                self,
            ) -> crate::common::RegisterFieldBool<14, 1, 0, NtccRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<14,1,0,NtccRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Stamping Start   STSTART. This bit is not implemented in TC39x A step silicon. This bit starts the external timer used for CAN FD messages. The source and the prescaler are identical to the timers A B C."]
            #[inline(always)]
            pub fn ststart(
                self,
            ) -> crate::common::RegisterFieldBool<15, 1, 0, NtccRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<15,1,0,NtccRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Trigger Source   TRIGSRC. This bit selects the trigger source for the different modes in the node        timer."]
            #[inline(always)]
            pub fn trigsrc(
                self,
            ) -> crate::common::RegisterField<18, 0x7, 1, 0, u8, NtccRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<18,0x7,1,0,u8, NtccRi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for NtccRi {
            #[inline(always)]
            fn default() -> NtccRi {
                <crate::RegValueT<NtccRi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct NtcttRi_SPEC;
        impl crate::sealed::RegSpec for NtcttRi_SPEC {
            type DataType = u32;
        }
        #[doc = "Node 0 Timer C Transmit Trigger Register\n resetvalue={Application Reset:0x30000}"]
        pub type NtcttRi = crate::RegValueT<NtcttRi_SPEC>;

        impl NtcttRi {
            #[doc = "Reload Value   RELOAD. This bit field contains the reload value for the timer. The timer will        restart when RELOAD is written."]
            #[inline(always)]
            pub fn reload(
                self,
            ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, NtcttRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xffff,1,0,u16, NtcttRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmit Message Object   TXMO. This transmit trigger is fixed to transmit buffer 3"]
            #[inline(always)]
            pub fn txmo(
                self,
            ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, NtcttRi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<16,0xff,1,0,u8, NtcttRi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Timer Start   STRT. This bit field controls the operation of the timer."]
            #[inline(always)]
            pub fn strt(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, NtcttRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<24,1,0,NtcttRi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for NtcttRi {
            #[inline(always)]
            fn default() -> NtcttRi {
                <crate::RegValueT<NtcttRi_SPEC> as RegisterValue<_>>::new(196608)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct NtrtRi_SPEC;
        impl crate::sealed::RegSpec for NtrtRi_SPEC {
            type DataType = u32;
        }
        #[doc = "Node 0 Timer Receive Timeout Register\n resetvalue={Application Reset:0x0}"]
        pub type NtrtRi = crate::RegValueT<NtrtRi_SPEC>;

        impl NtrtRi {
            #[doc = "Reload Value   RELOAD. This bit field contains the reload value for the timer. The timer will        start when RELOAD   8800  0 is written. After half the time of the RELOAD        value  the interrupt flags of the receive buffers will be cleared        automatically  to ensure  that no message receive will be missed."]
            #[inline(always)]
            pub fn reload(
                self,
            ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, NtrtRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xffff,1,0,u16, NtrtRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Timer Event Interrupt Enable   TEIE. This bit enables the node timer event interrupt of CAN node i. Bit field GRINT2.RETI selects the interrupt output line which becomes        activated at this type of interrupt."]
            #[inline(always)]
            pub fn teie(
                self,
            ) -> crate::common::RegisterFieldBool<22, 1, 0, NtrtRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<22,1,0,NtrtRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Timer Event   TE. This flag is set on a node timer transition from 1 to 0 in Receive        Timeout Mode. This bit must be reset  i.e Write to   8216 0  8217   by software         writing a   8216 1  8217  has no effect. An interrupt request is generated if TEIE   1."]
            #[inline(always)]
            pub fn te(
                self,
            ) -> crate::common::RegisterFieldBool<23, 1, 0, NtrtRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<23,1,0,NtrtRi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for NtrtRi {
            #[inline(always)]
            fn default() -> NtrtRi {
                <crate::RegValueT<NtrtRi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
    #[doc = "RX"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rx(pub(super) *mut u8);
    unsafe impl core::marker::Send for Rx {}
    unsafe impl core::marker::Sync for Rx {}
    impl Rx {
        #[doc = "Rx Buffer Configuration 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn rxbci(&self) -> crate::common::Reg<rx::RxbCi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "Rx Buffer FIFO Element Size Configuration 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn rxesci(&self) -> crate::common::Reg<rx::RxesCi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
        }
        #[doc = "Rx FIFO 0 Acknowledge 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn rxf0ai(&self) -> crate::common::Reg<rx::Rxf0Ai_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Rx FIFO 0 Configuration 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn rxf0ci(&self) -> crate::common::Reg<rx::Rxf0Ci_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Rx FIFO 0 Status 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn rxf0si(&self) -> crate::common::Reg<rx::Rxf0Si_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "Rx FIFO 1 Acknowledge 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn rxf1ai(&self) -> crate::common::Reg<rx::Rxf1Ai_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
        }
        #[doc = "Rx FIFO 1 Configuration 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn rxf1ci(&self) -> crate::common::Reg<rx::Rxf1Ci_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "Rx FIFO 1 Status 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn rxf1si(&self) -> crate::common::Reg<rx::Rxf1Si_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
        }
    }
    pub mod rx {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxbCi_SPEC;
        impl crate::sealed::RegSpec for RxbCi_SPEC {
            type DataType = u32;
        }
        #[doc = "Rx Buffer Configuration 0\n resetvalue={Application Reset:0x0}"]
        pub type RxbCi = crate::RegValueT<RxbCi_SPEC>;

        impl RxbCi {
            #[doc = "Rx Buffer Start Address   RBSA. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Configures the start address of the Rx Buffers section in the Message        RAM  32 bit word address . Also        used to reference debug messages A  B  C."]
            #[inline(always)]
            pub fn rbsa(
                self,
            ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, RxbCi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<2,0x3fff,1,0,u16, RxbCi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for RxbCi {
            #[inline(always)]
            fn default() -> RxbCi {
                <crate::RegValueT<RxbCi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxesCi_SPEC;
        impl crate::sealed::RegSpec for RxesCi_SPEC {
            type DataType = u32;
        }
        #[doc = "Rx Buffer FIFO Element Size Configuration 0\n resetvalue={Application Reset:0x0}"]
        pub type RxesCi = crate::RegValueT<RxesCi_SPEC>;

        impl RxesCi {
            #[doc = "Rx FIFO 0 Data Field Size   F0DS. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set. In case the data field size of an accepted CAN frame exceeds the data field size configured for the matching Rx Buffer or Rx FIFO  only the number of bytes as configured by RXESC are stored to the Rx Buffer resp. Rx FIFO element. The rest of the frame s data field is ignored."]
            #[inline(always)]
            pub fn f0ds(
                self,
            ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, RxesCi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0x7,1,0,u8, RxesCi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Rx FIFO 1 Data Field Size   F1DS. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set."]
            #[inline(always)]
            pub fn f1ds(
                self,
            ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, RxesCi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<4,0x7,1,0,u8, RxesCi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Rx Buffer Data Field Size   RBDS. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set."]
            #[inline(always)]
            pub fn rbds(
                self,
            ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, RxesCi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<8,0x7,1,0,u8, RxesCi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for RxesCi {
            #[inline(always)]
            fn default() -> RxesCi {
                <crate::RegValueT<RxesCi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxf0Ai_SPEC;
        impl crate::sealed::RegSpec for Rxf0Ai_SPEC {
            type DataType = u32;
        }
        #[doc = "Rx FIFO 0 Acknowledge 0\n resetvalue={Application Reset:0x0}"]
        pub type Rxf0Ai = crate::RegValueT<Rxf0Ai_SPEC>;

        impl Rxf0Ai {
            #[doc = "Rx FIFO 0 Acknowledge Index   F0AI. After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This will set the Rx FIFO 0 Get Index RXF0S.F0GI to F0AI   1 and update the FIFO 0 Fill Level RXF0S.F0FL."]
            #[inline(always)]
            pub fn f0ai(
                self,
            ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Rxf0Ai_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0x3f,1,0,u8, Rxf0Ai_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for Rxf0Ai {
            #[inline(always)]
            fn default() -> Rxf0Ai {
                <crate::RegValueT<Rxf0Ai_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxf0Ci_SPEC;
        impl crate::sealed::RegSpec for Rxf0Ci_SPEC {
            type DataType = u32;
        }
        #[doc = "Rx FIFO 0 Configuration 0\n resetvalue={Application Reset:0x0}"]
        pub type Rxf0Ci = crate::RegValueT<Rxf0Ci_SPEC>;

        impl Rxf0Ci {
            #[doc = "Rx FIFO 0 Start Address   F0SA. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Start address of Rx FIFO 0 in Message RAM  32 bit word address  see CROSSREFERENCE  ."]
            #[inline(always)]
            pub fn f0sa(
                self,
            ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, Rxf0Ci_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<2,0x3fff,1,0,u16, Rxf0Ci_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Rx FIFO 0 Size   F0S. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn f0s(
                self,
            ) -> crate::common::RegisterField<
                16,
                0x7f,
                1,
                0,
                rxf0ci::F0S,
                Rxf0Ci_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    16,
                    0x7f,
                    1,
                    0,
                    rxf0ci::F0S,
                    Rxf0Ci_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Rx FIFO 0 Watermark   F0WM. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn f0wm(
                self,
            ) -> crate::common::RegisterField<
                24,
                0x7f,
                1,
                0,
                rxf0ci::F0Wm,
                Rxf0Ci_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    24,
                    0x7f,
                    1,
                    0,
                    rxf0ci::F0Wm,
                    Rxf0Ci_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "FIFO 0 Operation Mode   F0OM. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. FIFO 0 can be operated in blocking or in overwrite mode  see CROSSREFERENCE  ."]
            #[inline(always)]
            pub fn f0om(
                self,
            ) -> crate::common::RegisterFieldBool<31, 1, 0, Rxf0Ci_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<31,1,0,Rxf0Ci_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for Rxf0Ci {
            #[inline(always)]
            fn default() -> Rxf0Ci {
                <crate::RegValueT<Rxf0Ci_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod rxf0ci {

            #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
            pub struct F0S_SPEC;
            pub type F0S = crate::EnumBitfieldStruct<u8, F0S_SPEC>;
            impl F0S {
                #[doc = "64 Rx FIFO 0 elements"]
                pub const REST_127: Self = Self::new(127);
            }
            #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
            pub struct F0Wm_SPEC;
            pub type F0Wm = crate::EnumBitfieldStruct<u8, F0Wm_SPEC>;
            impl F0Wm {
                #[doc = "Watermark interrupt disabled"]
                pub const REST_127: Self = Self::new(127);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxf0Si_SPEC;
        impl crate::sealed::RegSpec for Rxf0Si_SPEC {
            type DataType = u32;
        }
        #[doc = "Rx FIFO 0 Status 0\n resetvalue={Application Reset:0x0}"]
        pub type Rxf0Si = crate::RegValueT<Rxf0Si_SPEC>;

        impl Rxf0Si {
            #[doc = "Rx FIFO 0 Fill Level   F0FL. Number of elements stored in Rx FIFO 0  range 0 to 64."]
            #[inline(always)]
            pub fn f0fl(
                self,
            ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Rxf0Si_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0x7f,1,0,u8, Rxf0Si_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Rx FIFO 0 Get Index   F0GI. Rx FIFO 0 read index pointer  range 0 to 63."]
            #[inline(always)]
            pub fn f0gi(
                self,
            ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, Rxf0Si_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<8,0x3f,1,0,u8, Rxf0Si_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Rx FIFO 0 Put Index   F0PI. Rx FIFO 0 write index pointer  range 0 to 63."]
            #[inline(always)]
            pub fn f0pi(
                self,
            ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Rxf0Si_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<16,0x3f,1,0,u8, Rxf0Si_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Rx FIFO 0 Full   F0F"]
            #[inline(always)]
            pub fn f0f(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, Rxf0Si_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,Rxf0Si_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Rx FIFO 0 Message Lost   RF0L. This bit is a copy of interrupt flag IR.RF0L. When IR.RF0L is reset  this bit is also reset. Overwriting the oldest message when RXF0C.F0OM    1  will not set this flag."]
            #[inline(always)]
            pub fn rf0l(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, Rxf0Si_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<25,1,0,Rxf0Si_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for Rxf0Si {
            #[inline(always)]
            fn default() -> Rxf0Si {
                <crate::RegValueT<Rxf0Si_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxf1Ai_SPEC;
        impl crate::sealed::RegSpec for Rxf1Ai_SPEC {
            type DataType = u32;
        }
        #[doc = "Rx FIFO 1 Acknowledge 0\n resetvalue={Application Reset:0x0}"]
        pub type Rxf1Ai = crate::RegValueT<Rxf1Ai_SPEC>;

        impl Rxf1Ai {
            #[doc = "Rx FIFO 1 Acknowledge Index   F1AI. After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This will set the Rx FIFO 1 Get Index RXF1S.F1GI to F1AI   1 and update the FIFO 1 Fill Level RXF1S.F1FL"]
            #[inline(always)]
            pub fn f1ai(
                self,
            ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Rxf1Ai_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0x3f,1,0,u8, Rxf1Ai_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for Rxf1Ai {
            #[inline(always)]
            fn default() -> Rxf1Ai {
                <crate::RegValueT<Rxf1Ai_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxf1Ci_SPEC;
        impl crate::sealed::RegSpec for Rxf1Ci_SPEC {
            type DataType = u32;
        }
        #[doc = "Rx FIFO 1 Configuration 0\n resetvalue={Application Reset:0x0}"]
        pub type Rxf1Ci = crate::RegValueT<Rxf1Ci_SPEC>;

        impl Rxf1Ci {
            #[doc = "Rx FIFO 1 Start Address   F1SA. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Start address of Rx FIFO 1 in Message RAM  32 bit word address ."]
            #[inline(always)]
            pub fn f1sa(
                self,
            ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, Rxf1Ci_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<2,0x3fff,1,0,u16, Rxf1Ci_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Rx FIFO 1 Size   F1S. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn f1s(
                self,
            ) -> crate::common::RegisterField<
                16,
                0x7f,
                1,
                0,
                rxf1ci::F1S,
                Rxf1Ci_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    16,
                    0x7f,
                    1,
                    0,
                    rxf1ci::F1S,
                    Rxf1Ci_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Rx FIFO 1 Watermark   F1WM. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn f1wm(
                self,
            ) -> crate::common::RegisterField<
                24,
                0x7f,
                1,
                0,
                rxf1ci::F1Wm,
                Rxf1Ci_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    24,
                    0x7f,
                    1,
                    0,
                    rxf1ci::F1Wm,
                    Rxf1Ci_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "FIFO 1 Operation Mode   F1OM. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. FIFO 1 can be operated in blocking or in overwrite mode."]
            #[inline(always)]
            pub fn f1om(
                self,
            ) -> crate::common::RegisterFieldBool<31, 1, 0, Rxf1Ci_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<31,1,0,Rxf1Ci_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for Rxf1Ci {
            #[inline(always)]
            fn default() -> Rxf1Ci {
                <crate::RegValueT<Rxf1Ci_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod rxf1ci {

            #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
            pub struct F1S_SPEC;
            pub type F1S = crate::EnumBitfieldStruct<u8, F1S_SPEC>;
            impl F1S {
                #[doc = "64 Rx FIFO 1 elements"]
                pub const REST_127: Self = Self::new(127);
            }
            #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
            pub struct F1Wm_SPEC;
            pub type F1Wm = crate::EnumBitfieldStruct<u8, F1Wm_SPEC>;
            impl F1Wm {
                #[doc = "Watermark interrupt disabled"]
                pub const REST_127: Self = Self::new(127);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxf1Si_SPEC;
        impl crate::sealed::RegSpec for Rxf1Si_SPEC {
            type DataType = u32;
        }
        #[doc = "Rx FIFO 1 Status 0\n resetvalue={Application Reset:0x0}"]
        pub type Rxf1Si = crate::RegValueT<Rxf1Si_SPEC>;

        impl Rxf1Si {
            #[doc = "Rx FIFO 1 Fill Level   F1FL. Number of elements stored in Rx FIFO 1  range 0 to 64."]
            #[inline(always)]
            pub fn f1fl(
                self,
            ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Rxf1Si_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0x7f,1,0,u8, Rxf1Si_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Rx FIFO 1 Get Index   F1GI. Rx FIFO 1 read index pointer  range 0 to 63."]
            #[inline(always)]
            pub fn f1gi(
                self,
            ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, Rxf1Si_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<8,0x3f,1,0,u8, Rxf1Si_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Rx FIFO 1 Put Index   F1PI. Rx FIFO 1 write index pointer  range 0 to 63."]
            #[inline(always)]
            pub fn f1pi(
                self,
            ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Rxf1Si_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<16,0x3f,1,0,u8, Rxf1Si_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Rx FIFO 1 Full   F1F"]
            #[inline(always)]
            pub fn f1f(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, Rxf1Si_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,Rxf1Si_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Rx FIFO 1 Message Lost   RF1L. This bit is a copy of interrupt flag IR.RF1L. When IR.RF1L is reset  this bit is also reset. Overwriting the oldest message when RXF1C.F1OM    1  will not set this flag."]
            #[inline(always)]
            pub fn rf1l(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, Rxf1Si_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<25,1,0,Rxf1Si_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for Rxf1Si {
            #[inline(always)]
            fn default() -> Rxf1Si {
                <crate::RegValueT<Rxf1Si_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
    #[doc = "TT"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tt(pub(super) *mut u8);
    unsafe impl core::marker::Send for Tt {}
    unsafe impl core::marker::Sync for Tt {}
    impl Tt {
        #[doc = "TT Capture Time 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ttcpti(&self) -> crate::common::Reg<tt::TtcpTi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
        }
        #[doc = "TT Cycle Sync Mark 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ttcsmi(&self) -> crate::common::Reg<tt::TtcsMi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
        }
        #[doc = "TT Cycle Time   Count 0\n resetvalue={Application Reset:0x3F0000}"]
        #[inline(always)]
        pub const fn ttctci(&self) -> crate::common::Reg<tt::TtctCi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
        }
        #[doc = "TT Global Time Preset 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ttgtpi(&self) -> crate::common::Reg<tt::TtgtPi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
        }
        #[doc = "TT Interrupt Enable 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ttiei(&self) -> crate::common::Reg<tt::TtiEi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
        }
        #[doc = "TT Interrupt Register 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ttiri(&self) -> crate::common::Reg<tt::TtiRi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
        }
        #[doc = "TT Local   Global Time 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ttlgti(&self) -> crate::common::Reg<tt::TtlgTi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
        }
        #[doc = "TT Matrix Limits 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ttmlmi(&self) -> crate::common::Reg<tt::TtmlMi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "TT Operation Configuration 0\n resetvalue={Application Reset:0x10000}"]
        #[inline(always)]
        pub const fn ttocfi(&self) -> crate::common::Reg<tt::TtocFi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "TT Operation Control 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ttocni(&self) -> crate::common::Reg<tt::TtocNi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "TT Operation Status 0\n resetvalue={Application Reset:0x20000080}"]
        #[inline(always)]
        pub const fn ttosti(&self) -> crate::common::Reg<tt::TtosTi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
        }
        #[doc = "TT Reference Message Configuration 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ttrmci(&self) -> crate::common::Reg<tt::TtrmCi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "TT Trigger Memory Configuration 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn tttmci(&self) -> crate::common::Reg<tt::TttmCi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "TT Time Mark 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn tttmki(&self) -> crate::common::Reg<tt::TttmKi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
        }
        #[doc = "TUR Configuration 0\n resetvalue={Application Reset:0x10000000}"]
        #[inline(always)]
        pub const fn turcfi(&self) -> crate::common::Reg<tt::TurcFi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "TUR Numerator Actual 0\n resetvalue={Application Reset:0x10000}"]
        #[inline(always)]
        pub const fn turnai(&self) -> crate::common::Reg<tt::TurnAi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
        }
    }
    pub mod tt {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtcpTi_SPEC;
        impl crate::sealed::RegSpec for TtcpTi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Capture Time 0\n resetvalue={Application Reset:0x0}"]
        pub type TtcpTi = crate::RegValueT<TtcpTi_SPEC>;

        impl TtcpTi {
            #[doc = "Cycle Count Value   CCV. Cycle count value captured together with SWV. Captured cycle count value"]
            #[inline(always)]
            pub fn ccv(
                self,
            ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, TtcpTi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0x3f,1,0,u8, TtcpTi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Stop Watch Value   SWV. On a rising falling edge  as configured via TTOCN.SWP  at the Stop Watch        Trigger  when TTOCN.SWS is   8800    8220 00  8221  and TTIR.SWE is   8216 0  8217   the actual time        value as selected by TTOCN.SWS  cycle  local  global  is copied to SWV        and TTIR.SWE will be set to   8216 1  8217 . Capturing of the next stop watch value        is enabled by resetting TTIR.SWE. Captured Stop Watch value"]
            #[inline(always)]
            pub fn swv(
                self,
            ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, TtcpTi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<16,0xffff,1,0,u16, TtcpTi_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for TtcpTi {
            #[inline(always)]
            fn default() -> TtcpTi {
                <crate::RegValueT<TtcpTi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtcsMi_SPEC;
        impl crate::sealed::RegSpec for TtcsMi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Cycle Sync Mark 0\n resetvalue={Application Reset:0x0}"]
        pub type TtcsMi = crate::RegValueT<TtcsMi_SPEC>;

        impl TtcsMi {
            #[doc = "Cycle Sync Mark   CSM. The Cycle Sync Mark is measured in cycle time. It is updated when the        reference message becomes valid and retains its value until the next        reference message becomes valid. Captured cycle time"]
            #[inline(always)]
            pub fn csm(
                self,
            ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, TtcsMi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0xffff,1,0,u16, TtcsMi_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for TtcsMi {
            #[inline(always)]
            fn default() -> TtcsMi {
                <crate::RegValueT<TtcsMi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtctCi_SPEC;
        impl crate::sealed::RegSpec for TtctCi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Cycle Time   Count 0\n resetvalue={Application Reset:0x3F0000}"]
        pub type TtctCi = crate::RegValueT<TtctCi_SPEC>;

        impl TtctCi {
            #[doc = "Cycle Time   CT. Non fractional part of the difference of the node  8217 s local time and        Ref Mark. Cycle time value of TTCAN Basic Cycle"]
            #[inline(always)]
            pub fn ct(
                self,
            ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, TtctCi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0xffff,1,0,u16, TtctCi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cycle Count   CC. Number of actual Basic Cycle in the System Matrix"]
            #[inline(always)]
            pub fn cc(
                self,
            ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, TtctCi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<16,0x3f,1,0,u8, TtctCi_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for TtctCi {
            #[inline(always)]
            fn default() -> TtctCi {
                <crate::RegValueT<TtctCi_SPEC> as RegisterValue<_>>::new(4128768)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtgtPi_SPEC;
        impl crate::sealed::RegSpec for TtgtPi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Global Time Preset 0\n resetvalue={Application Reset:0x0}"]
        pub type TtgtPi = crate::RegValueT<TtgtPi_SPEC>;

        impl TtgtPi {
            #[doc = "Time Preset   TP. CTP is write protected while TTOCN.ESCN or TTOST.SPL are set."]
            #[inline(always)]
            pub fn tp(
                self,
            ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, TtgtPi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xffff,1,0,u16, TtgtPi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cycle Time Target Phase   CTP. CTP is write protected while TTOCN.ESCN or TTOST.SPL are set."]
            #[inline(always)]
            pub fn ctp(
                self,
            ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, TtgtPi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0xffff,1,0,u16, TtgtPi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for TtgtPi {
            #[inline(always)]
            fn default() -> TtgtPi {
                <crate::RegValueT<TtgtPi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtiEi_SPEC;
        impl crate::sealed::RegSpec for TtiEi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Interrupt Enable 0\n resetvalue={Application Reset:0x0}"]
        pub type TtiEi = crate::RegValueT<TtiEi_SPEC>;

        impl TtiEi {
            #[doc = "Start of Basic Cycle Interrupt Enable   SBCE"]
            #[inline(always)]
            pub fn sbce(
                self,
            ) -> crate::common::RegisterFieldBool<0, 1, 0, TtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<0,1,0,TtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Start of Matrix Cycle Interrupt Enable   SMCE"]
            #[inline(always)]
            pub fn smce(
                self,
            ) -> crate::common::RegisterFieldBool<1, 1, 0, TtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<1,1,0,TtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Change of Synchronization Mode Interrupt Enable   CSME"]
            #[inline(always)]
            pub fn csme(
                self,
            ) -> crate::common::RegisterFieldBool<2, 1, 0, TtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<2,1,0,TtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Start of Gap Interrupt Enable   SOGE"]
            #[inline(always)]
            pub fn soge(
                self,
            ) -> crate::common::RegisterFieldBool<3, 1, 0, TtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<3,1,0,TtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Register Time Mark Interrupt Enable   RTMIE"]
            #[inline(always)]
            pub fn rtmie(
                self,
            ) -> crate::common::RegisterFieldBool<4, 1, 0, TtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<4,1,0,TtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Trigger Time Mark Event Internal Interrupt Enable   TTMIE"]
            #[inline(always)]
            pub fn ttmie(
                self,
            ) -> crate::common::RegisterFieldBool<5, 1, 0, TtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<5,1,0,TtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Stop Watch Polarity Interrupt Enable   SWEE"]
            #[inline(always)]
            pub fn swee(
                self,
            ) -> crate::common::RegisterFieldBool<6, 1, 0, TtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<6,1,0,TtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Global Time Wrap Interrupt Enable   GTWE"]
            #[inline(always)]
            pub fn gtwe(
                self,
            ) -> crate::common::RegisterFieldBool<7, 1, 0, TtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<7,1,0,TtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Global Time Discontinuity Interrupt Enable   GTDE"]
            #[inline(always)]
            pub fn gtde(
                self,
            ) -> crate::common::RegisterFieldBool<8, 1, 0, TtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<8,1,0,TtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Global Time Error Interrupt Enable   GTEE"]
            #[inline(always)]
            pub fn gtee(
                self,
            ) -> crate::common::RegisterFieldBool<9, 1, 0, TtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<9,1,0,TtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Tx Count Underflow Interrupt Enable   TXUE"]
            #[inline(always)]
            pub fn txue(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, TtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,TtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Tx Count Overflow Interrupt Enable   TXOE"]
            #[inline(always)]
            pub fn txoe(
                self,
            ) -> crate::common::RegisterFieldBool<11, 1, 0, TtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<11,1,0,TtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Scheduling Error 1 Interrupt Enable   SE1E"]
            #[inline(always)]
            pub fn se1e(
                self,
            ) -> crate::common::RegisterFieldBool<12, 1, 0, TtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<12,1,0,TtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Scheduling Error 2 Interrupt Enable   SE2E"]
            #[inline(always)]
            pub fn se2e(
                self,
            ) -> crate::common::RegisterFieldBool<13, 1, 0, TtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<13,1,0,TtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Change Error Level Interrupt Enable   ELCE"]
            #[inline(always)]
            pub fn elce(
                self,
            ) -> crate::common::RegisterFieldBool<14, 1, 0, TtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<14,1,0,TtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Initialization Watch Trigger Interrupt Enable   IWTE"]
            #[inline(always)]
            pub fn iwte(
                self,
            ) -> crate::common::RegisterFieldBool<15, 1, 0, TtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<15,1,0,TtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Watch Trigger Interrupt Enable   WTE"]
            #[inline(always)]
            pub fn wte(
                self,
            ) -> crate::common::RegisterFieldBool<16, 1, 0, TtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<16,1,0,TtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Application Watchdog Interrupt Enable   AWE"]
            #[inline(always)]
            pub fn awe(
                self,
            ) -> crate::common::RegisterFieldBool<17, 1, 0, TtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<17,1,0,TtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Configuration Error Interrupt Enable   CERE"]
            #[inline(always)]
            pub fn cere(
                self,
            ) -> crate::common::RegisterFieldBool<18, 1, 0, TtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<18,1,0,TtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for TtiEi {
            #[inline(always)]
            fn default() -> TtiEi {
                <crate::RegValueT<TtiEi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtiRi_SPEC;
        impl crate::sealed::RegSpec for TtiRi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Interrupt Register 0\n resetvalue={Application Reset:0x0}"]
        pub type TtiRi = crate::RegValueT<TtiRi_SPEC>;

        impl TtiRi {
            #[doc = "Start of Basic Cycle   SBC"]
            #[inline(always)]
            pub fn sbc(
                self,
            ) -> crate::common::RegisterFieldBool<0, 1, 0, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<0,1,0,TtiRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Start of Matrix Cycle   SMC"]
            #[inline(always)]
            pub fn smc(
                self,
            ) -> crate::common::RegisterFieldBool<1, 1, 0, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<1,1,0,TtiRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Change of Synchronization Mode   CSM"]
            #[inline(always)]
            pub fn csm(
                self,
            ) -> crate::common::RegisterFieldBool<2, 1, 0, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<2,1,0,TtiRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Start of Gap   SOG"]
            #[inline(always)]
            pub fn sog(
                self,
            ) -> crate::common::RegisterFieldBool<3, 1, 0, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<3,1,0,TtiRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Register Time Mark Interrupt   RTMI. Set when time referenced by TTOCN.TMC         cycle  local  or global  equals TTTMK.TM         independent of the synchronization state."]
            #[inline(always)]
            pub fn rtmi(
                self,
            ) -> crate::common::RegisterFieldBool<4, 1, 0, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<4,1,0,TtiRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Trigger Time Mark Event Internal   TTMI. Internal time mark events are configured by trigger memory element TMIN.        Set when the trigger memory element becomes active  and the M CAN is in synchronization state In Gap or In Schedule."]
            #[inline(always)]
            pub fn ttmi(
                self,
            ) -> crate::common::RegisterFieldBool<5, 1, 0, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<5,1,0,TtiRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Stop Watch Polarity   SWE"]
            #[inline(always)]
            pub fn swe(
                self,
            ) -> crate::common::RegisterFieldBool<6, 1, 0, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<6,1,0,TtiRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Global Time Wrap   GTW"]
            #[inline(always)]
            pub fn gtw(
                self,
            ) -> crate::common::RegisterFieldBool<7, 1, 0, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<7,1,0,TtiRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Global Time Discontinuity   GTD"]
            #[inline(always)]
            pub fn gtd(
                self,
            ) -> crate::common::RegisterFieldBool<8, 1, 0, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<8,1,0,TtiRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Global Time Error   GTE. Synchronization deviation SD exceeds limit specified by TTOCF.LDSDL         TTCAN Level  160 0 2 only."]
            #[inline(always)]
            pub fn gte(
                self,
            ) -> crate::common::RegisterFieldBool<9, 1, 0, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<9,1,0,TtiRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Tx Count Underflow   TXU"]
            #[inline(always)]
            pub fn txu(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,TtiRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Tx Count Overflow   TXO"]
            #[inline(always)]
            pub fn txo(
                self,
            ) -> crate::common::RegisterFieldBool<11, 1, 0, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<11,1,0,TtiRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Scheduling Error 1   SE1"]
            #[inline(always)]
            pub fn se1(
                self,
            ) -> crate::common::RegisterFieldBool<12, 1, 0, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<12,1,0,TtiRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Scheduling Error 2   SE2"]
            #[inline(always)]
            pub fn se2(
                self,
            ) -> crate::common::RegisterFieldBool<13, 1, 0, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<13,1,0,TtiRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Level Changed   ELC. Not set when error level changed during initialization."]
            #[inline(always)]
            pub fn elc(
                self,
            ) -> crate::common::RegisterFieldBool<14, 1, 0, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<14,1,0,TtiRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Initialization Watch Trigger   IWT. The initialization is restarted by resetting IWT."]
            #[inline(always)]
            pub fn iwt(
                self,
            ) -> crate::common::RegisterFieldBool<15, 1, 0, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<15,1,0,TtiRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Watch Trigger   WT"]
            #[inline(always)]
            pub fn wt(
                self,
            ) -> crate::common::RegisterFieldBool<16, 1, 0, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<16,1,0,TtiRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Application Watchdog   AW"]
            #[inline(always)]
            pub fn aw(
                self,
            ) -> crate::common::RegisterFieldBool<17, 1, 0, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<17,1,0,TtiRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Configuration Error   CER. Trigger out of order."]
            #[inline(always)]
            pub fn cer(
                self,
            ) -> crate::common::RegisterFieldBool<18, 1, 0, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<18,1,0,TtiRi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for TtiRi {
            #[inline(always)]
            fn default() -> TtiRi {
                <crate::RegValueT<TtiRi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtlgTi_SPEC;
        impl crate::sealed::RegSpec for TtlgTi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Local   Global Time 0\n resetvalue={Application Reset:0x0}"]
        pub type TtlgTi = crate::RegValueT<TtlgTi_SPEC>;

        impl TtlgTi {
            #[doc = "Local Time   LT. Non fractional part of local time  incremented once each local NTU. Local time value of TTCAN node"]
            #[inline(always)]
            pub fn lt(
                self,
            ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, TtlgTi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0xffff,1,0,u16, TtlgTi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Global Time   GT. Non fractional part of the sum of the node  8217 s local time and its local        offset. Global time value of TTCAN network"]
            #[inline(always)]
            pub fn gt(
                self,
            ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, TtlgTi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<16,0xffff,1,0,u16, TtlgTi_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for TtlgTi {
            #[inline(always)]
            fn default() -> TtlgTi {
                <crate::RegValueT<TtlgTi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtmlMi_SPEC;
        impl crate::sealed::RegSpec for TtmlMi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Matrix Limits 0\n resetvalue={Application Reset:0x0}"]
        pub type TtmlMi = crate::RegValueT<TtmlMi_SPEC>;

        impl TtmlMi {
            #[doc = "Cycle Count Max   CCM. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. ISO 11898 4  5.2.1 requires  that only the listed cycle count values          are configured. Other values are possible but may lead to inconsistent          matrix cycles."]
            #[inline(always)]
            pub fn ccm(
                self,
            ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, TtmlMi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0x3f,1,0,u8, TtmlMi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cycle Start Synchronization   CSS. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Enables sync pulse output at start of cycle."]
            #[inline(always)]
            pub fn css(
                self,
            ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, TtmlMi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<6,0x3,1,0,u8, TtmlMi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Tx Enable Window   TXEW. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Length of Tx enable window  1 16 NTU cycles"]
            #[inline(always)]
            pub fn txew(
                self,
            ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, TtmlMi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<8,0xf,1,0,u8, TtmlMi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Expected Number of Tx Triggers   ENTT. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Expected number of Tx Triggers in one Matrix Cycle"]
            #[inline(always)]
            pub fn entt(
                self,
            ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, TtmlMi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0xfff,1,0,u16, TtmlMi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for TtmlMi {
            #[inline(always)]
            fn default() -> TtmlMi {
                <crate::RegValueT<TtmlMi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtocFi_SPEC;
        impl crate::sealed::RegSpec for TtocFi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Operation Configuration 0\n resetvalue={Application Reset:0x10000}"]
        pub type TtocFi = crate::RegValueT<TtocFi_SPEC>;

        impl TtocFi {
            #[doc = "Operation Mode   OM. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn om(
                self,
            ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, TtocFi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0x3,1,0,u8, TtocFi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Gap Enable   GEN. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn gen(
                self,
            ) -> crate::common::RegisterFieldBool<3, 1, 0, TtocFi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<3,1,0,TtocFi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Time Master   TM. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn tm(
                self,
            ) -> crate::common::RegisterFieldBool<4, 1, 0, TtocFi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<4,1,0,TtocFi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "LD of Synchronization Deviation Limit   LDSDL. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. The Synchronization Deviation Limit SDL is configured by its dual        logarithm LDSDL with SDL   2  LDSDL  160    160 5  .        It should not exceed the clock tolerance given by the CAN bit timing        configuration. LD of Synchronization Deviation Limit  SDL   8804  32  8230 4096"]
            #[inline(always)]
            pub fn ldsdl(
                self,
            ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, TtocFi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<5,0x7,1,0,u8, TtocFi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Initial Reference Trigger Offset   IRTO. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Positive offset  range from 0 to 127"]
            #[inline(always)]
            pub fn irto(
                self,
            ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, TtocFi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<8,0x7f,1,0,u8, TtocFi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Enable External Clock Synchronization   EECS. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. If enabled  TUR configuration  TURCF.NCL only  may be updated during        TTCAN operation."]
            #[inline(always)]
            pub fn eecs(
                self,
            ) -> crate::common::RegisterFieldBool<15, 1, 0, TtocFi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<15,1,0,TtocFi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Application Watchdog Limit   AWL. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. The application watchdog can be disabled by programming AWL to 0x00. Maximum time after which the application has to serve the application        watchdog. The application watchdog is incremented once each 256 NTUs."]
            #[inline(always)]
            pub fn awl(
                self,
            ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, TtocFi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0xff,1,0,u8, TtocFi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Enable Global Time Filtering   EGTF. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn egtf(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, TtocFi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<24,1,0,TtocFi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Enable Clock Calibration   ECC. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, TtocFi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<25,1,0,TtocFi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Event Trigger Polarity   EVTP. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn evtp(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, TtocFi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<26,1,0,TtocFi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for TtocFi {
            #[inline(always)]
            fn default() -> TtocFi {
                <crate::RegValueT<TtocFi_SPEC> as RegisterValue<_>>::new(65536)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtocNi_SPEC;
        impl crate::sealed::RegSpec for TtocNi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Operation Control 0\n resetvalue={Application Reset:0x0}"]
        pub type TtocNi = crate::RegValueT<TtocNi_SPEC>;

        impl TtocNi {
            #[doc = "Set Global time   SGT. Writing a   8216 1  8217  to SGT sets TTOST.WGDT if the node is the actual Time        Master. SGT is reset after one Host clock period. The global time preset        takes effect when the node transmits the next reference message with the        Master Ref Mark modified by the preset value written to TTGTP."]
            #[inline(always)]
            pub fn sgt(
                self,
            ) -> crate::common::RegisterFieldBool<0, 1, 0, TtocNi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<0,1,0,TtocNi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "External Clock Synchronization   ECS. Writing a   8216 1  8217  to ECS sets TTOST.WECS if the node is the actual Time        Master. ECS is reset after one Host clock period. The external clock        synchronization takes effect at the start of the next basic cycle."]
            #[inline(always)]
            pub fn ecs(
                self,
            ) -> crate::common::RegisterFieldBool<1, 1, 0, TtocNi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<1,1,0,TtocNi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Stop Watch Polarity   SWP"]
            #[inline(always)]
            pub fn swp(
                self,
            ) -> crate::common::RegisterFieldBool<2, 1, 0, TtocNi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<2,1,0,TtocNi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Stop Watch Source   SWS"]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, TtocNi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<3,0x3,1,0,u8, TtocNi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Register Time Mark Interrupt Pulse Enable   RTIE. Register time mark interrupts are configured by register TTTMK. A        register time mark interrupt pulse with the length of one TTCAN clock        period is generated when the time referenced by TTOCN.TMC  cycle  local         or global  equals TTTMK.TM  independent of the synchronization state."]
            #[inline(always)]
            pub fn rtie(
                self,
            ) -> crate::common::RegisterFieldBool<5, 1, 0, TtocNi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<5,1,0,TtocNi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Register Time Mark Compare   TMC. When changing the time mark reference  cycle  local  global time   it          is recommended to first write TMC     8220 00  8221   then reconfigure TTTMK  and          finally set TMC to the intended time reference."]
            #[inline(always)]
            pub fn tmc(
                self,
            ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, TtocNi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<6,0x3,1,0,u8, TtocNi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Trigger Time Mark Interrupt Pulse Enable   TTIE. External time mark events are configured by trigger memory element TMEX.        A trigger time mark interrupt pulse is generated when the trigger memory        element becomes active  and the M CAN is in synchronization state In Schedule or In Gap."]
            #[inline(always)]
            pub fn ttie(
                self,
            ) -> crate::common::RegisterFieldBool<8, 1, 0, TtocNi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<8,1,0,TtocNi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Gap Control Select   GCS"]
            #[inline(always)]
            pub fn gcs(
                self,
            ) -> crate::common::RegisterFieldBool<9, 1, 0, TtocNi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<9,1,0,TtocNi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Finish Gap   FGP. Set by the CPU  reset by each reference message"]
            #[inline(always)]
            pub fn fgp(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, TtocNi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,TtocNi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Time Mark Gap   TMG"]
            #[inline(always)]
            pub fn tmg(
                self,
            ) -> crate::common::RegisterFieldBool<11, 1, 0, TtocNi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<11,1,0,TtocNi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Next is Gap   NIG. This bit can only be set when the M CAN is the actual Time Master and when it is configured for external        event synchronized time triggered operation  TTOCF.GEN     8216 1  8217"]
            #[inline(always)]
            pub fn nig(
                self,
            ) -> crate::common::RegisterFieldBool<12, 1, 0, TtocNi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<12,1,0,TtocNi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "External Synchronization Control   ESCN. If enabled the M CAN synchronizes its cycle time phase to an external event signalled by a        rising edge at the event trigger."]
            #[inline(always)]
            pub fn escn(
                self,
            ) -> crate::common::RegisterFieldBool<13, 1, 0, TtocNi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<13,1,0,TtocNi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "TT Operation Control Register Locked   LCKC. Set by a write access to register TTOCN. Reset when the updated        configuration has been synchronized into the CAN clock domain."]
            #[inline(always)]
            pub fn lckc(
                self,
            ) -> crate::common::RegisterFieldBool<15, 1, 0, TtocNi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<15,1,0,TtocNi_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for TtocNi {
            #[inline(always)]
            fn default() -> TtocNi {
                <crate::RegValueT<TtocNi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtosTi_SPEC;
        impl crate::sealed::RegSpec for TtosTi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Operation Status 0\n resetvalue={Application Reset:0x20000080}"]
        pub type TtosTi = crate::RegValueT<TtosTi_SPEC>;

        impl TtosTi {
            #[doc = "Error Level   EL"]
            #[inline(always)]
            pub fn el(
                self,
            ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, TtosTi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0x3,1,0,u8, TtosTi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Master State   MS"]
            #[inline(always)]
            pub fn ms(
                self,
            ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, TtosTi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<2,0x3,1,0,u8, TtosTi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Synchronization State   SYS"]
            #[inline(always)]
            pub fn sys(
                self,
            ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, TtosTi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<4,0x3,1,0,u8, TtosTi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Quality of Global Time Phase   QGTP. Only relevant in TTCAN Level  160 0 and Level  160 2  otherwise fixed to   8216 0  8217 ."]
            #[inline(always)]
            pub fn qgtp(
                self,
            ) -> crate::common::RegisterFieldBool<6, 1, 0, TtosTi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<6,1,0,TtosTi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Quality of Clock Speed   QCS. Only relevant in TTCAN Level  160 0 and Level  160 2  otherwise fixed to   8216 1  8217 ."]
            #[inline(always)]
            pub fn qcs(
                self,
            ) -> crate::common::RegisterFieldBool<7, 1, 0, TtosTi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<7,1,0,TtosTi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Reference Trigger Offset   RTO. The Reference Trigger Offset value is a signed integer with a range from         127  0x81  to 127  0x7F . There is no notification when the lower limit        of  127 is reached. In case the M CAN becomes Time Master  MS 1 0      8220 11  8221    the reset of RTO is delayed due to        synchronization between Host and CAN clock domain. For time slaves the        value configured by TTOCF.IRTO is read. Actual Reference Trigger offset value"]
            #[inline(always)]
            pub fn rto(
                self,
            ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, TtosTi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<8,0xff,1,0,u8, TtosTi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Wait for Global Time Discontinuity   WGTD"]
            #[inline(always)]
            pub fn wgtd(
                self,
            ) -> crate::common::RegisterFieldBool<22, 1, 0, TtosTi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<22,1,0,TtosTi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Gap Finished Indicator   GFI. Set when the CPU writes TTOCN.FGP  or by a time mark interrupt if        TMG  160    160   8216 1  8217   or via event trigger input if TTOCN.GCS  160    160   8216 1  8217 . Not set by        Ref Trigger Gap or when Gap is finished by another node sending a        reference message."]
            #[inline(always)]
            pub fn gfi(
                self,
            ) -> crate::common::RegisterFieldBool<23, 1, 0, TtosTi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<23,1,0,TtosTi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Time Master Priority   TMP"]
            #[inline(always)]
            pub fn tmp(
                self,
            ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, TtosTi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<24,0x7,1,0,u8, TtosTi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Gap Started Indicator   GSI"]
            #[inline(always)]
            pub fn gsi(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, TtosTi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,TtosTi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Wait for Event   WFE"]
            #[inline(always)]
            pub fn wfe(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, TtosTi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<28,1,0,TtosTi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Application Watchdog Event   AWE. The application watchdog is served by reading TTOST. When the watchdog        is not served in time  bit AWE is set  all TTCAN communication is        stopped  and the M CAN is set into Bus Monitoring Mode."]
            #[inline(always)]
            pub fn awe(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, TtosTi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,TtosTi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Wait for External Clock Synchronization   WECS"]
            #[inline(always)]
            pub fn wecs(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, TtosTi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<30,1,0,TtosTi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Schedule Phase Lock   SPL. The bit is valid only when external synchronization is enabled         TTOCN.ESCN     8216 1  8217  . In this case it signals that the difference between        cycle time configured by TTGTP.CTP and the cycle time at the rising edge        at the event trigger is less or equal 9 NTU."]
            #[inline(always)]
            pub fn spl(
                self,
            ) -> crate::common::RegisterFieldBool<31, 1, 0, TtosTi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<31,1,0,TtosTi_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for TtosTi {
            #[inline(always)]
            fn default() -> TtosTi {
                <crate::RegValueT<TtosTi_SPEC> as RegisterValue<_>>::new(536871040)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtrmCi_SPEC;
        impl crate::sealed::RegSpec for TtrmCi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Reference Message Configuration 0\n resetvalue={Application Reset:0x0}"]
        pub type TtrmCi = crate::RegValueT<TtrmCi_SPEC>;

        impl TtrmCi {
            #[doc = "Reference Identifier   RID. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Identifier transmitted with reference message and used for reference        message filtering. Standard or extended reference identifier depending        on bit XTD. A standard identifier has to be written to ID 28 18 ."]
            #[inline(always)]
            pub fn rid(
                self,
            ) -> crate::common::RegisterField<
                0,
                0x1fffffff,
                1,
                0,
                u32,
                TtrmCi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    0,
                    0x1fffffff,
                    1,
                    0,
                    u32,
                    TtrmCi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Extended Identifier   XTD. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn xtd(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, TtrmCi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<30,1,0,TtrmCi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Reference Message Payload Select   RMPS. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Ignored in case of time slaves."]
            #[inline(always)]
            pub fn rmps(
                self,
            ) -> crate::common::RegisterFieldBool<31, 1, 0, TtrmCi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<31,1,0,TtrmCi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for TtrmCi {
            #[inline(always)]
            fn default() -> TtrmCi {
                <crate::RegValueT<TtrmCi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TttmCi_SPEC;
        impl crate::sealed::RegSpec for TttmCi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Trigger Memory Configuration 0\n resetvalue={Application Reset:0x0}"]
        pub type TttmCi = crate::RegValueT<TttmCi_SPEC>;

        impl TttmCi {
            #[doc = "Trigger Memory Start Address   TMSA. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Start address of Trigger Memory in Message RAM."]
            #[inline(always)]
            pub fn tmsa(
                self,
            ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, TttmCi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<2,0x3fff,1,0,u16, TttmCi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Trigger Memory Elements   TME. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn tme(
                self,
            ) -> crate::common::RegisterField<
                16,
                0x7f,
                1,
                0,
                tttmci::Tme,
                TttmCi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    16,
                    0x7f,
                    1,
                    0,
                    tttmci::Tme,
                    TttmCi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TttmCi {
            #[inline(always)]
            fn default() -> TttmCi {
                <crate::RegValueT<TttmCi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod tttmci {

            #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
            pub struct Tme_SPEC;
            pub type Tme = crate::EnumBitfieldStruct<u8, Tme_SPEC>;
            impl Tme {
                #[doc = "64 Trigger Memory elements"]
                pub const REST_127: Self = Self::new(127);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TttmKi_SPEC;
        impl crate::sealed::RegSpec for TttmKi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Time Mark 0\n resetvalue={Application Reset:0x0}"]
        pub type TttmKi = crate::RegValueT<TttmKi_SPEC>;

        impl TttmKi {
            #[doc = "Time Mark   TM. When using byte access to register TTTMK it is recommended to first          disable the time mark compare function  TTOCN.TMC     8220 00  8221   to avoid          compares on inconsistent register values."]
            #[inline(always)]
            pub fn tm(
                self,
            ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, TttmKi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xffff,1,0,u16, TttmKi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Time Mark Cycle Code   TICC. Cycle count for which the time mark is valid."]
            #[inline(always)]
            pub fn ticc(
                self,
            ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, TttmKi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x7f,1,0,u8, TttmKi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "TT Time Mark Register Locked   LCKM. Always set by a write access to registers TTOCN. Set by write access to        register TTTMK when TTOCN.TMC   8800   quot 00  8221 . Reset        when the registers have been synchronized into the CAN clock domain."]
            #[inline(always)]
            pub fn lckm(
                self,
            ) -> crate::common::RegisterFieldBool<31, 1, 0, TttmKi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<31,1,0,TttmKi_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for TttmKi {
            #[inline(always)]
            fn default() -> TttmKi {
                <crate::RegValueT<TttmKi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TurcFi_SPEC;
        impl crate::sealed::RegSpec for TurcFi_SPEC {
            type DataType = u32;
        }
        #[doc = "TUR Configuration 0\n resetvalue={Application Reset:0x10000000}"]
        pub type TurcFi = crate::RegValueT<TurcFi_SPEC>;

        impl TurcFi {
            #[doc = "Numerator Configuration Low   NCL. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Write access to the TUR Numerator Configuration Low is only possible        during configuration with TURCF.ELT     8216 0  8217  or if TTOCF.EECS  external        clock synchronization enabled  is set. When a new value for NCL is        written outside TT Configuration Mode  the new value takes effect when        TTOST.WECS is cleared to   8216 0  8217 . NCL is locked TTOST.WECS is   8216 1  8217 . If NC  160  lt   160 7  160 x  160 DC in TTCAN Level 1  then it is required that subsequent          time marks in the Trigger Memory must differ by at least 2 NTU."]
            #[inline(always)]
            pub fn ncl(
                self,
            ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, TurcFi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xffff,1,0,u16, TurcFi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Denominator Configuration   DC. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn dc(
                self,
            ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, TurcFi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x3fff,1,0,u16, TurcFi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Enable Local Time   ELT. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Local time is started by setting ELT. It remains active until ELT is          reset or until the next hardware reset. TURCF.DC is locked when          TURCF.ELT     8216 1  8217 . If ELT is written to   8216 0  8217   the readable value will          stay at   8216 1  8217  until the new value has been synchronized into the CAN          clock domain. During this time write access to the other bits of the          register remains locked."]
            #[inline(always)]
            pub fn elt(
                self,
            ) -> crate::common::RegisterFieldBool<31, 1, 0, TurcFi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<31,1,0,TurcFi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for TurcFi {
            #[inline(always)]
            fn default() -> TurcFi {
                <crate::RegValueT<TurcFi_SPEC> as RegisterValue<_>>::new(268435456)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TurnAi_SPEC;
        impl crate::sealed::RegSpec for TurnAi_SPEC {
            type DataType = u32;
        }
        #[doc = "TUR Numerator Actual 0\n resetvalue={Application Reset:0x10000}"]
        pub type TurnAi = crate::RegValueT<TurnAi_SPEC>;

        impl TurnAi {
            #[doc = "Numerator Actual Value   NAV"]
            #[inline(always)]
            pub fn nav(
                self,
            ) -> crate::common::RegisterField<
                0,
                0x3ffff,
                1,
                0,
                turnai::Nav,
                TurnAi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    0,
                    0x3ffff,
                    1,
                    0,
                    turnai::Nav,
                    TurnAi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TurnAi {
            #[inline(always)]
            fn default() -> TurnAi {
                <crate::RegValueT<TurnAi_SPEC> as RegisterValue<_>>::new(65536)
            }
        }
        pub mod turnai {

            #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
            pub struct Nav_SPEC;
            pub type Nav = crate::EnumBitfieldStruct<u32, Nav_SPEC>;
            impl Nav {
                #[doc = "Illegal value"]
                pub const REST_262143: Self = Self::new(262143);
            }
        }
    }
    #[doc = "TX"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tx(pub(super) *mut u8);
    unsafe impl core::marker::Send for Tx {}
    unsafe impl core::marker::Sync for Tx {}
    impl Tx {
        #[doc = "Tx Buffer Add Request 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txbari(&self) -> crate::common::Reg<tx::TxbaRi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "Tx Buffer Cancellation Finished 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txbcfi(&self) -> crate::common::Reg<tx::TxbcFi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
        }
        #[doc = "Tx Buffer Cancellation Finished Interrupt Enable 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txbciei(&self) -> crate::common::Reg<tx::TxbciEi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
        }
        #[doc = "Tx Buffer Cancellation Request 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txbcri(&self) -> crate::common::Reg<tx::TxbcRi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "Tx Buffer Configuration 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txbci(&self) -> crate::common::Reg<tx::TxbCi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Tx Buffer Request Pending 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txbrpi(&self) -> crate::common::Reg<tx::TxbrPi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "Tx Buffer Transmission Interrupt Enable 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txbtiei(&self) -> crate::common::Reg<tx::TxbtiEi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
        }
        #[doc = "Tx Buffer Transmission Occurred 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txbtoi(&self) -> crate::common::Reg<tx::TxbtOi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
        }
        #[doc = "Tx Event FIFO Acknowledge 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txefai(&self) -> crate::common::Reg<tx::TxefAi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
        }
        #[doc = "Tx Event FIFO Configuration 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txefci(&self) -> crate::common::Reg<tx::TxefCi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
        }
        #[doc = "Tx Event FIFO Status 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txefsi(&self) -> crate::common::Reg<tx::TxefSi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
        }
        #[doc = "Tx Buffer Element Size Configuration 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txesci(&self) -> crate::common::Reg<tx::TxesCi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Tx FIFO Queue Status 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txfqsi(&self) -> crate::common::Reg<tx::TxfqSi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
    }
    pub mod tx {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxbaRi_SPEC;
        impl crate::sealed::RegSpec for TxbaRi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Buffer Add Request 0\n resetvalue={Application Reset:0x0}"]
        pub type TxbaRi = crate::RegValueT<TxbaRi_SPEC>;

        impl TxbaRi {
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar0(
                self,
            ) -> crate::common::RegisterFieldBool<0, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<0,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar1(
                self,
            ) -> crate::common::RegisterFieldBool<1, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<1,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar2(
                self,
            ) -> crate::common::RegisterFieldBool<2, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<2,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar3(
                self,
            ) -> crate::common::RegisterFieldBool<3, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<3,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar4(
                self,
            ) -> crate::common::RegisterFieldBool<4, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<4,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar5(
                self,
            ) -> crate::common::RegisterFieldBool<5, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<5,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar6(
                self,
            ) -> crate::common::RegisterFieldBool<6, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<6,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar7(
                self,
            ) -> crate::common::RegisterFieldBool<7, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<7,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar8(
                self,
            ) -> crate::common::RegisterFieldBool<8, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<8,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar9(
                self,
            ) -> crate::common::RegisterFieldBool<9, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<9,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar10(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar11(
                self,
            ) -> crate::common::RegisterFieldBool<11, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<11,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar12(
                self,
            ) -> crate::common::RegisterFieldBool<12, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<12,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar13(
                self,
            ) -> crate::common::RegisterFieldBool<13, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<13,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar14(
                self,
            ) -> crate::common::RegisterFieldBool<14, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<14,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar15(
                self,
            ) -> crate::common::RegisterFieldBool<15, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<15,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar16(
                self,
            ) -> crate::common::RegisterFieldBool<16, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<16,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar17(
                self,
            ) -> crate::common::RegisterFieldBool<17, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<17,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar18(
                self,
            ) -> crate::common::RegisterFieldBool<18, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<18,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar19(
                self,
            ) -> crate::common::RegisterFieldBool<19, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<19,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar20(
                self,
            ) -> crate::common::RegisterFieldBool<20, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<20,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar21(
                self,
            ) -> crate::common::RegisterFieldBool<21, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<21,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar22(
                self,
            ) -> crate::common::RegisterFieldBool<22, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<22,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar23(
                self,
            ) -> crate::common::RegisterFieldBool<23, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<23,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar24(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<24,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar25(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<25,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar26(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<26,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar27(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<27,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar28(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<28,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar29(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<29,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar30(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<30,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar31(
                self,
            ) -> crate::common::RegisterFieldBool<31, 1, 0, TxbaRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<31,1,0,TxbaRi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for TxbaRi {
            #[inline(always)]
            fn default() -> TxbaRi {
                <crate::RegValueT<TxbaRi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxbcFi_SPEC;
        impl crate::sealed::RegSpec for TxbcFi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Buffer Cancellation Finished 0\n resetvalue={Application Reset:0x0}"]
        pub type TxbcFi = crate::RegValueT<TxbcFi_SPEC>;

        impl TxbcFi {
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf0(
                self,
            ) -> crate::common::RegisterFieldBool<0, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<0,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf1(
                self,
            ) -> crate::common::RegisterFieldBool<1, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<1,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf2(
                self,
            ) -> crate::common::RegisterFieldBool<2, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<2,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf3(
                self,
            ) -> crate::common::RegisterFieldBool<3, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<3,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf4(
                self,
            ) -> crate::common::RegisterFieldBool<4, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<4,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf5(
                self,
            ) -> crate::common::RegisterFieldBool<5, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<5,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf6(
                self,
            ) -> crate::common::RegisterFieldBool<6, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<6,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf7(
                self,
            ) -> crate::common::RegisterFieldBool<7, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<7,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf8(
                self,
            ) -> crate::common::RegisterFieldBool<8, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<8,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf9(
                self,
            ) -> crate::common::RegisterFieldBool<9, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<9,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf10(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<10,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf11(
                self,
            ) -> crate::common::RegisterFieldBool<11, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<11,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf12(
                self,
            ) -> crate::common::RegisterFieldBool<12, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<12,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf13(
                self,
            ) -> crate::common::RegisterFieldBool<13, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<13,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf14(
                self,
            ) -> crate::common::RegisterFieldBool<14, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<14,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf15(
                self,
            ) -> crate::common::RegisterFieldBool<15, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<15,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf16(
                self,
            ) -> crate::common::RegisterFieldBool<16, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<16,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf17(
                self,
            ) -> crate::common::RegisterFieldBool<17, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<17,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf18(
                self,
            ) -> crate::common::RegisterFieldBool<18, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<18,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf19(
                self,
            ) -> crate::common::RegisterFieldBool<19, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<19,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf20(
                self,
            ) -> crate::common::RegisterFieldBool<20, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<20,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf21(
                self,
            ) -> crate::common::RegisterFieldBool<21, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<21,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf22(
                self,
            ) -> crate::common::RegisterFieldBool<22, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<22,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf23(
                self,
            ) -> crate::common::RegisterFieldBool<23, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<23,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf24(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf25(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<25,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf26(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<26,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf27(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf28(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<28,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf29(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf30(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<30,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf31(
                self,
            ) -> crate::common::RegisterFieldBool<31, 1, 0, TxbcFi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<31,1,0,TxbcFi_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for TxbcFi {
            #[inline(always)]
            fn default() -> TxbcFi {
                <crate::RegValueT<TxbcFi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxbciEi_SPEC;
        impl crate::sealed::RegSpec for TxbciEi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Buffer Cancellation Finished Interrupt Enable 0\n resetvalue={Application Reset:0x0}"]
        pub type TxbciEi = crate::RegValueT<TxbciEi_SPEC>;

        impl TxbciEi {
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie0(
                self,
            ) -> crate::common::RegisterFieldBool<0, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<0,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie1(
                self,
            ) -> crate::common::RegisterFieldBool<1, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<1,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie2(
                self,
            ) -> crate::common::RegisterFieldBool<2, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<2,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie3(
                self,
            ) -> crate::common::RegisterFieldBool<3, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<3,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie4(
                self,
            ) -> crate::common::RegisterFieldBool<4, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<4,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie5(
                self,
            ) -> crate::common::RegisterFieldBool<5, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<5,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie6(
                self,
            ) -> crate::common::RegisterFieldBool<6, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<6,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie7(
                self,
            ) -> crate::common::RegisterFieldBool<7, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<7,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie8(
                self,
            ) -> crate::common::RegisterFieldBool<8, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<8,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie9(
                self,
            ) -> crate::common::RegisterFieldBool<9, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<9,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie10(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie11(
                self,
            ) -> crate::common::RegisterFieldBool<11, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<11,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie12(
                self,
            ) -> crate::common::RegisterFieldBool<12, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<12,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie13(
                self,
            ) -> crate::common::RegisterFieldBool<13, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<13,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie14(
                self,
            ) -> crate::common::RegisterFieldBool<14, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<14,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie15(
                self,
            ) -> crate::common::RegisterFieldBool<15, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<15,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie16(
                self,
            ) -> crate::common::RegisterFieldBool<16, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<16,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie17(
                self,
            ) -> crate::common::RegisterFieldBool<17, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<17,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie18(
                self,
            ) -> crate::common::RegisterFieldBool<18, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<18,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie19(
                self,
            ) -> crate::common::RegisterFieldBool<19, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<19,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie20(
                self,
            ) -> crate::common::RegisterFieldBool<20, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<20,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie21(
                self,
            ) -> crate::common::RegisterFieldBool<21, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<21,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie22(
                self,
            ) -> crate::common::RegisterFieldBool<22, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<22,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie23(
                self,
            ) -> crate::common::RegisterFieldBool<23, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<23,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie24(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<24,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie25(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<25,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie26(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<26,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie27(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<27,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie28(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<28,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie29(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<29,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie30(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<30,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie31(
                self,
            ) -> crate::common::RegisterFieldBool<31, 1, 0, TxbciEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<31,1,0,TxbciEi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for TxbciEi {
            #[inline(always)]
            fn default() -> TxbciEi {
                <crate::RegValueT<TxbciEi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxbcRi_SPEC;
        impl crate::sealed::RegSpec for TxbcRi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Buffer Cancellation Request 0\n resetvalue={Application Reset:0x0}"]
        pub type TxbcRi = crate::RegValueT<TxbcRi_SPEC>;

        impl TxbcRi {
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr0(
                self,
            ) -> crate::common::RegisterFieldBool<0, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<0,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr1(
                self,
            ) -> crate::common::RegisterFieldBool<1, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<1,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr2(
                self,
            ) -> crate::common::RegisterFieldBool<2, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<2,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr3(
                self,
            ) -> crate::common::RegisterFieldBool<3, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<3,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr4(
                self,
            ) -> crate::common::RegisterFieldBool<4, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<4,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr5(
                self,
            ) -> crate::common::RegisterFieldBool<5, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<5,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr6(
                self,
            ) -> crate::common::RegisterFieldBool<6, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<6,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr7(
                self,
            ) -> crate::common::RegisterFieldBool<7, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<7,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr8(
                self,
            ) -> crate::common::RegisterFieldBool<8, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<8,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr9(
                self,
            ) -> crate::common::RegisterFieldBool<9, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<9,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr10(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr11(
                self,
            ) -> crate::common::RegisterFieldBool<11, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<11,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr12(
                self,
            ) -> crate::common::RegisterFieldBool<12, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<12,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr13(
                self,
            ) -> crate::common::RegisterFieldBool<13, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<13,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr14(
                self,
            ) -> crate::common::RegisterFieldBool<14, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<14,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr15(
                self,
            ) -> crate::common::RegisterFieldBool<15, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<15,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr16(
                self,
            ) -> crate::common::RegisterFieldBool<16, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<16,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr17(
                self,
            ) -> crate::common::RegisterFieldBool<17, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<17,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr18(
                self,
            ) -> crate::common::RegisterFieldBool<18, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<18,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr19(
                self,
            ) -> crate::common::RegisterFieldBool<19, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<19,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr20(
                self,
            ) -> crate::common::RegisterFieldBool<20, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<20,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr21(
                self,
            ) -> crate::common::RegisterFieldBool<21, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<21,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr22(
                self,
            ) -> crate::common::RegisterFieldBool<22, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<22,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr23(
                self,
            ) -> crate::common::RegisterFieldBool<23, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<23,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr24(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<24,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr25(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<25,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr26(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<26,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr27(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<27,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr28(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<28,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr29(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<29,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr30(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<30,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr31(
                self,
            ) -> crate::common::RegisterFieldBool<31, 1, 0, TxbcRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<31,1,0,TxbcRi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for TxbcRi {
            #[inline(always)]
            fn default() -> TxbcRi {
                <crate::RegValueT<TxbcRi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxbCi_SPEC;
        impl crate::sealed::RegSpec for TxbCi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Buffer Configuration 0\n resetvalue={Application Reset:0x0}"]
        pub type TxbCi = crate::RegValueT<TxbCi_SPEC>;

        impl TxbCi {
            #[doc = "Tx Buffers Start Address   TBSA. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Start address of Tx Buffers section in Message RAM  32 bit word address ."]
            #[inline(always)]
            pub fn tbsa(
                self,
            ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, TxbCi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<2,0x3fff,1,0,u16, TxbCi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Number of Dedicated Transmit Buffers   NDTB. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Be aware that the sum of TFQS and NDTB may be not greater than 32.          There is no check for erroneous configurations. The Tx Buffers section          in the Message RAM starts with the dedicated Tx Buffers."]
            #[inline(always)]
            pub fn ndtb(
                self,
            ) -> crate::common::RegisterField<
                16,
                0x3f,
                1,
                0,
                txbci::Ndtb,
                TxbCi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    16,
                    0x3f,
                    1,
                    0,
                    txbci::Ndtb,
                    TxbCi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmit FIFO Queue Size   TFQS. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn tfqs(
                self,
            ) -> crate::common::RegisterField<
                24,
                0x3f,
                1,
                0,
                txbci::Tfqs,
                TxbCi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    24,
                    0x3f,
                    1,
                    0,
                    txbci::Tfqs,
                    TxbCi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Tx FIFO Queue Mode   TFQM. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set."]
            #[inline(always)]
            pub fn tfqm(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, TxbCi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<30,1,0,TxbCi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for TxbCi {
            #[inline(always)]
            fn default() -> TxbCi {
                <crate::RegValueT<TxbCi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod txbci {

            #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
            pub struct Ndtb_SPEC;
            pub type Ndtb = crate::EnumBitfieldStruct<u8, Ndtb_SPEC>;
            impl Ndtb {
                #[doc = "32 Dedicated Tx Buffers"]
                pub const REST_63: Self = Self::new(63);
            }
            #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
            pub struct Tfqs_SPEC;
            pub type Tfqs = crate::EnumBitfieldStruct<u8, Tfqs_SPEC>;
            impl Tfqs {
                #[doc = "32 Tx Buffers used for Tx FIFO Queue"]
                pub const REST_63: Self = Self::new(63);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxbrPi_SPEC;
        impl crate::sealed::RegSpec for TxbrPi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Buffer Request Pending 0\n resetvalue={Application Reset:0x0}"]
        pub type TxbrPi = crate::RegValueT<TxbrPi_SPEC>;

        impl TxbrPi {
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp0(
                self,
            ) -> crate::common::RegisterFieldBool<0, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<0,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp1(
                self,
            ) -> crate::common::RegisterFieldBool<1, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<1,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp2(
                self,
            ) -> crate::common::RegisterFieldBool<2, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<2,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp3(
                self,
            ) -> crate::common::RegisterFieldBool<3, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<3,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp4(
                self,
            ) -> crate::common::RegisterFieldBool<4, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<4,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp5(
                self,
            ) -> crate::common::RegisterFieldBool<5, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<5,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp6(
                self,
            ) -> crate::common::RegisterFieldBool<6, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<6,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp7(
                self,
            ) -> crate::common::RegisterFieldBool<7, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<7,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp8(
                self,
            ) -> crate::common::RegisterFieldBool<8, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<8,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp9(
                self,
            ) -> crate::common::RegisterFieldBool<9, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<9,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp10(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<10,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp11(
                self,
            ) -> crate::common::RegisterFieldBool<11, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<11,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp12(
                self,
            ) -> crate::common::RegisterFieldBool<12, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<12,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp13(
                self,
            ) -> crate::common::RegisterFieldBool<13, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<13,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp14(
                self,
            ) -> crate::common::RegisterFieldBool<14, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<14,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp15(
                self,
            ) -> crate::common::RegisterFieldBool<15, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<15,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp16(
                self,
            ) -> crate::common::RegisterFieldBool<16, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<16,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp17(
                self,
            ) -> crate::common::RegisterFieldBool<17, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<17,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp18(
                self,
            ) -> crate::common::RegisterFieldBool<18, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<18,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp19(
                self,
            ) -> crate::common::RegisterFieldBool<19, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<19,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp20(
                self,
            ) -> crate::common::RegisterFieldBool<20, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<20,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp21(
                self,
            ) -> crate::common::RegisterFieldBool<21, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<21,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp22(
                self,
            ) -> crate::common::RegisterFieldBool<22, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<22,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp23(
                self,
            ) -> crate::common::RegisterFieldBool<23, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<23,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp24(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp25(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<25,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp26(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<26,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp27(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp28(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<28,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp29(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp30(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<30,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp31(
                self,
            ) -> crate::common::RegisterFieldBool<31, 1, 0, TxbrPi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<31,1,0,TxbrPi_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for TxbrPi {
            #[inline(always)]
            fn default() -> TxbrPi {
                <crate::RegValueT<TxbrPi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxbtiEi_SPEC;
        impl crate::sealed::RegSpec for TxbtiEi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Buffer Transmission Interrupt Enable 0\n resetvalue={Application Reset:0x0}"]
        pub type TxbtiEi = crate::RegValueT<TxbtiEi_SPEC>;

        impl TxbtiEi {
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie0(
                self,
            ) -> crate::common::RegisterFieldBool<0, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<0,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie1(
                self,
            ) -> crate::common::RegisterFieldBool<1, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<1,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie2(
                self,
            ) -> crate::common::RegisterFieldBool<2, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<2,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie3(
                self,
            ) -> crate::common::RegisterFieldBool<3, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<3,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie4(
                self,
            ) -> crate::common::RegisterFieldBool<4, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<4,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie5(
                self,
            ) -> crate::common::RegisterFieldBool<5, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<5,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie6(
                self,
            ) -> crate::common::RegisterFieldBool<6, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<6,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie7(
                self,
            ) -> crate::common::RegisterFieldBool<7, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<7,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie8(
                self,
            ) -> crate::common::RegisterFieldBool<8, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<8,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie9(
                self,
            ) -> crate::common::RegisterFieldBool<9, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<9,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie10(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie11(
                self,
            ) -> crate::common::RegisterFieldBool<11, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<11,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie12(
                self,
            ) -> crate::common::RegisterFieldBool<12, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<12,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie13(
                self,
            ) -> crate::common::RegisterFieldBool<13, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<13,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie14(
                self,
            ) -> crate::common::RegisterFieldBool<14, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<14,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie15(
                self,
            ) -> crate::common::RegisterFieldBool<15, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<15,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie16(
                self,
            ) -> crate::common::RegisterFieldBool<16, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<16,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie17(
                self,
            ) -> crate::common::RegisterFieldBool<17, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<17,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie18(
                self,
            ) -> crate::common::RegisterFieldBool<18, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<18,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie19(
                self,
            ) -> crate::common::RegisterFieldBool<19, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<19,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie20(
                self,
            ) -> crate::common::RegisterFieldBool<20, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<20,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie21(
                self,
            ) -> crate::common::RegisterFieldBool<21, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<21,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie22(
                self,
            ) -> crate::common::RegisterFieldBool<22, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<22,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie23(
                self,
            ) -> crate::common::RegisterFieldBool<23, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<23,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie24(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<24,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie25(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<25,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie26(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<26,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie27(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<27,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie28(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<28,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie29(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<29,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie30(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<30,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie31(
                self,
            ) -> crate::common::RegisterFieldBool<31, 1, 0, TxbtiEi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<31,1,0,TxbtiEi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for TxbtiEi {
            #[inline(always)]
            fn default() -> TxbtiEi {
                <crate::RegValueT<TxbtiEi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxbtOi_SPEC;
        impl crate::sealed::RegSpec for TxbtOi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Buffer Transmission Occurred 0\n resetvalue={Application Reset:0x0}"]
        pub type TxbtOi = crate::RegValueT<TxbtOi_SPEC>;

        impl TxbtOi {
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to0(
                self,
            ) -> crate::common::RegisterFieldBool<0, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<0,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to1(
                self,
            ) -> crate::common::RegisterFieldBool<1, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<1,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to2(
                self,
            ) -> crate::common::RegisterFieldBool<2, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<2,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to3(
                self,
            ) -> crate::common::RegisterFieldBool<3, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<3,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to4(
                self,
            ) -> crate::common::RegisterFieldBool<4, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<4,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to5(
                self,
            ) -> crate::common::RegisterFieldBool<5, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<5,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to6(
                self,
            ) -> crate::common::RegisterFieldBool<6, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<6,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to7(
                self,
            ) -> crate::common::RegisterFieldBool<7, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<7,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to8(
                self,
            ) -> crate::common::RegisterFieldBool<8, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<8,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to9(
                self,
            ) -> crate::common::RegisterFieldBool<9, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<9,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to10(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<10,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to11(
                self,
            ) -> crate::common::RegisterFieldBool<11, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<11,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to12(
                self,
            ) -> crate::common::RegisterFieldBool<12, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<12,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to13(
                self,
            ) -> crate::common::RegisterFieldBool<13, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<13,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to14(
                self,
            ) -> crate::common::RegisterFieldBool<14, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<14,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to15(
                self,
            ) -> crate::common::RegisterFieldBool<15, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<15,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to16(
                self,
            ) -> crate::common::RegisterFieldBool<16, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<16,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to17(
                self,
            ) -> crate::common::RegisterFieldBool<17, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<17,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to18(
                self,
            ) -> crate::common::RegisterFieldBool<18, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<18,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to19(
                self,
            ) -> crate::common::RegisterFieldBool<19, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<19,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to20(
                self,
            ) -> crate::common::RegisterFieldBool<20, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<20,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to21(
                self,
            ) -> crate::common::RegisterFieldBool<21, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<21,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to22(
                self,
            ) -> crate::common::RegisterFieldBool<22, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<22,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to23(
                self,
            ) -> crate::common::RegisterFieldBool<23, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<23,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to24(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to25(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<25,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to26(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<26,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to27(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to28(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<28,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to29(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to30(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<30,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to31(
                self,
            ) -> crate::common::RegisterFieldBool<31, 1, 0, TxbtOi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<31,1,0,TxbtOi_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for TxbtOi {
            #[inline(always)]
            fn default() -> TxbtOi {
                <crate::RegValueT<TxbtOi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxefAi_SPEC;
        impl crate::sealed::RegSpec for TxefAi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Event FIFO Acknowledge 0\n resetvalue={Application Reset:0x0}"]
        pub type TxefAi = crate::RegValueT<TxefAi_SPEC>;

        impl TxefAi {
            #[doc = "Event FIFO Acknowledge Index   EFAI. After the Host has read an element or a sequence of elements from the Tx        Event FIFO it has to write the index of the last element read from Tx        Event FIFO to EFAI. This will set the Tx Event FIFO Get Index TXEFS.EFGI        to EFAI  160    160 1 and update the FIFO 0 Fill Level TXEFS.EFFL."]
            #[inline(always)]
            pub fn efai(
                self,
            ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, TxefAi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0x1f,1,0,u8, TxefAi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for TxefAi {
            #[inline(always)]
            fn default() -> TxefAi {
                <crate::RegValueT<TxefAi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxefCi_SPEC;
        impl crate::sealed::RegSpec for TxefCi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Event FIFO Configuration 0\n resetvalue={Application Reset:0x0}"]
        pub type TxefCi = crate::RegValueT<TxefCi_SPEC>;

        impl TxefCi {
            #[doc = "Event FIFO Start Address   EFSA. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Start address of Tx Event FIFO in Message RAM  32 bit word address ."]
            #[inline(always)]
            pub fn efsa(
                self,
            ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, TxefCi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<2,0x3fff,1,0,u16, TxefCi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Event FIFO Size   EFS. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. The Tx Event FIFO elements are indexed from 0 to EFS   1"]
            #[inline(always)]
            pub fn efs(
                self,
            ) -> crate::common::RegisterField<
                16,
                0x3f,
                1,
                0,
                txefci::Efs,
                TxefCi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    16,
                    0x3f,
                    1,
                    0,
                    txefci::Efs,
                    TxefCi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Event FIFO Watermark   EFWM. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn efwm(
                self,
            ) -> crate::common::RegisterField<
                24,
                0x3f,
                1,
                0,
                txefci::Efwm,
                TxefCi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    24,
                    0x3f,
                    1,
                    0,
                    txefci::Efwm,
                    TxefCi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TxefCi {
            #[inline(always)]
            fn default() -> TxefCi {
                <crate::RegValueT<TxefCi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod txefci {

            #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
            pub struct Efs_SPEC;
            pub type Efs = crate::EnumBitfieldStruct<u8, Efs_SPEC>;
            impl Efs {
                #[doc = "32 Tx Event FIFO elements"]
                pub const REST_63: Self = Self::new(63);
            }
            #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
            pub struct Efwm_SPEC;
            pub type Efwm = crate::EnumBitfieldStruct<u8, Efwm_SPEC>;
            impl Efwm {
                #[doc = "Watermark interrupt disabled"]
                pub const REST_63: Self = Self::new(63);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxefSi_SPEC;
        impl crate::sealed::RegSpec for TxefSi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Event FIFO Status 0\n resetvalue={Application Reset:0x0}"]
        pub type TxefSi = crate::RegValueT<TxefSi_SPEC>;

        impl TxefSi {
            #[doc = "Event FIFO Fill Level   EFFL. Number of elements stored in Tx Event FIFO  range 0 to 32."]
            #[inline(always)]
            pub fn effl(
                self,
            ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, TxefSi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0x3f,1,0,u8, TxefSi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Event FIFO Get Index   EFGI. Tx Event FIFO read index pointer  range 0 to 31."]
            #[inline(always)]
            pub fn efgi(
                self,
            ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, TxefSi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<8,0x1f,1,0,u8, TxefSi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Event FIFO Put Index   EFPI. Tx Event FIFO write index pointer  range 0 to 31."]
            #[inline(always)]
            pub fn efpi(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, TxefSi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, TxefSi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Event FIFO Full   EFF"]
            #[inline(always)]
            pub fn eff(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, TxefSi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,TxefSi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Tx Event FIFO Element Lost   TEFL. This bit is a copy of interrupt flag IR.TEFL. When IR.TEFL is reset  this bit is also reset."]
            #[inline(always)]
            pub fn tefl(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, TxefSi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<25,1,0,TxefSi_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for TxefSi {
            #[inline(always)]
            fn default() -> TxefSi {
                <crate::RegValueT<TxefSi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxesCi_SPEC;
        impl crate::sealed::RegSpec for TxesCi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Buffer Element Size Configuration 0\n resetvalue={Application Reset:0x0}"]
        pub type TxesCi = crate::RegValueT<TxesCi_SPEC>;

        impl TxesCi {
            #[doc = "Tx Buffer Data Field Size   TBDS. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set. In case the data length code DLC of a Tx Buffer element is configured to a value higher than the Tx Buffer data field size TXESC.TBDS  the bytes not defined by the Tx Buffer are transmitted as  0xCC   padding bytes ."]
            #[inline(always)]
            pub fn tbds(
                self,
            ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, TxesCi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0x7,1,0,u8, TxesCi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for TxesCi {
            #[inline(always)]
            fn default() -> TxesCi {
                <crate::RegValueT<TxesCi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxfqSi_SPEC;
        impl crate::sealed::RegSpec for TxfqSi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx FIFO Queue Status 0\n resetvalue={Application Reset:0x0}"]
        pub type TxfqSi = crate::RegValueT<TxfqSi_SPEC>;

        impl TxfqSi {
            #[doc = "Tx FIFO Free Level   TFFL. Number of consecutive free Tx FIFO elements starting from TFGI  range 0        to 32. Read as zero when Tx Queue operation is configured  TXBC.TFQM            8216 1  8217   In case of mixed configurations where dedicated Tx Buffers are          combined with a Tx FIFO or a Tx Queue  the Put and Get Indices          indicate the number of the Tx Buffer starting with the first dedicated          Tx Buffers. Example  For a configuration of 12 dedicated Tx Buffers          and a Tx FIFO of 20 Buffers a Put Index of 15 points to the fourth          buffer of the Tx FIFO."]
            #[inline(always)]
            pub fn tffl(
                self,
            ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, TxfqSi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0x3f,1,0,u8, TxfqSi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Tx FIFO Get Index   TFGI. Tx FIFO read index pointer  range 0 to 31. Read as zero when Tx Queue   operation is configured  TXBC.TFQM    1  ."]
            #[inline(always)]
            pub fn tfgi(
                self,
            ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, TxfqSi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<8,0x1f,1,0,u8, TxfqSi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Tx FIFO Queue Put Index   TFQPI. Tx FIFO Queue write index pointer  range 0 to 31."]
            #[inline(always)]
            pub fn tfqpi(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, TxfqSi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, TxfqSi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Tx FIFO Queue Full   TFQF"]
            #[inline(always)]
            pub fn tfqf(
                self,
            ) -> crate::common::RegisterFieldBool<21, 1, 0, TxfqSi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<21,1,0,TxfqSi_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for TxfqSi {
            #[inline(always)]
            fn default() -> TxfqSi {
                <crate::RegValueT<TxfqSi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
