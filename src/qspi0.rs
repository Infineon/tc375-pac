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
#[doc = r"QSPI"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspi0(pub(super) *mut u8);
unsafe impl core::marker::Send for Qspi0 {}
unsafe impl core::marker::Sync for Qspi0 {}
impl Qspi0 {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "Basic Configuration Register\n resetvalue={Application Reset:0x0F871C71}"]
    #[inline(always)]
    pub const fn bacon(&self) -> crate::common::Reg<self::Bacon_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "BACON ENTRY Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn baconentry(&self) -> crate::common::Reg<self::Baconentry_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "DATA ENTRY Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dataentryx(
        &self,
    ) -> [crate::common::Reg<self::DataentrYx_SPEC, crate::common::W>; 8] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x64usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x64usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x64usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x64usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x64usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x64usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x64usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x64usize + 0x1cusize)),
            ]
        }
    }

    #[doc = "Configuration Extension 0\n resetvalue={Application Reset:0x1450}"]
    #[inline(always)]
    pub const fn econz(&self) -> [crate::common::Reg<self::EcoNz_SPEC, crate::common::RW>; 8] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x1cusize)),
            ]
        }
    }

    #[doc = "Flags Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn flagsclear(&self) -> crate::common::Reg<self::Flagsclear_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(84usize)) }
    }

    #[doc = "Global Configuration Register\n resetvalue={Application Reset:0x0F30FF}"]
    #[inline(always)]
    pub const fn globalcon(&self) -> crate::common::Reg<self::Globalcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "Global Configuration Register 1\n resetvalue={Application Reset:0x50000}"]
    #[inline(always)]
    pub const fn globalcon1(&self) -> crate::common::Reg<self::Globalcon1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0C0C000}"]
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

    #[doc = "Move Counter Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mc(&self) -> crate::common::Reg<self::Mc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(164usize)) }
    }

    #[doc = "Move Counter control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mccon(&self) -> crate::common::Reg<self::Mccon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(168usize)) }
    }

    #[doc = "MIX ENTRY Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mixentry(&self) -> crate::common::Reg<self::Mixentry_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(92usize)) }
    }

    #[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(232usize)) }
    }

    #[doc = "Port Input Select Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pisel(&self) -> crate::common::Reg<self::Pisel_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }

    #[doc = "RX EXIT Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rxexit(&self) -> crate::common::Reg<self::Rxexit_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }

    #[doc = "RX EXIT Debug Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rxexitd(&self) -> crate::common::Reg<self::Rxexitd_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }

    #[doc = "Slave Select Output Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ssoc(&self) -> crate::common::Reg<self::Ssoc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }

    #[doc = "Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn status(&self) -> crate::common::Reg<self::Status_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "Status Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn status1(&self) -> crate::common::Reg<self::Status1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }

    #[doc = "Extra Large Data Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn xxlcon(&self) -> crate::common::Reg<self::Xxlcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(88usize)) }
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
pub struct Bacon_SPEC;
impl crate::sealed::RegSpec for Bacon_SPEC {
    type DataType = u32;
}
#[doc = "Basic Configuration Register\n resetvalue={Application Reset:0x0F871C71}"]
pub type Bacon = crate::RegValueT<Bacon_SPEC>;

impl Bacon {
    #[doc = "Last Word in a Frame   LAST. Defines if the following data word is last in the current frame or not"]
    #[inline(always)]
    pub fn last(self) -> crate::common::RegisterFieldBool<0, 1, 0, Bacon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Bacon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Prescaler for the Idle Delay   IPRE. Length in T PER units"]
    #[inline(always)]
    pub fn ipre(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, Bacon_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x7,1,0,u8, Bacon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Idle Delay Length   IDLE. Defines the length of both idle delays  IDLEA and IDLEB  in T PER units pre scaled with IPRE"]
    #[inline(always)]
    pub fn idle(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, Bacon_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x7,1,0,u8, Bacon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Prescaler for the Leading Delay   LPRE. Length in T PER units"]
    #[inline(always)]
    pub fn lpre(
        self,
    ) -> crate::common::RegisterField<7, 0x7, 1, 0, u8, Bacon_SPEC, crate::common::R> {
        crate::common::RegisterField::<7,0x7,1,0,u8, Bacon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Leading Delay Length   LEAD. Defines the length of the leading delay  in T PER units pre scaled with LPRE"]
    #[inline(always)]
    pub fn lead(
        self,
    ) -> crate::common::RegisterField<10, 0x7, 1, 0, u8, Bacon_SPEC, crate::common::R> {
        crate::common::RegisterField::<10,0x7,1,0,u8, Bacon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Prescaler for the Trailing Delay   TPRE. Length in T PER units"]
    #[inline(always)]
    pub fn tpre(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, Bacon_SPEC, crate::common::R> {
        crate::common::RegisterField::<13,0x7,1,0,u8, Bacon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Trailing Delay Length   TRAIL. Defines the length of the trailing delay  in T PER units pre scaled with TPRE"]
    #[inline(always)]
    pub fn trail(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Bacon_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Bacon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Parity Type   PARTYP. Valid for both receive and transmit direction"]
    #[inline(always)]
    pub fn partyp(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Bacon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Bacon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "User Interrupt at the PT1 Event in the Subsequent Frames   UINT. This bit is an enable signal for the PT1 event routed to the User        Interrupt Service Request. The interrupt signals are generated until        disabled with the next BACON."]
    #[inline(always)]
    pub fn uint(self) -> crate::common::RegisterFieldBool<20, 1, 0, Bacon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Bacon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Shift MSB or LSB First   MSB. This bit sets the shift direction of the shift register. If the MSB        option is set  and the data is a block longer than 32 bits  the block        must be fed into the TXFIFO in reverse direction  from the end of the        block until its beginning."]
    #[inline(always)]
    pub fn msb(self) -> crate::common::RegisterFieldBool<21, 1, 0, Bacon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Bacon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Byte   BYTE. Defines if data length is expressed in bits or bytes. In Slave Mode BYTE must be  0 ."]
    #[inline(always)]
    pub fn byte(self) -> crate::common::RegisterFieldBool<22, 1, 0, Bacon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Bacon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Data Length   DL. Defines the data length in bits or bytes of one data block  depending on        the setting of the bit field BYTE. For the maximum baud rate of 50  160 MBaud  the minimum data length possible        is four."]
    #[inline(always)]
    pub fn dl(
        self,
    ) -> crate::common::RegisterField<23, 0x1f, 1, 0, u8, Bacon_SPEC, crate::common::R> {
        crate::common::RegisterField::<23,0x1f,1,0,u8, Bacon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Channel Select   CS. Selects the channel to which the subsequent data entry belongs  channel          the SLSO signal to be activated and the corresponding ECON        configuration extension  This bit field selects one slave in a range of 0 to 15  by driving one        SLSO signal out of 16 available. In case of an external demux mode  this bit field appears on the lines        SLS01 to SLSO4 as it is  additionally inverted or not  as defined in the SSOC register."]
    #[inline(always)]
    pub fn cs(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Bacon_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Bacon_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Bacon {
    #[inline(always)]
    fn default() -> Bacon {
        <crate::RegValueT<Bacon_SPEC> as RegisterValue<_>>::new(260512881)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Baconentry_SPEC;
impl crate::sealed::RegSpec for Baconentry_SPEC {
    type DataType = u32;
}
#[doc = "BACON ENTRY Register\n resetvalue={Application Reset:0x0}"]
pub type Baconentry = crate::RegValueT<Baconentry_SPEC>;

impl Baconentry {
    #[doc = "Entry Point to the TxFIFO   E"]
    #[inline(always)]
    pub fn e(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Baconentry_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Baconentry_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Baconentry {
    #[inline(always)]
    fn default() -> Baconentry {
        <crate::RegValueT<Baconentry_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Sleep Mode Enable Control   EDIS. Used to enable the module  8217 s sleep mode."]
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
pub struct DataentrYx_SPEC;
impl crate::sealed::RegSpec for DataentrYx_SPEC {
    type DataType = u32;
}
#[doc = "DATA ENTRY Register 0\n resetvalue={Application Reset:0x0}"]
pub type DataentrYx = crate::RegValueT<DataentrYx_SPEC>;

impl DataentrYx {
    #[doc = "Entry Point to the TxFIFO   E"]
    #[inline(always)]
    pub fn e(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, DataentrYx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, DataentrYx_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for DataentrYx {
    #[inline(always)]
    fn default() -> DataentrYx {
        <crate::RegValueT<DataentrYx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcoNz_SPEC;
impl crate::sealed::RegSpec for EcoNz_SPEC {
    type DataType = u32;
}
#[doc = "Configuration Extension 0\n resetvalue={Application Reset:0x1450}"]
pub type EcoNz = crate::RegValueT<EcoNz_SPEC>;

impl EcoNz {
    #[doc = "Time Quantum   Q. Defines the time quantum length used by A  B  and C to define the baud        rate and duty cycle by. This prescaler cascades the prescaler        GLOBALCON.TQ."]
    #[inline(always)]
    pub fn q(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, EcoNz_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, EcoNz_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bit Segment 1   A. Length expressed in time quantums of ECONz.Q."]
    #[inline(always)]
    pub fn a(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, EcoNz_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, EcoNz_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bit Segment 2   B. Length expressed in time quantums of ECONz.Q."]
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, EcoNz_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, EcoNz_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bit Segment 3   C. Length expressed in time quantums of ECONz.Q."]
    #[inline(always)]
    pub fn c(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, EcoNz_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, EcoNz_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Phase   CPH. Delay of one half SCLK clock cycle."]
    #[inline(always)]
    pub fn cph(self) -> crate::common::RegisterFieldBool<12, 1, 0, EcoNz_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, EcoNz_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clock Polarity   CPOL. Idle level of the shift clock signal at the SCLK pin"]
    #[inline(always)]
    pub fn cpol(self) -> crate::common::RegisterFieldBool<13, 1, 0, EcoNz_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, EcoNz_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Parity Check   PAREN. This bit field enables both the parity generation in transmit and parity        check in receive direction."]
    #[inline(always)]
    pub fn paren(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, EcoNz_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, EcoNz_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Permutate bytes to   from Big Endian   BE"]
    #[inline(always)]
    pub fn be(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, EcoNz_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, EcoNz_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for EcoNz {
    #[inline(always)]
    fn default() -> EcoNz {
        <crate::RegValueT<EcoNz_SPEC> as RegisterValue<_>>::new(5200)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flagsclear_SPEC;
impl crate::sealed::RegSpec for Flagsclear_SPEC {
    type DataType = u32;
}
#[doc = "Flags Clear Register\n resetvalue={Application Reset:0x0}"]
pub type Flagsclear = crate::RegValueT<Flagsclear_SPEC>;

impl Flagsclear {
    #[doc = "Write Only Bits for Clearing the Error Flags   ERRORCLEARS. Writing 1 clears the corresponding error flag in the ERORRFLAGS bit        field. Reading returns 0."]
    #[inline(always)]
    pub fn errorclears(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, Flagsclear_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16, Flagsclear_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Transmit Event Flag Clear   TXC. Write of 1 clears the STATUS .TXF        bit. Write of 0 has no effect. Read delivers 0."]
    #[inline(always)]
    pub fn txc(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Flagsclear_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9,1,0,Flagsclear_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Receive Event Flag Clear   RXC. Write of 1 clears the STATUS .RXF        bit. Write of 0 has no effect. Read delivers 0."]
    #[inline(always)]
    pub fn rxc(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Flagsclear_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10,1,0,Flagsclear_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "PT1 Event Flag Clear   PT1C. Write of 1 clears the STATUS .PT1F        bit. Write of 0 has no effect. Read delivers 0."]
    #[inline(always)]
    pub fn pt1c(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Flagsclear_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11,1,0,Flagsclear_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "PT2 Event Flag Clear   PT2C. Write of 1 clears the STATUS .PT2F        bit. Write of 0 has no effect. Read delivers 0."]
    #[inline(always)]
    pub fn pt2c(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Flagsclear_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12,1,0,Flagsclear_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "User Event Flag Clear   USRC. Write of 1 clears the STATUS .USRF        bit. Write of 0 has no effect. Read delivers 0."]
    #[inline(always)]
    pub fn usrc(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Flagsclear_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15,1,0,Flagsclear_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Flagsclear {
    #[inline(always)]
    fn default() -> Flagsclear {
        <crate::RegValueT<Flagsclear_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Globalcon_SPEC;
impl crate::sealed::RegSpec for Globalcon_SPEC {
    type DataType = u32;
}
#[doc = "Global Configuration Register\n resetvalue={Application Reset:0x0F30FF}"]
pub type Globalcon = crate::RegValueT<Globalcon_SPEC>;

impl Globalcon {
    #[doc = "Global Time Quantum Length   TQ. Common n divider scaling the baud rates of all channels in direction of        higher or lower baud rates. Must not be changed during a running        transaction."]
    #[inline(always)]
    pub fn tq(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Globalcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Globalcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status Injection   SI. Selects if the status register content injection into the RxFIFO is        enabled or disabled. The status injections  if enabled  is performed after each data block         depending on the BACON .DL        and BYTE setting."]
    #[inline(always)]
    pub fn si(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Globalcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Globalcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Time Out Value for the Expect Phase   EXPECT. expressed in T QSPI units"]
    #[inline(always)]
    pub fn expect(
        self,
    ) -> crate::common::RegisterField<10, 0xf, 1, 0, u8, Globalcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0xf,1,0,u8, Globalcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Loop Back Control   LB. Selects if the transmit output is internally connected to the receive        input for test purposes. For detailed description  see the Loop Back Mode section."]
    #[inline(always)]
    pub fn lb(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Globalcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Globalcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Delayed Mode for SLSO0   DEL0. Switches the delayed mode  external slave select expansion mode  on and        off"]
    #[inline(always)]
    pub fn del0(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Globalcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Globalcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Strobe Delay for SLSO0 in Delayed Mode   STROBE. Defines the length of the SLSO0 delay in T Q time units as defined for channel z   T Q units  selected by the current BACON .CS         if GLOBALCON .DEL0  160    160 1."]
    #[inline(always)]
    pub fn strobe(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Globalcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Globalcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Stop on RxFIFO Full   SRF. If this bit is set  the data fetching out of the TxFIFO by the shift        register stops when the RxFIFO is full  in order to prevent RxFIFO        overflow. Master mode only."]
    #[inline(always)]
    pub fn srf(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Globalcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21,1,0,Globalcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slave Transmit Idle State Polarity   STIP. This bit determines the logic level of the Slave Mode transmit signal        MRST when the QSPI slave select input signals are inactive         PISEL.SLSIS  160   185   160 000 B  ."]
    #[inline(always)]
    pub fn stip(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Globalcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,Globalcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Bit   EN. Used to request transition between PAUSE and RUN mode per software.        Cleared by hardware automatically at leaving the following states         disabled  suspend and sleep. In order to determine if the requested state has actually been reached         the STATUS .PHASE        bit field should be polled. See also Operation Modes."]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Globalcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,Globalcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Master Slave Mode   MS. Selects if the module operates in master or slave mode. This bit field must be configured before the first write to the TXFIFO."]
    #[inline(always)]
    pub fn ms(
        self,
    ) -> crate::common::RegisterField<25, 0x3, 1, 0, u8, Globalcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<25,0x3,1,0,u8, Globalcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Automatic Reset Enable   AREN. Enables the reset of the GLOBALCON .EN        on baud rate and spike error in slave mode."]
    #[inline(always)]
    pub fn aren(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Globalcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,Globalcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Select   CLKSEL. Selects the clock source for the asynchronous block."]
    #[inline(always)]
    pub fn clksel(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Globalcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,Globalcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bits for resetting sub modules per software   RESETS. Write to this bit field triggers a reset operation. The reset operation   cycle count depends on the Clock setup in the 2 Clock domains. The shortest time is 3 x T fast   3 x T slow. With the added margin the delay will be  10 times slowest clock between SPB and Kernel. The duration of reset operation depends on the used clock setups. It is 10 times of the slowest clock between SPB and Kernel. CLOCKSEL shall be enabled to use GLOBALCON.RESETS. After the Reset the CLOCKSEL will be also cleared in case of 10 11. For resetting the whole module kernel  use alternatively the registers KRST0   xa0    xa0  KRST1 reset mechanism."]
    #[inline(always)]
    pub fn resets(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, Globalcon_SPEC, crate::common::W> {
        crate::common::RegisterField::<30,0x3,1,0,u8, Globalcon_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Globalcon {
    #[inline(always)]
    fn default() -> Globalcon {
        <crate::RegValueT<Globalcon_SPEC> as RegisterValue<_>>::new(995583)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Globalcon1_SPEC;
impl crate::sealed::RegSpec for Globalcon1_SPEC {
    type DataType = u32;
}
#[doc = "Global Configuration Register 1\n resetvalue={Application Reset:0x50000}"]
pub type Globalcon1 = crate::RegValueT<Globalcon1_SPEC>;

impl Globalcon1 {
    #[doc = "Error Enable Bits   ERRORENS. Bits for enabling interrupt on all available error types"]
    #[inline(always)]
    pub fn errorens(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, Globalcon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16, Globalcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Tx Interrupt Event Enable   TXEN. Enables the Tx interrupt."]
    #[inline(always)]
    pub fn txen(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Globalcon1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Globalcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RX Interrupt Event Enable   RXEN. Enables the Rx interrupt."]
    #[inline(always)]
    pub fn rxen(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Globalcon1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Globalcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt on PT1 Event Enable   PT1EN. Enables the PT interrupt on an PT1 event  as selected by the PT1 bit        field."]
    #[inline(always)]
    pub fn pt1en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Globalcon1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Globalcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt on PT2 Event Enable   PT2EN. Enables the PT interrupt on an PT2 event  as selected by the PT2 bit        field."]
    #[inline(always)]
    pub fn pt2en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Globalcon1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Globalcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt on USR Event Enable   USREN. Enables the USR interrupt on an USR event  as selected by the PT1 bit        field. In Move Counter mode of operation  User Interrupt line is reused for          the IAL and IBL interrupts  which can be enabled by MCCON .IALEN          and MCCON .IBLEN.          Bit USREN does not influence these interrupts."]
    #[inline(always)]
    pub fn usren(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Globalcon1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Globalcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit FIFO Interrupt Threshold   TXFIFOINT. In Combined Mode  as long as the TXFIFO filling level is equal or less        than this threshold  than each move of data or configuration from the        TXFIFO triggers a transmit interrupt. In Batch Mode  interrupt is generated only at the moment of filling        level falling below the threshold level. In Single Mode  this bit field is don  8217 t care. Reset value of the level is 01 ."]
    #[inline(always)]
    pub fn txfifoint(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Globalcon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Globalcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive FIFO Interrupt Threshold   RXFIFOINT. In Combined Mode  as long as the RXFIFO filling level is equal or        greater than this threshold  than each move of data or status  if        enabled  into the RXFIFO triggers a receive interrupt. In Batch mode  interrupt is generated only at the moment of filling        level exceeding this threshold level. In Single Mode  this bit field is don  8217 t care. Reset value of the level is 01 ."]
    #[inline(always)]
    pub fn rxfifoint(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, Globalcon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, Globalcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Phase Transition Event 1   PT1. Selects the first phase transition to trigger the PT interrupt."]
    #[inline(always)]
    pub fn pt1(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, Globalcon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x7,1,0,u8, Globalcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Phase Transition Event 2   PT2. Selects the second phase transition to trigger the PT2 interrupt. In Master Mode  the following events are available. In Slave Mode  only the SLSI signal  rising edge  triggers the interrupt for PT2. For this purpose  always use the setting 101  EOF . This interrupt is independent of the CS value."]
    #[inline(always)]
    pub fn pt2(
        self,
    ) -> crate::common::RegisterField<23, 0x7, 1, 0, u8, Globalcon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<23,0x7,1,0,u8, Globalcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TXFIFO Mode   TXFM. Selects the TXFIFO mode."]
    #[inline(always)]
    pub fn txfm(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, Globalcon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3,1,0,u8, Globalcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RXFIFO Mode   RXFM. Selects the RXFIFO mode."]
    #[inline(always)]
    pub fn rxfm(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, Globalcon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, Globalcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Globalcon1 {
    #[inline(always)]
    fn default() -> Globalcon1 {
        <crate::RegValueT<Globalcon1_SPEC> as RegisterValue<_>>::new(327680)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0C0C000}"]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(12632064)
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
pub struct Mc_SPEC;
impl crate::sealed::RegSpec for Mc_SPEC {
    type DataType = u32;
}
#[doc = "Move Counter Register\n resetvalue={Application Reset:0x0}"]
pub type Mc = crate::RegValueT<Mc_SPEC>;

impl Mc {
    #[doc = "Move Count   MCOUNT. Defines the number of moves to be performed in short mode  in range of 1        to 8191."]
    #[inline(always)]
    pub fn mcount(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, Mc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1fff,1,0,u16, Mc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Current Status of the Move Counter   CURRENT. Shows the current status of the Move Counter  that is  how many data blocks are to be transmitted until the end of the frame."]
    #[inline(always)]
    pub fn current(
        self,
    ) -> crate::common::RegisterField<16, 0x1fff, 1, 0, u16, Mc_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x1fff,1,0,u16, Mc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Mc {
    #[inline(always)]
    fn default() -> Mc {
        <crate::RegValueT<Mc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mccon_SPEC;
impl crate::sealed::RegSpec for Mccon_SPEC {
    type DataType = u32;
}
#[doc = "Move Counter control Register\n resetvalue={Application Reset:0x0}"]
pub type Mccon = crate::RegValueT<Mccon_SPEC>;

impl Mccon {
    #[doc = "Prescaler for the Trailing Delay 2   TPRE2. Trailing delay injected in the configuration register for the last data        if T2EN is set. Length in units T PER"]
    #[inline(always)]
    pub fn tpre2(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Mccon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Mccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Last Trailing Delay   TRAIL2. Trailing delay injected in the configuration register for the last data        if T2EN is set. Defines the length of the leading delay  in T PER units pre scaled with TPRE"]
    #[inline(always)]
    pub fn trail2(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, Mccon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x7,1,0,u8, Mccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Before Last Enable   IBLEN. Enable bit for this event."]
    #[inline(always)]
    pub fn iblen(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Mccon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Mccon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Before Last Flag   IBLF. Flag bit for this event."]
    #[inline(always)]
    pub fn iblf(self) -> crate::common::RegisterFieldBool<17, 1, 0, Mccon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Mccon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit for IBLF   IBLC. Writing 1 clears the IBLF. Writing 0 has no effect. Returns 0 on read."]
    #[inline(always)]
    pub fn iblc(self) -> crate::common::RegisterFieldBool<18, 1, 0, Mccon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Mccon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit for IBLF   IBLS. Writing 1 sets the IBLF and triggers an interrupt  if enabled . Writing        0 has no effect. Returns 0 on read."]
    #[inline(always)]
    pub fn ibls(self) -> crate::common::RegisterFieldBool<19, 1, 0, Mccon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, Mccon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt After Last Enable   IALEN. Enable bit for this event."]
    #[inline(always)]
    pub fn ialen(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Mccon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Mccon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt After Last Flag   IALF. Flag bit for this event."]
    #[inline(always)]
    pub fn ialf(self) -> crate::common::RegisterFieldBool<21, 1, 0, Mccon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Mccon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit for IALF   IALC. Writing 1 clears the IALF. Writing 0 has no effect. Returns 0 on read."]
    #[inline(always)]
    pub fn ialc(self) -> crate::common::RegisterFieldBool<22, 1, 0, Mccon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, Mccon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit for IALF   IALS. Writing 1 sets the IALF and triggers an interrupt  if enabled . Writing        0 has no effect. Returns 0 on read."]
    #[inline(always)]
    pub fn ials(self) -> crate::common::RegisterFieldBool<23, 1, 0, Mccon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, Mccon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "TRAIL 2 Injection Enable   T2EN. This bit has to be configured before the transmission of the frame        starts. If set  a new value for the last trailing delay will be injected for the        last data block  as defined with the bit field TRAIL2. If not set  the TRAIL value from the latest BACON will be valid also as the last trailing delay."]
    #[inline(always)]
    pub fn t2en(self) -> crate::common::RegisterFieldBool<30, 1, 0, Mccon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Mccon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Move Counter Enable   MCEN. Enables the Move Counter feature. If enabled  the MCOUNT value is taken        in consideration  otherwise the standard continuous mode is active."]
    #[inline(always)]
    pub fn mcen(self) -> crate::common::RegisterFieldBool<31, 1, 0, Mccon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Mccon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mccon {
    #[inline(always)]
    fn default() -> Mccon {
        <crate::RegValueT<Mccon_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mixentry_SPEC;
impl crate::sealed::RegSpec for Mixentry_SPEC {
    type DataType = u32;
}
#[doc = "MIX ENTRY Register\n resetvalue={Application Reset:0x0}"]
pub type Mixentry = crate::RegValueT<Mixentry_SPEC>;

impl Mixentry {
    #[doc = "Entry Point to the TxFIFO   E"]
    #[inline(always)]
    pub fn e(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Mixentry_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Mixentry_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Mixentry {
    #[inline(always)]
    fn default() -> Mixentry {
        <crate::RegValueT<Mixentry_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Master Mode Receive Input Select. MRIS selects one out of eight MRST receive input lines  used in Master        Mode. Note that not all inputs are used in every device of the family.        Selecting an unused input returns a continuous low value. The following signal sources are available in this product  if supported        by the package"]
    #[inline(always)]
    pub fn mris(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Pisel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slave Mode Receive Input Select. SRIS selects one out of eight MTSR receive input lines  used in Slave        Mode. Note that not all inputs are used in every device of the family.        Selecting an unused input returns a continuous low value. The following signal sources are available in this product  if supported        by the package"]
    #[inline(always)]
    pub fn sris(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, Pisel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slave Mode Clock Input Select. SCIS selects one out of eight module kernel SCLK input lines that is        used as clock input line in slave mode. Note that not all inputs are        used in every device of the family. Selecting an unused input returns a        continuous low value. The following signal sources are available in this product  if supported        by the package"]
    #[inline(always)]
    pub fn scis(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Pisel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slave Mode Slave Select Input Selection. The SLSIS must be programmed properly before the slave mode is set with GLOBALCON.MODE and the module is set to RUN mode. The following signal sources are available in this product  if supported by the package"]
    #[inline(always)]
    pub fn slsis(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, Pisel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8, Pisel_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct Rxexit_SPEC;
impl crate::sealed::RegSpec for Rxexit_SPEC {
    type DataType = u32;
}
#[doc = "RX EXIT Register\n resetvalue={Application Reset:0x0}"]
pub type Rxexit = crate::RegValueT<Rxexit_SPEC>;

impl Rxexit {
    #[doc = "Read Point from the RxFIFO   E"]
    #[inline(always)]
    pub fn e(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Rxexit_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Rxexit_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Rxexit {
    #[inline(always)]
    fn default() -> Rxexit {
        <crate::RegValueT<Rxexit_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxexitd_SPEC;
impl crate::sealed::RegSpec for Rxexitd_SPEC {
    type DataType = u32;
}
#[doc = "RX EXIT Debug Register\n resetvalue={Application Reset:0x0}"]
pub type Rxexitd = crate::RegValueT<Rxexitd_SPEC>;

impl Rxexitd {
    #[doc = "Read Point from the RxFIFO   E"]
    #[inline(always)]
    pub fn e(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Rxexitd_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Rxexitd_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Rxexitd {
    #[inline(always)]
    fn default() -> Rxexitd {
        <crate::RegValueT<Rxexitd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssoc_SPEC;
impl crate::sealed::RegSpec for Ssoc_SPEC {
    type DataType = u32;
}
#[doc = "Slave Select Output Control Register\n resetvalue={Application Reset:0x0}"]
pub type Ssoc = crate::RegValueT<Ssoc_SPEC>;

impl Ssoc {
    #[doc = "Active Output Level for the SLSO Outputs   AOL. The idle level is the inverted one.   8220 0  8221  at certain position means active low level for the corresponding        SLSO.   8220 1  8221  means active high."]
    #[inline(always)]
    pub fn aol(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Ssoc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Ssoc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Bits for the SLSO Outputs   OEN. In disabled state the SLSO output drives the idle level as defined by        the AOL bit field.   8220 0  8221  at certain position means that the corresponding SLSO is disabled.   8220 1  8221  means enabled."]
    #[inline(always)]
    pub fn oen(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Ssoc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Ssoc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ssoc {
    #[inline(always)]
    fn default() -> Ssoc {
        <crate::RegValueT<Ssoc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status_SPEC;
impl crate::sealed::RegSpec for Status_SPEC {
    type DataType = u32;
}
#[doc = "Status Register\n resetvalue={Application Reset:0x0}"]
pub type Status = crate::RegValueT<Status_SPEC>;

impl Status {
    #[doc = "Sticky Flags Signalling Errors   ERRORFLAGS. Writing 1 sets the error Flag and triggers an error interrupt  if        enabled. Writing 0 has no effect."]
    #[inline(always)]
    pub fn errorflags(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, Status_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16, Status_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit Interrupt Request Flag   TXF. Flags an occurrence of a request to feed the TXFIFO  which is generated        when an element is fetched from the FIFO  and the FIFO filling level is        equal or less than the set threshold level. Writing 1 sets the flag and triggers an interrupt if GLOBALCON1 .TXEN  160    160 1. Writing 0 has no effect."]
    #[inline(always)]
    pub fn txf(self) -> crate::common::RegisterFieldBool<9, 1, 0, Status_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Status_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive Interrupt Request Flag   RXF. Flags an occurrence of a request to empty the RXFIFO  which is generated        when an element is written into the FIFO  and the FIFO filling level is        equal or greater than the set threshold level. Writing 1 sets the flag and triggers an interrupt if GLOBALCON1 .RXEN  160    160 1. Writing 0 has no effect."]
    #[inline(always)]
    pub fn rxf(self) -> crate::common::RegisterFieldBool<10, 1, 0, Status_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Status_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Phase Transition 1 Flag   PT1F. Flags an occurrence of a PT1 event  as selected with the GLOBALCON1 .PT1         and triggers an interrupt if GLOBALCON1 .PT1EN  160    160 1. Writing 1 sets the flag and triggers an error interrupt. Writing 0 has no effect."]
    #[inline(always)]
    pub fn pt1f(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Status_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Status_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Phase Transition 2 Flag   PT2F. In master mode  flags an occurrence of a PT2 event  as selected with the GLOBALCON1 .PT2         and triggers an interrupt if GLOBALCON1 .PT2EN  160    160 1. In slave mode  set by the SLSI deactivated event. Writing 1 sets the flag and triggers an error interrupt. Writing 0 has no effect."]
    #[inline(always)]
    pub fn pt2f(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Status_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Status_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "User Interrupt Request Flag   USRF. Flags an occurrence of an USR event. Writing 1 sets the flag and triggers an interrupt if GLOBALCON1 .USREN  160    160 1. Writing 0 has no effect."]
    #[inline(always)]
    pub fn usrf(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Status_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Status_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "TXFIFO Filling Level   TXFIFOLEVEL. Shows how many entries in the TXFIFO are waiting for transmission"]
    #[inline(always)]
    pub fn txfifolevel(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Status_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Status_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RXFIFO Filling Level   RXFIFOLEVEL. Shows how many entries in the RXFIFO are waiting for software to move        them to RAM"]
    #[inline(always)]
    pub fn rxfifolevel(
        self,
    ) -> crate::common::RegisterField<19, 0x7, 1, 0, u8, Status_SPEC, crate::common::R> {
        crate::common::RegisterField::<19,0x7,1,0,u8, Status_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Currently Active Slave Select Flag   SLAVESEL. Displays the currently active slave select."]
    #[inline(always)]
    pub fn slavesel(
        self,
    ) -> crate::common::RegisterField<22, 0xf, 1, 0, u8, Status_SPEC, crate::common::R> {
        crate::common::RegisterField::<22,0xf,1,0,u8, Status_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Received Parity Value   RPV. Displays the last received parity bit  if parity was enabled. Else if        the parity is disabled  reads 0."]
    #[inline(always)]
    pub fn rpv(self) -> crate::common::RegisterFieldBool<26, 1, 0, Status_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Status_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmitted Parity Value   TPV. Displays the last transmitted parity bit  if parity was enabled. Else 0."]
    #[inline(always)]
    pub fn tpv(self) -> crate::common::RegisterFieldBool<27, 1, 0, Status_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Status_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Flags the ongoing phase   PHASE. Displays the current phase number. Relevant only in master mode. In        slave mode this bit field indicates always 0. Not 0 means busy."]
    #[inline(always)]
    pub fn phase(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Status_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Status_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Status {
    #[inline(always)]
    fn default() -> Status {
        <crate::RegValueT<Status_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status1_SPEC;
impl crate::sealed::RegSpec for Status1_SPEC {
    type DataType = u32;
}
#[doc = "Status Register 1\n resetvalue={Application Reset:0x0}"]
pub type Status1 = crate::RegValueT<Status1_SPEC>;

impl Status1 {
    #[doc = "Number of bits shifted out   BITCOUNT. Supports up to 256 bits. A BITCOUNT value of greater than 0 indicates that a transmission is in progress. The value is not accurate  it may be lower than the number of bits shifted. After transmission of the last bit  BITCOUNT is set to zero."]
    #[inline(always)]
    pub fn bitcount(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Status1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Status1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Baud Rate Deviation Enable   BRDEN. Enables the signal path."]
    #[inline(always)]
    pub fn brden(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Status1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Status1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Baud Rate Deviation Flag   BRD. Shows if baud rate deviation has been detected. Write of 1 sets the bit        and raises the event per software. Write of 0 has no effect."]
    #[inline(always)]
    pub fn brd(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Status1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Status1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Spike Detection Enable   SPDEN. Enables the signal path."]
    #[inline(always)]
    pub fn spden(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Status1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Status1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Spike Detection Flag   SPD. Shows if spike has been detected. Write of 1 sets the bit and raises the        event per software. Write of 0 has no effect."]
    #[inline(always)]
    pub fn spd(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Status1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Status1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Status1 {
    #[inline(always)]
    fn default() -> Status1 {
        <crate::RegValueT<Status1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xxlcon_SPEC;
impl crate::sealed::RegSpec for Xxlcon_SPEC {
    type DataType = u32;
}
#[doc = "Extra Large Data Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Xxlcon = crate::RegValueT<Xxlcon_SPEC>;

impl Xxlcon {
    #[doc = "Extended Data Length   XDL. Defines the length of the data block in bytes in range of 2 to 65536.        Overrides BACON .DL        when BACON .BYTE 1        and BACON .DL 0."]
    #[inline(always)]
    pub fn xdl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Xxlcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Xxlcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extended Data Length   BYTECOUNT. In the XXL mode  shows the current state of the internal byte down        counter  bytes remaining to be sent . In short and long modes  the value        of this bit field is don  8217 t care."]
    #[inline(always)]
    pub fn bytecount(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Xxlcon_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Xxlcon_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Xxlcon {
    #[inline(always)]
    fn default() -> Xxlcon {
        <crate::RegValueT<Xxlcon_SPEC> as RegisterValue<_>>::new(0)
    }
}
