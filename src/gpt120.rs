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
#[doc = r"GPT12"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt120(pub(super) *mut u8);
unsafe impl core::marker::Send for Gpt120 {}
unsafe impl core::marker::Sync for Gpt120 {}
impl Gpt120 {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "Capture and Reload Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn caprel(&self) -> crate::common::Reg<self::Caprel_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Identification Register\n resetvalue={Application Reset:0x68C000,Application Reset:0x68C001}"]
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

    #[doc = "OCDS Control and Status Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(232usize)) }
    }

    #[doc = "Port Input Select Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pisel(&self) -> crate::common::Reg<self::Pisel_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }

    #[doc = "Timer T2 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t2(&self) -> crate::common::Reg<self::T2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "Timer T2 Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t2con(&self) -> crate::common::Reg<self::T2Con_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "Timer T3 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t3(&self) -> crate::common::Reg<self::T3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }

    #[doc = "Timer T3 Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t3con(&self) -> crate::common::Reg<self::T3Con_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Timer T4 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t4(&self) -> crate::common::Reg<self::T4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "Timer T4 Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t4con(&self) -> crate::common::Reg<self::T4Con_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "Timer T5 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t5(&self) -> crate::common::Reg<self::T5_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "Timer T5 Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t5con(&self) -> crate::common::Reg<self::T5Con_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Timer T6 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t6(&self) -> crate::common::Reg<self::T6_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }

    #[doc = "Timer T6 Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t6con(&self) -> crate::common::Reg<self::T6Con_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
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
pub struct Caprel_SPEC;
impl crate::sealed::RegSpec for Caprel_SPEC {
    type DataType = u32;
}
#[doc = "Capture and Reload Register\n resetvalue={Application Reset:0x0}"]
pub type Caprel = crate::RegValueT<Caprel_SPEC>;

impl Caprel {
    #[doc = "Current reload value or Captured value   CAPREL"]
    #[inline(always)]
    pub fn caprel(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Caprel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Caprel_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Caprel {
    #[inline(always)]
    fn default() -> Caprel {
        <crate::RegValueT<Caprel_SPEC> as RegisterValue<_>>::new(0)
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
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Identification Register\n resetvalue={Application Reset:0x68C000,Application Reset:0x68C001}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MODREV. This bit field indicates the revision number of the module  01 H   160    160 first        revision ."]
    #[inline(always)]
    pub fn modrev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MODTYPE. This bit field is C0 H . It defines a        32 bit module"]
    #[inline(always)]
    pub fn modtype(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number   MODNUMBER. This bit field defines the module identification number. For the GPT12 module the module identification number is 68 H ."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(6864896)
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel        reset will be executed if the reset bits of both kernel registers are        set. The RST bit will be cleared  re set to   180 0  180   by the BPI after the kernel        reset was executed."]
    #[inline(always)]
    pub fn rst(self) -> crate::common::RegisterFieldBool<0, 1, 0, Krst0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Krst0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Kernel Reset Status   RSTSTAT. This bit indicates wether a kernel reset was executed or not. This bit        is set after the execution of a kernel reset in the same clock cycle in        which the reset bits are cleared. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related KRSTCLR register."]
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel reset registers is set. The RST bit will be cleared  re set to  0   by the BPI after the kernel reset was executed."]
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
pub struct Pisel_SPEC;
impl crate::sealed::RegSpec for Pisel_SPEC {
    type DataType = u32;
}
#[doc = "Port Input Select Register\n resetvalue={Application Reset:0x0}"]
pub type Pisel = crate::RegValueT<Pisel_SPEC>;

impl Pisel {
    #[doc = "Input Select for T2IN   IST2IN"]
    #[inline(always)]
    pub fn ist2in(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Pisel_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pisel_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Select for T2EUD   IST2EUD"]
    #[inline(always)]
    pub fn ist2eud(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Pisel_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pisel_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Select for T3IN   IST3IN"]
    #[inline(always)]
    pub fn ist3in(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Pisel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for T3EUD   IST3EUD"]
    #[inline(always)]
    pub fn ist3eud(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Pisel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for T4IN   IST4IN"]
    #[inline(always)]
    pub fn ist4in(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Pisel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for T4EUD   IST4EUD"]
    #[inline(always)]
    pub fn ist4eud(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Pisel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for T5IN   IST5IN"]
    #[inline(always)]
    pub fn ist5in(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Pisel_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Pisel_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Select for T5EUD   IST5EUD"]
    #[inline(always)]
    pub fn ist5eud(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Pisel_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Pisel_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Select for T6IN   IST6IN"]
    #[inline(always)]
    pub fn ist6in(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Pisel_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Pisel_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Select for T6EUD   IST6EUD"]
    #[inline(always)]
    pub fn ist6eud(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Pisel_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Pisel_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Select for CAPIN   ISCAPIN"]
    #[inline(always)]
    pub fn iscapin(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Pisel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pisel {
    #[inline(always)]
    fn default() -> Pisel {
        <crate::RegValueT<Pisel_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T2_SPEC;
impl crate::sealed::RegSpec for T2_SPEC {
    type DataType = u32;
}
#[doc = "Timer T2 Register\n resetvalue={Application Reset:0x0}"]
pub type T2 = crate::RegValueT<T2_SPEC>;

impl T2 {
    #[doc = "Timer T2   T2. Contains the current value of Timer T2."]
    #[inline(always)]
    pub fn t2(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, T2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, T2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T2 {
    #[inline(always)]
    fn default() -> T2 {
        <crate::RegValueT<T2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T2Con_SPEC;
impl crate::sealed::RegSpec for T2Con_SPEC {
    type DataType = u32;
}
#[doc = "Timer T2 Control Register\n resetvalue={Application Reset:0x0}"]
pub type T2Con = crate::RegValueT<T2Con_SPEC>;

impl T2Con {
    #[doc = "Timer T2 Input Parameter Selection   T2I. Depends on the operating mode  see respective sections for encoding  CROSSREFERENCE for Timer Mode and Gated Timer Mode CROSSREFERENCE for Counter Mode CROSSREFERENCE for Incremental Interface Mode"]
    #[inline(always)]
    pub fn t2i(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, T2Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, T2Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T2 Mode Control  Basic Operating Mode    T2M"]
    #[inline(always)]
    pub fn t2m(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, T2Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x7,1,0,u8, T2Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T2 Run Bit   T2R. This bit only controls timer T2 if bit T2RC   0."]
    #[inline(always)]
    pub fn t2r(self) -> crate::common::RegisterFieldBool<6, 1, 0, T2Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, T2Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T2 Up Down Control   T2UD. This bit only directly controls count direction of T2 if bit T2UDE   0."]
    #[inline(always)]
    pub fn t2ud(self) -> crate::common::RegisterFieldBool<7, 1, 0, T2Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, T2Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T2 External Up Down Enable   T2UDE"]
    #[inline(always)]
    pub fn t2ude(self) -> crate::common::RegisterFieldBool<8, 1, 0, T2Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, T2Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T2 Remote Control   T2RC"]
    #[inline(always)]
    pub fn t2rc(self) -> crate::common::RegisterFieldBool<9, 1, 0, T2Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, T2Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T2 Interrupt Disable   T2IRDIS"]
    #[inline(always)]
    pub fn t2irdis(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, T2Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, T2Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T2 Edge Detection   T2EDGE. The bit is set each time a count edge is detected. T2EDGE must be cleared by software."]
    #[inline(always)]
    pub fn t2edge(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, T2Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, T2Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T2 Count Direction Change   T2CHDIR. The bit is set each time the count direction of timer T2 changes. T2CHDIR must be cleared by software."]
    #[inline(always)]
    pub fn t2chdir(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, T2Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, T2Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T2 Rotation Direction   T2RDIR"]
    #[inline(always)]
    pub fn t2rdir(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, T2Con_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, T2Con_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for T2Con {
    #[inline(always)]
    fn default() -> T2Con {
        <crate::RegValueT<T2Con_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T3_SPEC;
impl crate::sealed::RegSpec for T3_SPEC {
    type DataType = u32;
}
#[doc = "Timer T3 Register\n resetvalue={Application Reset:0x0}"]
pub type T3 = crate::RegValueT<T3_SPEC>;

impl T3 {
    #[doc = "Timer T3   T3. Contains the current value of Timer T3."]
    #[inline(always)]
    pub fn t3(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, T3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, T3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T3 {
    #[inline(always)]
    fn default() -> T3 {
        <crate::RegValueT<T3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T3Con_SPEC;
impl crate::sealed::RegSpec for T3Con_SPEC {
    type DataType = u32;
}
#[doc = "Timer T3 Control Register\n resetvalue={Application Reset:0x0}"]
pub type T3Con = crate::RegValueT<T3Con_SPEC>;

impl T3Con {
    #[doc = "Timer T3 Input Parameter Selection   T3I. Depends on the operating mode  see respective sections for encoding  CROSSREFERENCE for Timer Mode and Gated Timer Mode CROSSREFERENCE for Counter Mode CROSSREFERENCE for Incremental Interface Mode"]
    #[inline(always)]
    pub fn t3i(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, T3Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, T3Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T3 Mode Control   T3M"]
    #[inline(always)]
    pub fn t3m(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, T3Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x7,1,0,u8, T3Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T3 Run Bit   T3R"]
    #[inline(always)]
    pub fn t3r(self) -> crate::common::RegisterFieldBool<6, 1, 0, T3Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, T3Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T3 Up Down Control   T3UD. This bit only directly controls count direction of T3 if bit T3UDE   0."]
    #[inline(always)]
    pub fn t3ud(self) -> crate::common::RegisterFieldBool<7, 1, 0, T3Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, T3Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T3 External Up Down Enable   T3UDE"]
    #[inline(always)]
    pub fn t3ude(self) -> crate::common::RegisterFieldBool<8, 1, 0, T3Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, T3Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Overflow Underflow Output Enable   T3OE"]
    #[inline(always)]
    pub fn t3oe(self) -> crate::common::RegisterFieldBool<9, 1, 0, T3Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, T3Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T3 Overflow Toggle Latch   T3OTL. Toggles on each overflow underflow of T3. Can be set or cleared by        software  see separate description"]
    #[inline(always)]
    pub fn t3otl(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, T3Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, T3Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GPT1 Block Prescaler Control   BPS1. Selects the basic clock for block GPT1  see also CROSSREFERENCE"]
    #[inline(always)]
    pub fn bps1(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, T3Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x3,1,0,u8, T3Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T3 Edge Detection Flag   T3EDGE. The bit is set each time a count edge is detected. T3EDGE must be        cleared by software."]
    #[inline(always)]
    pub fn t3edge(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, T3Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, T3Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T3 Count Direction Change Flag   T3CHDIR. This bit is set each time the count direction of timer T3 changes.        T3CHDIR must be cleared by software."]
    #[inline(always)]
    pub fn t3chdir(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, T3Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, T3Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T3 Rotation Direction Flag   T3RDIR"]
    #[inline(always)]
    pub fn t3rdir(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, T3Con_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, T3Con_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for T3Con {
    #[inline(always)]
    fn default() -> T3Con {
        <crate::RegValueT<T3Con_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T4_SPEC;
impl crate::sealed::RegSpec for T4_SPEC {
    type DataType = u32;
}
#[doc = "Timer T4 Register\n resetvalue={Application Reset:0x0}"]
pub type T4 = crate::RegValueT<T4_SPEC>;

impl T4 {
    #[doc = "Timer T4   T4. Contains the current value of Timer T4."]
    #[inline(always)]
    pub fn t4(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, T4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, T4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T4 {
    #[inline(always)]
    fn default() -> T4 {
        <crate::RegValueT<T4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T4Con_SPEC;
impl crate::sealed::RegSpec for T4Con_SPEC {
    type DataType = u32;
}
#[doc = "Timer T4 Control Register\n resetvalue={Application Reset:0x0}"]
pub type T4Con = crate::RegValueT<T4Con_SPEC>;

impl T4Con {
    #[doc = "Timer T4 Input Parameter Selection   T4I. Depends on the operating mode  see respective sections for encoding  CROSSREFERENCE for Timer Mode and Gated Timer Mode CROSSREFERENCE for Counter Mode CROSSREFERENCE for Incremental Interface Mode"]
    #[inline(always)]
    pub fn t4i(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, T4Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, T4Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T4 Mode Control  Basic Operating Mode    T4M"]
    #[inline(always)]
    pub fn t4m(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, T4Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x7,1,0,u8, T4Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T4 Run Bit   T4R. This bit only controls timer T4 if bit T4RC   0."]
    #[inline(always)]
    pub fn t4r(self) -> crate::common::RegisterFieldBool<6, 1, 0, T4Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, T4Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T4 Up Down Control   T4UD. This bit only directly controls count direction of T4 if bit T4UDE   0."]
    #[inline(always)]
    pub fn t4ud(self) -> crate::common::RegisterFieldBool<7, 1, 0, T4Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, T4Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T4 External Up Down Enable   T4UDE"]
    #[inline(always)]
    pub fn t4ude(self) -> crate::common::RegisterFieldBool<8, 1, 0, T4Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, T4Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T4 Remote Control   T4RC"]
    #[inline(always)]
    pub fn t4rc(self) -> crate::common::RegisterFieldBool<9, 1, 0, T4Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, T4Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Timer T2 Enable   CLRT2EN. Enables the automatic clearing of timer T2 upon a falling edge of the selected T4EUD input."]
    #[inline(always)]
    pub fn clrt2en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, T4Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, T4Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Timer T3 Enable   CLRT3EN. Enables the automatic clearing of timer T3 upon a falling edge of the selected T4IN input."]
    #[inline(always)]
    pub fn clrt3en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, T4Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, T4Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T4 Interrupt Disable   T4IRDIS"]
    #[inline(always)]
    pub fn t4irdis(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, T4Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, T4Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T4 Edge Detection   T4EDGE. The bit is set each time a count edge is detected. T4EDGE has to be cleared by software."]
    #[inline(always)]
    pub fn t4edge(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, T4Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, T4Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T4 Count Direction Change   T4CHDIR. The bit is set each time the count direction of timer T4 changes. T4CHDIR must be cleared by software."]
    #[inline(always)]
    pub fn t4chdir(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, T4Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, T4Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T4 Rotation Direction   T4RDIR"]
    #[inline(always)]
    pub fn t4rdir(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, T4Con_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, T4Con_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for T4Con {
    #[inline(always)]
    fn default() -> T4Con {
        <crate::RegValueT<T4Con_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T5_SPEC;
impl crate::sealed::RegSpec for T5_SPEC {
    type DataType = u32;
}
#[doc = "Timer T5 Register\n resetvalue={Application Reset:0x0}"]
pub type T5 = crate::RegValueT<T5_SPEC>;

impl T5 {
    #[doc = "Timer T5   T5. Contains the current value of Timer T5."]
    #[inline(always)]
    pub fn t5(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, T5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, T5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T5 {
    #[inline(always)]
    fn default() -> T5 {
        <crate::RegValueT<T5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T5Con_SPEC;
impl crate::sealed::RegSpec for T5Con_SPEC {
    type DataType = u32;
}
#[doc = "Timer T5 Control Register\n resetvalue={Application Reset:0x0}"]
pub type T5Con = crate::RegValueT<T5Con_SPEC>;

impl T5Con {
    #[doc = "Timer T5 Input Parameter Selection   T5I. Depends on the operating mode  see respective sections for encoding  CROSSREFERENCE for Timer Mode and Gated Timer Mode CROSSREFERENCE for Counter Mode"]
    #[inline(always)]
    pub fn t5i(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, T5Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, T5Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T5 Mode Control  Basic Operating Mode    T5M. In the implementation the T5M bit field has only 2 bits and T5CON bit          5 is reserved  AI00045855 ."]
    #[inline(always)]
    pub fn t5m(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, T5Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x7,1,0,u8, T5Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T5 Run Bit   T5R. This bit only controls timer T5 if bit T5RC   0."]
    #[inline(always)]
    pub fn t5r(self) -> crate::common::RegisterFieldBool<6, 1, 0, T5Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, T5Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T5 Up Down Control   T5UD. This bit only directly controls count direction of T5 if bit T5UDE   0."]
    #[inline(always)]
    pub fn t5ud(self) -> crate::common::RegisterFieldBool<7, 1, 0, T5Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, T5Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T5 External Up Down Enable   T5UDE"]
    #[inline(always)]
    pub fn t5ude(self) -> crate::common::RegisterFieldBool<8, 1, 0, T5Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, T5Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T5 Remote Control   T5RC"]
    #[inline(always)]
    pub fn t5rc(self) -> crate::common::RegisterFieldBool<9, 1, 0, T5Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, T5Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T3 Capture Trigger Enable   CT3"]
    #[inline(always)]
    pub fn ct3(self) -> crate::common::RegisterFieldBool<10, 1, 0, T5Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, T5Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Register CAPREL Capture Trigger Selection   CI"]
    #[inline(always)]
    pub fn ci(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, T5Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, T5Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T5 Clear Enable Bit   T5CLR"]
    #[inline(always)]
    pub fn t5clr(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, T5Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, T5Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T5 Capture Mode Enable   T5SC"]
    #[inline(always)]
    pub fn t5sc(self) -> crate::common::RegisterFieldBool<15, 1, 0, T5Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, T5Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for T5Con {
    #[inline(always)]
    fn default() -> T5Con {
        <crate::RegValueT<T5Con_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T6_SPEC;
impl crate::sealed::RegSpec for T6_SPEC {
    type DataType = u32;
}
#[doc = "Timer T6 Register\n resetvalue={Application Reset:0x0}"]
pub type T6 = crate::RegValueT<T6_SPEC>;

impl T6 {
    #[doc = "Timer T6   T6. Contains the current value of Timer T6."]
    #[inline(always)]
    pub fn t6(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, T6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, T6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T6 {
    #[inline(always)]
    fn default() -> T6 {
        <crate::RegValueT<T6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T6Con_SPEC;
impl crate::sealed::RegSpec for T6Con_SPEC {
    type DataType = u32;
}
#[doc = "Timer T6 Control Register\n resetvalue={Application Reset:0x0}"]
pub type T6Con = crate::RegValueT<T6Con_SPEC>;

impl T6Con {
    #[doc = "Timer T6 Input Parameter Selection   T6I. Depends on the operating mode  see respective sections for encoding  CROSSREFERENCE for Timer Mode and Gated Timer Mode CROSSREFERENCE for Counter Mode"]
    #[inline(always)]
    pub fn t6i(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, T6Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, T6Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T6 Mode Control  Basic Operating Mode    T6M"]
    #[inline(always)]
    pub fn t6m(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, T6Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x7,1,0,u8, T6Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T6 Run Bit   T6R"]
    #[inline(always)]
    pub fn t6r(self) -> crate::common::RegisterFieldBool<6, 1, 0, T6Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, T6Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T6 Up Down Control   T6UD. This bit only directly controls count direction of T6 if bit T6UDE   0."]
    #[inline(always)]
    pub fn t6ud(self) -> crate::common::RegisterFieldBool<7, 1, 0, T6Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, T6Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T6 External Up Down Enable   T6UDE"]
    #[inline(always)]
    pub fn t6ude(self) -> crate::common::RegisterFieldBool<8, 1, 0, T6Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, T6Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Overflow Underflow Output Enable   T6OE"]
    #[inline(always)]
    pub fn t6oe(self) -> crate::common::RegisterFieldBool<9, 1, 0, T6Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, T6Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T6 Overflow Toggle Latch   T6OTL. Toggles on each overflow underflow of timer T6. Can be set or cleared by        software  see separate description"]
    #[inline(always)]
    pub fn t6otl(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, T6Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, T6Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GPT2 Block Prescaler Control   BPS2. Selects the basic clock for block GPT2  see also CROSSREFERENCE"]
    #[inline(always)]
    pub fn bps2(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, T6Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x3,1,0,u8, T6Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T6 Clear Enable Bit   T6CLR"]
    #[inline(always)]
    pub fn t6clr(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, T6Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, T6Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T6 Reload Mode Enable   T6SR"]
    #[inline(always)]
    pub fn t6sr(self) -> crate::common::RegisterFieldBool<15, 1, 0, T6Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, T6Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for T6Con {
    #[inline(always)]
    fn default() -> T6Con {
        <crate::RegValueT<T6Con_SPEC> as RegisterValue<_>>::new(0)
    }
}
