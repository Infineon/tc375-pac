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
#[doc = r"General Purpose I O Ports and Peripheral I O Lines"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P00(pub(super) *mut u8);
unsafe impl core::marker::Send for P00 {}
unsafe impl core::marker::Sync for P00 {}
impl P00 {
    #[doc = "Port 00 Access Enable Register 0\n resetvalue={:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "Port 00 Emergency Stop Register\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn esr(&self) -> crate::common::Reg<self::Esr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }

    #[doc = "Port 00 Identification Register\n resetvalue={:0x0C8C000}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Port 00 Input Register\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn r#in(&self) -> crate::common::Reg<self::In_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "Port 00 Input Output Control Register 0\n resetvalue={:0x0,:0x0,:0x10101010,:0x10101010}"]
    #[inline(always)]
    pub const fn iocr0(&self) -> crate::common::Reg<self::Iocr0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "Port 00 Input Output Control Register 12\n resetvalue={:0x0,:0x0,:0x10101010,:0x10101010}"]
    #[inline(always)]
    pub const fn iocr12(&self) -> crate::common::Reg<self::Iocr12_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Port 00 Input Output Control Register 4\n resetvalue={:0x0,:0x0,:0x10101010,:0x10101010}"]
    #[inline(always)]
    pub const fn iocr4(&self) -> crate::common::Reg<self::Iocr4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Port 00 Input Output Control Register 8\n resetvalue={:0x0,:0x0,:0x10101010,:0x10101010}"]
    #[inline(always)]
    pub const fn iocr8(&self) -> crate::common::Reg<self::Iocr8_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "Port 00 Output Modification Clear Register\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn omcr(&self) -> crate::common::Reg<self::Omcr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }

    #[doc = "Port 00 Output Modification Clear Register 0\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn omcr0(&self) -> crate::common::Reg<self::Omcr0_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "Port 00 Output Modification Clear Register 12\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn omcr12(&self) -> crate::common::Reg<self::Omcr12_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(140usize)) }
    }

    #[doc = "Port 00 Output Modification Clear Register 4\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn omcr4(&self) -> crate::common::Reg<self::Omcr4_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }

    #[doc = "Port 00 Output Modification Clear Register 8\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn omcr8(&self) -> crate::common::Reg<self::Omcr8_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }

    #[doc = "Port 00 Output Modification Register\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn omr(&self) -> crate::common::Reg<self::Omr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }

    #[doc = "Port 00 Output Modification Set Register\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn omsr(&self) -> crate::common::Reg<self::Omsr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }

    #[doc = "Port 00 Output Modification Set Register 0\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn omsr0(&self) -> crate::common::Reg<self::Omsr0_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(112usize)) }
    }

    #[doc = "Port 00 Output Modification Set Register 12\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn omsr12(&self) -> crate::common::Reg<self::Omsr12_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(124usize)) }
    }

    #[doc = "Port 00 Output Modification Set Register 4\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn omsr4(&self) -> crate::common::Reg<self::Omsr4_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(116usize)) }
    }

    #[doc = "Port 00 Output Modification Set Register 8\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn omsr8(&self) -> crate::common::Reg<self::Omsr8_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(120usize)) }
    }

    #[doc = "Port 00 Output Register\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn out(&self) -> crate::common::Reg<self::Out_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Port 00 Pin Controller Select Register\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn pcsr(&self) -> crate::common::Reg<self::Pcsr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(100usize)) }
    }

    #[doc = "Port 00 Pin Function Decision Control Register\n resetvalue={:0x0,:0x0,After SSW execution:0x0,After SSW execution:0x0}"]
    #[inline(always)]
    pub const fn pdisc(&self) -> crate::common::Reg<self::Pdisc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }

    #[doc = "Port 00 Pad Driver Mode Register 0\n resetvalue={:0x0,:0x0,After SSW execution:0x22222222,After SSW execution:0x22222222}"]
    #[inline(always)]
    pub const fn pdr0(&self) -> crate::common::Reg<self::Pdr0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "Port 00 Pad Driver Mode Register 1\n resetvalue={:0x0,:0x0,After SSW execution:0x22222222,After SSW execution:0x22222222}"]
    #[inline(always)]
    pub const fn pdr1(&self) -> crate::common::Reg<self::Pdr1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accen0_SPEC;
impl crate::sealed::RegSpec for Accen0_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Access Enable Register 0\n resetvalue={:0x0FFFFFFFF}"]
pub type Accen0 = crate::RegValueT<Accen0_SPEC>;

impl Accen0 {
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
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
pub struct Esr_SPEC;
impl crate::sealed::RegSpec for Esr_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Emergency Stop Register\n resetvalue={:0x0}"]
pub type Esr = crate::RegValueT<Esr_SPEC>;

impl Esr {
    #[doc = "Emergency Stop Enable for Pin 15. This bit enables the emergency stop function for all GPIO lines. If the        emergency stop condition is met and enabled  the output selection is        automatically switched from alternate output function to GPIO input        function."]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Pin 15. This bit enables the emergency stop function for all GPIO lines. If the        emergency stop condition is met and enabled  the output selection is        automatically switched from alternate output function to GPIO input        function."]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Pin 15. This bit enables the emergency stop function for all GPIO lines. If the        emergency stop condition is met and enabled  the output selection is        automatically switched from alternate output function to GPIO input        function."]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Pin 15. This bit enables the emergency stop function for all GPIO lines. If the        emergency stop condition is met and enabled  the output selection is        automatically switched from alternate output function to GPIO input        function."]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Pin 15. This bit enables the emergency stop function for all GPIO lines. If the        emergency stop condition is met and enabled  the output selection is        automatically switched from alternate output function to GPIO input        function."]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Pin 15. This bit enables the emergency stop function for all GPIO lines. If the        emergency stop condition is met and enabled  the output selection is        automatically switched from alternate output function to GPIO input        function."]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Pin 15. This bit enables the emergency stop function for all GPIO lines. If the        emergency stop condition is met and enabled  the output selection is        automatically switched from alternate output function to GPIO input        function."]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Pin 15. This bit enables the emergency stop function for all GPIO lines. If the        emergency stop condition is met and enabled  the output selection is        automatically switched from alternate output function to GPIO input        function."]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Pin 15. This bit enables the emergency stop function for all GPIO lines. If the        emergency stop condition is met and enabled  the output selection is        automatically switched from alternate output function to GPIO input        function."]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Pin 15. This bit enables the emergency stop function for all GPIO lines. If the        emergency stop condition is met and enabled  the output selection is        automatically switched from alternate output function to GPIO input        function."]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Pin 15. This bit enables the emergency stop function for all GPIO lines. If the        emergency stop condition is met and enabled  the output selection is        automatically switched from alternate output function to GPIO input        function."]
    #[inline(always)]
    pub fn en10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Pin 15. This bit enables the emergency stop function for all GPIO lines. If the        emergency stop condition is met and enabled  the output selection is        automatically switched from alternate output function to GPIO input        function."]
    #[inline(always)]
    pub fn en11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Pin 15. This bit enables the emergency stop function for all GPIO lines. If the        emergency stop condition is met and enabled  the output selection is        automatically switched from alternate output function to GPIO input        function."]
    #[inline(always)]
    pub fn en12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Pin 15. This bit enables the emergency stop function for all GPIO lines. If the        emergency stop condition is met and enabled  the output selection is        automatically switched from alternate output function to GPIO input        function."]
    #[inline(always)]
    pub fn en13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Pin 15. This bit enables the emergency stop function for all GPIO lines. If the        emergency stop condition is met and enabled  the output selection is        automatically switched from alternate output function to GPIO input        function."]
    #[inline(always)]
    pub fn en14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Pin 15. This bit enables the emergency stop function for all GPIO lines. If the        emergency stop condition is met and enabled  the output selection is        automatically switched from alternate output function to GPIO input        function."]
    #[inline(always)]
    pub fn en15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Esr {
    #[inline(always)]
    fn default() -> Esr {
        <crate::RegValueT<Esr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Identification Register\n resetvalue={:0x0C8C000}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number. This bit field indicates the revision number of the TC39x module  01 H   160    160 first revision ."]
    #[inline(always)]
    pub fn modrev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type. This bit field is C0 H . It defines a        32 bit module"]
    #[inline(always)]
    pub fn modtype(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number. This bit field defines the module identification number. The value for        the Ports module is 00C8"]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(13156352)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct In_SPEC;
impl crate::sealed::RegSpec for In_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Input Register\n resetvalue={:0x0}"]
pub type In = crate::RegValueT<In_SPEC>;

impl In {
    #[doc = "Input Bit 15. This bit indicates the level at the input pin Pn.x."]
    #[inline(always)]
    pub fn p0(self) -> crate::common::RegisterFieldBool<0, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Bit 15. This bit indicates the level at the input pin Pn.x."]
    #[inline(always)]
    pub fn p1(self) -> crate::common::RegisterFieldBool<1, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Bit 15. This bit indicates the level at the input pin Pn.x."]
    #[inline(always)]
    pub fn p2(self) -> crate::common::RegisterFieldBool<2, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Bit 15. This bit indicates the level at the input pin Pn.x."]
    #[inline(always)]
    pub fn p3(self) -> crate::common::RegisterFieldBool<3, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Bit 15. This bit indicates the level at the input pin Pn.x."]
    #[inline(always)]
    pub fn p4(self) -> crate::common::RegisterFieldBool<4, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Bit 15. This bit indicates the level at the input pin Pn.x."]
    #[inline(always)]
    pub fn p5(self) -> crate::common::RegisterFieldBool<5, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Bit 15. This bit indicates the level at the input pin Pn.x."]
    #[inline(always)]
    pub fn p6(self) -> crate::common::RegisterFieldBool<6, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Bit 15. This bit indicates the level at the input pin Pn.x."]
    #[inline(always)]
    pub fn p7(self) -> crate::common::RegisterFieldBool<7, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Bit 15. This bit indicates the level at the input pin Pn.x."]
    #[inline(always)]
    pub fn p8(self) -> crate::common::RegisterFieldBool<8, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Bit 15. This bit indicates the level at the input pin Pn.x."]
    #[inline(always)]
    pub fn p9(self) -> crate::common::RegisterFieldBool<9, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Bit 15. This bit indicates the level at the input pin Pn.x."]
    #[inline(always)]
    pub fn p10(self) -> crate::common::RegisterFieldBool<10, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Bit 15. This bit indicates the level at the input pin Pn.x."]
    #[inline(always)]
    pub fn p11(self) -> crate::common::RegisterFieldBool<11, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Bit 15. This bit indicates the level at the input pin Pn.x."]
    #[inline(always)]
    pub fn p12(self) -> crate::common::RegisterFieldBool<12, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Bit 15. This bit indicates the level at the input pin Pn.x."]
    #[inline(always)]
    pub fn p13(self) -> crate::common::RegisterFieldBool<13, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Bit 15. This bit indicates the level at the input pin Pn.x."]
    #[inline(always)]
    pub fn p14(self) -> crate::common::RegisterFieldBool<14, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Bit 15. This bit indicates the level at the input pin Pn.x."]
    #[inline(always)]
    pub fn p15(self) -> crate::common::RegisterFieldBool<15, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for In {
    #[inline(always)]
    fn default() -> In {
        <crate::RegValueT<In_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iocr0_SPEC;
impl crate::sealed::RegSpec for Iocr0_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Input Output Control Register 0\n resetvalue={:0x0,:0x0,:0x10101010,:0x10101010}"]
pub type Iocr0 = crate::RegValueT<Iocr0_SPEC>;

impl Iocr0 {
    #[doc = "Port Control for Pin 3. This bit field defines the Port n line x functionality according to Table          only input selection apply."]
    #[inline(always)]
    pub fn pc0(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, Iocr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8, Iocr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Control for Pin 3. This bit field defines the Port n line x functionality according to Table          only input selection apply."]
    #[inline(always)]
    pub fn pc1(
        self,
    ) -> crate::common::RegisterField<11, 0x1f, 1, 0, u8, Iocr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1f,1,0,u8, Iocr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Control for Pin 3. This bit field defines the Port n line x functionality according to Table          only input selection apply."]
    #[inline(always)]
    pub fn pc2(
        self,
    ) -> crate::common::RegisterField<19, 0x1f, 1, 0, u8, Iocr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<19,0x1f,1,0,u8, Iocr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Control for Pin 3. This bit field defines the Port n line x functionality according to Table          only input selection apply."]
    #[inline(always)]
    pub fn pc3(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, Iocr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, Iocr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Iocr0 {
    #[inline(always)]
    fn default() -> Iocr0 {
        <crate::RegValueT<Iocr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iocr12_SPEC;
impl crate::sealed::RegSpec for Iocr12_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Input Output Control Register 12\n resetvalue={:0x0,:0x0,:0x10101010,:0x10101010}"]
pub type Iocr12 = crate::RegValueT<Iocr12_SPEC>;

impl Iocr12 {
    #[doc = "Port Control for Port 00 Pin 15. This bit field defines the Port n line x functionality according to Table          only input selection apply."]
    #[inline(always)]
    pub fn pc12(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, Iocr12_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8, Iocr12_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Control for Port 00 Pin 15. This bit field defines the Port n line x functionality according to Table          only input selection apply."]
    #[inline(always)]
    pub fn pc13(
        self,
    ) -> crate::common::RegisterField<11, 0x1f, 1, 0, u8, Iocr12_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1f,1,0,u8, Iocr12_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Control for Port 00 Pin 15. This bit field defines the Port n line x functionality according to Table          only input selection apply."]
    #[inline(always)]
    pub fn pc14(
        self,
    ) -> crate::common::RegisterField<19, 0x1f, 1, 0, u8, Iocr12_SPEC, crate::common::RW> {
        crate::common::RegisterField::<19,0x1f,1,0,u8, Iocr12_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Control for Port 00 Pin 15. This bit field defines the Port n line x functionality according to Table          only input selection apply."]
    #[inline(always)]
    pub fn pc15(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, Iocr12_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, Iocr12_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Iocr12 {
    #[inline(always)]
    fn default() -> Iocr12 {
        <crate::RegValueT<Iocr12_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iocr4_SPEC;
impl crate::sealed::RegSpec for Iocr4_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Input Output Control Register 4\n resetvalue={:0x0,:0x0,:0x10101010,:0x10101010}"]
pub type Iocr4 = crate::RegValueT<Iocr4_SPEC>;

impl Iocr4 {
    #[doc = "Port Control for Port 00 Pin 7. This bit field defines the Port n line x functionality according to Table          only input selection apply."]
    #[inline(always)]
    pub fn pc4(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, Iocr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8, Iocr4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Control for Port 00 Pin 7. This bit field defines the Port n line x functionality according to Table          only input selection apply."]
    #[inline(always)]
    pub fn pc5(
        self,
    ) -> crate::common::RegisterField<11, 0x1f, 1, 0, u8, Iocr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1f,1,0,u8, Iocr4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Control for Port 00 Pin 7. This bit field defines the Port n line x functionality according to Table          only input selection apply."]
    #[inline(always)]
    pub fn pc6(
        self,
    ) -> crate::common::RegisterField<19, 0x1f, 1, 0, u8, Iocr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<19,0x1f,1,0,u8, Iocr4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Control for Port 00 Pin 7. This bit field defines the Port n line x functionality according to Table          only input selection apply."]
    #[inline(always)]
    pub fn pc7(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, Iocr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, Iocr4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Iocr4 {
    #[inline(always)]
    fn default() -> Iocr4 {
        <crate::RegValueT<Iocr4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iocr8_SPEC;
impl crate::sealed::RegSpec for Iocr8_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Input Output Control Register 8\n resetvalue={:0x0,:0x0,:0x10101010,:0x10101010}"]
pub type Iocr8 = crate::RegValueT<Iocr8_SPEC>;

impl Iocr8 {
    #[doc = "Port Control for Port 00 Pin 11. This bit field defines the Port n line x functionality according to Table          only input selection apply."]
    #[inline(always)]
    pub fn pc8(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, Iocr8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8, Iocr8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Control for Port 00 Pin 11. This bit field defines the Port n line x functionality according to Table          only input selection apply."]
    #[inline(always)]
    pub fn pc9(
        self,
    ) -> crate::common::RegisterField<11, 0x1f, 1, 0, u8, Iocr8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1f,1,0,u8, Iocr8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Control for Port 00 Pin 11. This bit field defines the Port n line x functionality according to Table          only input selection apply."]
    #[inline(always)]
    pub fn pc10(
        self,
    ) -> crate::common::RegisterField<19, 0x1f, 1, 0, u8, Iocr8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<19,0x1f,1,0,u8, Iocr8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Control for Port 00 Pin 11. This bit field defines the Port n line x functionality according to Table          only input selection apply."]
    #[inline(always)]
    pub fn pc11(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, Iocr8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, Iocr8_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Iocr8 {
    #[inline(always)]
    fn default() -> Iocr8 {
        <crate::RegValueT<Iocr8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Omcr_SPEC;
impl crate::sealed::RegSpec for Omcr_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Output Modification Clear Register\n resetvalue={:0x0}"]
pub type Omcr = crate::RegValueT<Omcr_SPEC>;

impl Omcr {
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl0(self) -> crate::common::RegisterFieldBool<16, 1, 0, Omcr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Omcr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl1(self) -> crate::common::RegisterFieldBool<17, 1, 0, Omcr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Omcr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl2(self) -> crate::common::RegisterFieldBool<18, 1, 0, Omcr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Omcr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl3(self) -> crate::common::RegisterFieldBool<19, 1, 0, Omcr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, Omcr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl4(self) -> crate::common::RegisterFieldBool<20, 1, 0, Omcr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, Omcr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl5(self) -> crate::common::RegisterFieldBool<21, 1, 0, Omcr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, Omcr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl6(self) -> crate::common::RegisterFieldBool<22, 1, 0, Omcr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, Omcr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl7(self) -> crate::common::RegisterFieldBool<23, 1, 0, Omcr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, Omcr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl8(self) -> crate::common::RegisterFieldBool<24, 1, 0, Omcr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Omcr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl9(self) -> crate::common::RegisterFieldBool<25, 1, 0, Omcr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Omcr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl10(self) -> crate::common::RegisterFieldBool<26, 1, 0, Omcr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Omcr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl11(self) -> crate::common::RegisterFieldBool<27, 1, 0, Omcr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, Omcr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl12(self) -> crate::common::RegisterFieldBool<28, 1, 0, Omcr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Omcr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl13(self) -> crate::common::RegisterFieldBool<29, 1, 0, Omcr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, Omcr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl14(self) -> crate::common::RegisterFieldBool<30, 1, 0, Omcr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Omcr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl15(self) -> crate::common::RegisterFieldBool<31, 1, 0, Omcr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Omcr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Omcr {
    #[inline(always)]
    fn default() -> Omcr {
        <crate::RegValueT<Omcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Omcr0_SPEC;
impl crate::sealed::RegSpec for Omcr0_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Output Modification Clear Register 0\n resetvalue={:0x0}"]
pub type Omcr0 = crate::RegValueT<Omcr0_SPEC>;

impl Omcr0 {
    #[doc = "Clear Bit 3. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl0(self) -> crate::common::RegisterFieldBool<16, 1, 0, Omcr0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Omcr0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 3. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl1(self) -> crate::common::RegisterFieldBool<17, 1, 0, Omcr0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Omcr0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 3. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl2(self) -> crate::common::RegisterFieldBool<18, 1, 0, Omcr0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Omcr0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 3. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl3(self) -> crate::common::RegisterFieldBool<19, 1, 0, Omcr0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, Omcr0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Omcr0 {
    #[inline(always)]
    fn default() -> Omcr0 {
        <crate::RegValueT<Omcr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Omcr12_SPEC;
impl crate::sealed::RegSpec for Omcr12_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Output Modification Clear Register 12\n resetvalue={:0x0}"]
pub type Omcr12 = crate::RegValueT<Omcr12_SPEC>;

impl Omcr12 {
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl12(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Omcr12_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Omcr12_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl13(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Omcr12_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, Omcr12_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl14(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Omcr12_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Omcr12_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl15(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Omcr12_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Omcr12_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Omcr12 {
    #[inline(always)]
    fn default() -> Omcr12 {
        <crate::RegValueT<Omcr12_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Omcr4_SPEC;
impl crate::sealed::RegSpec for Omcr4_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Output Modification Clear Register 4\n resetvalue={:0x0}"]
pub type Omcr4 = crate::RegValueT<Omcr4_SPEC>;

impl Omcr4 {
    #[doc = "Clear Bit 7. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl4(self) -> crate::common::RegisterFieldBool<20, 1, 0, Omcr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, Omcr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 7. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl5(self) -> crate::common::RegisterFieldBool<21, 1, 0, Omcr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, Omcr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 7. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl6(self) -> crate::common::RegisterFieldBool<22, 1, 0, Omcr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, Omcr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 7. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl7(self) -> crate::common::RegisterFieldBool<23, 1, 0, Omcr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, Omcr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Omcr4 {
    #[inline(always)]
    fn default() -> Omcr4 {
        <crate::RegValueT<Omcr4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Omcr8_SPEC;
impl crate::sealed::RegSpec for Omcr8_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Output Modification Clear Register 8\n resetvalue={:0x0}"]
pub type Omcr8 = crate::RegValueT<Omcr8_SPEC>;

impl Omcr8 {
    #[doc = "Clear Bit 11. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl8(self) -> crate::common::RegisterFieldBool<24, 1, 0, Omcr8_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Omcr8_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 11. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl9(self) -> crate::common::RegisterFieldBool<25, 1, 0, Omcr8_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Omcr8_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 11. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl10(self) -> crate::common::RegisterFieldBool<26, 1, 0, Omcr8_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Omcr8_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 11. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl11(self) -> crate::common::RegisterFieldBool<27, 1, 0, Omcr8_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, Omcr8_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Omcr8 {
    #[inline(always)]
    fn default() -> Omcr8 {
        <crate::RegValueT<Omcr8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Omr_SPEC;
impl crate::sealed::RegSpec for Omr_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Output Modification Register\n resetvalue={:0x0}"]
pub type Omr = crate::RegValueT<Omr_SPEC>;

impl Omr {
    #[doc = "Set Bit 15. Setting this bit will set or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn ps0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn ps1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn ps2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn ps3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn ps4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn ps5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn ps6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn ps7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn ps8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn ps9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn ps10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn ps11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn ps12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn ps13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn ps14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn ps15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn pcl0(self) -> crate::common::RegisterFieldBool<16, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn pcl1(self) -> crate::common::RegisterFieldBool<17, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn pcl2(self) -> crate::common::RegisterFieldBool<18, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn pcl3(self) -> crate::common::RegisterFieldBool<19, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn pcl4(self) -> crate::common::RegisterFieldBool<20, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn pcl5(self) -> crate::common::RegisterFieldBool<21, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn pcl6(self) -> crate::common::RegisterFieldBool<22, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn pcl7(self) -> crate::common::RegisterFieldBool<23, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn pcl8(self) -> crate::common::RegisterFieldBool<24, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn pcl9(self) -> crate::common::RegisterFieldBool<25, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn pcl10(self) -> crate::common::RegisterFieldBool<26, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn pcl11(self) -> crate::common::RegisterFieldBool<27, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn pcl12(self) -> crate::common::RegisterFieldBool<28, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn pcl13(self) -> crate::common::RegisterFieldBool<29, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn pcl14(self) -> crate::common::RegisterFieldBool<30, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit 15. Setting this bit will clear or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn pcl15(self) -> crate::common::RegisterFieldBool<31, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Omr {
    #[inline(always)]
    fn default() -> Omr {
        <crate::RegValueT<Omr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Omsr_SPEC;
impl crate::sealed::RegSpec for Omsr_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Output Modification Set Register\n resetvalue={:0x0}"]
pub type Omsr = crate::RegValueT<Omsr_SPEC>;

impl Omsr {
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Omsr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Omsr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Omsr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Omsr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Omsr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Omsr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Omsr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Omsr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Omsr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Omsr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Omsr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Omsr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Omsr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Omsr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Omsr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Omsr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Omsr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Omsr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Omsr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Omsr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Omsr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Omsr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Omsr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, Omsr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Omsr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, Omsr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Omsr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, Omsr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Omsr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, Omsr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Omsr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Omsr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Omsr {
    #[inline(always)]
    fn default() -> Omsr {
        <crate::RegValueT<Omsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Omsr0_SPEC;
impl crate::sealed::RegSpec for Omsr0_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Output Modification Set Register 0\n resetvalue={:0x0}"]
pub type Omsr0 = crate::RegValueT<Omsr0_SPEC>;

impl Omsr0 {
    #[doc = "Set Bit 3. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Omsr0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Omsr0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 3. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Omsr0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Omsr0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 3. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Omsr0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Omsr0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 3. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Omsr0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Omsr0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Omsr0 {
    #[inline(always)]
    fn default() -> Omsr0 {
        <crate::RegValueT<Omsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Omsr12_SPEC;
impl crate::sealed::RegSpec for Omsr12_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Output Modification Set Register 12\n resetvalue={:0x0}"]
pub type Omsr12 = crate::RegValueT<Omsr12_SPEC>;

impl Omsr12 {
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Omsr12_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, Omsr12_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Omsr12_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, Omsr12_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Omsr12_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, Omsr12_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Omsr12_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Omsr12_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Omsr12 {
    #[inline(always)]
    fn default() -> Omsr12 {
        <crate::RegValueT<Omsr12_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Omsr4_SPEC;
impl crate::sealed::RegSpec for Omsr4_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Output Modification Set Register 4\n resetvalue={:0x0}"]
pub type Omsr4 = crate::RegValueT<Omsr4_SPEC>;

impl Omsr4 {
    #[doc = "Set Bit 7. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Omsr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Omsr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 7. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Omsr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Omsr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 7. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Omsr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Omsr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 7. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Omsr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Omsr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Omsr4 {
    #[inline(always)]
    fn default() -> Omsr4 {
        <crate::RegValueT<Omsr4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Omsr8_SPEC;
impl crate::sealed::RegSpec for Omsr8_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Output Modification Set Register 8\n resetvalue={:0x0}"]
pub type Omsr8 = crate::RegValueT<Omsr8_SPEC>;

impl Omsr8 {
    #[doc = "Set Bit 11. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Omsr8_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Omsr8_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 11. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Omsr8_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Omsr8_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 11. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Omsr8_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Omsr8_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit 11. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Omsr8_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, Omsr8_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Omsr8 {
    #[inline(always)]
    fn default() -> Omsr8 {
        <crate::RegValueT<Omsr8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Out_SPEC;
impl crate::sealed::RegSpec for Out_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Output Register\n resetvalue={:0x0}"]
pub type Out = crate::RegValueT<Out_SPEC>;

impl Out {
    #[doc = "Output Bit 15. This bit determines the level at the output pin Pn.x if the output is        selected as GPIO output. Pn.x can also be set or cleared by control bits of the Pn OMSR  Pn OMCR        or Pn OMR registers."]
    #[inline(always)]
    pub fn p0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Output Bit 15. This bit determines the level at the output pin Pn.x if the output is        selected as GPIO output. Pn.x can also be set or cleared by control bits of the Pn OMSR  Pn OMCR        or Pn OMR registers."]
    #[inline(always)]
    pub fn p1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Output Bit 15. This bit determines the level at the output pin Pn.x if the output is        selected as GPIO output. Pn.x can also be set or cleared by control bits of the Pn OMSR  Pn OMCR        or Pn OMR registers."]
    #[inline(always)]
    pub fn p2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Output Bit 15. This bit determines the level at the output pin Pn.x if the output is        selected as GPIO output. Pn.x can also be set or cleared by control bits of the Pn OMSR  Pn OMCR        or Pn OMR registers."]
    #[inline(always)]
    pub fn p3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Output Bit 15. This bit determines the level at the output pin Pn.x if the output is        selected as GPIO output. Pn.x can also be set or cleared by control bits of the Pn OMSR  Pn OMCR        or Pn OMR registers."]
    #[inline(always)]
    pub fn p4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Output Bit 15. This bit determines the level at the output pin Pn.x if the output is        selected as GPIO output. Pn.x can also be set or cleared by control bits of the Pn OMSR  Pn OMCR        or Pn OMR registers."]
    #[inline(always)]
    pub fn p5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Output Bit 15. This bit determines the level at the output pin Pn.x if the output is        selected as GPIO output. Pn.x can also be set or cleared by control bits of the Pn OMSR  Pn OMCR        or Pn OMR registers."]
    #[inline(always)]
    pub fn p6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Output Bit 15. This bit determines the level at the output pin Pn.x if the output is        selected as GPIO output. Pn.x can also be set or cleared by control bits of the Pn OMSR  Pn OMCR        or Pn OMR registers."]
    #[inline(always)]
    pub fn p7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Output Bit 15. This bit determines the level at the output pin Pn.x if the output is        selected as GPIO output. Pn.x can also be set or cleared by control bits of the Pn OMSR  Pn OMCR        or Pn OMR registers."]
    #[inline(always)]
    pub fn p8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Output Bit 15. This bit determines the level at the output pin Pn.x if the output is        selected as GPIO output. Pn.x can also be set or cleared by control bits of the Pn OMSR  Pn OMCR        or Pn OMR registers."]
    #[inline(always)]
    pub fn p9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Output Bit 15. This bit determines the level at the output pin Pn.x if the output is        selected as GPIO output. Pn.x can also be set or cleared by control bits of the Pn OMSR  Pn OMCR        or Pn OMR registers."]
    #[inline(always)]
    pub fn p10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Output Bit 15. This bit determines the level at the output pin Pn.x if the output is        selected as GPIO output. Pn.x can also be set or cleared by control bits of the Pn OMSR  Pn OMCR        or Pn OMR registers."]
    #[inline(always)]
    pub fn p11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Output Bit 15. This bit determines the level at the output pin Pn.x if the output is        selected as GPIO output. Pn.x can also be set or cleared by control bits of the Pn OMSR  Pn OMCR        or Pn OMR registers."]
    #[inline(always)]
    pub fn p12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Output Bit 15. This bit determines the level at the output pin Pn.x if the output is        selected as GPIO output. Pn.x can also be set or cleared by control bits of the Pn OMSR  Pn OMCR        or Pn OMR registers."]
    #[inline(always)]
    pub fn p13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Output Bit 15. This bit determines the level at the output pin Pn.x if the output is        selected as GPIO output. Pn.x can also be set or cleared by control bits of the Pn OMSR  Pn OMCR        or Pn OMR registers."]
    #[inline(always)]
    pub fn p14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Output Bit 15. This bit determines the level at the output pin Pn.x if the output is        selected as GPIO output. Pn.x can also be set or cleared by control bits of the Pn OMSR  Pn OMCR        or Pn OMR registers."]
    #[inline(always)]
    pub fn p15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Out {
    #[inline(always)]
    fn default() -> Out {
        <crate::RegValueT<Out_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcsr_SPEC;
impl crate::sealed::RegSpec for Pcsr_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Pin Controller Select Register\n resetvalue={:0x0}"]
pub type Pcsr = crate::RegValueT<Pcsr_SPEC>;

impl Pcsr {
    #[doc = "Output Select for Pin 15. This bit enables or disables EVADC control of the pulls for Pull Down        Diagnostics  PDD    Multiplexer Diagnostics  MD  feature."]
    #[inline(always)]
    pub fn sel10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Pcsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Pcsr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Output Select for Pin 15. This bit enables or disables EVADC control of the pulls for Pull Down        Diagnostics  PDD    Multiplexer Diagnostics  MD  feature."]
    #[inline(always)]
    pub fn sel11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Pcsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Pcsr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Pcsr {
    #[inline(always)]
    fn default() -> Pcsr {
        <crate::RegValueT<Pcsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdisc_SPEC;
impl crate::sealed::RegSpec for Pdisc_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Pin Function Decision Control Register\n resetvalue={:0x0,:0x0,After SSW execution:0x0,After SSW execution:0x0}"]
pub type Pdisc = crate::RegValueT<Pdisc_SPEC>;

impl Pdisc {
    #[doc = "Pin Function Decision Control for Pin 15. This bit selects the function of the port pad."]
    #[inline(always)]
    pub fn pdis0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Pdisc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pdisc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pin Function Decision Control for Pin 15. This bit selects the function of the port pad."]
    #[inline(always)]
    pub fn pdis1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Pdisc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pdisc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pin Function Decision Control for Pin 15. This bit selects the function of the port pad."]
    #[inline(always)]
    pub fn pdis2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Pdisc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Pdisc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pin Function Decision Control for Pin 15. This bit selects the function of the port pad."]
    #[inline(always)]
    pub fn pdis3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Pdisc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Pdisc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pin Function Decision Control for Pin 15. This bit selects the function of the port pad."]
    #[inline(always)]
    pub fn pdis4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Pdisc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pdisc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pin Function Decision Control for Pin 15. This bit selects the function of the port pad."]
    #[inline(always)]
    pub fn pdis5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Pdisc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Pdisc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pin Function Decision Control for Pin 15. This bit selects the function of the port pad."]
    #[inline(always)]
    pub fn pdis6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Pdisc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Pdisc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pin Function Decision Control for Pin 15. This bit selects the function of the port pad."]
    #[inline(always)]
    pub fn pdis7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Pdisc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Pdisc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pin Function Decision Control for Pin 15. This bit selects the function of the port pad."]
    #[inline(always)]
    pub fn pdis8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Pdisc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Pdisc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pin Function Decision Control for Pin 15. This bit selects the function of the port pad."]
    #[inline(always)]
    pub fn pdis9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Pdisc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Pdisc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pin Function Decision Control for Pin 15. This bit selects the function of the port pad."]
    #[inline(always)]
    pub fn pdis10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Pdisc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Pdisc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pin Function Decision Control for Pin 15. This bit selects the function of the port pad."]
    #[inline(always)]
    pub fn pdis11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Pdisc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Pdisc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pin Function Decision Control for Pin 15. This bit selects the function of the port pad."]
    #[inline(always)]
    pub fn pdis12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Pdisc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Pdisc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pin Function Decision Control for Pin 15. This bit selects the function of the port pad."]
    #[inline(always)]
    pub fn pdis13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Pdisc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Pdisc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pin Function Decision Control for Pin 15. This bit selects the function of the port pad."]
    #[inline(always)]
    pub fn pdis14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Pdisc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Pdisc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pin Function Decision Control for Pin 15. This bit selects the function of the port pad."]
    #[inline(always)]
    pub fn pdis15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Pdisc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Pdisc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Pdisc {
    #[inline(always)]
    fn default() -> Pdisc {
        <crate::RegValueT<Pdisc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdr0_SPEC;
impl crate::sealed::RegSpec for Pdr0_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Pad Driver Mode Register 0\n resetvalue={:0x0,:0x0,After SSW execution:0x22222222,After SSW execution:0x22222222}"]
pub type Pdr0 = crate::RegValueT<Pdr0_SPEC>;

impl Pdr0 {
    #[doc = "Pad Driver Mode for Pin 7"]
    #[inline(always)]
    pub fn pd0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Level Selection for Pin 7"]
    #[inline(always)]
    pub fn pl0(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Driver Mode for Pin 7"]
    #[inline(always)]
    pub fn pd1(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Level Selection for Pin 7"]
    #[inline(always)]
    pub fn pl1(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Driver Mode for Pin 7"]
    #[inline(always)]
    pub fn pd2(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Level Selection for Pin 7"]
    #[inline(always)]
    pub fn pl2(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Driver Mode for Pin 7"]
    #[inline(always)]
    pub fn pd3(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Level Selection for Pin 7"]
    #[inline(always)]
    pub fn pl3(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Driver Mode for Pin 7"]
    #[inline(always)]
    pub fn pd4(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Level Selection for Pin 7"]
    #[inline(always)]
    pub fn pl4(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Driver Mode for Pin 7"]
    #[inline(always)]
    pub fn pd5(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Level Selection for Pin 7"]
    #[inline(always)]
    pub fn pl5(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Driver Mode for Pin 7"]
    #[inline(always)]
    pub fn pd6(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Level Selection for Pin 7"]
    #[inline(always)]
    pub fn pl6(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Driver Mode for Pin 7"]
    #[inline(always)]
    pub fn pd7(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Level Selection for Pin 7"]
    #[inline(always)]
    pub fn pl7(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pdr0 {
    #[inline(always)]
    fn default() -> Pdr0 {
        <crate::RegValueT<Pdr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdr1_SPEC;
impl crate::sealed::RegSpec for Pdr1_SPEC {
    type DataType = u32;
}
#[doc = "Port 00 Pad Driver Mode Register 1\n resetvalue={:0x0,:0x0,After SSW execution:0x22222222,After SSW execution:0x22222222}"]
pub type Pdr1 = crate::RegValueT<Pdr1_SPEC>;

impl Pdr1 {
    #[doc = "Pad Driver Mode for Pin 15"]
    #[inline(always)]
    pub fn pd8(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Pdr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Pdr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Level Selection for Pin 15"]
    #[inline(always)]
    pub fn pl8(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Pdr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Pdr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Driver Mode for Pin 15"]
    #[inline(always)]
    pub fn pd9(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Pdr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Pdr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Level Selection for Pin 15"]
    #[inline(always)]
    pub fn pl9(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Pdr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Pdr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Driver Mode for Pin 15"]
    #[inline(always)]
    pub fn pd10(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Pdr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Pdr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Level Selection for Pin 15"]
    #[inline(always)]
    pub fn pl10(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Pdr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Pdr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Driver Mode for Pin 15"]
    #[inline(always)]
    pub fn pd11(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Pdr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Pdr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Level Selection for Pin 15"]
    #[inline(always)]
    pub fn pl11(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Pdr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Pdr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Driver Mode for Pin 15"]
    #[inline(always)]
    pub fn pd12(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Pdr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Pdr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Level Selection for Pin 15"]
    #[inline(always)]
    pub fn pl12(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, Pdr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, Pdr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Driver Mode for Pin 15"]
    #[inline(always)]
    pub fn pd13(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Pdr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Pdr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Level Selection for Pin 15"]
    #[inline(always)]
    pub fn pl13(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, Pdr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8, Pdr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Driver Mode for Pin 15"]
    #[inline(always)]
    pub fn pd14(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, Pdr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, Pdr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Level Selection for Pin 15"]
    #[inline(always)]
    pub fn pl14(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, Pdr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3,1,0,u8, Pdr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Driver Mode for Pin 15"]
    #[inline(always)]
    pub fn pd15(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, Pdr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, Pdr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Level Selection for Pin 15"]
    #[inline(always)]
    pub fn pl15(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, Pdr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, Pdr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pdr1 {
    #[inline(always)]
    fn default() -> Pdr1 {
        <crate::RegValueT<Pdr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
