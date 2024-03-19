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
#[doc = r"SMU"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smu(pub(super) *mut u8);
unsafe impl core::marker::Send for Smu {}
unsafe impl core::marker::Sync for Smu {}
impl Smu {
    #[doc = "SMU core Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2044usize)) }
    }

    #[doc = "Alarm Debug Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn adi(&self) -> [crate::common::Reg<self::ADi_SPEC, crate::common::R>; 12] {
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
            ]
        }
    }

    #[doc = "Alarm Executed Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn aex(&self) -> crate::common::Reg<self::Aex_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(112usize)) }
    }

    #[doc = "Alarm Executed Status Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn aexclr(&self) -> crate::common::Reg<self::Aexclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(116usize)) }
    }

    #[doc = "Alarm and Fault Counter\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn afcnt(&self) -> crate::common::Reg<self::Afcnt_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "Alarm Global Configuration\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn agc(&self) -> crate::common::Reg<self::Agc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }

    #[doc = "Alarm Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn agi(&self) -> [crate::common::Reg<self::AGi_SPEC, crate::common::RW>; 12] {
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
            ]
        }
    }

    #[doc = "SMU core FSP Configuration Register\n resetvalue={Application Reset:0x0,Application Reset:0x30000}"]
    #[inline(always)]
    pub const fn agifsp(&self) -> [crate::common::Reg<self::AGiFsp_SPEC, crate::common::RW>; 12] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0x2cusize)),
            ]
        }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Command Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cmd(&self) -> crate::common::Reg<self::Cmd_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Debug Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn dbg(&self) -> crate::common::Reg<self::Dbg_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }

    #[doc = "Fault Signaling Protocol\n resetvalue={PowerOn Reset:0x3FFF00}"]
    #[inline(always)]
    pub const fn fsp(&self) -> crate::common::Reg<self::Fsp_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x089C001}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Key Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn keys(&self) -> crate::common::Reg<self::Keys_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2024usize)) }
    }

    #[doc = "Port Control\n resetvalue={PowerOn Reset:0x08000}"]
    #[inline(always)]
    pub const fn pctl(&self) -> crate::common::Reg<self::Pctl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "Register Monitor Control\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rmctl(&self) -> crate::common::Reg<self::Rmctl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(768usize)) }
    }

    #[doc = "Register Monitor Error Flags\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rmef(&self) -> crate::common::Reg<self::Rmef_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(772usize)) }
    }

    #[doc = "Register Monitor Self Test Status\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rmsts(&self) -> crate::common::Reg<self::Rmsts_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(776usize)) }
    }

    #[doc = "Recovery Timer 0 Alarm Configuration 0\n resetvalue={Application Reset:0x0A80108}"]
    #[inline(always)]
    pub const fn rtac00(&self) -> crate::common::Reg<self::Rtac00_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }

    #[doc = "Recovery Timer 0 Alarm Configuration 1\n resetvalue={Application Reset:0x0C800B8}"]
    #[inline(always)]
    pub const fn rtac01(&self) -> crate::common::Reg<self::Rtac01_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(100usize)) }
    }

    #[doc = "Recovery Timer 1 Alarm Configuration 0\n resetvalue={Application Reset:0x0E800D8}"]
    #[inline(always)]
    pub const fn rtac10(&self) -> crate::common::Reg<self::Rtac10_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(104usize)) }
    }

    #[doc = "Recovery Timer 1 Alarm Configuration 1\n resetvalue={Application Reset:0x0F800F8}"]
    #[inline(always)]
    pub const fn rtac11(&self) -> crate::common::Reg<self::Rtac11_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(108usize)) }
    }

    #[doc = "Recovery Timer Configuration\n resetvalue={Application Reset:0x3FFF03}"]
    #[inline(always)]
    pub const fn rtc(&self) -> crate::common::Reg<self::Rtc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }

    #[doc = "Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sts(&self) -> crate::common::Reg<self::Sts_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "AGRP"]
    #[inline(always)]
    pub fn agicfj(self) -> [self::AGiCFj; 12] {
        unsafe {
            [
                self::AGiCFj(self.0.add(0x100usize + 0x0usize)),
                self::AGiCFj(self.0.add(0x100usize + 0x4usize)),
                self::AGiCFj(self.0.add(0x100usize + 0x8usize)),
                self::AGiCFj(self.0.add(0x100usize + 0xcusize)),
                self::AGiCFj(self.0.add(0x100usize + 0x10usize)),
                self::AGiCFj(self.0.add(0x100usize + 0x14usize)),
                self::AGiCFj(self.0.add(0x100usize + 0x18usize)),
                self::AGiCFj(self.0.add(0x100usize + 0x1cusize)),
                self::AGiCFj(self.0.add(0x100usize + 0x20usize)),
                self::AGiCFj(self.0.add(0x100usize + 0x24usize)),
                self::AGiCFj(self.0.add(0x100usize + 0x28usize)),
                self::AGiCFj(self.0.add(0x100usize + 0x2cusize)),
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
#[doc = "SMU core Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accen0 = crate::RegValueT<Accen0_SPEC>;

impl Accen0 {
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
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
pub struct ADi_SPEC;
impl crate::sealed::RegSpec for ADi_SPEC {
    type DataType = u32;
}
#[doc = "Alarm Debug Register\n resetvalue={PowerOn Reset:0x0}"]
pub type ADi = crate::RegValueT<ADi_SPEC>;

impl ADi {
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df0(self) -> crate::common::RegisterFieldBool<0, 1, 0, ADi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, ADi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df1(self) -> crate::common::RegisterFieldBool<1, 1, 0, ADi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, ADi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df2(self) -> crate::common::RegisterFieldBool<2, 1, 0, ADi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, ADi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df4(self) -> crate::common::RegisterFieldBool<4, 1, 0, ADi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, ADi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df5(self) -> crate::common::RegisterFieldBool<5, 1, 0, ADi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, ADi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df6(self) -> crate::common::RegisterFieldBool<6, 1, 0, ADi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, ADi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df7(self) -> crate::common::RegisterFieldBool<7, 1, 0, ADi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, ADi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df8(self) -> crate::common::RegisterFieldBool<8, 1, 0, ADi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, ADi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df9(self) -> crate::common::RegisterFieldBool<9, 1, 0, ADi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, ADi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df10(self) -> crate::common::RegisterFieldBool<10, 1, 0, ADi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, ADi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df11(self) -> crate::common::RegisterFieldBool<11, 1, 0, ADi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, ADi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df12(self) -> crate::common::RegisterFieldBool<12, 1, 0, ADi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, ADi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df13(self) -> crate::common::RegisterFieldBool<13, 1, 0, ADi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, ADi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df14(self) -> crate::common::RegisterFieldBool<14, 1, 0, ADi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, ADi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df22(self) -> crate::common::RegisterFieldBool<22, 1, 0, ADi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, ADi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df23(self) -> crate::common::RegisterFieldBool<23, 1, 0, ADi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, ADi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df24(self) -> crate::common::RegisterFieldBool<24, 1, 0, ADi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, ADi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for ADi {
    #[inline(always)]
    fn default() -> ADi {
        <crate::RegValueT<ADi_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aex_SPEC;
impl crate::sealed::RegSpec for Aex_SPEC {
    type DataType = u32;
}
#[doc = "Alarm Executed Status Register\n resetvalue={Application Reset:0x0}"]
pub type Aex = crate::RegValueT<Aex_SPEC>;

impl Aex {
    #[doc = "IRQ0 Request Status   IRQ0STS. This bit indicates whether a IRQ0 request was serviced or not. This bit        is set by the SMU core after a alarm configured for IRQ0 is detected. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn irq0sts(self) -> crate::common::RegisterFieldBool<0, 1, 0, Aex_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Aex_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "IRQ1 Request Status   IRQ1STS. This bit indicates whether a IRQ1 request was serviced or not. This bit        is set by the SMU core after a alarm configured for IRQ1 is detected. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn irq1sts(self) -> crate::common::RegisterFieldBool<1, 1, 0, Aex_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Aex_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "IRQ2 Request Status   IRQ2STS. This bit indicates whether a IRQ2 request was serviced or not. This bit        is set by the SMU core after a alarm configured for IRQ2 is detected. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn irq2sts(self) -> crate::common::RegisterFieldBool<2, 1, 0, Aex_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Aex_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RST0 Request Status   RST0STS. This bit indicates whether a RST0 request was serviced or not. This bit        is set by the SMU core after a alarm configured for RST0 is detected. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst0sts(self) -> crate::common::RegisterFieldBool<3, 1, 0, Aex_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Aex_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RST1 Request Status   RST1STS. This bit indicates whether a RST1 request was serviced or not. This bit        is set by the SMU core after a alarm configured for RST1 is detected. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst1sts(self) -> crate::common::RegisterFieldBool<4, 1, 0, Aex_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Aex_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RST2 Request Status   RST2STS. This bit indicates whether a RST2 request was serviced or not. This bit        is set by the SMU core after a alarm configured for RST2 is detected. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst2sts(self) -> crate::common::RegisterFieldBool<5, 1, 0, Aex_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Aex_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RST3 Request Status   RST3STS. This bit indicates whether a RST3 request was serviced or not. This bit        is set by the SMU core after a alarm configured for RST3 is detected. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst3sts(self) -> crate::common::RegisterFieldBool<6, 1, 0, Aex_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Aex_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RST4 Request Status   RST4STS. This bit indicates whether a RST4 request was serviced or not. This bit        is set by the SMU core after a alarm configured for RST4 is detected. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst4sts(self) -> crate::common::RegisterFieldBool<7, 1, 0, Aex_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Aex_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RST5 Request Status   RST5STS. This bit indicates whether a RST5 request was serviced or not. This bit        is set by the SMU core after a alarm configured for RST5 is detected. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst5sts(self) -> crate::common::RegisterFieldBool<8, 1, 0, Aex_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Aex_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NMI Request Status   NMISTS. This bit indicates whether a NMI request was serviced or not. This bit        is set by the SMU core after a alarm configured for NMI is detected. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn nmists(self) -> crate::common::RegisterFieldBool<9, 1, 0, Aex_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Aex_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "EMS Request Status   EMSSTS. This bit indicates whether a EMS request  triggered by an alarm  not        SMU ActivatePES   was serviced or not. This bit is set by the SMU core        after a alarm configured for EMS is detected. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn emssts(self) -> crate::common::RegisterFieldBool<11, 1, 0, Aex_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Aex_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "IRQ0 AEM   IRQ0AEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for IRQ0 Request where this alarm        handler was blocked because of AEX.IRQ0STS. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn irq0aem(self) -> crate::common::RegisterFieldBool<16, 1, 0, Aex_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Aex_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "IRQ1 AEM   IRQ1AEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for IRQ1 Request where this alarm        handler was blocked because of AEX.IRQ1STS. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn irq1aem(self) -> crate::common::RegisterFieldBool<17, 1, 0, Aex_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Aex_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "IRQ2 AEM   IRQ2AEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for IRQ2 Request where this alarm        handler was blocked because of AEX.IRQ2STS. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn irq2aem(self) -> crate::common::RegisterFieldBool<18, 1, 0, Aex_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Aex_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RST0 AEM   RST0AEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for CPU RST0 Request where this        alarm handler was blocked because of AEX.RST0STS. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst0aem(self) -> crate::common::RegisterFieldBool<19, 1, 0, Aex_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Aex_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RST1 AEM   RST1AEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for CPU RST1 Request where this        alarm handler was blocked because of AEX.RST1STS. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst1aem(self) -> crate::common::RegisterFieldBool<20, 1, 0, Aex_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Aex_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RST2 AEM   RST2AEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for CPU RST2 Request where this        alarm handler was blocked because of AEX.RST2STS. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst2aem(self) -> crate::common::RegisterFieldBool<21, 1, 0, Aex_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Aex_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RST3 AEM   RST3AEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for CPU RST3 Request where this        alarm handler was blocked because of AEX.RST3STS. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst3aem(self) -> crate::common::RegisterFieldBool<22, 1, 0, Aex_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Aex_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RST4 AEM   RST4AEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for CPU RST4 Request where this        alarm handler was blocked because of AEX.RST4STS. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst4aem(self) -> crate::common::RegisterFieldBool<23, 1, 0, Aex_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Aex_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RST5 AEM   RST5AEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for CPU RST5 Request where this        alarm handler was blocked because of AEX.RST5STS. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst5aem(self) -> crate::common::RegisterFieldBool<24, 1, 0, Aex_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Aex_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NMI AEM   NMIAEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for NMI Request where this alarm        handler was blocked because of AEX.NMISTS. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn nmiaem(self) -> crate::common::RegisterFieldBool<25, 1, 0, Aex_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Aex_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "EMS AEM   EMSAEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for EMS Request where this alarm        handler was blocked because of AEX.EMSSTS. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn emsaem(self) -> crate::common::RegisterFieldBool<27, 1, 0, Aex_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Aex_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Aex {
    #[inline(always)]
    fn default() -> Aex {
        <crate::RegValueT<Aex_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aexclr_SPEC;
impl crate::sealed::RegSpec for Aexclr_SPEC {
    type DataType = u32;
}
#[doc = "Alarm Executed Status Clear Register\n resetvalue={Application Reset:0x0}"]
pub type Aexclr = crate::RegValueT<Aexclr_SPEC>;

impl Aexclr {
    #[doc = "IRQ0 Request Status Clear   IRQ0CLR. Read always as 0."]
    #[inline(always)]
    pub fn irq0clr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Aexclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Aexclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "IRQ1 Request Status Clear   IRQ1CLR. Read always as 0."]
    #[inline(always)]
    pub fn irq1clr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Aexclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Aexclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "IRQ2 Request Status Clear   IRQ2CLR. Read always as 0."]
    #[inline(always)]
    pub fn irq2clr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Aexclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Aexclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "RST0 Request Status Clear   RST0CLR. Read always as 0."]
    #[inline(always)]
    pub fn rst0clr(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Aexclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Aexclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "RST1 Request Status Clear   RST1CLR. Read always as 0."]
    #[inline(always)]
    pub fn rst1clr(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Aexclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Aexclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "RST2 Request Status Clear   RST2CLR. Read always as 0."]
    #[inline(always)]
    pub fn rst2clr(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Aexclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Aexclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "RST3 Request Status Clear   RST3CLR. Read always as 0."]
    #[inline(always)]
    pub fn rst3clr(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Aexclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Aexclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "RST4 Request Status Clear   RST4CLR. Read always as 0."]
    #[inline(always)]
    pub fn rst4clr(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Aexclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Aexclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "RST5 Request Status Clear   RST5CLR. Read always as 0."]
    #[inline(always)]
    pub fn rst5clr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Aexclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Aexclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "NMI Request Status Clear   NMICLR. Read always as 0."]
    #[inline(always)]
    pub fn nmiclr(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Aexclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Aexclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "EMS Request Status Clear   EMSCLR. Read always as 0."]
    #[inline(always)]
    pub fn emsclr(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Aexclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, Aexclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "IRQ0 AEM Status Clear   IRQ0AEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn irq0aemclr(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Aexclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Aexclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "IRQ1 AEM Status Clear   IRQ1AEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn irq1aemclr(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Aexclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Aexclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "IRQ2 AEM Status Clear   IRQ2AEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn irq2aemclr(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Aexclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Aexclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "RST0 AEM Status Clear   RST0AEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn rst0aemclr(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Aexclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, Aexclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "RST1 AEM Status Clear   RST1AEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn rst1aemclr(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Aexclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, Aexclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "RST2 AEM Status Clear   RST2AEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn rst2aemclr(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Aexclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, Aexclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "RST3 AEM Status Clear   RST3AEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn rst3aemclr(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Aexclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, Aexclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "RST4 AEM Status Clear   RST4AEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn rst4aemclr(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Aexclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, Aexclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "RST5 AEM Status Clear   RST5AEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn rst5aemclr(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Aexclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Aexclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "NMI AEM Status Clear   NMIAEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn nmiaemclr(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Aexclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Aexclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "EMS AEM Status Clear   EMSAEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn emsaemclr(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Aexclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, Aexclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Aexclr {
    #[inline(always)]
    fn default() -> Aexclr {
        <crate::RegValueT<Aexclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Afcnt_SPEC;
impl crate::sealed::RegSpec for Afcnt_SPEC {
    type DataType = u32;
}
#[doc = "Alarm and Fault Counter\n resetvalue={PowerOn Reset:0x0}"]
pub type Afcnt = crate::RegValueT<Afcnt_SPEC>;

impl Afcnt {
    #[doc = "Fault Counter.   FCNT. This field is incremented by hardware when the SMU core state machine        goes from the RUN state to the FAULT state  see CROSSREFERENCE  .        The counter value holds if the maximum value is reached."]
    #[inline(always)]
    pub fn fcnt(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Afcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Afcnt_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Alarm Counter.   ACNT. This field is incremented by hardware when the SMU core processes an        internal action related to an alarm event  see CROSSREFERENCE  .        The counter value holds if the maximum value is reached."]
    #[inline(always)]
    pub fn acnt(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, Afcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xfff,1,0,u16, Afcnt_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Fault Counter Overflow.   FCO. This bit is set by hardware if the FCNT counter reached the maximum        value and an increment condition is present."]
    #[inline(always)]
    pub fn fco(self) -> crate::common::RegisterFieldBool<30, 1, 0, Afcnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Afcnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm Counter Overflow.   ACO. This bit is set by hardware if the ACNT counter reached the maximum        value and an increment condition is present."]
    #[inline(always)]
    pub fn aco(self) -> crate::common::RegisterFieldBool<31, 1, 0, Afcnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Afcnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Afcnt {
    #[inline(always)]
    fn default() -> Afcnt {
        <crate::RegValueT<Afcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agc_SPEC;
impl crate::sealed::RegSpec for Agc_SPEC {
    type DataType = u32;
}
#[doc = "Alarm Global Configuration\n resetvalue={Application Reset:0x0}"]
pub type Agc = crate::RegValueT<Agc_SPEC>;

impl Agc {
    #[doc = "Interrupt Generation Configuration Set 0   IGCS0. Defines the output value of the interrupt request vector when the alarm        configuration flag selects the interrupt configuration set 0. Enables to        issue an interrupt request to several CPUs  see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn igcs0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Agc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0x7, 1, 0, u8, Agc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Generation Configuration Set 1   IGCS1. Defines the output value of the interrupt request vector when the alarm        configuration flag selects the interrupt configuration set 1. Enables to        issue an interrupt request to several CPUs  see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn igcs1(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, Agc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4, 0x7, 1, 0, u8, Agc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Generation Configuration Set 2   IGCS2. Defines the output value of the interrupt request vector when the alarm        configuration flag selects the interrupt configuration set 2. Enables to        issue an interrupt request to several CPUs  see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn igcs2(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Agc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8, 0x7, 1, 0, u8, Agc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU Reset Configuration Set   RCS. Defines the output value of the CPU reset request vector when the alarm configuration flag selects the CPU Reset Configuration Set. Enables to issue an reset request to several CPUs if required. More complex reset scenarios can be handled by using software interrupts. Setting the bit n to 1 enables issuing a reset request to CPUn."]
    #[inline(always)]
    pub fn rcs(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Agc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Agc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Emergency Stop   PES. This field enables control of the Port Emergency Stop  PES  feature        independently for each internal action. When an action is triggered and        if the corresponding bit  as defined below  is set  the hardware        triggers automatically a port emergency stop request. Each bit of PES is        allocated to an action as follows"]
    #[inline(always)]
    pub fn pes(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, Agc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, Agc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable FAULT to RUN State Transition   EFRST. See CROSSREFERENCE chapter for the usage of this field."]
    #[inline(always)]
    pub fn efrst(self) -> crate::common::RegisterFieldBool<29, 1, 0, Agc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Agc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Agc {
    #[inline(always)]
    fn default() -> Agc {
        <crate::RegValueT<Agc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AGi_SPEC;
impl crate::sealed::RegSpec for AGi_SPEC {
    type DataType = u32;
}
#[doc = "Alarm Status Register\n resetvalue={Application Reset:0x0}"]
pub type AGi = crate::RegValueT<AGi_SPEC>;

impl AGi {
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf0(self) -> crate::common::RegisterFieldBool<0, 1, 0, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, AGi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf1(self) -> crate::common::RegisterFieldBool<1, 1, 0, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, AGi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf2(self) -> crate::common::RegisterFieldBool<2, 1, 0, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, AGi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf4(self) -> crate::common::RegisterFieldBool<4, 1, 0, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, AGi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf5(self) -> crate::common::RegisterFieldBool<5, 1, 0, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, AGi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf6(self) -> crate::common::RegisterFieldBool<6, 1, 0, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, AGi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf7(self) -> crate::common::RegisterFieldBool<7, 1, 0, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, AGi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf8(self) -> crate::common::RegisterFieldBool<8, 1, 0, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, AGi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf9(self) -> crate::common::RegisterFieldBool<9, 1, 0, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, AGi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf10(self) -> crate::common::RegisterFieldBool<10, 1, 0, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, AGi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf11(self) -> crate::common::RegisterFieldBool<11, 1, 0, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, AGi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf12(self) -> crate::common::RegisterFieldBool<12, 1, 0, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, AGi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf13(self) -> crate::common::RegisterFieldBool<13, 1, 0, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, AGi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf14(self) -> crate::common::RegisterFieldBool<14, 1, 0, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, AGi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf22(self) -> crate::common::RegisterFieldBool<22, 1, 0, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, AGi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf23(self) -> crate::common::RegisterFieldBool<23, 1, 0, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, AGi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf24(self) -> crate::common::RegisterFieldBool<24, 1, 0, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, AGi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for AGi {
    #[inline(always)]
    fn default() -> AGi {
        <crate::RegValueT<AGi_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AGiFsp_SPEC;
impl crate::sealed::RegSpec for AGiFsp_SPEC {
    type DataType = u32;
}
#[doc = "SMU core FSP Configuration Register\n resetvalue={Application Reset:0x0,Application Reset:0x30000}"]
pub type AGiFsp = crate::RegValueT<AGiFsp_SPEC>;

impl AGiFsp {
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe0(self) -> crate::common::RegisterFieldBool<0, 1, 0, AGiFsp_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, AGiFsp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe1(self) -> crate::common::RegisterFieldBool<1, 1, 0, AGiFsp_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, AGiFsp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe2(self) -> crate::common::RegisterFieldBool<2, 1, 0, AGiFsp_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, AGiFsp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe4(self) -> crate::common::RegisterFieldBool<4, 1, 0, AGiFsp_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, AGiFsp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe5(self) -> crate::common::RegisterFieldBool<5, 1, 0, AGiFsp_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, AGiFsp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe6(self) -> crate::common::RegisterFieldBool<6, 1, 0, AGiFsp_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, AGiFsp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe7(self) -> crate::common::RegisterFieldBool<7, 1, 0, AGiFsp_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, AGiFsp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe8(self) -> crate::common::RegisterFieldBool<8, 1, 0, AGiFsp_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, AGiFsp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe9(self) -> crate::common::RegisterFieldBool<9, 1, 0, AGiFsp_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, AGiFsp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, AGiFsp_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, AGiFsp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, AGiFsp_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, AGiFsp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, AGiFsp_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, AGiFsp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, AGiFsp_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, AGiFsp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, AGiFsp_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, AGiFsp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, AGiFsp_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, AGiFsp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, AGiFsp_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, AGiFsp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, AGiFsp_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, AGiFsp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for AGiFsp {
    #[inline(always)]
    fn default() -> AGiFsp {
        <crate::RegValueT<AGiFsp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clc_SPEC;
impl crate::sealed::RegSpec for Clc_SPEC {
    type DataType = u32;
}
#[doc = "Clock Control Register\n resetvalue={Application Reset:0x0}"]
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
    #[doc = "Sleep Mode Enable Control   EDIS. Used to control module  8217 s sleep mode. Sleep Mode is not supported by the safety applications. During the        process of entering and resuming from sleep mode  the intended        processing of alarm events is not guaranteed."]
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
        <crate::RegValueT<Clc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd_SPEC;
impl crate::sealed::RegSpec for Cmd_SPEC {
    type DataType = u32;
}
#[doc = "Command Register\n resetvalue={Application Reset:0x0}"]
pub type Cmd = crate::RegValueT<Cmd_SPEC>;

impl Cmd {
    #[doc = "Implements the SMU core Command Interface.   CMD. See CROSSREFERENCE for the command encoding. Read as 0."]
    #[inline(always)]
    pub fn cmd(self) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Cmd_SPEC, crate::common::W> {
        crate::common::RegisterField::<0, 0xf, 1, 0, u8, Cmd_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Implements the SMU core Command Interface.   ARG. Argument to be used with the command. See CROSSREFERENCE for the argument encoding. Read as 0."]
    #[inline(always)]
    pub fn arg(self) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Cmd_SPEC, crate::common::W> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, Cmd_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Cmd {
    #[inline(always)]
    fn default() -> Cmd {
        <crate::RegValueT<Cmd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbg_SPEC;
impl crate::sealed::RegSpec for Dbg_SPEC {
    type DataType = u32;
}
#[doc = "Debug Register\n resetvalue={PowerOn Reset:0x0}"]
pub type Dbg = crate::RegValueT<Dbg_SPEC>;

impl Dbg {
    #[doc = "Running state of the SMU core State Machine   SSM"]
    #[inline(always)]
    pub fn ssm(self) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Dbg_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0x3, 1, 0, u8, Dbg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Dbg {
    #[inline(always)]
    fn default() -> Dbg {
        <crate::RegValueT<Dbg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsp_SPEC;
impl crate::sealed::RegSpec for Fsp_SPEC {
    type DataType = u32;
}
#[doc = "Fault Signaling Protocol\n resetvalue={PowerOn Reset:0x3FFF00}"]
pub type Fsp = crate::RegValueT<Fsp_SPEC>;

impl Fsp {
    #[doc = "Prescaler1   PRE1. Dividing factor to apply to the reference clock fBACK. It is assumed that the maximal value for fBACK is 100 MHz with a precision of 5 . The divided clock is used as reference to generate the timing of the fault signaling protocol fault state. It is only allowed to write PRE1 when the SMU is not in the Fault State and FSP is in Bi stable protocol mode. Also  it is not allowed to write to the PRE1 when at least one recovery timer is running. The frequency of the divided clock  called FSMU FS  is defined as follows"]
    #[inline(always)]
    pub fn pre1(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Fsp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0x7, 1, 0, u8, Fsp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Prescaler2   PRE2. Dividing factor to apply to the reference clock fBACK in order to generate the timing of the fault free state for the dynamic dual rail and time switching modes of the fault signaling protocol. It is only allowed to write PPRE2 when the SMU is not in the Fault State and FSP is in Bi stable protocol mode. The frequency of the divided clock  called FSMU FFS  is defined as follows"]
    #[inline(always)]
    pub fn pre2(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, Fsp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3, 0x3, 1, 0, u8, Fsp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fault Signaling Protocol configuration   MODE"]
    #[inline(always)]
    pub fn mode(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, Fsp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5, 0x3, 1, 0, u8, Fsp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Port Emergency Stop  PES    PES. When this bit is set a Port Emergency Stop is automatically requested        when an alarm event configured to start the Fault Signaling Protocol is        detected."]
    #[inline(always)]
    pub fn pes(self) -> crate::common::RegisterFieldBool<7, 1, 0, Fsp_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Fsp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Specifies the FSP fault state duration   TFSP LOW. TFSP FS TSMU FS    concatenate TFSP HIGH TFSP LOW    1 . TFSP LOW shall be specified as a number of FSMU FS ticks. TFSP LOW is defined so that the minimum duration is greater than 250 us. It can not be changed by software."]
    #[inline(always)]
    pub fn tfsp_low(
        self,
    ) -> crate::common::RegisterField<8, 0x3fff, 1, 0, u16, Fsp_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3fff,1,0,u16, Fsp_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Specifies the FSP fault state duration   TFSP HIGH. TFSP FS  TSMU FS    concatenate TFSP HIGH TFSP LOW    1 . TFSP HIGH shall be specified as a number of FSMU FS ticks. TFSP HIGH and PRE1 shall enable to configure a fault state duration of 500 ms. It is only allowed to write TFSP HIGH when the SMU is not in the Fault State and FSP is in Bi stable protocol mode."]
    #[inline(always)]
    pub fn tfsp_high(
        self,
    ) -> crate::common::RegisterField<22, 0x3ff, 1, 0, u16, Fsp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3ff,1,0,u16, Fsp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Fsp {
    #[inline(always)]
    fn default() -> Fsp {
        <crate::RegValueT<Fsp_SPEC> as RegisterValue<_>>::new(4194048)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x089C001}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MOD REV. This bit field defines the module revision number. The value of a module        revision starts with 01H  first revision ."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MOD TYPE. The bit field is set to C0H which defines the module as a 32 bit module."]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MOD NUMBER. This bit field defines a module identification number. The value for the SMU module is 0089H."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(9027585)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keys_SPEC;
impl crate::sealed::RegSpec for Keys_SPEC {
    type DataType = u32;
}
#[doc = "Key Register\n resetvalue={Application Reset:0x0}"]
pub type Keys = crate::RegValueT<Keys_SPEC>;

impl Keys {
    #[doc = "Configuration Lock   CFGLCK. The SMU core configuration is only possible if this field is set to        0xBC. Refer to CROSSREFERENCE for the list of registers controlled by this field."]
    #[inline(always)]
    pub fn cfglck(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Keys_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Keys_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Permanent Lock   PERLCK. If this field is set to 0xFF  no further configuration of the SMU core        is possible. Refer to CROSSREFERENCE for the list of registers controlled by this field."]
    #[inline(always)]
    pub fn perlck(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Keys_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Keys_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Keys {
    #[inline(always)]
    fn default() -> Keys {
        <crate::RegValueT<Keys_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Suspend State   SUSSTA. Read as 0  must be written with 0."]
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
pub struct Pctl_SPEC;
impl crate::sealed::RegSpec for Pctl_SPEC {
    type DataType = u32;
}
#[doc = "Port Control\n resetvalue={PowerOn Reset:0x08000}"]
pub type Pctl = crate::RegValueT<Pctl_SPEC>;

impl Pctl {
    #[doc = "Port Direction.   HWDIR. This bitfield directly controls the value of the FSP DIR output signal. Also        refer to the General Purpose I O Ports chapter for the HW DIR signal        specification."]
    #[inline(always)]
    pub fn hwdir(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Pctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Pctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Enable   HWEN. This bitfied directly controls the value of the FSP EN output signal. When        set to 11b the port output is directly driven by SMU core  FSP 1 0  . Also        refer to the General Purpose I O Ports chapter for the HW EN signal        specification."]
    #[inline(always)]
    pub fn hwen(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Pctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Pctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Glitch Filter for ErrorPin SMU FSP0 to SCU enable   GFSCU EN"]
    #[inline(always)]
    pub fn gfscu_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Pctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Glitch Filter for ErrorPin SMU FSP0 to register SMU STS enable   GFSTS EN"]
    #[inline(always)]
    pub fn gfsts_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Pctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Pctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PAD Configuration Select   PCS. This bit controls the latching of the SMU core FSP  Error Pin  PAD        configuration signals to ensure that upon an application reset or system        reset the SMU core FSP  Error Pin  PAD configuration is not affected.        This field is only reset by power on reset. Only with the first transition from 0 to 1 of          this field the SMU core FSP is operational. Any further configuration          change in this bit field has no effect to the hardware . The fields HWDIR  HWEN and PCS shall be          configured with a single software write command. Configuring each          bit field separately may lead to configuration inconsistencies. Refer          to CROSSREFERENCE for the overview of the SMU core FSP  Error Pin  connectivity. The Error Pin Pad shall be configured to the          targeted function in the Port control logic before the SMU core takes          over the control."]
    #[inline(always)]
    pub fn pcs(self) -> crate::common::RegisterFieldBool<7, 1, 0, Pctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Pctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Pctl {
    #[inline(always)]
    fn default() -> Pctl {
        <crate::RegValueT<Pctl_SPEC> as RegisterValue<_>>::new(32768)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmctl_SPEC;
impl crate::sealed::RegSpec for Rmctl_SPEC {
    type DataType = u32;
}
#[doc = "Register Monitor Control\n resetvalue={Application Reset:0x0}"]
pub type Rmctl = crate::RegValueT<Rmctl_SPEC>;

impl Rmctl {
    #[doc = "Test Enable.   TE31. This bit controls the test of the of the register monitor safety mechanism. Setting this bit starts the selft test of the safety flip flops in the corresponding module."]
    #[inline(always)]
    pub fn te0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Rmctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Rmctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Test Enable.   TE31. This bit controls the test of the of the register monitor safety mechanism. Setting this bit starts the selft test of the safety flip flops in the corresponding module."]
    #[inline(always)]
    pub fn te1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Rmctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Rmctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Test Enable.   TE31. This bit controls the test of the of the register monitor safety mechanism. Setting this bit starts the selft test of the safety flip flops in the corresponding module."]
    #[inline(always)]
    pub fn te2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Rmctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Rmctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Test Enable.   TE31. This bit controls the test of the of the register monitor safety mechanism. Setting this bit starts the selft test of the safety flip flops in the corresponding module."]
    #[inline(always)]
    pub fn te3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Rmctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Rmctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Test Enable.   TE31. This bit controls the test of the of the register monitor safety mechanism. Setting this bit starts the selft test of the safety flip flops in the corresponding module."]
    #[inline(always)]
    pub fn te4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Rmctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Rmctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Test Enable.   TE31. This bit controls the test of the of the register monitor safety mechanism. Setting this bit starts the selft test of the safety flip flops in the corresponding module."]
    #[inline(always)]
    pub fn te5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Rmctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Rmctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Test Enable.   TE31. This bit controls the test of the of the register monitor safety mechanism. Setting this bit starts the selft test of the safety flip flops in the corresponding module."]
    #[inline(always)]
    pub fn te6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Rmctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Rmctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Test Enable.   TE31. This bit controls the test of the of the register monitor safety mechanism. Setting this bit starts the selft test of the safety flip flops in the corresponding module."]
    #[inline(always)]
    pub fn te7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Rmctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Rmctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Test Enable.   TE31. This bit controls the test of the of the register monitor safety mechanism. Setting this bit starts the selft test of the safety flip flops in the corresponding module."]
    #[inline(always)]
    pub fn te8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Rmctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Rmctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Test Enable.   TE31. This bit controls the test of the of the register monitor safety mechanism. Setting this bit starts the selft test of the safety flip flops in the corresponding module."]
    #[inline(always)]
    pub fn te9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Rmctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Rmctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Test Enable.   TE31. This bit controls the test of the of the register monitor safety mechanism. Setting this bit starts the selft test of the safety flip flops in the corresponding module."]
    #[inline(always)]
    pub fn te10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Rmctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Rmctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Rmctl {
    #[inline(always)]
    fn default() -> Rmctl {
        <crate::RegValueT<Rmctl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmef_SPEC;
impl crate::sealed::RegSpec for Rmef_SPEC {
    type DataType = u32;
}
#[doc = "Register Monitor Error Flags\n resetvalue={Application Reset:0x0}"]
pub type Rmef = crate::RegValueT<Rmef_SPEC>;

impl Rmef {
    #[doc = "Status flag related to the different instances of the register monitor safety mechanism.   EF31. It reports a real flip flop failure in non test mode as well as an        unexpected behavior in test mode. This flag can only be cleared by software  a set by software has no        effect"]
    #[inline(always)]
    pub fn ef0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Rmef_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Rmef_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag related to the different instances of the register monitor safety mechanism.   EF31. It reports a real flip flop failure in non test mode as well as an        unexpected behavior in test mode. This flag can only be cleared by software  a set by software has no        effect"]
    #[inline(always)]
    pub fn ef1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Rmef_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Rmef_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag related to the different instances of the register monitor safety mechanism.   EF31. It reports a real flip flop failure in non test mode as well as an        unexpected behavior in test mode. This flag can only be cleared by software  a set by software has no        effect"]
    #[inline(always)]
    pub fn ef2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Rmef_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Rmef_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag related to the different instances of the register monitor safety mechanism.   EF31. It reports a real flip flop failure in non test mode as well as an        unexpected behavior in test mode. This flag can only be cleared by software  a set by software has no        effect"]
    #[inline(always)]
    pub fn ef3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Rmef_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Rmef_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag related to the different instances of the register monitor safety mechanism.   EF31. It reports a real flip flop failure in non test mode as well as an        unexpected behavior in test mode. This flag can only be cleared by software  a set by software has no        effect"]
    #[inline(always)]
    pub fn ef4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Rmef_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Rmef_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag related to the different instances of the register monitor safety mechanism.   EF31. It reports a real flip flop failure in non test mode as well as an        unexpected behavior in test mode. This flag can only be cleared by software  a set by software has no        effect"]
    #[inline(always)]
    pub fn ef5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Rmef_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Rmef_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag related to the different instances of the register monitor safety mechanism.   EF31. It reports a real flip flop failure in non test mode as well as an        unexpected behavior in test mode. This flag can only be cleared by software  a set by software has no        effect"]
    #[inline(always)]
    pub fn ef6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Rmef_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Rmef_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag related to the different instances of the register monitor safety mechanism.   EF31. It reports a real flip flop failure in non test mode as well as an        unexpected behavior in test mode. This flag can only be cleared by software  a set by software has no        effect"]
    #[inline(always)]
    pub fn ef7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Rmef_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Rmef_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag related to the different instances of the register monitor safety mechanism.   EF31. It reports a real flip flop failure in non test mode as well as an        unexpected behavior in test mode. This flag can only be cleared by software  a set by software has no        effect"]
    #[inline(always)]
    pub fn ef8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Rmef_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Rmef_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag related to the different instances of the register monitor safety mechanism.   EF31. It reports a real flip flop failure in non test mode as well as an        unexpected behavior in test mode. This flag can only be cleared by software  a set by software has no        effect"]
    #[inline(always)]
    pub fn ef9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Rmef_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Rmef_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status flag related to the different instances of the register monitor safety mechanism.   EF31. It reports a real flip flop failure in non test mode as well as an        unexpected behavior in test mode. This flag can only be cleared by software  a set by software has no        effect"]
    #[inline(always)]
    pub fn ef10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Rmef_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Rmef_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Rmef {
    #[inline(always)]
    fn default() -> Rmef {
        <crate::RegValueT<Rmef_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmsts_SPEC;
impl crate::sealed::RegSpec for Rmsts_SPEC {
    type DataType = u32;
}
#[doc = "Register Monitor Self Test Status\n resetvalue={Application Reset:0x0}"]
pub type Rmsts = crate::RegValueT<Rmsts_SPEC>;

impl Rmsts {
    #[doc = "Ready flag related to the different instances of the register monitor safety mechanism.   STS31. A logical   8216 1  8217  of this bit indicates that the register monitor test has        been executed. This bit can only be cleared by software  a set by        software has no effect."]
    #[inline(always)]
    pub fn sts0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Rmsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Rmsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Ready flag related to the different instances of the register monitor safety mechanism.   STS31. A logical   8216 1  8217  of this bit indicates that the register monitor test has        been executed. This bit can only be cleared by software  a set by        software has no effect."]
    #[inline(always)]
    pub fn sts1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Rmsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Rmsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Ready flag related to the different instances of the register monitor safety mechanism.   STS31. A logical   8216 1  8217  of this bit indicates that the register monitor test has        been executed. This bit can only be cleared by software  a set by        software has no effect."]
    #[inline(always)]
    pub fn sts2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Rmsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Rmsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Ready flag related to the different instances of the register monitor safety mechanism.   STS31. A logical   8216 1  8217  of this bit indicates that the register monitor test has        been executed. This bit can only be cleared by software  a set by        software has no effect."]
    #[inline(always)]
    pub fn sts3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Rmsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Rmsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Ready flag related to the different instances of the register monitor safety mechanism.   STS31. A logical   8216 1  8217  of this bit indicates that the register monitor test has        been executed. This bit can only be cleared by software  a set by        software has no effect."]
    #[inline(always)]
    pub fn sts4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Rmsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Rmsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Ready flag related to the different instances of the register monitor safety mechanism.   STS31. A logical   8216 1  8217  of this bit indicates that the register monitor test has        been executed. This bit can only be cleared by software  a set by        software has no effect."]
    #[inline(always)]
    pub fn sts5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Rmsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Rmsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Ready flag related to the different instances of the register monitor safety mechanism.   STS31. A logical   8216 1  8217  of this bit indicates that the register monitor test has        been executed. This bit can only be cleared by software  a set by        software has no effect."]
    #[inline(always)]
    pub fn sts6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Rmsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Rmsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Ready flag related to the different instances of the register monitor safety mechanism.   STS31. A logical   8216 1  8217  of this bit indicates that the register monitor test has        been executed. This bit can only be cleared by software  a set by        software has no effect."]
    #[inline(always)]
    pub fn sts7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Rmsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Rmsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Ready flag related to the different instances of the register monitor safety mechanism.   STS31. A logical   8216 1  8217  of this bit indicates that the register monitor test has        been executed. This bit can only be cleared by software  a set by        software has no effect."]
    #[inline(always)]
    pub fn sts8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Rmsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Rmsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Ready flag related to the different instances of the register monitor safety mechanism.   STS31. A logical   8216 1  8217  of this bit indicates that the register monitor test has        been executed. This bit can only be cleared by software  a set by        software has no effect."]
    #[inline(always)]
    pub fn sts9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Rmsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Rmsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Ready flag related to the different instances of the register monitor safety mechanism.   STS31. A logical   8216 1  8217  of this bit indicates that the register monitor test has        been executed. This bit can only be cleared by software  a set by        software has no effect."]
    #[inline(always)]
    pub fn sts10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Rmsts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Rmsts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Rmsts {
    #[inline(always)]
    fn default() -> Rmsts {
        <crate::RegValueT<Rmsts_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtac00_SPEC;
impl crate::sealed::RegSpec for Rtac00_SPEC {
    type DataType = u32;
}
#[doc = "Recovery Timer 0 Alarm Configuration 0\n resetvalue={Application Reset:0x0A80108}"]
pub type Rtac00 = crate::RegValueT<Rtac00_SPEC>;

impl Rtac00 {
    #[doc = "Group Index 0.   GID0. This field enables to specify if an alarm from this alarm group can use        the recovery timer 0. The functionality of this field is described in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn gid0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Rtac00_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Rtac00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Identifier 0.   ALID0. This field specifies the alarm index related to the group index        specified in GID0."]
    #[inline(always)]
    pub fn alid0(
        self,
    ) -> crate::common::RegisterField<4, 0x1f, 1, 0, u8, Rtac00_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1f,1,0,u8, Rtac00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Group Index 1.   GID1. This field enables to specify if an alarm from this alarm group can use        the recovery timer 0. The functionality of this field is described in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn gid1(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Rtac00_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Rtac00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Identifier 1.   ALID1. This field specifies the alarm index related to the group index        specified in GID1."]
    #[inline(always)]
    pub fn alid1(
        self,
    ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, Rtac00_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x1f,1,0,u8, Rtac00_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Rtac00 {
    #[inline(always)]
    fn default() -> Rtac00 {
        <crate::RegValueT<Rtac00_SPEC> as RegisterValue<_>>::new(11010312)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtac01_SPEC;
impl crate::sealed::RegSpec for Rtac01_SPEC {
    type DataType = u32;
}
#[doc = "Recovery Timer 0 Alarm Configuration 1\n resetvalue={Application Reset:0x0C800B8}"]
pub type Rtac01 = crate::RegValueT<Rtac01_SPEC>;

impl Rtac01 {
    #[doc = "Group Index 2.   GID2. This field enables to specify if an alarm from this alarm group can use        the recovery timer 0. The functionality of this field is described in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn gid2(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Rtac01_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Rtac01_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Identifier 0.   ALID2. This field specifies the alarm index related to the group index        specified in GID2."]
    #[inline(always)]
    pub fn alid2(
        self,
    ) -> crate::common::RegisterField<4, 0x1f, 1, 0, u8, Rtac01_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1f,1,0,u8, Rtac01_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Group Index 3.   GID3. This field enables to specify if an alarm from this alarm group can use        the recovery timer 3. The functionality of this field is described in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn gid3(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Rtac01_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Rtac01_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Identifier 1.   ALID3. This field specifies the alarm index related to the group index        specified in GID3."]
    #[inline(always)]
    pub fn alid3(
        self,
    ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, Rtac01_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x1f,1,0,u8, Rtac01_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Rtac01 {
    #[inline(always)]
    fn default() -> Rtac01 {
        <crate::RegValueT<Rtac01_SPEC> as RegisterValue<_>>::new(13107384)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtac10_SPEC;
impl crate::sealed::RegSpec for Rtac10_SPEC {
    type DataType = u32;
}
#[doc = "Recovery Timer 1 Alarm Configuration 0\n resetvalue={Application Reset:0x0E800D8}"]
pub type Rtac10 = crate::RegValueT<Rtac10_SPEC>;

impl Rtac10 {
    #[doc = "Group Index 0.   GID0. This field enables to specify if an alarm from this alarm group can use the recovery timer 1. The functionality of this field is described in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn gid0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Rtac10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Rtac10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Identifier 0.   ALID0. This field specifies the alarm index related to the group index specified in GID0."]
    #[inline(always)]
    pub fn alid0(
        self,
    ) -> crate::common::RegisterField<4, 0x1f, 1, 0, u8, Rtac10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1f,1,0,u8, Rtac10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Group Index 1.   GID1. This field enables to specify if an alarm from this alarm group can use the recovery timer 1. The functionality of this field is described in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn gid1(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Rtac10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Rtac10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Identifier 1.   ALID1. This field specifies the alarm index related to the group index specified in GID1."]
    #[inline(always)]
    pub fn alid1(
        self,
    ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, Rtac10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x1f,1,0,u8, Rtac10_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Rtac10 {
    #[inline(always)]
    fn default() -> Rtac10 {
        <crate::RegValueT<Rtac10_SPEC> as RegisterValue<_>>::new(15204568)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtac11_SPEC;
impl crate::sealed::RegSpec for Rtac11_SPEC {
    type DataType = u32;
}
#[doc = "Recovery Timer 1 Alarm Configuration 1\n resetvalue={Application Reset:0x0F800F8}"]
pub type Rtac11 = crate::RegValueT<Rtac11_SPEC>;

impl Rtac11 {
    #[doc = "Group Index 2.   GID2. This field enables to specify if an alarm from this alarm group can use the recovery timer 1. The functionality of this field is described in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn gid2(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Rtac11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Rtac11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Identifier 2.   ALID2. This field specifies the alarm index related to the group index specified in GID2."]
    #[inline(always)]
    pub fn alid2(
        self,
    ) -> crate::common::RegisterField<4, 0x1f, 1, 0, u8, Rtac11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1f,1,0,u8, Rtac11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Group Index 3.   GID3. This field enables to specify if an alarm from this alarm group can use the recovery timer 1. The functionality of this field is described in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn gid3(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Rtac11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Rtac11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Identifier 3.   ALID3. This field specifies the alarm index related to the group index specified in GID3."]
    #[inline(always)]
    pub fn alid3(
        self,
    ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, Rtac11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x1f,1,0,u8, Rtac11_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Rtac11 {
    #[inline(always)]
    fn default() -> Rtac11 {
        <crate::RegValueT<Rtac11_SPEC> as RegisterValue<_>>::new(16253176)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc_SPEC;
impl crate::sealed::RegSpec for Rtc_SPEC {
    type DataType = u32;
}
#[doc = "Recovery Timer Configuration\n resetvalue={Application Reset:0x3FFF03}"]
pub type Rtc = crate::RegValueT<Rtc_SPEC>;

impl Rtc {
    #[doc = "RT0 Enable Bit   RT0E"]
    #[inline(always)]
    pub fn rt0e(self) -> crate::common::RegisterFieldBool<0, 1, 0, Rtc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Rtc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "RT1 Enable Bit   RT1E"]
    #[inline(always)]
    pub fn rt1e(self) -> crate::common::RegisterFieldBool<1, 1, 0, Rtc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Rtc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Recovery Timer Duration   RTD. This field specifies the maximum duration of the recovery timer. When the timer counter reaches the programmed value  the internal alarm rt timeout is issued. The timer is stopped by a SMU RTStop   command before the recovery timer. RTD shall be specified as a number of the FSMU FS clock ticks. It is not allowed to write to the RTD when at least one recovery timer is running."]
    #[inline(always)]
    pub fn rtd(
        self,
    ) -> crate::common::RegisterField<8, 0xffffff, 1, 0, u32, Rtc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xffffff,1,0,u32, Rtc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Rtc {
    #[inline(always)]
    fn default() -> Rtc {
        <crate::RegValueT<Rtc_SPEC> as RegisterValue<_>>::new(4194051)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts_SPEC;
impl crate::sealed::RegSpec for Sts_SPEC {
    type DataType = u32;
}
#[doc = "Status Register\n resetvalue={Application Reset:0x0}"]
pub type Sts = crate::RegValueT<Sts_SPEC>;

impl Sts {
    #[doc = "Last command received   CMD. Same encoding as CMD field of SMU CMD register"]
    #[inline(always)]
    pub fn cmd(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Sts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0xf, 1, 0, u8, Sts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Last command argument received   ARG. Same encoding as ARG field of SMU CMD register"]
    #[inline(always)]
    pub fn arg(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Sts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, Sts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Result of last received command   RES"]
    #[inline(always)]
    pub fn res(self) -> crate::common::RegisterFieldBool<8, 1, 0, Sts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Sts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm Status Clear Enable   ASCE. This bit controls if a status flag set in an AG lt x gt  register upon        detection of an alarm event can be cleared by software or not. When ASCE is enabled software shall write a 1 to the bit position in        AG lt x gt  to clear the bit  W1C . When a W1C action takes place the ASCE bit        is automatically cleared to 0 by hardware and software shall set the        ASCE bit again by using the SMU ASCE   command."]
    #[inline(always)]
    pub fn asce(self) -> crate::common::RegisterFieldBool<9, 1, 0, Sts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Sts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fault Signaling Protocol status   FSP. FSP 0    FSP STS 0  input signal FSP 1    FSP STS 1  input signal This field is updated by hardware every clock cycle  therefore a software clear on write is not meaningful for this field. Note  When the FSP 0  and or FSP 1  is set in fault state by the SMU stdby this bitfield does not reflect the actual state of the ErrorPins. Indeed  the SMU stdby sets the ErrorPins in high impedence state to indicate the presence of a fault."]
    #[inline(always)]
    pub fn fsp(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Sts_SPEC, crate::common::R> {
        crate::common::RegisterField::<10, 0x3, 1, 0, u8, Sts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Fault State Timing Status   FSTS. This bit indicates if the minimum timing duration of the FSP fault state        has been reached or not. The bit is cleared by hardware when the fault        state is entered."]
    #[inline(always)]
    pub fn fsts(self) -> crate::common::RegisterFieldBool<12, 1, 0, Sts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Sts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Recovery Timer 0 Status   RTS0. See CROSSREFERENCE for the usage of this field."]
    #[inline(always)]
    pub fn rts0(self) -> crate::common::RegisterFieldBool<16, 1, 0, Sts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Sts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Recovery Timer 0 Missed Event   RTME0. See CROSSREFERENCE for the usage of this field."]
    #[inline(always)]
    pub fn rtme0(self) -> crate::common::RegisterFieldBool<17, 1, 0, Sts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Sts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Recovery Timer 1 Status   RTS1. See CROSSREFERENCE for the usage of this field."]
    #[inline(always)]
    pub fn rts1(self) -> crate::common::RegisterFieldBool<18, 1, 0, Sts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Sts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Recovery Timer 1 Missed Event   RTME1. See CROSSREFERENCE for the usage of this field."]
    #[inline(always)]
    pub fn rtme1(self) -> crate::common::RegisterFieldBool<19, 1, 0, Sts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Sts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Sts {
    #[inline(always)]
    fn default() -> Sts {
        <crate::RegValueT<Sts_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc = "AGRP"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AGiCFj(pub(super) *mut u8);
unsafe impl core::marker::Send for AGiCFj {}
unsafe impl core::marker::Sync for AGiCFj {}
impl AGiCFj {
    #[doc = "Alarm Configuration Register\n resetvalue={Application Reset:0x0,Application Reset:0x1FC00,Application Reset:0x30000}"]
    #[inline(always)]
    pub const fn agicfj_(&self) -> [crate::common::Reg<agicfj::AGiCFj_SPEC, crate::common::RW>; 3] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x8usize)),
            ]
        }
    }
}
pub mod agicfj {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AGiCFj_SPEC;
    impl crate::sealed::RegSpec for AGiCFj_SPEC {
        type DataType = u32;
    }
    #[doc = "Alarm Configuration Register\n resetvalue={Application Reset:0x0,Application Reset:0x1FC00,Application Reset:0x30000}"]
    pub type AGiCFj = crate::RegValueT<AGiCFj_SPEC>;

    impl AGiCFj {
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group."]
        #[inline(always)]
        pub fn cf0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, AGiCFj_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group."]
        #[inline(always)]
        pub fn cf1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, AGiCFj_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group."]
        #[inline(always)]
        pub fn cf2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, AGiCFj_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group."]
        #[inline(always)]
        pub fn cf4(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, AGiCFj_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group."]
        #[inline(always)]
        pub fn cf5(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, AGiCFj_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group."]
        #[inline(always)]
        pub fn cf6(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, AGiCFj_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group."]
        #[inline(always)]
        pub fn cf7(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, AGiCFj_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group."]
        #[inline(always)]
        pub fn cf8(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, AGiCFj_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group."]
        #[inline(always)]
        pub fn cf9(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, AGiCFj_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9,1,0,AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group."]
        #[inline(always)]
        pub fn cf10(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, AGiCFj_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group."]
        #[inline(always)]
        pub fn cf11(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, AGiCFj_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<11,1,0,AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group."]
        #[inline(always)]
        pub fn cf12(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, AGiCFj_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12,1,0,AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group."]
        #[inline(always)]
        pub fn cf13(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, AGiCFj_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13,1,0,AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group."]
        #[inline(always)]
        pub fn cf14(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, AGiCFj_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group."]
        #[inline(always)]
        pub fn cf22(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, AGiCFj_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<22,1,0,AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group."]
        #[inline(always)]
        pub fn cf23(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, AGiCFj_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<23,1,0,AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group."]
        #[inline(always)]
        pub fn cf24(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, AGiCFj_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<24,1,0,AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for AGiCFj {
        #[inline(always)]
        fn default() -> AGiCFj {
            <crate::RegValueT<AGiCFj_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
