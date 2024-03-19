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
#[doc = r"EVADC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evadc(pub(super) *mut u8);
unsafe impl core::marker::Send for Evadc {}
unsafe impl core::marker::Sync for Evadc {}
impl Evadc {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "Access Protection Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn accprot0(&self) -> crate::common::Reg<self::Accprot0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }

    #[doc = "Access Protection Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn accprot1(&self) -> crate::common::Reg<self::Accprot1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(140usize)) }
    }

    #[doc = "Access Protection Register 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn accprot2(&self) -> crate::common::Reg<self::Accprot2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "External Multiplexer Interface Select Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn emuxsel(&self) -> crate::common::Reg<self::Emuxsel_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1008usize)) }
    }

    #[doc = "Global Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globcfg(&self) -> crate::common::Reg<self::Globcfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={PowerOn Reset:0x0C5C013}"]
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
    #[doc = "FC"]
    #[inline(always)]
    pub fn fc(self) -> [self::Fc; 4] {
        unsafe {
            [
                self::Fc(self.0.add(0x3400usize + 0x0usize)),
                self::Fc(self.0.add(0x3400usize + 0x100usize)),
                self::Fc(self.0.add(0x3400usize + 0x200usize)),
                self::Fc(self.0.add(0x3400usize + 0x300usize)),
            ]
        }
    }
    #[doc = "GLOB"]
    #[inline(always)]
    pub fn glob(self) -> self::Glob {
        unsafe { self::Glob(self.0.add(160usize)) }
    }
    #[doc = "G"]
    #[inline(always)]
    pub fn g(self) -> [self::G; 12] {
        unsafe {
            [
                self::G(self.0.add(0x400usize + 0x0usize)),
                self::G(self.0.add(0x400usize + 0x400usize)),
                self::G(self.0.add(0x400usize + 0x800usize)),
                self::G(self.0.add(0x400usize + 0xc00usize)),
                self::G(self.0.add(0x400usize + 0x1000usize)),
                self::G(self.0.add(0x400usize + 0x1400usize)),
                self::G(self.0.add(0x400usize + 0x1800usize)),
                self::G(self.0.add(0x400usize + 0x1c00usize)),
                self::G(self.0.add(0x400usize + 0x2000usize)),
                self::G(self.0.add(0x400usize + 0x2400usize)),
                self::G(self.0.add(0x400usize + 0x2800usize)),
                self::G(self.0.add(0x400usize + 0x2c00usize)),
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
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
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
pub struct Accprot0_SPEC;
impl crate::sealed::RegSpec for Accprot0_SPEC {
    type DataType = u32;
}
#[doc = "Access Protection Register 0\n resetvalue={Application Reset:0x0}"]
pub type Accprot0 = crate::RegValueT<Accprot0_SPEC>;

impl Accprot0 {
    #[doc = "Access Protection Channel Control  Primary Groups. Each bit of this bitfield is associated with the corresponding group. 0  Full access to registers 1  Write access to channel control registers is blocked"]
    #[inline(always)]
    pub fn apcp(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Accprot0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Accprot0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Protection Channel Control  Secondary Groups. Each bit of this bitfield is associated with the corresponding group. 0  Full access to registers 1  Write access to channel control registers is blocked"]
    #[inline(always)]
    pub fn apcs(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Accprot0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Accprot0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Protection Initialization  Primary Groups. Each bit of this bitfield is associated with the corresponding group. 0  Full access to registers 1  Write access to initialization registers is blocked"]
    #[inline(always)]
    pub fn apip(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Accprot0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Accprot0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Protection Initialization  Secondary Groups. Each bit of this bitfield is associated with the corresponding group. 0  Full access to registers 1  Write access to initialization registers is blocked"]
    #[inline(always)]
    pub fn apis(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Accprot0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Accprot0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Accprot0 {
    #[inline(always)]
    fn default() -> Accprot0 {
        <crate::RegValueT<Accprot0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accprot1_SPEC;
impl crate::sealed::RegSpec for Accprot1_SPEC {
    type DataType = u32;
}
#[doc = "Access Protection Register 1\n resetvalue={Application Reset:0x0}"]
pub type Accprot1 = crate::RegValueT<Accprot1_SPEC>;

impl Accprot1 {
    #[doc = "Access Protection Service Request  Primary Groups. Each bit of this bitfield is associated with the corresponding group. 0  Full access to registers 1  Write access to service request registers is blocked"]
    #[inline(always)]
    pub fn apsp(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Accprot1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Accprot1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Protection Service Request  Secondary Groups. Each bit of this bitfield is associated with the corresponding group. 0  Full access to registers 1  Write access to service request registers is blocked"]
    #[inline(always)]
    pub fn apss(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Accprot1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Accprot1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Protection Result Registers  Primary Groups. Each bit of this bitfield is associated with the corresponding group. 0  Full access to registers 1  Write access to result registers is blocked"]
    #[inline(always)]
    pub fn aprp(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Accprot1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Accprot1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Protection Result Registers  Secondary Groups. Each bit of this bitfield is associated with the corresponding group. 0  Full access to registers 1  Write access to result registers is blocked"]
    #[inline(always)]
    pub fn aprs(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Accprot1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Accprot1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Accprot1 {
    #[inline(always)]
    fn default() -> Accprot1 {
        <crate::RegValueT<Accprot1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accprot2_SPEC;
impl crate::sealed::RegSpec for Accprot2_SPEC {
    type DataType = u32;
}
#[doc = "Access Protection Register 2\n resetvalue={Application Reset:0x0}"]
pub type Accprot2 = crate::RegValueT<Accprot2_SPEC>;

impl Accprot2 {
    #[doc = "Access Protection Fast Compare Channels. Each bit of this bitfield is associated with the corresponding channel. 0  Full access to registers 1  Write access to fast compare channel registers is blocked"]
    #[inline(always)]
    pub fn apf(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Accprot2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Accprot2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Protection Global Configuration   APGC"]
    #[inline(always)]
    pub fn apgc(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accprot2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Accprot2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Protection External Multiplexer   APEM"]
    #[inline(always)]
    pub fn apem(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accprot2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,Accprot2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Protection Test Function   APTF"]
    #[inline(always)]
    pub fn aptf(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accprot2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,Accprot2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Accprot2 {
    #[inline(always)]
    fn default() -> Accprot2 {
        <crate::RegValueT<Accprot2_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control of the module. Also the analog section        is disabled by clearing ANONS."]
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
pub struct Emuxsel_SPEC;
impl crate::sealed::RegSpec for Emuxsel_SPEC {
    type DataType = u32;
}
#[doc = "External Multiplexer Interface Select Register\n resetvalue={Application Reset:0x0}"]
pub type Emuxsel = crate::RegValueT<Emuxsel_SPEC>;

impl Emuxsel {
    #[doc = "External Multiplexer Group for Interface 0. Defines the group whose external multiplexer control signals are routed        to EMUX interface 0  pins EMUX0x ."]
    #[inline(always)]
    pub fn emuxgrp0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Emuxsel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Emuxsel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Multiplexer Group for Interface 1. Defines the group whose external multiplexer control signals are routed        to EMUX interface 1  pins EMUX1x ."]
    #[inline(always)]
    pub fn emuxgrp1(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Emuxsel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Emuxsel_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Emuxsel {
    #[inline(always)]
    fn default() -> Emuxsel {
        <crate::RegValueT<Emuxsel_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Unsynchronized Clock Generation   USC. Defines the way the analog clock is generated."]
    #[inline(always)]
    pub fn usc(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Globcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Globcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Supply Voltage Level   SUPLEV. Adjusts the analog circuitry to the supply voltage used in the        application system. Make sure to keep SUPLEV  160    160  00 or 01 in the case of a 5  160 V supply."]
    #[inline(always)]
    pub fn suplev(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, Globcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x3,1,0,u8, Globcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Write Control for Control Parameters   CPWC"]
    #[inline(always)]
    pub fn cpwc(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Globcfg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Globcfg_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Start Up Calibration   SUCAL. Writing 1 to bit SUCAL initiates the start up calibration phase of all enabled analog converters  except for the fast compare channels . Before and during start up calibration  all converters must be inactive. After reset this is the case anyway."]
    #[inline(always)]
    pub fn sucal(
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
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={PowerOn Reset:0x0C5C013}"]
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
    #[doc = "Module Number   MOD NUMBER. Indicates the module identification number   00C5   SARADC ."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(12959763)
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
    #[doc = "OCDS Suspend Control   SUS. Controls the sensitivity to the suspend signal coming from the OCDS        Trigger Switch  OTGS  Not listed combinations are reserved.  Same as 0000  ."]
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

#[doc = "FC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fc(pub(super) *mut u8);
unsafe impl core::marker::Send for Fc {}
unsafe impl core::marker::Sync for Fc {}
impl Fc {
    #[doc = "Boundary Flag Register  FC Channel 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcxfcbfl(&self) -> crate::common::Reg<fc::FCxFcbfl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "Fast Compare Control Register  FC Channel 0\n resetvalue={Application Reset:0x0C20}"]
    #[inline(always)]
    pub const fn fcxfcctrl(&self) -> crate::common::Reg<fc::FCxFcctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Fast Comp. Hysteresis Register  FC Channel 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcxfchyst(&self) -> crate::common::Reg<fc::FCxFchyst_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Fast Compare Mode Register  FC Channel 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcxfcm(&self) -> crate::common::Reg<fc::FCxFcm_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Fast Compare Ramp Register 0  FC Channel 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcxfcramp0(&self) -> crate::common::Reg<fc::FCxFcramp0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Fast Compare Ramp Register 1  FC Channel 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcxfcramp1(&self) -> crate::common::Reg<fc::FCxFcramp1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
}
pub mod fc {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCxFcbfl_SPEC;
    impl crate::sealed::RegSpec for FCxFcbfl_SPEC {
        type DataType = u32;
    }
    #[doc = "Boundary Flag Register  FC Channel 0\n resetvalue={Application Reset:0x0}"]
    pub type FCxFcbfl = crate::RegValueT<FCxFcbfl_SPEC>;

    impl FCxFcbfl {
        #[doc = "Boundary Flag   BFL"]
        #[inline(always)]
        pub fn bfl(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, FCxFcbfl_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0,1,0,FCxFcbfl_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Boundary Flag Activation Select   BFA"]
        #[inline(always)]
        pub fn bfa(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, FCxFcbfl_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,FCxFcbfl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Boundary Flag Inversion Control   BFI"]
        #[inline(always)]
        pub fn bfi(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, FCxFcbfl_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,FCxFcbfl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Boundary Flag Software Control   BFS"]
        #[inline(always)]
        pub fn bfs(
            self,
        ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, FCxFcbfl_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<12,0x3,1,0,u8, FCxFcbfl_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Boundary Flag Mode Control   BFM"]
        #[inline(always)]
        pub fn bfm(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, FCxFcbfl_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<16,1,0,FCxFcbfl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Boundary Flag Value   BFV. Defines the logic value that replaces the compare result while the gate        input is inactive  low  in lock mode  see also bitfield GTMODE in        register GxFCCTRL"]
        #[inline(always)]
        pub fn bfv(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, FCxFcbfl_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<17,1,0,FCxFcbfl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Boundary Flag Node Pointer. For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved. Shall be        implemented as 1111  no common output."]
        #[inline(always)]
        pub fn bflnp(
            self,
        ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, FCxFcbfl_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0xf,1,0,u8, FCxFcbfl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Fast Compare Result   FCR. Indicates the last generated compare result If the input signal equals the reference value the analog comparator          will return either  quot above quot  or  quot below quot ."]
        #[inline(always)]
        pub fn fcr(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, FCxFcbfl_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<28,1,0,FCxFcbfl_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Valid Flag   VF. Indicates a new result in bit FCR. Bit VF is automatically cleared upon reading register FCxFCBFL."]
        #[inline(always)]
        pub fn vf(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, FCxFcbfl_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<31,1,0,FCxFcbfl_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for FCxFcbfl {
        #[inline(always)]
        fn default() -> FCxFcbfl {
            <crate::RegValueT<FCxFcbfl_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCxFcctrl_SPEC;
    impl crate::sealed::RegSpec for FCxFcctrl_SPEC {
        type DataType = u32;
    }
    #[doc = "Fast Compare Control Register  FC Channel 0\n resetvalue={Application Reset:0x0C20}"]
    pub type FCxFcctrl = crate::RegValueT<FCxFcctrl_SPEC>;

    impl FCxFcctrl {
        #[doc = "Sample Time Control for Fast Comparisons   STCF. Number of additional clock cycles to be added to the minimum sample        phase of 2 analog clock cycles  Coding and resulting sample time see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn stcf(
            self,
        ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, FCxFcctrl_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1f,1,0,u8, FCxFcctrl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Reference Precharge Enable   RPE. Enabled after reset."]
        #[inline(always)]
        pub fn rpe(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, FCxFcctrl_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,FCxFcctrl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Analog Input Precharge Enable for Fast Comparisons   AIPF. The buffer is enabled automatically while AIPF   8800  00 ."]
        #[inline(always)]
        pub fn aipf(
            self,
        ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, FCxFcctrl_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<6,0x3,1,0,u8, FCxFcctrl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Channel Event Mode   CHEVMODE. Generate a channel event  For a service request summary  refer to CROSSREFERENCE ."]
        #[inline(always)]
        pub fn chevmode(
            self,
        ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, FCxFcctrl_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x3,1,0,u8, FCxFcctrl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Divider Factor for the Analog Internal Clock   DIVA. Defines the frequency of the analog converter clock f ADCI  base clock for conversion steps   derived from the peripheral clock  f ADCI   f ADC   CP."]
        #[inline(always)]
        pub fn diva(
            self,
        ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, FCxFcctrl_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<10,0x1f,1,0,u8, FCxFcctrl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Write Control for Control Parameters   CPWC"]
        #[inline(always)]
        pub fn cpwc(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, FCxFcctrl_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<15,1,0,FCxFcctrl_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "External Trigger Input Selection   XTSEL. The connected trigger input signals are listed in the product specific        appendix. The selected input signal can be used as a trigger source or as a gate          signal  depending on the operating mode."]
        #[inline(always)]
        pub fn xtsel(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, FCxFcctrl_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, FCxFcctrl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "External Trigger Level   XTLVL. Current level of the selected trigger input"]
        #[inline(always)]
        pub fn xtlvl(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, FCxFcctrl_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<20,1,0,FCxFcctrl_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Trigger Operating Mode   XTMODE"]
        #[inline(always)]
        pub fn xtmode(
            self,
        ) -> crate::common::RegisterField<21, 0x3, 1, 0, u8, FCxFcctrl_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<21,0x3,1,0,u8, FCxFcctrl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "External Trigger Polarity   XTPOL"]
        #[inline(always)]
        pub fn xtpol(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, FCxFcctrl_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<23,1,0,FCxFcctrl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Gate Operating Mode   GTMODE"]
        #[inline(always)]
        pub fn gtmode(
            self,
        ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, FCxFcctrl_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0x3,1,0,u8, FCxFcctrl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Fast Compare Channel  Cannel Number   FCCHNR. Selects the input signal to be compared  1C   8230  1F select internal        signals  used for testing   all other channel numbers are reserved. See   8220 Common Input Signals  8221  in the product specific appendix."]
        #[inline(always)]
        pub fn fcchnr(
            self,
        ) -> crate::common::RegisterField<26, 0x1f, 1, 0, u8, FCxFcctrl_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<26,0x1f,1,0,u8, FCxFcctrl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Write Control for Trigger Gate Configuration   XTWC"]
        #[inline(always)]
        pub fn xtwc(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, FCxFcctrl_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<31,1,0,FCxFcctrl_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for FCxFcctrl {
        #[inline(always)]
        fn default() -> FCxFcctrl {
            <crate::RegValueT<FCxFcctrl_SPEC> as RegisterValue<_>>::new(3104)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCxFchyst_SPEC;
    impl crate::sealed::RegSpec for FCxFchyst_SPEC {
        type DataType = u32;
    }
    #[doc = "Fast Comp. Hysteresis Register  FC Channel 0\n resetvalue={Application Reset:0x0}"]
    pub type FCxFchyst = crate::RegValueT<FCxFchyst_SPEC>;

    impl FCxFchyst {
        #[doc = "Lower Delta Value   DELTAMINUS. This value is subtracted from the reference value while the last result        is 1"]
        #[inline(always)]
        pub fn deltaminus(
            self,
        ) -> crate::common::RegisterField<2, 0x3ff, 1, 0, u16, FCxFchyst_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x3ff,1,0,u16, FCxFchyst_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Upper Delta Value   DELTAPLUS. This value is added to the reference value while the last result is 0"]
        #[inline(always)]
        pub fn deltaplus(
            self,
        ) -> crate::common::RegisterField<18, 0x3ff, 1, 0, u16, FCxFchyst_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<18,0x3ff,1,0,u16, FCxFchyst_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for FCxFchyst {
        #[inline(always)]
        fn default() -> FCxFchyst {
            <crate::RegValueT<FCxFchyst_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCxFcm_SPEC;
    impl crate::sealed::RegSpec for FCxFcm_SPEC {
        type DataType = u32;
    }
    #[doc = "Fast Compare Mode Register  FC Channel 0\n resetvalue={Application Reset:0x0}"]
    pub type FCxFcm = crate::RegValueT<FCxFcm_SPEC>;

    impl FCxFcm {
        #[doc = "Run Control for Compare Channel   RUNCOMP. Defines the basic run conditions of the fast compare channel."]
        #[inline(always)]
        pub fn runcomp(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, FCxFcm_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3,1,0,u8, FCxFcm_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Run Control for Ramp   RUNRAMP. Defines the run conditions for the ramp generation. Before changing the operating mode  stop the ramp timer  i.e. RUNRAMP            00 . REQTRx is the selected trigger          signal."]
        #[inline(always)]
        pub fn runramp(
            self,
        ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, FCxFcm_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x3,1,0,u8, FCxFcm_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Fast Compare Ramp Direction   FCRDIR"]
        #[inline(always)]
        pub fn fcrdir(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, FCxFcm_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,FCxFcm_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Analog Converter Control   ANON. The extended wakeup time is required before the analog part is fully          operable  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn anon(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, FCxFcm_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,FCxFcm_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Analog Clock Synchronization Delay   ACSD. Defines the delay of the analog clock in clocks after the sync signal. Do not exceed the current DIVA setting for the clock delay. Valid only          if the phase synchronizer is selected  USC  160    160  0"]
        #[inline(always)]
        pub fn acsd(
            self,
        ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, FCxFcm_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<6,0x3,1,0,u8, FCxFcm_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Fast Compare Trigger Interval   FCTRIV. Defines the interval at which fast compare operations are triggered in        steps of 16   215  1  f ADC ."]
        #[inline(always)]
        pub fn fctriv(
            self,
        ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, FCxFcm_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xff,1,0,u8, FCxFcm_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Generation   SRG. For a service request summary  refer to CROSSREFERENCE ."]
        #[inline(always)]
        pub fn srg(
            self,
        ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, FCxFcm_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x3,1,0,u8, FCxFcm_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Automatic Update Enable   AUE. Defines the source of the value s  in bitfield FCREF."]
        #[inline(always)]
        pub fn aue(
            self,
        ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, FCxFcm_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<18,0x3,1,0,u8, FCxFcm_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Sample Synchronization Enable   SSE. See section CROSSREFERENCE ."]
        #[inline(always)]
        pub fn sse(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, FCxFcm_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<20,1,0,FCxFcm_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Write Control for Fast Compare Mode Configuration   FCMWC"]
        #[inline(always)]
        pub fn fcmwc(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, FCxFcm_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<21,1,0,FCxFcm_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Fast Compare Reference Value   FCREF. The input level is compared to this value. The resulting reference level is V AREF   1  xa0 024   xd7   lt FCREF gt ."]
        #[inline(always)]
        pub fn fcref(
            self,
        ) -> crate::common::RegisterField<22, 0x3ff, 1, 0, u16, FCxFcm_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<22,0x3ff,1,0,u16, FCxFcm_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for FCxFcm {
        #[inline(always)]
        fn default() -> FCxFcm {
            <crate::RegValueT<FCxFcm_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCxFcramp0_SPEC;
    impl crate::sealed::RegSpec for FCxFcramp0_SPEC {
        type DataType = u32;
    }
    #[doc = "Fast Compare Ramp Register 0  FC Channel 0\n resetvalue={Application Reset:0x0}"]
    pub type FCxFcramp0 = crate::RegValueT<FCxFcramp0_SPEC>;

    impl FCxFcramp0 {
        #[doc = "Fast Compare Ramp Compare Value A   FCRCOMPA. The content of FCRCOMPA is copied to the ramp counter when a ramp is        started  i.e. upon the selected trigger event. The ramp counter defines        the reference value during ramp generation. It is  therefore  copied to        bitfield FCREF when the ramp is started and each time the counter        changes. FCRCOMPA is also used in alternate value mode while the gate is active         high ."]
        #[inline(always)]
        pub fn fcrcompa(
            self,
        ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, FCxFcramp0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3ff,1,0,u16, FCxFcramp0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Fast Compare Ramp Step Width   FCRSTEP. Configures the prescaler for FCRCOUNT in increments of 8   215  1  f ADC ."]
        #[inline(always)]
        pub fn fcrstep(
            self,
        ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, FCxFcramp0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xff,1,0,u8, FCxFcramp0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Write Control for Fast Compare Stepwidth   FSWC"]
        #[inline(always)]
        pub fn fswc(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, FCxFcramp0_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<31,1,0,FCxFcramp0_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for FCxFcramp0 {
        #[inline(always)]
        fn default() -> FCxFcramp0 {
            <crate::RegValueT<FCxFcramp0_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCxFcramp1_SPEC;
    impl crate::sealed::RegSpec for FCxFcramp1_SPEC {
        type DataType = u32;
    }
    #[doc = "Fast Compare Ramp Register 1  FC Channel 0\n resetvalue={Application Reset:0x0}"]
    pub type FCxFcramp1 = crate::RegValueT<FCxFcramp1_SPEC>;

    impl FCxFcramp1 {
        #[doc = "Fast Compare Ramp Compare Value B   FCRCOMPB. Defines the stop level of the generated ramp. FCRCOMPB is also used in alternate value mode while the gate is inactive         low ."]
        #[inline(always)]
        pub fn fcrcompb(
            self,
        ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, FCxFcramp1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3ff,1,0,u16, FCxFcramp1_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for FCxFcramp1 {
        #[inline(always)]
        fn default() -> FCxFcramp1 {
            <crate::RegValueT<FCxFcramp1_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "GLOB"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Glob(pub(super) *mut u8);
unsafe impl core::marker::Send for Glob {}
unsafe impl core::marker::Sync for Glob {}
impl Glob {
    #[doc = "Global Boundary Select Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globbound(&self) -> crate::common::Reg<glob::Globbound_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Global Event Flag Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globeflag(&self) -> crate::common::Reg<glob::Globeflag_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }
    #[doc = "Global Event Node Pointer Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globevnp(&self) -> crate::common::Reg<glob::Globevnp_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }
    #[doc = "Input Class Register 0  Global\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globiclassi(
        &self,
    ) -> [crate::common::Reg<glob::GlobiclasSi_SPEC, crate::common::RW>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
            ]
        }
    }
    #[doc = "Global Result Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globrcr(&self) -> crate::common::Reg<glob::Globrcr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(480usize)) }
    }
    #[doc = "Global Result Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globres(&self) -> crate::common::Reg<glob::Globres_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(608usize)) }
    }
    #[doc = "Global Result Register  Debug\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globresd(&self) -> crate::common::Reg<glob::Globresd_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(736usize)) }
    }
    #[doc = "Global Test Enable Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globte(&self) -> crate::common::Reg<glob::Globte_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(196usize)) }
    }
    #[doc = "Global Test Functions Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globtf(&self) -> crate::common::Reg<glob::Globtf_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(192usize)) }
    }
}
pub mod glob {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Globbound_SPEC;
    impl crate::sealed::RegSpec for Globbound_SPEC {
        type DataType = u32;
    }
    #[doc = "Global Boundary Select Register\n resetvalue={Application Reset:0x0}"]
    pub type Globbound = crate::RegValueT<Globbound_SPEC>;

    impl Globbound {
        #[doc = "Boundary Value 0 for Limit Checking   BOUNDARY0. This value is compared against the conversion result."]
        #[inline(always)]
        pub fn boundary0(
            self,
        ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, Globbound_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xfff,1,0,u16, Globbound_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Boundary Value 1 for Limit Checking   BOUNDARY1. This value is compared against the conversion result."]
        #[inline(always)]
        pub fn boundary1(
            self,
        ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, Globbound_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xfff,1,0,u16, Globbound_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Globbound {
        #[inline(always)]
        fn default() -> Globbound {
            <crate::RegValueT<Globbound_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Globeflag_SPEC;
    impl crate::sealed::RegSpec for Globeflag_SPEC {
        type DataType = u32;
    }
    #[doc = "Global Event Flag Register\n resetvalue={Application Reset:0x0}"]
    pub type Globeflag = crate::RegValueT<Globeflag_SPEC>;

    impl Globeflag {
        #[doc = "Global Result Event   REVGLB"]
        #[inline(always)]
        pub fn revglb(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, Globeflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,Globeflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Clear Global Result Event   REVGLBCLR"]
        #[inline(always)]
        pub fn revglbclr(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, Globeflag_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<24,1,0,Globeflag_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for Globeflag {
        #[inline(always)]
        fn default() -> Globeflag {
            <crate::RegValueT<Globeflag_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Globevnp_SPEC;
    impl crate::sealed::RegSpec for Globevnp_SPEC {
        type DataType = u32;
    }
    #[doc = "Global Event Node Pointer Register\n resetvalue={Application Reset:0x0}"]
    pub type Globevnp = crate::RegValueT<Globevnp_SPEC>;

    impl Globevnp {
        #[doc = "Service Request Node Pointer Global Result. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn rev0np(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Globevnp_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, Globevnp_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Globevnp {
        #[inline(always)]
        fn default() -> Globevnp {
            <crate::RegValueT<Globevnp_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GlobiclasSi_SPEC;
    impl crate::sealed::RegSpec for GlobiclasSi_SPEC {
        type DataType = u32;
    }
    #[doc = "Input Class Register 0  Global\n resetvalue={Application Reset:0x0}"]
    pub type GlobiclasSi = crate::RegValueT<GlobiclasSi_SPEC>;

    impl GlobiclasSi {
        #[doc = "Sample Time Control for Standard Conversions   STCS. Number of additional clock cycles to be added to the minimum sample        phase of 2 sample clock cycles  Coding and resulting sample time see CROSSREFERENCE . For conversions of external channels  the value from bitfield STCE can        be used."]
        #[inline(always)]
        pub fn stcs(
            self,
        ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, GlobiclasSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1f,1,0,u8, GlobiclasSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Analog Input Precharge Control for Standard Conversions   AIPS. Buffer must be enabled by BE  160    160 1  see GxANCFG  ."]
        #[inline(always)]
        pub fn aips(
            self,
        ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, GlobiclasSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<6,0x3,1,0,u8, GlobiclasSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Conversion Mode for Standard Conversions   CMS"]
        #[inline(always)]
        pub fn cms(
            self,
        ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, GlobiclasSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x3,1,0,u8, GlobiclasSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Spread Early Sample Point for Standard Conversions   SESPS"]
        #[inline(always)]
        pub fn sesps(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, GlobiclasSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<10,1,0,GlobiclasSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Sample Time Control for EMUX Conversions   STCE. Number of additional clock cycles to be added to the minimum sample        phase of 2 sample clock cycles  Coding and resulting sample time see CROSSREFERENCE . For conversions of standard channels  the value from bitfield STCS is        used."]
        #[inline(always)]
        pub fn stce(
            self,
        ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, GlobiclasSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1f,1,0,u8, GlobiclasSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Analog Input Precharge Control for EMUX Conversions   AIPE. Buffer must be enabled by BE  160    160 1  see GxANCFG  ."]
        #[inline(always)]
        pub fn aipe(
            self,
        ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, GlobiclasSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<22,0x3,1,0,u8, GlobiclasSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Conversion Mode for EMUX Conversions   CME"]
        #[inline(always)]
        pub fn cme(
            self,
        ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, GlobiclasSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0x3,1,0,u8, GlobiclasSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Spread Early Sample Point for EMUX Conversions   SESPE"]
        #[inline(always)]
        pub fn sespe(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, GlobiclasSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<26,1,0,GlobiclasSi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GlobiclasSi {
        #[inline(always)]
        fn default() -> GlobiclasSi {
            <crate::RegValueT<GlobiclasSi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Globrcr_SPEC;
    impl crate::sealed::RegSpec for Globrcr_SPEC {
        type DataType = u32;
    }
    #[doc = "Global Result Control Register\n resetvalue={Application Reset:0x0}"]
    pub type Globrcr = crate::RegValueT<Globrcr_SPEC>;

    impl Globrcr {
        #[doc = "Data Reduction Control   DRCTR. Defines how result values are stored accumulated in this register for        the final result. The data reduction counter DRC can be loaded from this        bitfield. Coding see CROSSREFERENCE Only        standard data reduction is available for the global result register         i.e. DMM is assumed as 00 ."]
        #[inline(always)]
        pub fn drctr(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Globrcr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, Globrcr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Wait for Read Mode Enable   WFR. Refer also to  Wait for Read Mode ."]
        #[inline(always)]
        pub fn wfr(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, Globrcr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<24,1,0,Globrcr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Generation Enable   SRGEN"]
        #[inline(always)]
        pub fn srgen(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, Globrcr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<31,1,0,Globrcr_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Globrcr {
        #[inline(always)]
        fn default() -> Globrcr {
            <crate::RegValueT<Globrcr_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Globres_SPEC;
    impl crate::sealed::RegSpec for Globres_SPEC {
        type DataType = u32;
    }
    #[doc = "Global Result Register\n resetvalue={Application Reset:0x0}"]
    pub type Globres = crate::RegValueT<Globres_SPEC>;

    impl Globres {
        #[doc = "Result of most recent conversion   RESULT. The position of the result bits within this bitfield depends on the        configured operating mode. Refer to CROSSREFERENCE ."]
        #[inline(always)]
        pub fn result(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Globres_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, Globres_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Group Number   GNR. Indicates the group to which the channel number in bitfield CHNR refers."]
        #[inline(always)]
        pub fn gnr(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Globres_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, Globres_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Channel Number   CHNR. Indicates the channel number corresponding to the value in bitfield        RESULT."]
        #[inline(always)]
        pub fn chnr(
            self,
        ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, Globres_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<20,0x1f,1,0,u8, Globres_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "External Multiplexer Setting   EMUX. Indicates the setting of the external multiplexer  corresponding to the        value in bitfield RESULT."]
        #[inline(always)]
        pub fn emux(
            self,
        ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, Globres_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<25,0x7,1,0,u8, Globres_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Converted Request Source   CRS. Indicates the request source that as requested the conversion to which        the result value in bitfield RESULT belongs."]
        #[inline(always)]
        pub fn crs(
            self,
        ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, Globres_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<28,0x3,1,0,u8, Globres_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Valid Flag   VF. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, Globres_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<31,1,0,Globres_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Globres {
        #[inline(always)]
        fn default() -> Globres {
            <crate::RegValueT<Globres_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Globresd_SPEC;
    impl crate::sealed::RegSpec for Globresd_SPEC {
        type DataType = u32;
    }
    #[doc = "Global Result Register  Debug\n resetvalue={Application Reset:0x0}"]
    pub type Globresd = crate::RegValueT<Globresd_SPEC>;

    impl Globresd {
        #[doc = "Result of most recent conversion   RESULT. The position of the result bits within this bitfield depends on the        configured operating mode. Refer to CROSSREFERENCE ."]
        #[inline(always)]
        pub fn result(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Globresd_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, Globresd_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Group Number   GNR. Indicates the group to which the channel number in bitfield CHNR refers."]
        #[inline(always)]
        pub fn gnr(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Globresd_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, Globresd_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Channel Number   CHNR. Indicates the channel number corresponding to the value in bitfield        RESULT."]
        #[inline(always)]
        pub fn chnr(
            self,
        ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, Globresd_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<20,0x1f,1,0,u8, Globresd_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "External Multiplexer Setting   EMUX. Indicates the setting of the external multiplexer  corresponding to the        value in bitfield RESULT."]
        #[inline(always)]
        pub fn emux(
            self,
        ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, Globresd_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<25,0x7,1,0,u8, Globresd_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Converted Request Source   CRS. Indicates the request source that as requested the conversion to which        the result value in bitfield RESULT belongs."]
        #[inline(always)]
        pub fn crs(
            self,
        ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, Globresd_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<28,0x3,1,0,u8, Globresd_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Valid Flag   VF. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, Globresd_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<31,1,0,Globresd_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for Globresd {
        #[inline(always)]
        fn default() -> Globresd {
            <crate::RegValueT<Globresd_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Globte_SPEC;
    impl crate::sealed::RegSpec for Globte_SPEC {
        type DataType = u32;
    }
    #[doc = "Global Test Enable Register\n resetvalue={Application Reset:0x0}"]
    pub type Globte = crate::RegValueT<Globte_SPEC>;

    impl Globte {
        #[doc = "Test Function Enable  Primary Groups. Each bit of this bitfield is associated with the corresponding group. 0  No test functions for group x sequences 1  Test functions can be activated by group x sequences"]
        #[inline(always)]
        pub fn tfep(
            self,
        ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Globte_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xf,1,0,u8, Globte_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Test Function Enable  Secondary Groups. Each bit of this bitfield is associated with the corresponding group. 0  No test functions for group x sequences 1  Test functions can be activated by group x sequences"]
        #[inline(always)]
        pub fn tfes(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Globte_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xf,1,0,u8, Globte_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Globte {
        #[inline(always)]
        fn default() -> Globte {
            <crate::RegValueT<Globte_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Globtf_SPEC;
    impl crate::sealed::RegSpec for Globtf_SPEC {
        type DataType = u32;
    }
    #[doc = "Global Test Functions Register\n resetvalue={Application Reset:0x0}"]
    pub type Globtf = crate::RegValueT<Globtf_SPEC>;

    impl Globtf {
        #[doc = "Conversion Diagnostics Channel   CDCH. Defines the channel number to be used for diagnostic conversions. Applies to MDPD  MDPU."]
        #[inline(always)]
        pub fn cdch(
            self,
        ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Globtf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xf,1,0,u8, Globtf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Conversion Diagnostics Group   CDGR. Defines the group number to be used for diagnostic conversions. Applies to all test functions of primary and secondary groups."]
        #[inline(always)]
        pub fn cdgr(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Globtf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0xf,1,0,u8, Globtf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Converter Diagnostics Enable   CDEN"]
        #[inline(always)]
        pub fn cden(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, Globtf_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,Globtf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Converter Diagnostics Pull Devices Select   CDSEL"]
        #[inline(always)]
        pub fn cdsel(
            self,
        ) -> crate::common::RegisterField<9, 0x3, 1, 0, u8, Globtf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<9,0x3,1,0,u8, Globtf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Write Control for Conversion Diagnostics   CDWC"]
        #[inline(always)]
        pub fn cdwc(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, Globtf_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<15,1,0,Globtf_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Pull Down Diagnostics Enable   PDD. Channels with pull down diagnostics device are marked in the          product specific appendix."]
        #[inline(always)]
        pub fn pdd(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, Globtf_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<16,1,0,Globtf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Multiplexer Diagnostics Pull Down Devices Enable. Connecting combinations of pull up and or pull down devices generate        various loads for testing. Channels with pull down diagnostics device are marked in the          product specific appendix."]
        #[inline(always)]
        pub fn mdpd(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, Globtf_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<17,1,0,Globtf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Multiplexer Diagnostics Pull Up Devices Enable. Connecting combinations of pull up and or pull down devices generate various loads for testing. Channels with pull up diagnostics device are marked in the product specific appendix."]
        #[inline(always)]
        pub fn mdpu(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, Globtf_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<18,1,0,Globtf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Write Control for Multiplexer Diagnostics   MDWC"]
        #[inline(always)]
        pub fn mdwc(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, Globtf_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<23,1,0,Globtf_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for Globtf {
        #[inline(always)]
        fn default() -> Globtf {
            <crate::RegValueT<Globtf_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "G"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct G(pub(super) *mut u8);
unsafe impl core::marker::Send for G {}
unsafe impl core::marker::Sync for G {}
impl G {
    #[doc = "Alias Register  Group 0\n resetvalue={Application Reset:0x100}"]
    #[inline(always)]
    pub const fn gxalias(&self) -> crate::common::Reg<g::GxAlias_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(176usize)) }
    }
    #[doc = "Analog Fct. Config. Register  Group 0\n resetvalue={Application Reset:0x300004}"]
    #[inline(always)]
    pub const fn gxancfg(&self) -> crate::common::Reg<g::GxAncfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }
    #[doc = "Arbitration Config. Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxarbcfg(&self) -> crate::common::Reg<g::GxArbcfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }
    #[doc = "Arbitration Priority Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxarbpr(&self) -> crate::common::Reg<g::GxArbpr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }
    #[doc = "Boundary Select Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxbound(&self) -> crate::common::Reg<g::GxBound_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(184usize)) }
    }
    #[doc = "Channel Event Flag Clear Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxcefclr(&self) -> crate::common::Reg<g::GxCefclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(400usize)) }
    }
    #[doc = "Channel Event Flag Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxceflag(&self) -> crate::common::Reg<g::GxCeflag_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(384usize)) }
    }
    #[doc = "Channel Event Node Pointer Reg. 0  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxcevnp0(&self) -> crate::common::Reg<g::GxCevnp0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(416usize)) }
    }
    #[doc = "Channel Event Node Pointer Reg. 1  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxcevnp1(&self) -> crate::common::Reg<g::GxCevnp1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(420usize)) }
    }
    #[doc = "Group 0  Channel 0 Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxchctry(&self) -> [crate::common::Reg<g::GxChctRy_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x3cusize)),
            ]
        }
    }
    #[doc = "Ext. Multiplexer Channel Select Reg.  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxemuxcs(&self) -> crate::common::Reg<g::GxEmuxcs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(500usize)) }
    }
    #[doc = "External Multiplexer Control Reg.  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxemuxctr(&self) -> crate::common::Reg<g::GxEmuxctr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(496usize)) }
    }
    #[doc = "Input Class Register 0  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxiclassi(&self) -> [crate::common::Reg<g::GxIclasSi_SPEC, crate::common::RW>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0xa0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0xa0usize + 0x4usize)),
            ]
        }
    }
    #[doc = "Group 0 Result Control Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxrcry(&self) -> [crate::common::Reg<g::GxRcRy_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x3cusize)),
            ]
        }
    }
    #[doc = "Result Event Flag Clear Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxrefclr(&self) -> crate::common::Reg<g::GxRefclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(404usize)) }
    }
    #[doc = "Result Event Flag Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxreflag(&self) -> crate::common::Reg<g::GxReflag_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(388usize)) }
    }
    #[doc = "Group 0 Result Reg. 0  Debug\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxresdy(&self) -> [crate::common::Reg<g::GxResDy_SPEC, crate::common::R>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x3cusize)),
            ]
        }
    }
    #[doc = "Group 0 Result Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxresy(&self) -> [crate::common::Reg<g::GxReSy_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x3cusize)),
            ]
        }
    }
    #[doc = "Result Event Node Pointer Reg. 0  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxrevnp0(&self) -> crate::common::Reg<g::GxRevnp0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(432usize)) }
    }
    #[doc = "Result Event Node Pointer Reg. 1  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxrevnp1(&self) -> crate::common::Reg<g::GxRevnp1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(436usize)) }
    }
    #[doc = "Source Event Flag Clear Reg.  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxsefclr(&self) -> crate::common::Reg<g::GxSefclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(408usize)) }
    }
    #[doc = "Source Event Flag Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxseflag(&self) -> crate::common::Reg<g::GxSeflag_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(392usize)) }
    }
    #[doc = "Source Event Node Pointer Reg.  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxsevnp(&self) -> crate::common::Reg<g::GxSevnp_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(448usize)) }
    }
    #[doc = "Service Request Software Activation Trigger  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxsract(&self) -> crate::common::Reg<g::GxSract_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(456usize)) }
    }
    #[doc = "Synchronization Control Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxsynctr(&self) -> crate::common::Reg<g::GxSynctr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(192usize)) }
    }
    #[doc = "Trigger Control Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxtrctr(&self) -> crate::common::Reg<g::GxTrctr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Valid Flag Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxvfr(&self) -> crate::common::Reg<g::GxVfr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(504usize)) }
    }
    #[doc = "Q"]
    #[inline(always)]
    pub fn q(self) -> [g::Q; 3] {
        unsafe {
            [
                g::Q(self.0.add(0x100usize + 0x0usize)),
                g::Q(self.0.add(0x100usize + 0x20usize)),
                g::Q(self.0.add(0x100usize + 0x40usize)),
            ]
        }
    }
}
pub mod g {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxAlias_SPEC;
    impl crate::sealed::RegSpec for GxAlias_SPEC {
        type DataType = u32;
    }
    #[doc = "Alias Register  Group 0\n resetvalue={Application Reset:0x100}"]
    pub type GxAlias = crate::RegValueT<GxAlias_SPEC>;

    impl GxAlias {
        #[doc = "Alias Value for CH0 Conversion Requests   ALIAS0. Indicates the channel that is converted instead of channel CH0. The        conversion is done with the settings defined for channel CH0."]
        #[inline(always)]
        pub fn alias0(
            self,
        ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, GxAlias_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1f,1,0,u8, GxAlias_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Alias Value for CH1 Conversion Requests   ALIAS1. Indicates the channel that is converted instead of channel CH1. The        conversion is done with the settings defined for channel CH1."]
        #[inline(always)]
        pub fn alias1(
            self,
        ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, GxAlias_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x1f,1,0,u8, GxAlias_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxAlias {
        #[inline(always)]
        fn default() -> GxAlias {
            <crate::RegValueT<GxAlias_SPEC> as RegisterValue<_>>::new(256)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxAncfg_SPEC;
    impl crate::sealed::RegSpec for GxAncfg_SPEC {
        type DataType = u32;
    }
    #[doc = "Analog Fct. Config. Register  Group 0\n resetvalue={Application Reset:0x300004}"]
    pub type GxAncfg = crate::RegValueT<GxAncfg_SPEC>;

    impl GxAncfg {
        #[doc = "Idle Precharge Enable   IPE"]
        #[inline(always)]
        pub fn ipe(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, GxAncfg_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,GxAncfg_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Input Buffer Enable   BE"]
        #[inline(always)]
        pub fn be(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, GxAncfg_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,GxAncfg_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Reference Precharge Enable   RPE. Enabled after reset."]
        #[inline(always)]
        pub fn rpe(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, GxAncfg_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,GxAncfg_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Reference Precharge Control   RPC"]
        #[inline(always)]
        pub fn rpc(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, GxAncfg_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,GxAncfg_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Calibration Sample Time Control   CALSTC"]
        #[inline(always)]
        pub fn calstc(
            self,
        ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, GxAncfg_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x3,1,0,u8, GxAncfg_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Disable Post Calibration   DPCAL"]
        #[inline(always)]
        pub fn dpcal(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, GxAncfg_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,GxAncfg_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Analog Clock Synchronization Delay   ACSD. Defines the delay of the analog clock in clocks after the sync signal. Do not exceed the current DIVA setting for the clock delay. Valid only          if the phase synchronizer is selected  USC  160    160  0"]
        #[inline(always)]
        pub fn acsd(
            self,
        ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, GxAncfg_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x7,1,0,u8, GxAncfg_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Sample Synchronization Enable   SSE. See section CROSSREFERENCE ."]
        #[inline(always)]
        pub fn sse(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, GxAncfg_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<19,1,0,GxAncfg_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Divider Factor for the Analog Internal Clock   DIVA. Defines the frequency of the analog converter clock f ADCI  base clock for conversion steps   derived from the peripheral clock  f ADCI   f ADC   CP."]
        #[inline(always)]
        pub fn diva(
            self,
        ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, GxAncfg_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0x1f,1,0,u8, GxAncfg_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Double Clock for the MSB Conversion   DCMSB. Selects an additional clock cycle for the conversion step of the MSB."]
        #[inline(always)]
        pub fn dcmsb(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, GxAncfg_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<25,1,0,GxAncfg_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxAncfg {
        #[inline(always)]
        fn default() -> GxAncfg {
            <crate::RegValueT<GxAncfg_SPEC> as RegisterValue<_>>::new(3145732)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxArbcfg_SPEC;
    impl crate::sealed::RegSpec for GxArbcfg_SPEC {
        type DataType = u32;
    }
    #[doc = "Arbitration Config. Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxArbcfg = crate::RegValueT<GxArbcfg_SPEC>;

    impl GxArbcfg {
        #[doc = "Analog Converter Control   ANONC. Defines the value of bitfield ANONS in a stand alone converter or a        converter in master mode. Coding see ANONS or CROSSREFERENCE ."]
        #[inline(always)]
        pub fn anonc(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, GxArbcfg_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3,1,0,u8, GxArbcfg_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Analog Converter Control Status   ANONS. Defined by bitfield ANONC in a stand alone kernel or a kernel in master        mode. In slave mode  this bitfield is defined by bitfield ANONC of the        respective master kernel. See also CROSSREFERENCE ."]
        #[inline(always)]
        pub fn anons(
            self,
        ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, GxArbcfg_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0x3,1,0,u8, GxArbcfg_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Currently Converted Request Source   CSRC. Indicates the arbitration slot number of the current  BUSY  160    160 1  or of        the last  BUSY  160    160 0  conversion. This bitfield is updated when a        conversion is started."]
        #[inline(always)]
        pub fn csrc(
            self,
        ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, GxArbcfg_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<18,0x3,1,0,u8, GxArbcfg_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Channel Number   CHNR. Indicates the current or last converted analog input channel. This        bitfield is updated when a conversion is started."]
        #[inline(always)]
        pub fn chnr(
            self,
        ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, GxArbcfg_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<20,0x1f,1,0,u8, GxArbcfg_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Synchronous Conversion Running   SYNRUN. Indicates that a synchronized    parallel  conversion is currently        running."]
        #[inline(always)]
        pub fn synrun(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, GxArbcfg_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<25,1,0,GxArbcfg_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Start Up Calibration Active Indication   CAL. Indicates the start up calibration phase of the corresponding analog        converter. Start conversions only after the start up calibration phase is          complete  because a start up calibration will abort a running          conversion."]
        #[inline(always)]
        pub fn cal(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, GxArbcfg_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<28,1,0,GxArbcfg_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Converter Busy Flag   BUSY"]
        #[inline(always)]
        pub fn busy(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, GxArbcfg_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<30,1,0,GxArbcfg_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Sample Phase Flag   SAMPLE"]
        #[inline(always)]
        pub fn sample(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, GxArbcfg_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<31,1,0,GxArbcfg_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for GxArbcfg {
        #[inline(always)]
        fn default() -> GxArbcfg {
            <crate::RegValueT<GxArbcfg_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxArbpr_SPEC;
    impl crate::sealed::RegSpec for GxArbpr_SPEC {
        type DataType = u32;
    }
    #[doc = "Arbitration Priority Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxArbpr = crate::RegValueT<GxArbpr_SPEC>;

    impl GxArbpr {
        #[doc = "Priority of Request Source i. Arbitration priority of request source i  at input i"]
        #[inline(always)]
        pub fn prio0(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, GxArbpr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3,1,0,u8, GxArbpr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Conversion Start Mode of Request Source 2"]
        #[inline(always)]
        pub fn csm0(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, GxArbpr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,GxArbpr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Priority of Request Source i. Arbitration priority of request source i  at input i"]
        #[inline(always)]
        pub fn prio1(
            self,
        ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, GxArbpr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x3,1,0,u8, GxArbpr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Conversion Start Mode of Request Source 2"]
        #[inline(always)]
        pub fn csm1(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, GxArbpr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,GxArbpr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Priority of Request Source i. Arbitration priority of request source i  at input i"]
        #[inline(always)]
        pub fn prio2(
            self,
        ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, GxArbpr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x3,1,0,u8, GxArbpr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Conversion Start Mode of Request Source 2"]
        #[inline(always)]
        pub fn csm2(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, GxArbpr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<11,1,0,GxArbpr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Arbitration Source Input 2 Enable. Enables the associated arbitration source input of the arbiter. The        request source bits are not modified by write actions to ASENi."]
        #[inline(always)]
        pub fn asen0(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, GxArbpr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<24,1,0,GxArbpr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Arbitration Source Input 2 Enable. Enables the associated arbitration source input of the arbiter. The        request source bits are not modified by write actions to ASENi."]
        #[inline(always)]
        pub fn asen1(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, GxArbpr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<25,1,0,GxArbpr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Arbitration Source Input 2 Enable. Enables the associated arbitration source input of the arbiter. The        request source bits are not modified by write actions to ASENi."]
        #[inline(always)]
        pub fn asen2(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, GxArbpr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<26,1,0,GxArbpr_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxArbpr {
        #[inline(always)]
        fn default() -> GxArbpr {
            <crate::RegValueT<GxArbpr_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxBound_SPEC;
    impl crate::sealed::RegSpec for GxBound_SPEC {
        type DataType = u32;
    }
    #[doc = "Boundary Select Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxBound = crate::RegValueT<GxBound_SPEC>;

    impl GxBound {
        #[doc = "Boundary Value 0 for Limit Checking   BOUNDARY0. This value is compared against the conversion result."]
        #[inline(always)]
        pub fn boundary0(
            self,
        ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, GxBound_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xfff,1,0,u16, GxBound_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Boundary Value 1 for Limit Checking   BOUNDARY1. This value is compared against the conversion result."]
        #[inline(always)]
        pub fn boundary1(
            self,
        ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, GxBound_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xfff,1,0,u16, GxBound_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxBound {
        #[inline(always)]
        fn default() -> GxBound {
            <crate::RegValueT<GxBound_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxCefclr_SPEC;
    impl crate::sealed::RegSpec for GxCefclr_SPEC {
        type DataType = u32;
    }
    #[doc = "Channel Event Flag Clear Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxCefclr = crate::RegValueT<GxCefclr_SPEC>;

    impl GxCefclr {
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, GxCefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<0,1,0,GxCefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, GxCefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<1,1,0,GxCefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, GxCefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<2,1,0,GxCefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev3(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, GxCefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<3,1,0,GxCefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev4(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, GxCefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<4,1,0,GxCefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev5(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, GxCefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<5,1,0,GxCefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev6(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, GxCefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<6,1,0,GxCefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev7(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, GxCefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<7,1,0,GxCefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev8(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, GxCefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<8,1,0,GxCefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev9(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, GxCefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<9,1,0,GxCefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev10(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, GxCefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<10,1,0,GxCefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev11(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, GxCefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<11,1,0,GxCefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev12(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, GxCefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<12,1,0,GxCefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev13(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, GxCefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<13,1,0,GxCefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev14(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, GxCefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<14,1,0,GxCefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev15(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, GxCefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<15,1,0,GxCefclr_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for GxCefclr {
        #[inline(always)]
        fn default() -> GxCefclr {
            <crate::RegValueT<GxCefclr_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxCeflag_SPEC;
    impl crate::sealed::RegSpec for GxCeflag_SPEC {
        type DataType = u32;
    }
    #[doc = "Channel Event Flag Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxCeflag = crate::RegValueT<GxCeflag_SPEC>;

    impl GxCeflag {
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, GxCeflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,GxCeflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, GxCeflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,GxCeflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, GxCeflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,GxCeflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev3(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, GxCeflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,GxCeflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev4(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, GxCeflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,GxCeflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev5(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, GxCeflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,GxCeflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev6(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, GxCeflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,GxCeflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev7(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, GxCeflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,GxCeflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev8(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, GxCeflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,GxCeflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev9(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, GxCeflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9,1,0,GxCeflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev10(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, GxCeflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,GxCeflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev11(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, GxCeflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<11,1,0,GxCeflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev12(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, GxCeflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12,1,0,GxCeflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev13(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, GxCeflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13,1,0,GxCeflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev14(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, GxCeflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,GxCeflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev15(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, GxCeflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,GxCeflag_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxCeflag {
        #[inline(always)]
        fn default() -> GxCeflag {
            <crate::RegValueT<GxCeflag_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxCevnp0_SPEC;
    impl crate::sealed::RegSpec for GxCevnp0_SPEC {
        type DataType = u32;
    }
    #[doc = "Channel Event Node Pointer Reg. 0  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxCevnp0 = crate::RegValueT<GxCevnp0_SPEC>;

    impl GxCevnp0 {
        #[doc = "Service Request Node Pointer Channel Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn cev0np(
            self,
        ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, GxCevnp0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xf,1,0,u8, GxCevnp0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Channel Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn cev1np(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, GxCevnp0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0xf,1,0,u8, GxCevnp0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Channel Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn cev2np(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, GxCevnp0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xf,1,0,u8, GxCevnp0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Channel Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn cev3np(
            self,
        ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, GxCevnp0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0xf,1,0,u8, GxCevnp0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Channel Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn cev4np(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, GxCevnp0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, GxCevnp0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Channel Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn cev5np(
            self,
        ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, GxCevnp0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0xf,1,0,u8, GxCevnp0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Channel Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn cev6np(
            self,
        ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, GxCevnp0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0xf,1,0,u8, GxCevnp0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Channel Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn cev7np(
            self,
        ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, GxCevnp0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0xf,1,0,u8, GxCevnp0_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxCevnp0 {
        #[inline(always)]
        fn default() -> GxCevnp0 {
            <crate::RegValueT<GxCevnp0_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxCevnp1_SPEC;
    impl crate::sealed::RegSpec for GxCevnp1_SPEC {
        type DataType = u32;
    }
    #[doc = "Channel Event Node Pointer Reg. 1  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxCevnp1 = crate::RegValueT<GxCevnp1_SPEC>;

    impl GxCevnp1 {
        #[doc = "Service Request Node Pointer Channel Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn cev8np(
            self,
        ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, GxCevnp1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xf,1,0,u8, GxCevnp1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Channel Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn cev9np(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, GxCevnp1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0xf,1,0,u8, GxCevnp1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Channel Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn cev10np(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, GxCevnp1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xf,1,0,u8, GxCevnp1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Channel Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn cev11np(
            self,
        ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, GxCevnp1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0xf,1,0,u8, GxCevnp1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Channel Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn cev12np(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, GxCevnp1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, GxCevnp1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Channel Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn cev13np(
            self,
        ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, GxCevnp1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0xf,1,0,u8, GxCevnp1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Channel Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn cev14np(
            self,
        ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, GxCevnp1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0xf,1,0,u8, GxCevnp1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Channel Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn cev15np(
            self,
        ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, GxCevnp1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0xf,1,0,u8, GxCevnp1_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxCevnp1 {
        #[inline(always)]
        fn default() -> GxCevnp1 {
            <crate::RegValueT<GxCevnp1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxChctRy_SPEC;
    impl crate::sealed::RegSpec for GxChctRy_SPEC {
        type DataType = u32;
    }
    #[doc = "Group 0  Channel 0 Control Register\n resetvalue={Application Reset:0x0}"]
    pub type GxChctRy = crate::RegValueT<GxChctRy_SPEC>;

    impl GxChctRy {
        #[doc = "Input Class Select   ICLSEL"]
        #[inline(always)]
        pub fn iclsel(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, GxChctRy_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3,1,0,u8, GxChctRy_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Lower Boundary Select   BNDSELL"]
        #[inline(always)]
        pub fn bndsell(
            self,
        ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, GxChctRy_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x3,1,0,u8, GxChctRy_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Upper Boundary Select   BNDSELU"]
        #[inline(always)]
        pub fn bndselu(
            self,
        ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, GxChctRy_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<6,0x3,1,0,u8, GxChctRy_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Channel Event Mode   CHEVMODE. Generate a channel event  with limit checking The        boundary band is defined as the area where the result is less than or        equal to the selected upper boundary and greater than or equal to the        selected lower boundary  see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn chevmode(
            self,
        ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, GxChctRy_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x3,1,0,u8, GxChctRy_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Synchronization Request   SYNC"]
        #[inline(always)]
        pub fn sync(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, GxChctRy_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,GxChctRy_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Reference Input Selection   REFSEL. Defines the reference voltage input to be used for conversions on this        channel. Some channels cannot select an        alternate reference."]
        #[inline(always)]
        pub fn refsel(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, GxChctRy_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<11,1,0,GxChctRy_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "BoundaryExtension   BNDSELX. While BNDSELX   8800  0000   bitfields        BNDSELU and BNDSELL are concatenated and select the corresponding result        register as lower boundary."]
        #[inline(always)]
        pub fn bndselx(
            self,
        ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, GxChctRy_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0xf,1,0,u8, GxChctRy_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Result Register   RESREG"]
        #[inline(always)]
        pub fn resreg(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, GxChctRy_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, GxChctRy_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Result Target   RESTGT"]
        #[inline(always)]
        pub fn restgt(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, GxChctRy_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<20,1,0,GxChctRy_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Result Position   RESPOS"]
        #[inline(always)]
        pub fn respos(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, GxChctRy_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<21,1,0,GxChctRy_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Broken Wire Detection Channel   BWDCH. Reserved        combinations shall select VAGND."]
        #[inline(always)]
        pub fn bwdch(
            self,
        ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, GxChctRy_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0x3,1,0,u8, GxChctRy_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Broken Wire Detection Enable   BWDEN"]
        #[inline(always)]
        pub fn bwden(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, GxChctRy_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<30,1,0,GxChctRy_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxChctRy {
        #[inline(always)]
        fn default() -> GxChctRy {
            <crate::RegValueT<GxChctRy_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxEmuxcs_SPEC;
    impl crate::sealed::RegSpec for GxEmuxcs_SPEC {
        type DataType = u32;
    }
    #[doc = "Ext. Multiplexer Channel Select Reg.  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxEmuxcs = crate::RegValueT<GxEmuxcs_SPEC>;

    impl GxEmuxcs {
        #[doc = "External Multiplexer Channel Select   EMUXCH. Defines the channel s  to which the external multiplexer control is        applied. EMXCSS  160    160 0  Channel number the lower 5 bits select an arbitrary channel  valid numbers are limited        by the number of available channels  unused bits shall be 0  EMXCSS  160    160 1  Channel enable each bit enables the associated channel  multiple channels can be        selected enabled"]
        #[inline(always)]
        pub fn emuxch(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, GxEmuxcs_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, GxEmuxcs_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxEmuxcs {
        #[inline(always)]
        fn default() -> GxEmuxcs {
            <crate::RegValueT<GxEmuxcs_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxEmuxctr_SPEC;
    impl crate::sealed::RegSpec for GxEmuxctr_SPEC {
        type DataType = u32;
    }
    #[doc = "External Multiplexer Control Reg.  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxEmuxctr = crate::RegValueT<GxEmuxctr_SPEC>;

    impl GxEmuxctr {
        #[doc = "External Multiplexer Start Selection   EMUXSET. Defines the initial selection for the external multiplexer."]
        #[inline(always)]
        pub fn emuxset(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, GxEmuxctr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,u8, GxEmuxctr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "External Multiplexer Mode   EMUXMODE. Not listed combinations are reserved. For        single step mode  sequence mode and block mode  Select the start value        with EMUXMODE  000 before selecting the        respective mode."]
        #[inline(always)]
        pub fn emuxmode(
            self,
        ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, GxEmuxctr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x7,1,0,u8, GxEmuxctr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "External Multiplexer Coding Scheme   EMXCOD"]
        #[inline(always)]
        pub fn emxcod(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, GxEmuxctr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12,1,0,GxEmuxctr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "External Multiplexer Sample Time Control   EMXST"]
        #[inline(always)]
        pub fn emxst(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, GxEmuxctr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13,1,0,GxEmuxctr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "External Multiplexer Channel Selection Style   EMXCSS"]
        #[inline(always)]
        pub fn emxcss(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, GxEmuxctr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,GxEmuxctr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Write Control for EMUX Configuration   EMXWC"]
        #[inline(always)]
        pub fn emxwc(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, GxEmuxctr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<15,1,0,GxEmuxctr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "External Multiplexer Actual Selection   EMUXACT. Defines the current value for the external multiplexer selection. This        bitfield is loaded from bitfield EMUXSET and modified according to the        operating mode selected by bitfield EMUXMODE."]
        #[inline(always)]
        pub fn emuxact(
            self,
        ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, GxEmuxctr_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0x7,1,0,u8, GxEmuxctr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "External Multiplexer Channel Selection for Block Mode   EMUXCCB. Defines the channel that switches EMUXACT when converted. In block mode         all EMUX channels use the same control value."]
        #[inline(always)]
        pub fn emuxccb(
            self,
        ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, GxEmuxctr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0x1f,1,0,u8, GxEmuxctr_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxEmuxctr {
        #[inline(always)]
        fn default() -> GxEmuxctr {
            <crate::RegValueT<GxEmuxctr_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxIclasSi_SPEC;
    impl crate::sealed::RegSpec for GxIclasSi_SPEC {
        type DataType = u32;
    }
    #[doc = "Input Class Register 0  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxIclasSi = crate::RegValueT<GxIclasSi_SPEC>;

    impl GxIclasSi {
        #[doc = "Sample Time Control for Standard Conversions   STCS. Number of additional clock cycles to be added to the minimum sample        phase of 2 sample clock cycles  Coding and resulting sample time see CROSSREFERENCE . For conversions of external channels  the value from bitfield STCE can        be used."]
        #[inline(always)]
        pub fn stcs(
            self,
        ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, GxIclasSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1f,1,0,u8, GxIclasSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Analog Input Precharge Control for Standard Conversions   AIPS. Buffer must be enabled by BE  160    160 1  see GxANCFG  ."]
        #[inline(always)]
        pub fn aips(
            self,
        ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, GxIclasSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<6,0x3,1,0,u8, GxIclasSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Conversion Mode for Standard Conversions   CMS"]
        #[inline(always)]
        pub fn cms(
            self,
        ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, GxIclasSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x3,1,0,u8, GxIclasSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Spread Early Sample Point for Standard Conversions   SESPS"]
        #[inline(always)]
        pub fn sesps(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, GxIclasSi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,GxIclasSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Sample Time Control for EMUX Conversions   STCE. Number of additional clock cycles to be added to the minimum sample        phase of 2 sample clock cycles  Coding and resulting sample time see CROSSREFERENCE . For conversions of standard channels  the value from bitfield STCS is        used."]
        #[inline(always)]
        pub fn stce(
            self,
        ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, GxIclasSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1f,1,0,u8, GxIclasSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Analog Input Precharge Control for EMUX Conversions   AIPE. Buffer must be enabled by BE  160    160 1  see GxANCFG  ."]
        #[inline(always)]
        pub fn aipe(
            self,
        ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, GxIclasSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<22,0x3,1,0,u8, GxIclasSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Conversion Mode for EMUX Conversions   CME"]
        #[inline(always)]
        pub fn cme(
            self,
        ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, GxIclasSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0x3,1,0,u8, GxIclasSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Spread Early Sample Point for EMUX Conversions   SESPE"]
        #[inline(always)]
        pub fn sespe(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, GxIclasSi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<26,1,0,GxIclasSi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxIclasSi {
        #[inline(always)]
        fn default() -> GxIclasSi {
            <crate::RegValueT<GxIclasSi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxRcRy_SPEC;
    impl crate::sealed::RegSpec for GxRcRy_SPEC {
        type DataType = u32;
    }
    #[doc = "Group 0 Result Control Register 0\n resetvalue={Application Reset:0x0}"]
    pub type GxRcRy = crate::RegValueT<GxRcRy_SPEC>;

    impl GxRcRy {
        #[doc = "Data Reduction Control   DRCTR. Defines how result values are stored accumulated in this register for        the final result. The data reduction counter DRC can be loaded from this        bitfield. The function of bitfield DRCTR is determined by bitfield DMM."]
        #[inline(always)]
        pub fn drctr(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, GxRcRy_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, GxRcRy_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Data Modification Mode   DMM. See CROSSREFERENCE"]
        #[inline(always)]
        pub fn dmm(
            self,
        ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, GxRcRy_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0x3,1,0,u8, GxRcRy_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Wait for Read Mode Enable   WFR"]
        #[inline(always)]
        pub fn wfr(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, GxRcRy_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<24,1,0,GxRcRy_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "FIFO Mode Enable   FEN"]
        #[inline(always)]
        pub fn fen(
            self,
        ) -> crate::common::RegisterField<25, 0x3, 1, 0, u8, GxRcRy_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<25,0x3,1,0,u8, GxRcRy_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Generation Enable   SRGEN"]
        #[inline(always)]
        pub fn srgen(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, GxRcRy_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<31,1,0,GxRcRy_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxRcRy {
        #[inline(always)]
        fn default() -> GxRcRy {
            <crate::RegValueT<GxRcRy_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxRefclr_SPEC;
    impl crate::sealed::RegSpec for GxRefclr_SPEC {
        type DataType = u32;
    }
    #[doc = "Result Event Flag Clear Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxRefclr = crate::RegValueT<GxRefclr_SPEC>;

    impl GxRefclr {
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, GxRefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<0,1,0,GxRefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, GxRefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<1,1,0,GxRefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, GxRefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<2,1,0,GxRefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev3(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, GxRefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<3,1,0,GxRefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev4(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, GxRefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<4,1,0,GxRefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev5(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, GxRefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<5,1,0,GxRefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev6(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, GxRefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<6,1,0,GxRefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev7(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, GxRefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<7,1,0,GxRefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev8(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, GxRefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<8,1,0,GxRefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev9(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, GxRefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<9,1,0,GxRefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev10(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, GxRefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<10,1,0,GxRefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev11(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, GxRefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<11,1,0,GxRefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev12(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, GxRefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<12,1,0,GxRefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev13(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, GxRefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<13,1,0,GxRefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev14(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, GxRefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<14,1,0,GxRefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev15(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, GxRefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<15,1,0,GxRefclr_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for GxRefclr {
        #[inline(always)]
        fn default() -> GxRefclr {
            <crate::RegValueT<GxRefclr_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxReflag_SPEC;
    impl crate::sealed::RegSpec for GxReflag_SPEC {
        type DataType = u32;
    }
    #[doc = "Result Event Flag Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxReflag = crate::RegValueT<GxReflag_SPEC>;

    impl GxReflag {
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, GxReflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,GxReflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, GxReflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,GxReflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, GxReflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,GxReflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev3(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, GxReflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,GxReflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev4(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, GxReflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,GxReflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev5(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, GxReflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,GxReflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev6(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, GxReflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,GxReflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev7(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, GxReflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,GxReflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev8(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, GxReflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,GxReflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev9(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, GxReflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9,1,0,GxReflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev10(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, GxReflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,GxReflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev11(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, GxReflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<11,1,0,GxReflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev12(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, GxReflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12,1,0,GxReflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev13(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, GxReflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13,1,0,GxReflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev14(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, GxReflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,GxReflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev15(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, GxReflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,GxReflag_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxReflag {
        #[inline(always)]
        fn default() -> GxReflag {
            <crate::RegValueT<GxReflag_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxResDy_SPEC;
    impl crate::sealed::RegSpec for GxResDy_SPEC {
        type DataType = u32;
    }
    #[doc = "Group 0 Result Reg. 0  Debug\n resetvalue={Application Reset:0x0}"]
    pub type GxResDy = crate::RegValueT<GxResDy_SPEC>;

    impl GxResDy {
        #[doc = "Result of Most Recent Conversion   RESULT. The position of the result bits within this bitfield depends on the        configured operating mode. Refer to CROSSREFERENCE ."]
        #[inline(always)]
        pub fn result(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, GxResDy_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, GxResDy_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Data Reduction Counter   DRC. Indicates the number of values still to be accumulated for the final        result. The final result is available and valid flag VF is set when        bitfield DRC becomes zero  by decrementing or by reload . See CROSSREFERENCE"]
        #[inline(always)]
        pub fn drc(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, GxResDy_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, GxResDy_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Channel Number   CHNR. Indicates the channel number corresponding to the value in bitfield        RESULT."]
        #[inline(always)]
        pub fn chnr(
            self,
        ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, GxResDy_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<20,0x1f,1,0,u8, GxResDy_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "External Multiplexer Setting   EMUX. Indicates the setting of the external multiplexer  corresponding to the        value in bitfield RESULT. Available in GxRESD0 only. Use GxRESD0 if EMUX information is required."]
        #[inline(always)]
        pub fn emux(
            self,
        ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, GxResDy_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<25,0x7,1,0,u8, GxResDy_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Converted Request Source   CRS. Indicates the request source that as requested the conversion to which        the result value in bitfield RESULT belongs."]
        #[inline(always)]
        pub fn crs(
            self,
        ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, GxResDy_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<28,0x3,1,0,u8, GxResDy_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Valid Flag   VF. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, GxResDy_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<31,1,0,GxResDy_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for GxResDy {
        #[inline(always)]
        fn default() -> GxResDy {
            <crate::RegValueT<GxResDy_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxReSy_SPEC;
    impl crate::sealed::RegSpec for GxReSy_SPEC {
        type DataType = u32;
    }
    #[doc = "Group 0 Result Register 0\n resetvalue={Application Reset:0x0}"]
    pub type GxReSy = crate::RegValueT<GxReSy_SPEC>;

    impl GxReSy {
        #[doc = "Result of Most Recent Conversion   RESULT. The position of the result bits within this bitfield depends on the        configured operating mode. Refer to CROSSREFERENCE . Bitfield RESULT is writeable by the application to set the initial value        for min max detection."]
        #[inline(always)]
        pub fn result(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, GxReSy_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, GxReSy_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Data Reduction Counter   DRC. Indicates the number of values still to be accumulated for the final        result. The final result is available and valid flag VF is set when        bitfield DRC becomes zero  by decrementing or by reload . See CROSSREFERENCE"]
        #[inline(always)]
        pub fn drc(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, GxReSy_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, GxReSy_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Channel Number   CHNR. Indicates the channel number corresponding to the value in bitfield        RESULT."]
        #[inline(always)]
        pub fn chnr(
            self,
        ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, GxReSy_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<20,0x1f,1,0,u8, GxReSy_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "External Multiplexer Setting   EMUX. Indicates the setting of the external multiplexer  corresponding to the        value in bitfield RESULT. Available in GxRES0 only. Use GxRES0 if EMUX information is required."]
        #[inline(always)]
        pub fn emux(
            self,
        ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, GxReSy_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<25,0x7,1,0,u8, GxReSy_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Converted Request Source   CRS. Indicates the request source that as requested the conversion to which        the result value in bitfield RESULT belongs."]
        #[inline(always)]
        pub fn crs(
            self,
        ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, GxReSy_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<28,0x3,1,0,u8, GxReSy_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Valid Flag   VF. Indicates a new result in bitfield RESULT. Bit VF is automatically cleared upon reading register GxRESy."]
        #[inline(always)]
        pub fn vf(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, GxReSy_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<31,1,0,GxReSy_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for GxReSy {
        #[inline(always)]
        fn default() -> GxReSy {
            <crate::RegValueT<GxReSy_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxRevnp0_SPEC;
    impl crate::sealed::RegSpec for GxRevnp0_SPEC {
        type DataType = u32;
    }
    #[doc = "Result Event Node Pointer Reg. 0  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxRevnp0 = crate::RegValueT<GxRevnp0_SPEC>;

    impl GxRevnp0 {
        #[doc = "Service Request Node Pointer Result Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn rev0np(
            self,
        ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, GxRevnp0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xf,1,0,u8, GxRevnp0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Result Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn rev1np(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, GxRevnp0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0xf,1,0,u8, GxRevnp0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Result Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn rev2np(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, GxRevnp0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xf,1,0,u8, GxRevnp0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Result Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn rev3np(
            self,
        ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, GxRevnp0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0xf,1,0,u8, GxRevnp0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Result Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn rev4np(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, GxRevnp0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, GxRevnp0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Result Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn rev5np(
            self,
        ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, GxRevnp0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0xf,1,0,u8, GxRevnp0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Result Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn rev6np(
            self,
        ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, GxRevnp0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0xf,1,0,u8, GxRevnp0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Result Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn rev7np(
            self,
        ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, GxRevnp0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0xf,1,0,u8, GxRevnp0_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxRevnp0 {
        #[inline(always)]
        fn default() -> GxRevnp0 {
            <crate::RegValueT<GxRevnp0_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxRevnp1_SPEC;
    impl crate::sealed::RegSpec for GxRevnp1_SPEC {
        type DataType = u32;
    }
    #[doc = "Result Event Node Pointer Reg. 1  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxRevnp1 = crate::RegValueT<GxRevnp1_SPEC>;

    impl GxRevnp1 {
        #[doc = "Service Request Node Pointer Result Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn rev8np(
            self,
        ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, GxRevnp1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xf,1,0,u8, GxRevnp1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Result Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn rev9np(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, GxRevnp1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0xf,1,0,u8, GxRevnp1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Result Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn rev10np(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, GxRevnp1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xf,1,0,u8, GxRevnp1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Result Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn rev11np(
            self,
        ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, GxRevnp1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0xf,1,0,u8, GxRevnp1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Result Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn rev12np(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, GxRevnp1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, GxRevnp1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Result Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn rev13np(
            self,
        ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, GxRevnp1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0xf,1,0,u8, GxRevnp1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Result Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn rev14np(
            self,
        ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, GxRevnp1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0xf,1,0,u8, GxRevnp1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node Pointer Result Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn rev15np(
            self,
        ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, GxRevnp1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0xf,1,0,u8, GxRevnp1_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxRevnp1 {
        #[inline(always)]
        fn default() -> GxRevnp1 {
            <crate::RegValueT<GxRevnp1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxSefclr_SPEC;
    impl crate::sealed::RegSpec for GxSefclr_SPEC {
        type DataType = u32;
    }
    #[doc = "Source Event Flag Clear Reg.  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxSefclr = crate::RegValueT<GxSefclr_SPEC>;

    impl GxSefclr {
        #[doc = "Clear Source Event i"]
        #[inline(always)]
        pub fn sev0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, GxSefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<0,1,0,GxSefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Source Event i"]
        #[inline(always)]
        pub fn sev1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, GxSefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<1,1,0,GxSefclr_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Source Event i"]
        #[inline(always)]
        pub fn sev2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, GxSefclr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<2,1,0,GxSefclr_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for GxSefclr {
        #[inline(always)]
        fn default() -> GxSefclr {
            <crate::RegValueT<GxSefclr_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxSeflag_SPEC;
    impl crate::sealed::RegSpec for GxSeflag_SPEC {
        type DataType = u32;
    }
    #[doc = "Source Event Flag Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxSeflag = crate::RegValueT<GxSeflag_SPEC>;

    impl GxSeflag {
        #[doc = "Source Event i"]
        #[inline(always)]
        pub fn sev0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, GxSeflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,GxSeflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Source Event i"]
        #[inline(always)]
        pub fn sev1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, GxSeflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,GxSeflag_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Source Event i"]
        #[inline(always)]
        pub fn sev2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, GxSeflag_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,GxSeflag_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxSeflag {
        #[inline(always)]
        fn default() -> GxSeflag {
            <crate::RegValueT<GxSeflag_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxSevnp_SPEC;
    impl crate::sealed::RegSpec for GxSevnp_SPEC {
        type DataType = u32;
    }
    #[doc = "Source Event Node Pointer Reg.  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxSevnp = crate::RegValueT<GxSevnp_SPEC>;

    impl GxSevnp {
        #[doc = "Service Request Node Pointer Source Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn sev0np(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xf,
            1,
            0,
            gxsevnp::Sev0Np,
            GxSevnp_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xf,
                1,
                0,
                gxsevnp::Sev0Np,
                GxSevnp_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Service Request Node Pointer Source Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn sev1np(
            self,
        ) -> crate::common::RegisterField<
            4,
            0xf,
            1,
            0,
            gxsevnp::Sev1Np,
            GxSevnp_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0xf,
                1,
                0,
                gxsevnp::Sev1Np,
                GxSevnp_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Service Request Node Pointer Source Event i. Routes the corresponding event trigger to one of the service request        lines  nodes . For shared service request lines see common groups in the          product specific appendix. Not listed combinations are reserved."]
        #[inline(always)]
        pub fn sev2np(
            self,
        ) -> crate::common::RegisterField<
            8,
            0xf,
            1,
            0,
            gxsevnp::Sev2Np,
            GxSevnp_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0xf,
                1,
                0,
                gxsevnp::Sev2Np,
                GxSevnp_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for GxSevnp {
        #[inline(always)]
        fn default() -> GxSevnp {
            <crate::RegValueT<GxSevnp_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod gxsevnp {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Sev0Np_SPEC;
        pub type Sev0Np = crate::EnumBitfieldStruct<u8, Sev0Np_SPEC>;
        impl Sev0Np {
            #[doc = "No service request line selected"]
            pub const CONST_F_15: Self = Self::new(15);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Sev1Np_SPEC;
        pub type Sev1Np = crate::EnumBitfieldStruct<u8, Sev1Np_SPEC>;
        impl Sev1Np {
            #[doc = "No service request line selected"]
            pub const CONST_F_15: Self = Self::new(15);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Sev2Np_SPEC;
        pub type Sev2Np = crate::EnumBitfieldStruct<u8, Sev2Np_SPEC>;
        impl Sev2Np {
            #[doc = "No service request line selected"]
            pub const CONST_F_15: Self = Self::new(15);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxSract_SPEC;
    impl crate::sealed::RegSpec for GxSract_SPEC {
        type DataType = u32;
    }
    #[doc = "Service Request Software Activation Trigger  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxSract = crate::RegValueT<GxSract_SPEC>;

    impl GxSract {
        #[doc = "Activate Group Service Request Node 3   AGSR3"]
        #[inline(always)]
        pub fn agsr0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, GxSract_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<0,1,0,GxSract_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Activate Group Service Request Node 3   AGSR3"]
        #[inline(always)]
        pub fn agsr1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, GxSract_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<1,1,0,GxSract_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Activate Group Service Request Node 3   AGSR3"]
        #[inline(always)]
        pub fn agsr2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, GxSract_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<2,1,0,GxSract_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Activate Group Service Request Node 3   AGSR3"]
        #[inline(always)]
        pub fn agsr3(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, GxSract_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<3,1,0,GxSract_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Activate Shared Service Request Node 3   ASSR3"]
        #[inline(always)]
        pub fn assr0(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, GxSract_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<8,1,0,GxSract_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Activate Shared Service Request Node 3   ASSR3"]
        #[inline(always)]
        pub fn assr1(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, GxSract_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<9,1,0,GxSract_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Activate Shared Service Request Node 3   ASSR3"]
        #[inline(always)]
        pub fn assr2(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, GxSract_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<10,1,0,GxSract_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Activate Shared Service Request Node 3   ASSR3"]
        #[inline(always)]
        pub fn assr3(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, GxSract_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<11,1,0,GxSract_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for GxSract {
        #[inline(always)]
        fn default() -> GxSract {
            <crate::RegValueT<GxSract_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxSynctr_SPEC;
    impl crate::sealed::RegSpec for GxSynctr_SPEC {
        type DataType = u32;
    }
    #[doc = "Synchronization Control Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxSynctr = crate::RegValueT<GxSynctr_SPEC>;

    impl GxSynctr {
        #[doc = "Start Selection   STSEL. Controls the synchronization mechanism of the ADC kernel. Control inputs CIx see CROSSREFERENCE            connected kernels see product specific appendix."]
        #[inline(always)]
        pub fn stsel(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, GxSynctr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3,1,0,u8, GxSynctr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Evaluate Ready Input Ri. Enables the ready input signal for a kernel of a synchronization group."]
        #[inline(always)]
        pub fn evalr1(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, GxSynctr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,GxSynctr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Evaluate Ready Input Ri. Enables the ready input signal for a kernel of a synchronization group."]
        #[inline(always)]
        pub fn evalr2(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, GxSynctr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,GxSynctr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Evaluate Ready Input Ri. Enables the ready input signal for a kernel of a synchronization group."]
        #[inline(always)]
        pub fn evalr3(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, GxSynctr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,GxSynctr_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxSynctr {
        #[inline(always)]
        fn default() -> GxSynctr {
            <crate::RegValueT<GxSynctr_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxTrctr_SPEC;
    impl crate::sealed::RegSpec for GxTrctr_SPEC {
        type DataType = u32;
    }
    #[doc = "Trigger Control Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxTrctr = crate::RegValueT<GxTrctr_SPEC>;

    impl GxTrctr {
        #[doc = "Trigger Sequence Counter   TSC. Controls the effect of an incoming internal trigger  If TSC  gt  00   decrement TSC by one."]
        #[inline(always)]
        pub fn tsc(
            self,
        ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, GxTrctr_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0x3f,1,0,u8, GxTrctr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Queue Active   QACT. Indicates that request source Q2 is currently executing a sequence. Cleared by writing 1 to bit COV."]
        #[inline(always)]
        pub fn qact(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, GxTrctr_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<14,1,0,GxTrctr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Overflow Detected   OV. Indicates that a trigger has been activated while the queue was still        active  QACT  160    160 1 . Cleared by writing 1 to bit COV."]
        #[inline(always)]
        pub fn ov(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, GxTrctr_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<15,1,0,GxTrctr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Trigger Sequence Counter Start Value   TSCSET. Defines the initial value of the trigger sequence counter TSC. TSC is reloaded with the value in TSCSET  when a trigger occurs while        TSC   00 . TSCSET is automatically copied to TSC when being written. CROSSREFERENCE"]
        #[inline(always)]
        pub fn tscset(
            self,
        ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, GxTrctr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x3f,1,0,u8, GxTrctr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Internal Trigger Input Selection   ITSEL. Internal triggers are generated by the respective source events. Enable a source event in the selected request source to generate an internal trigger signal. The selected trigger signal is internally connected to gate input GxREQGTP of this request source. It is selected when XTSEL   GTSEL   1111 ."]
        #[inline(always)]
        pub fn itsel(
            self,
        ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, GxTrctr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0x3,1,0,u8, GxTrctr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Disable   SRDIS. Controls if the source event of the selected trigger source also        activates a service request."]
        #[inline(always)]
        pub fn srdis(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, GxTrctr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<28,1,0,GxTrctr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Clear Overflow Flag   COV"]
        #[inline(always)]
        pub fn cov(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, GxTrctr_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<31,1,0,GxTrctr_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for GxTrctr {
        #[inline(always)]
        fn default() -> GxTrctr {
            <crate::RegValueT<GxTrctr_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxVfr_SPEC;
    impl crate::sealed::RegSpec for GxVfr_SPEC {
        type DataType = u32;
    }
    #[doc = "Valid Flag Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxVfr = crate::RegValueT<GxVfr_SPEC>;

    impl GxVfr {
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, GxVfr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, GxVfr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, GxVfr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf3(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, GxVfr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf4(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, GxVfr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf5(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, GxVfr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf6(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, GxVfr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf7(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, GxVfr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf8(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, GxVfr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf9(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, GxVfr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9,1,0,GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf10(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, GxVfr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf11(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, GxVfr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<11,1,0,GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf12(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, GxVfr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12,1,0,GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf13(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, GxVfr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13,1,0,GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf14(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, GxVfr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf15(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, GxVfr_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxVfr {
        #[inline(always)]
        fn default() -> GxVfr {
            <crate::RegValueT<GxVfr_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc = "Q"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Q(pub(super) *mut u8);
    unsafe impl core::marker::Send for Q {}
    unsafe impl core::marker::Sync for Q {}
    impl Q {
        #[doc = "Queue 0 Register 0  Group 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn gxq0ri(&self) -> crate::common::Reg<q::GxQ0Ri_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "Queue 0 Backup Register  Group 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn gxqburi(&self) -> crate::common::Reg<q::GxQbuRi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "Queue 0 Source Contr. Register  Group 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn gxqctrli(&self) -> crate::common::Reg<q::GxQctrLi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Queue 0 Input Register  Group 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn gxqinri(&self) -> crate::common::Reg<q::GxQinRi_SPEC, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "Queue 0 Mode Register  Group 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn gxqmri(&self) -> crate::common::Reg<q::GxQmRi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "Queue 0 Status Register  Group 0\n resetvalue={Application Reset:0x20}"]
        #[inline(always)]
        pub const fn gxqsri(&self) -> crate::common::Reg<q::GxQsRi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Queue 0 Requ. Timer Mode Reg.  Group 0\n resetvalue={Application Reset:0x0FFC00000}"]
        #[inline(always)]
        pub const fn gxreqtmi(&self) -> crate::common::Reg<q::GxReqtMi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
        }
        #[doc = "Queue 0 Requ. Timer Status Reg.  Group 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn gxreqtsi(&self) -> crate::common::Reg<q::GxReqtSi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
        }
    }
    pub mod q {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GxQ0Ri_SPEC;
        impl crate::sealed::RegSpec for GxQ0Ri_SPEC {
            type DataType = u32;
        }
        #[doc = "Queue 0 Register 0  Group 0\n resetvalue={Application Reset:0x0}"]
        pub type GxQ0Ri = crate::RegValueT<GxQ0Ri_SPEC>;

        impl GxQ0Ri {
            #[doc = "Request Channel Number   REQCHNR. Indicates the channel number to be converted."]
            #[inline(always)]
            pub fn reqchnr(
                self,
            ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, GxQ0Ri_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0x1f,1,0,u8, GxQ0Ri_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Refill   RF. Indicates the handling of handled requests."]
            #[inline(always)]
            pub fn rf(
                self,
            ) -> crate::common::RegisterFieldBool<5, 1, 0, GxQ0Ri_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<5,1,0,GxQ0Ri_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Enable Source Interrupt   ENSI"]
            #[inline(always)]
            pub fn ensi(
                self,
            ) -> crate::common::RegisterFieldBool<6, 1, 0, GxQ0Ri_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<6,1,0,GxQ0Ri_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "External Trigger   EXTR. Enables external trigger events."]
            #[inline(always)]
            pub fn extr(
                self,
            ) -> crate::common::RegisterFieldBool<7, 1, 0, GxQ0Ri_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<7,1,0,GxQ0Ri_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Channel Number Valid   V. Indicates a valid queue entry in queue register 0."]
            #[inline(always)]
            pub fn v(
                self,
            ) -> crate::common::RegisterFieldBool<8, 1, 0, GxQ0Ri_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<8,1,0,GxQ0Ri_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Pull Down Diagnostics Enable   PDD. Channels with pull down diagnostics device are marked in the          product specific appendix."]
            #[inline(always)]
            pub fn pdd(
                self,
            ) -> crate::common::RegisterFieldBool<9, 1, 0, GxQ0Ri_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<9,1,0,GxQ0Ri_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Multiplexer Diagnostics Pull Devices Enable   MDPD MDPU. Connecting combinations of pull up and or pull down devices generate        various loads for testing. Channels with multiplexer diagnostics pull devices are marked in the          product specific appendix."]
            #[inline(always)]
            pub fn mdpd(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, GxQ0Ri_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<10,1,0,GxQ0Ri_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Multiplexer Diagnostics Pull Devices Enable   MDPD MDPU. Connecting combinations of pull up and or pull down devices generate        various loads for testing. Channels with multiplexer diagnostics pull devices are marked in the          product specific appendix."]
            #[inline(always)]
            pub fn mdpu(
                self,
            ) -> crate::common::RegisterFieldBool<11, 1, 0, GxQ0Ri_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<11,1,0,GxQ0Ri_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Converter Diagnostics Enable   CDEN"]
            #[inline(always)]
            pub fn cden(
                self,
            ) -> crate::common::RegisterFieldBool<12, 1, 0, GxQ0Ri_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<12,1,0,GxQ0Ri_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Converter Diagnostics Pull Devices Select   CDSEL"]
            #[inline(always)]
            pub fn cdsel(
                self,
            ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, GxQ0Ri_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<13,0x3,1,0,u8, GxQ0Ri_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for GxQ0Ri {
            #[inline(always)]
            fn default() -> GxQ0Ri {
                <crate::RegValueT<GxQ0Ri_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GxQbuRi_SPEC;
        impl crate::sealed::RegSpec for GxQbuRi_SPEC {
            type DataType = u32;
        }
        #[doc = "Queue 0 Backup Register  Group 0\n resetvalue={Application Reset:0x0}"]
        pub type GxQbuRi = crate::RegValueT<GxQbuRi_SPEC>;

        impl GxQbuRi {
            #[doc = "Request Channel Number   REQCHNR. The channel number of the aborted conversion that has been requested by        this request source"]
            #[inline(always)]
            pub fn reqchnr(
                self,
            ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, GxQbuRi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0x1f,1,0,u8, GxQbuRi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Refill   RF. The refill control bit of the aborted conversion"]
            #[inline(always)]
            pub fn rf(
                self,
            ) -> crate::common::RegisterFieldBool<5, 1, 0, GxQbuRi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<5,1,0,GxQbuRi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Enable Source Interrupt   ENSI. The enable source interrupt control bit of the aborted conversion"]
            #[inline(always)]
            pub fn ensi(
                self,
            ) -> crate::common::RegisterFieldBool<6, 1, 0, GxQbuRi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<6,1,0,GxQbuRi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "External Trigger   EXTR. The external trigger control bit of the aborted conversion"]
            #[inline(always)]
            pub fn extr(
                self,
            ) -> crate::common::RegisterFieldBool<7, 1, 0, GxQbuRi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<7,1,0,GxQbuRi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Channel Number Valid   V. Indicates if the entry  REQCHNR  RF  TR  ENSI  in the queue backup        register is valid. Bit V is set when a running conversion  that has been        requested by this request source  is aborted  it is cleared when the        aborted conversion is restarted."]
            #[inline(always)]
            pub fn v(
                self,
            ) -> crate::common::RegisterFieldBool<8, 1, 0, GxQbuRi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<8,1,0,GxQbuRi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Pull Down Diagnostics Enable   PDD. Channels with pull down diagnostics device are marked in the          product specific appendix."]
            #[inline(always)]
            pub fn pdd(
                self,
            ) -> crate::common::RegisterFieldBool<9, 1, 0, GxQbuRi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<9,1,0,GxQbuRi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Multiplexer Diagnostics Pull Down Devices Enable. Connecting combinations of pull up and or pull down devices generate        various loads for testing. Channels with multiplexer diagnostics pull devices are marked in the          product specific appendix."]
            #[inline(always)]
            pub fn mdpd(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, GxQbuRi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<10,1,0,GxQbuRi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Multiplexer Diagnostics Pull Up Devices Enable. Connecting combinations of pull up and or pull down devices generate        various loads for testing. Channels with multiplexer diagnostics pull devices are marked in the          product specific appendix."]
            #[inline(always)]
            pub fn mdpu(
                self,
            ) -> crate::common::RegisterFieldBool<11, 1, 0, GxQbuRi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<11,1,0,GxQbuRi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Converter Diagnostics Enable   CDEN"]
            #[inline(always)]
            pub fn cden(
                self,
            ) -> crate::common::RegisterFieldBool<12, 1, 0, GxQbuRi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<12,1,0,GxQbuRi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Converter Diagnostics Pull Devices Select   CDSEL"]
            #[inline(always)]
            pub fn cdsel(
                self,
            ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, GxQbuRi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<13,0x3,1,0,u8, GxQbuRi_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for GxQbuRi {
            #[inline(always)]
            fn default() -> GxQbuRi {
                <crate::RegValueT<GxQbuRi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GxQctrLi_SPEC;
        impl crate::sealed::RegSpec for GxQctrLi_SPEC {
            type DataType = u32;
        }
        #[doc = "Queue 0 Source Contr. Register  Group 0\n resetvalue={Application Reset:0x0}"]
        pub type GxQctrLi = crate::RegValueT<GxQctrLi_SPEC>;

        impl GxQctrLi {
            #[doc = "Source specific Result Register   SRCRESREG"]
            #[inline(always)]
            pub fn srcresreg(
                self,
            ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, GxQctrLi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xf,1,0,u8, GxQctrLi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Trigger Source Selection   TRSEL. Daisy chaining via source events from the corresponding adjacent request source Qi. Use XTMODE   10 B ."]
            #[inline(always)]
            pub fn trsel(
                self,
            ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, GxQctrLi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<6,0x3,1,0,u8, GxQctrLi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "External Trigger Input Selection   XTSEL. The connected trigger input signals are listed in the product specific        appendix. XTSEL  160    160  1111 uses the          selected gate input as trigger source  ENGT must be 0X  ."]
            #[inline(always)]
            pub fn xtsel(
                self,
            ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, GxQctrLi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<8,0xf,1,0,u8, GxQctrLi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "External Trigger Level   XTLVL. Current level of the selected trigger input"]
            #[inline(always)]
            pub fn xtlvl(
                self,
            ) -> crate::common::RegisterFieldBool<12, 1, 0, GxQctrLi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<12,1,0,GxQctrLi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Trigger Operating Mode   XTMODE"]
            #[inline(always)]
            pub fn xtmode(
                self,
            ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, GxQctrLi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<13,0x3,1,0,u8, GxQctrLi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Write Control for Trigger Configuration   XTWC"]
            #[inline(always)]
            pub fn xtwc(
                self,
            ) -> crate::common::RegisterFieldBool<15, 1, 0, GxQctrLi_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<15,1,0,GxQctrLi_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Gate Input Selection   GTSEL. The connected gate input signals are listed in the product specific        appendix. GTSEL  160    160  1111 uses the          selected internal trigger source for queue 2."]
            #[inline(always)]
            pub fn gtsel(
                self,
            ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, GxQctrLi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0xf,1,0,u8, GxQctrLi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Gate Input Level   GTLVL. Current level of the selected gate input"]
            #[inline(always)]
            pub fn gtlvl(
                self,
            ) -> crate::common::RegisterFieldBool<20, 1, 0, GxQctrLi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<20,1,0,GxQctrLi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Write Control for Gate Configuration   GTWC"]
            #[inline(always)]
            pub fn gtwc(
                self,
            ) -> crate::common::RegisterFieldBool<23, 1, 0, GxQctrLi_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<23,1,0,GxQctrLi_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Timer Mode Enable   TMEN"]
            #[inline(always)]
            pub fn tmen(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, GxQctrLi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<28,1,0,GxQctrLi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Write Control for Timer Mode   TMWC"]
            #[inline(always)]
            pub fn tmwc(
                self,
            ) -> crate::common::RegisterFieldBool<31, 1, 0, GxQctrLi_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<31,1,0,GxQctrLi_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for GxQctrLi {
            #[inline(always)]
            fn default() -> GxQctrLi {
                <crate::RegValueT<GxQctrLi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GxQinRi_SPEC;
        impl crate::sealed::RegSpec for GxQinRi_SPEC {
            type DataType = u32;
        }
        #[doc = "Queue 0 Input Register  Group 0\n resetvalue={Application Reset:0x0}"]
        pub type GxQinRi = crate::RegValueT<GxQinRi_SPEC>;

        impl GxQinRi {
            #[doc = "Request Channel Number   REQCHNR. Defines the channel number to be converted. Not available channel numbers are treated as channel 0."]
            #[inline(always)]
            pub fn reqchnr(
                self,
            ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, GxQinRi_SPEC, crate::common::W>
            {
                crate::common::RegisterField::<0,0x1f,1,0,u8, GxQinRi_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Refill   RF"]
            #[inline(always)]
            pub fn rf(
                self,
            ) -> crate::common::RegisterFieldBool<5, 1, 0, GxQinRi_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<5,1,0,GxQinRi_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Enable Source Interrupt   ENSI"]
            #[inline(always)]
            pub fn ensi(
                self,
            ) -> crate::common::RegisterFieldBool<6, 1, 0, GxQinRi_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<6,1,0,GxQinRi_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "External Trigger   EXTR. Enables the external trigger functionality. To use external triggers  enable them by setting bit GxQMRy.ENTR."]
            #[inline(always)]
            pub fn extr(
                self,
            ) -> crate::common::RegisterFieldBool<7, 1, 0, GxQinRi_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<7,1,0,GxQinRi_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Pull Down Diagnostics Enable   PDD. Channels with pull down diagnostics device are marked in the          product specific appendix."]
            #[inline(always)]
            pub fn pdd(
                self,
            ) -> crate::common::RegisterFieldBool<9, 1, 0, GxQinRi_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<9,1,0,GxQinRi_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Multiplexer Diagnostics Pull Devices Enable   MDPD MDPU. Connecting combinations of pull up and or pull down devices generate        various loads for testing. Channels with multiplexer diagnostics pull devices are marked in the          product specific appendix."]
            #[inline(always)]
            pub fn mdpd(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, GxQinRi_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<10,1,0,GxQinRi_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Multiplexer Diagnostics Pull Devices Enable   MDPD MDPU. Connecting combinations of pull up and or pull down devices generate        various loads for testing. Channels with multiplexer diagnostics pull devices are marked in the          product specific appendix."]
            #[inline(always)]
            pub fn mdpu(
                self,
            ) -> crate::common::RegisterFieldBool<11, 1, 0, GxQinRi_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<11,1,0,GxQinRi_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Converter Diagnostics Enable   CDEN"]
            #[inline(always)]
            pub fn cden(
                self,
            ) -> crate::common::RegisterFieldBool<12, 1, 0, GxQinRi_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<12,1,0,GxQinRi_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Converter Diagnostics Pull Devices Select   CDSEL"]
            #[inline(always)]
            pub fn cdsel(
                self,
            ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, GxQinRi_SPEC, crate::common::W>
            {
                crate::common::RegisterField::<13,0x3,1,0,u8, GxQinRi_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for GxQinRi {
            #[inline(always)]
            fn default() -> GxQinRi {
                <crate::RegValueT<GxQinRi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GxQmRi_SPEC;
        impl crate::sealed::RegSpec for GxQmRi_SPEC {
            type DataType = u32;
        }
        #[doc = "Queue 0 Mode Register  Group 0\n resetvalue={Application Reset:0x0}"]
        pub type GxQmRi = crate::RegValueT<GxQmRi_SPEC>;

        impl GxQmRi {
            #[doc = "Enable Gate   ENGT. Selects the gating functionality for the request source. REQGTx is the selected gating signal."]
            #[inline(always)]
            pub fn engt(
                self,
            ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, GxQmRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0x3,1,0,u8, GxQmRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Enable External Trigger   ENTR"]
            #[inline(always)]
            pub fn entr(
                self,
            ) -> crate::common::RegisterFieldBool<2, 1, 0, GxQmRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<2,1,0,GxQmRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Clear Valid Bit   CLRV"]
            #[inline(always)]
            pub fn clrv(
                self,
            ) -> crate::common::RegisterFieldBool<8, 1, 0, GxQmRi_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<8,1,0,GxQmRi_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Trigger Event   TREV. Generates a software trigger for the request source. To trigger the request timer instead  write 1 to bit GxREQTIMi.REQTS."]
            #[inline(always)]
            pub fn trev(
                self,
            ) -> crate::common::RegisterFieldBool<9, 1, 0, GxQmRi_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<9,1,0,GxQmRi_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Flush Queue   FLUSH"]
            #[inline(always)]
            pub fn flush(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, GxQmRi_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<10,1,0,GxQmRi_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Clear Event Flag   CEV"]
            #[inline(always)]
            pub fn cev(
                self,
            ) -> crate::common::RegisterFieldBool<11, 1, 0, GxQmRi_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<11,1,0,GxQmRi_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Repeat Disable   RPTDIS"]
            #[inline(always)]
            pub fn rptdis(
                self,
            ) -> crate::common::RegisterFieldBool<16, 1, 0, GxQmRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<16,1,0,GxQmRi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for GxQmRi {
            #[inline(always)]
            fn default() -> GxQmRi {
                <crate::RegValueT<GxQmRi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GxQsRi_SPEC;
        impl crate::sealed::RegSpec for GxQsRi_SPEC {
            type DataType = u32;
        }
        #[doc = "Queue 0 Status Register  Group 0\n resetvalue={Application Reset:0x20}"]
        pub type GxQsRi = crate::RegValueT<GxQsRi_SPEC>;

        impl GxQsRi {
            #[doc = "Filling Level for Queue   FILL. Indicates the number of valid queue entries. It is incremented each time        a new entry is written to QINRx or by an enabled refill mechanism. It is        decremented each time a requested conversion has been started. A new        entry is ignored if the filling level has reached its maximum value. Maximum fill level for primary groups  8 entries Maximum fill level for secondary groups  16 entries"]
            #[inline(always)]
            pub fn fill(
                self,
            ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, GxQsRi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0xf,1,0,u8, GxQsRi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Queue Empty   EMPTY"]
            #[inline(always)]
            pub fn empty(
                self,
            ) -> crate::common::RegisterFieldBool<5, 1, 0, GxQsRi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<5,1,0,GxQsRi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Gate Level   REQGT. Monitors the level at the selected REQGT input."]
            #[inline(always)]
            pub fn reqgt(
                self,
            ) -> crate::common::RegisterFieldBool<7, 1, 0, GxQsRi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<7,1,0,GxQsRi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Event Detected   EV. Indicates that an event has been detected while at least one valid entry        has been in the queue  queue register 0 or backup stage . Once set  this        bit is cleared automatically when the requested conversion is started."]
            #[inline(always)]
            pub fn ev(
                self,
            ) -> crate::common::RegisterFieldBool<8, 1, 0, GxQsRi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<8,1,0,GxQsRi_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for GxQsRi {
            #[inline(always)]
            fn default() -> GxQsRi {
                <crate::RegValueT<GxQsRi_SPEC> as RegisterValue<_>>::new(32)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GxReqtMi_SPEC;
        impl crate::sealed::RegSpec for GxReqtMi_SPEC {
            type DataType = u32;
        }
        #[doc = "Queue 0 Requ. Timer Mode Reg.  Group 0\n resetvalue={Application Reset:0x0FFC00000}"]
        pub type GxReqtMi = crate::RegValueT<GxReqtMi_SPEC>;

        impl GxReqtMi {
            #[doc = "Sequence Mode   SEQMOD. Selects how the request timer controls the conversion sequence. Before changing the operating mode  stop the sequence timer  i.e. SEQMOD          00 ."]
            #[inline(always)]
            pub fn seqmod(
                self,
            ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, GxReqtMi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0x3,1,0,u8, GxReqtMi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Sequence Timer  Set Value   SEQTIMSET. Initial value for the sequence timer in steps of 16   215  t PER .        This value is loaded into SEQTIMER when a new request timer period is        started."]
            #[inline(always)]
            pub fn seqtimset(
                self,
            ) -> crate::common::RegisterField<6, 0x3ff, 1, 0, u16, GxReqtMi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<6,0x3ff,1,0,u16, GxReqtMi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Request Timer Start Trigger   REQTS"]
            #[inline(always)]
            pub fn reqts(
                self,
            ) -> crate::common::RegisterFieldBool<16, 1, 0, GxReqtMi_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<16,1,0,GxReqtMi_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Enable External Trigger   ENTR"]
            #[inline(always)]
            pub fn entr(
                self,
            ) -> crate::common::RegisterFieldBool<17, 1, 0, GxReqtMi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<17,1,0,GxReqtMi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Sequence Timer  Switch Off Value   SEQTIMOFF. The generated trigger signal is disabled when the timer value is equal to or below this threshold."]
            #[inline(always)]
            pub fn seqtimoff(
                self,
            ) -> crate::common::RegisterField<22, 0x3ff, 1, 0, u16, GxReqtMi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<22,0x3ff,1,0,u16, GxReqtMi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for GxReqtMi {
            #[inline(always)]
            fn default() -> GxReqtMi {
                <crate::RegValueT<GxReqtMi_SPEC> as RegisterValue<_>>::new(4290772992)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GxReqtSi_SPEC;
        impl crate::sealed::RegSpec for GxReqtSi_SPEC {
            type DataType = u32;
        }
        #[doc = "Queue 0 Requ. Timer Status Reg.  Group 0\n resetvalue={Application Reset:0x0}"]
        pub type GxReqtSi = crate::RegValueT<GxReqtSi_SPEC>;

        impl GxReqtSi {
            #[doc = "Sequence Timer   SEQTIMER. Counts the request timer periods in steps of 16   215  t ADC .        This timer is loaded from bitfield SEQTIMSET at the beginning of a        period."]
            #[inline(always)]
            pub fn seqtimer(
                self,
            ) -> crate::common::RegisterField<6, 0x3ff, 1, 0, u16, GxReqtSi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<6,0x3ff,1,0,u16, GxReqtSi_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for GxReqtSi {
            #[inline(always)]
            fn default() -> GxReqtSi {
                <crate::RegValueT<GxReqtSi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
