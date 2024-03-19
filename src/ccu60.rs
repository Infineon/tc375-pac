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
#[doc = r"CCU6"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccu60(pub(super) *mut u8);
unsafe impl core::marker::Send for Ccu60 {}
unsafe impl core::marker::Sync for Ccu60 {}
impl Ccu60 {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "Compare Register for T13\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cc63r(&self) -> crate::common::Reg<self::Cc63R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(88usize)) }
    }

    #[doc = "Compare Shadow Register for T13\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cc63sr(&self) -> crate::common::Reg<self::Cc63Sr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(92usize)) }
    }

    #[doc = "Capture Compare Register for Channel CC60\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cc6xr(&self) -> [crate::common::Reg<self::Cc6XR_SPEC, crate::common::R>; 3] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x8usize)),
            ]
        }
    }

    #[doc = "Capture Compare Shadow Reg. for Channel CC60\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cc6xsr(&self) -> [crate::common::Reg<self::Cc6XSr_SPEC, crate::common::RW>; 3] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x40usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x40usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x40usize + 0x8usize)),
            ]
        }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Compare State Modification Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cmpmodif(&self) -> crate::common::Reg<self::Cmpmodif_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(100usize)) }
    }

    #[doc = "Compare State Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cmpstat(&self) -> crate::common::Reg<self::Cmpstat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x5400,Application Reset:0x5409}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Interrupt Enable Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ien(&self) -> crate::common::Reg<self::Ien_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(176usize)) }
    }

    #[doc = "Input Monitoring Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn imon(&self) -> crate::common::Reg<self::Imon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(152usize)) }
    }

    #[doc = "Interrupt Node Pointer Register\n resetvalue={Application Reset:0x3940}"]
    #[inline(always)]
    pub const fn inp(&self) -> crate::common::Reg<self::Inp_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(172usize)) }
    }

    #[doc = "Interrupt Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn is(&self) -> crate::common::Reg<self::Is_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }

    #[doc = "Interrupt Status Reset Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn isr(&self) -> crate::common::Reg<self::Isr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(168usize)) }
    }

    #[doc = "Interrupt Status Set Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn iss(&self) -> crate::common::Reg<self::Iss_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(164usize)) }
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

    #[doc = "Kernel State Control Sensitivity Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn kscsr(&self) -> crate::common::Reg<self::Kscsr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Lost Indicator Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn li(&self) -> crate::common::Reg<self::Li_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(156usize)) }
    }

    #[doc = "Module Configuration Register\n resetvalue={Application Reset:0x08007,Application Reset:0x7}"]
    #[inline(always)]
    pub const fn mcfg(&self) -> crate::common::Reg<self::Mcfg_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }

    #[doc = "Multi Channel Mode Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mcmctr(&self) -> crate::common::Reg<self::Mcmctr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }

    #[doc = "Multi Channel Mode Output Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mcmout(&self) -> crate::common::Reg<self::Mcmout_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }

    #[doc = "Multi Channel Mode Output Shadow Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mcmouts(&self) -> crate::common::Reg<self::Mcmouts_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(140usize)) }
    }

    #[doc = "Modulation Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn modctr(&self) -> crate::common::Reg<self::Modctr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "CCU60 Module Output Select Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mosel(&self) -> crate::common::Reg<self::Mosel_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }

    #[doc = "OCDS Control and Status Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(232usize)) }
    }

    #[doc = "Port Input Select Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pisel0(&self) -> crate::common::Reg<self::Pisel0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "Port Input Select Register 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pisel2(&self) -> crate::common::Reg<self::Pisel2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Passive State Level Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pslr(&self) -> crate::common::Reg<self::Pslr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }

    #[doc = "Timer T12 Counter Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t12(&self) -> crate::common::Reg<self::T12_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Dead Time Control Register for Timer12\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t12dtc(&self) -> crate::common::Reg<self::T12Dtc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }

    #[doc = "T12 Mode Select Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t12msel(&self) -> crate::common::Reg<self::T12Msel_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(104usize)) }
    }

    #[doc = "Timer 12 Period Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t12pr(&self) -> crate::common::Reg<self::T12Pr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "Timer T13 Counter Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t13(&self) -> crate::common::Reg<self::T13_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }

    #[doc = "Timer 13 Period Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t13pr(&self) -> crate::common::Reg<self::T13Pr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(84usize)) }
    }

    #[doc = "Timer Control Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tctr0(&self) -> crate::common::Reg<self::Tctr0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(112usize)) }
    }

    #[doc = "Timer Control Register 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tctr2(&self) -> crate::common::Reg<self::Tctr2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(116usize)) }
    }

    #[doc = "Timer Control Register 4\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tctr4(&self) -> crate::common::Reg<self::Tctr4_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(120usize)) }
    }

    #[doc = "Trap Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn trpctr(&self) -> crate::common::Reg<self::Trpctr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
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
pub struct Cc63R_SPEC;
impl crate::sealed::RegSpec for Cc63R_SPEC {
    type DataType = u32;
}
#[doc = "Compare Register for T13\n resetvalue={Application Reset:0x0}"]
pub type Cc63R = crate::RegValueT<Cc63R_SPEC>;

impl Cc63R {
    #[doc = "Channel CC63 Compare Value   CCV. The bit field CCV contains the value  that is compared to the T13 counter value."]
    #[inline(always)]
    pub fn ccv(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Cc63R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Cc63R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Cc63R {
    #[inline(always)]
    fn default() -> Cc63R {
        <crate::RegValueT<Cc63R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cc63Sr_SPEC;
impl crate::sealed::RegSpec for Cc63Sr_SPEC {
    type DataType = u32;
}
#[doc = "Compare Shadow Register for T13\n resetvalue={Application Reset:0x0}"]
pub type Cc63Sr = crate::RegValueT<Cc63Sr_SPEC>;

impl Cc63Sr {
    #[doc = "Shadow Register for Channel CC63 Compare Value   CCS. The bit field contents of CCS is transferred to the bit field CCV during a shadow transfer."]
    #[inline(always)]
    pub fn ccs(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Cc63Sr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Cc63Sr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Cc63Sr {
    #[inline(always)]
    fn default() -> Cc63Sr {
        <crate::RegValueT<Cc63Sr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cc6XR_SPEC;
impl crate::sealed::RegSpec for Cc6XR_SPEC {
    type DataType = u32;
}
#[doc = "Capture Compare Register for Channel CC60\n resetvalue={Application Reset:0x0}"]
pub type Cc6XR = crate::RegValueT<Cc6XR_SPEC>;

impl Cc6XR {
    #[doc = "Capture Compare Value   CCV. In compare mode  the bit fields CCV contain the values  that are compared to the T12 counter value. In capture mode  the captured value of T12 can be read from these registers."]
    #[inline(always)]
    pub fn ccv(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Cc6XR_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Cc6XR_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Cc6XR {
    #[inline(always)]
    fn default() -> Cc6XR {
        <crate::RegValueT<Cc6XR_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cc6XSr_SPEC;
impl crate::sealed::RegSpec for Cc6XSr_SPEC {
    type DataType = u32;
}
#[doc = "Capture Compare Shadow Reg. for Channel CC60\n resetvalue={Application Reset:0x0}"]
pub type Cc6XSr = crate::RegValueT<Cc6XSr_SPEC>;

impl Cc6XSr {
    #[doc = "Shadow Register for Channel x Capture Compare Value   CCS. In compare mode  the bit fields contents of CCS are transferred to the bit fields CCV for the corresponding channel during a shadow transfer. In capture mode  the captured value of T12 can be read from these registers."]
    #[inline(always)]
    pub fn ccs(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Cc6XSr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Cc6XSr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Cc6XSr {
    #[inline(always)]
    fn default() -> Cc6XSr {
        <crate::RegValueT<Cc6XSr_SPEC> as RegisterValue<_>>::new(0)
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
pub struct Cmpmodif_SPEC;
impl crate::sealed::RegSpec for Cmpmodif_SPEC {
    type DataType = u32;
}
#[doc = "Compare State Modification Register\n resetvalue={Application Reset:0x0}"]
pub type Cmpmodif = crate::RegValueT<Cmpmodif_SPEC>;

impl Cmpmodif {
    #[doc = "Capture Compare Status Modification Bits MCC62S  x   0  1  2    MCC62S. These bits are used to set the corresponding bits CC6xST by SW. The functionality of a write access to bits concerning the same        capture compare state bit is shown in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn mcc60s(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Cmpmodif_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cmpmodif_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture Compare Status Modification Bits MCC62S  x   0  1  2    MCC62S. These bits are used to set the corresponding bits CC6xST by SW. The functionality of a write access to bits concerning the same        capture compare state bit is shown in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn mcc61s(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Cmpmodif_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cmpmodif_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture Compare Status Modification Bits MCC62S  x   0  1  2    MCC62S. These bits are used to set the corresponding bits CC6xST by SW. The functionality of a write access to bits concerning the same        capture compare state bit is shown in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn mcc62s(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Cmpmodif_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cmpmodif_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture Compare Status Modification Bit MCC63S   MCC63S. This bit is used to set the corresponding bit CC63ST by SW. The functionality of a write access to bits concerning the same capture compare state bit is shown in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn mcc63s(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cmpmodif_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cmpmodif_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture Compare Status Modification Bits MCC62R  x   0  1  2    MCC62R. These bits are used to clear the corresponding bits CC6xST by SW. The functionality of a write access to bits concerning the same        capture compare state bit is shown in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn mcc60r(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Cmpmodif_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cmpmodif_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture Compare Status Modification Bits MCC62R  x   0  1  2    MCC62R. These bits are used to clear the corresponding bits CC6xST by SW. The functionality of a write access to bits concerning the same        capture compare state bit is shown in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn mcc61r(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Cmpmodif_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Cmpmodif_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture Compare Status Modification Bits MCC62R  x   0  1  2    MCC62R. These bits are used to clear the corresponding bits CC6xST by SW. The functionality of a write access to bits concerning the same        capture compare state bit is shown in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn mcc62r(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Cmpmodif_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Cmpmodif_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture Compare Status Modification Bits MCC63R   MCC63R. This bit is used to clear the corresponding bit CC63ST by SW. The functionality of a write access to bits concerning the same capture compare state bit is shown in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn mcc63r(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Cmpmodif_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, Cmpmodif_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Cmpmodif {
    #[inline(always)]
    fn default() -> Cmpmodif {
        <crate::RegValueT<Cmpmodif_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpstat_SPEC;
impl crate::sealed::RegSpec for Cmpstat_SPEC {
    type DataType = u32;
}
#[doc = "Compare State Register\n resetvalue={Application Reset:0x0}"]
pub type Cmpstat = crate::RegValueT<Cmpstat_SPEC>;

impl Cmpstat {
    #[doc = "Capture Compare State Bits for CC62  x   0  1  2    CC62ST. Bits CC6xST monitor the state of the capture compare channels. Bits        CC6xST  x  160    160 0  1  2  are related to T12 and are set and cleared        according to the T12 switching rules."]
    #[inline(always)]
    pub fn cc60st(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Cmpstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cmpstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture Compare State Bits for CC62  x   0  1  2    CC62ST. Bits CC6xST monitor the state of the capture compare channels. Bits        CC6xST  x  160    160 0  1  2  are related to T12 and are set and cleared        according to the T12 switching rules."]
    #[inline(always)]
    pub fn cc61st(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Cmpstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cmpstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture Compare State Bits for CC62  x   0  1  2    CC62ST. Bits CC6xST monitor the state of the capture compare channels. Bits        CC6xST  x  160    160 0  1  2  are related to T12 and are set and cleared        according to the T12 switching rules."]
    #[inline(always)]
    pub fn cc62st(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Cmpstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cmpstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Sampled Hall Pattern Bits   CCPOS62. Bits CCPOS6x  x   0  1  2  are indicating the value of the input Hall pattern that has been compared to the current and expected value. The value is sampled when the event HCRDY  Hall Compare Ready  occurs."]
    #[inline(always)]
    pub fn ccpos60(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Cmpstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cmpstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Sampled Hall Pattern Bits   CCPOS62. Bits CCPOS6x  x   0  1  2  are indicating the value of the input Hall pattern that has been compared to the current and expected value. The value is sampled when the event HCRDY  Hall Compare Ready  occurs."]
    #[inline(always)]
    pub fn ccpos61(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Cmpstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cmpstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Sampled Hall Pattern Bits   CCPOS62. Bits CCPOS6x  x   0  1  2  are indicating the value of the input Hall pattern that has been compared to the current and expected value. The value is sampled when the event HCRDY  Hall Compare Ready  occurs."]
    #[inline(always)]
    pub fn ccpos62(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Cmpstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cmpstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture Compare State Bit for CC63   CC63ST. Bit CC63ST monitors the state of the compare channel. Bit CC63ST is related to T13 and is set and cleared according to the T13 switching rules."]
    #[inline(always)]
    pub fn cc63st(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cmpstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cmpstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Passive State Select for Compare Outputs CC62  x   0  1  2    CC62PS. Bits CC6xPS select the state of the corresponding compare channel  that is considered to be the passive state. During the passive state  the passive level  defined in register PSLR  is driven by the output pin. Bits CC6xPS  x   0  1  2  are related to T12. In capture mode  these bits are not used."]
    #[inline(always)]
    pub fn cc60ps(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Cmpstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cmpstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Passive State Select for Compare Outputs CC62  x   0  1  2    CC62PS. Bits CC6xPS select the state of the corresponding compare channel  that is considered to be the passive state. During the passive state  the passive level  defined in register PSLR  is driven by the output pin. Bits CC6xPS  x   0  1  2  are related to T12. In capture mode  these bits are not used."]
    #[inline(always)]
    pub fn cc61ps(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Cmpstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Cmpstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Passive State Select for Compare Outputs CC62  x   0  1  2    CC62PS. Bits CC6xPS select the state of the corresponding compare channel  that is considered to be the passive state. During the passive state  the passive level  defined in register PSLR  is driven by the output pin. Bits CC6xPS  x   0  1  2  are related to T12. In capture mode  these bits are not used."]
    #[inline(always)]
    pub fn cc62ps(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Cmpstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Cmpstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Passive State Select for Compare Outputs COUT62  x   0  1  2    COUT62PS. Bits COUT6xPS select the state of the corresponding compare channel  that is considered to be the passive state. During the passive state  the passive level  defined in register PSLR  is driven by the output pin. Bits COUT6xPS  x   0  1  2  are related to T12. In capture mode  these bits are not used."]
    #[inline(always)]
    pub fn cout60ps(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Cmpstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Cmpstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Passive State Select for Compare Outputs COUT62  x   0  1  2    COUT62PS. Bits COUT6xPS select the state of the corresponding compare channel  that is considered to be the passive state. During the passive state  the passive level  defined in register PSLR  is driven by the output pin. Bits COUT6xPS  x   0  1  2  are related to T12. In capture mode  these bits are not used."]
    #[inline(always)]
    pub fn cout61ps(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Cmpstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Cmpstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Passive State Select for Compare Outputs COUT62  x   0  1  2    COUT62PS. Bits COUT6xPS select the state of the corresponding compare channel  that is considered to be the passive state. During the passive state  the passive level  defined in register PSLR  is driven by the output pin. Bits COUT6xPS  x   0  1  2  are related to T12. In capture mode  these bits are not used."]
    #[inline(always)]
    pub fn cout62ps(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Cmpstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Cmpstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Passive State Select for Compare Output COUT63   COUT63PS. Bit COUT63PS selects the state of the corresponding compare channel  that is considered to be the passive state. During the passive state  the passive level  defined in register PSLR  is driven by the output pin. Bit COUT63PS is related to T13."]
    #[inline(always)]
    pub fn cout63ps(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Cmpstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Cmpstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "T13 Inverted Modulation   T13IM. Bit T13IM inverts the T13 signal for the modulation of the CC6x and COUT6x  x   0  1  2  signals."]
    #[inline(always)]
    pub fn t13im(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Cmpstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Cmpstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Cmpstat {
    #[inline(always)]
    fn default() -> Cmpstat {
        <crate::RegValueT<Cmpstat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x5400,Application Reset:0x5409}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MODREV. MODREV defines the module revision number. The value of a module        revision starts with 01 H  first        revision   02 H          03 H    8230 up        to FF H ."]
    #[inline(always)]
    pub fn modrev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MODNUM. This bit field defines the module identification number for the CCU6  54 H"]
    #[inline(always)]
    pub fn modnum(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Id {
    #[inline(always)]
    fn default() -> Id {
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(21504)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien_SPEC;
impl crate::sealed::RegSpec for Ien_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Enable Register\n resetvalue={Application Reset:0x0}"]
pub type Ien = crate::RegValueT<Ien_SPEC>;

impl Ien {
    #[doc = "Capture  Compare Match Rising Edge Interrupt Enable for Channel CC6x ENCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn encc60r(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture  Compare Match Falling Edge Interrupt Enable for Channel CC6x ENCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn encc60f(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture  Compare Match Rising Edge Interrupt Enable for Channel CC6x ENCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn encc61r(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture  Compare Match Falling Edge Interrupt Enable for Channel CC6x ENCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn encc61f(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture  Compare Match Rising Edge Interrupt Enable for Channel CC6x ENCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn encc62r(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture  Compare Match Falling Edge Interrupt Enable for Channel CC6x ENCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn encc62f(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt for T12 One Match   ENT12OM"]
    #[inline(always)]
    pub fn ent12om(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt for T12 Period Match   ENT12PM"]
    #[inline(always)]
    pub fn ent12pm(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt for T13 Compare Match   ENT13CM"]
    #[inline(always)]
    pub fn ent13cm(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt for T13 Period Match   ENT13PM"]
    #[inline(always)]
    pub fn ent13pm(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt for Trap Flag   ENTRPF"]
    #[inline(always)]
    pub fn entrpf(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt for Correct Hall Event   ENCHE"]
    #[inline(always)]
    pub fn enche(self) -> crate::common::RegisterFieldBool<12, 1, 0, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt for Wrong Hall Event   ENWHE"]
    #[inline(always)]
    pub fn enwhe(self) -> crate::common::RegisterFieldBool<13, 1, 0, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Idle   ENIDLE. This bit enables the automatic entering of the idle state  bit IDLE will be set  after a wrong hall event has been detected  bit WHE is set . During the idle state  the bit field MCMP is automatically cleared."]
    #[inline(always)]
    pub fn enidle(self) -> crate::common::RegisterFieldBool<14, 1, 0, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Multi Channel Mode Shadow Transfer Interrupt   ENSTR"]
    #[inline(always)]
    pub fn enstr(self) -> crate::common::RegisterFieldBool<15, 1, 0, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ien {
    #[inline(always)]
    fn default() -> Ien {
        <crate::RegValueT<Ien_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imon_SPEC;
impl crate::sealed::RegSpec for Imon_SPEC {
    type DataType = u32;
}
#[doc = "Input Monitoring Register\n resetvalue={Application Reset:0x0}"]
pub type Imon = crate::RegValueT<Imon_SPEC>;

impl Imon {
    #[doc = "Lost Bit Event   LBE. This bit determines if a lost bit event has occurred. A lost bit event        occurs when a selected event occurs again with the previous event        captured  IMON.x remains set  and its lost indicator is enabled  for at        least one of the monitored input signals. The bit can be cleared by        writing a 1 to the same bit position  while writing a 0 has no effect."]
    #[inline(always)]
    pub fn lbe(self) -> crate::common::RegisterFieldBool<0, 1, 0, Imon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Imon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Event indication for input signal CCPOS2   CCPOS2I. The bit determines if the selected event has occurred via an edge        detection. The bit can be cleared by writing a 1 to the same bit        position  while writing a 0 has no effect. The dedicated edge is indicated for a selected event if          Hysteretic like Control or Capture modes are initialized in          T12MSEL.MSEL6x. If these modes are not selected  then all edges will          be indicated as an event for the inputs."]
    #[inline(always)]
    pub fn ccpos0i(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Imon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Imon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Event indication for input signal CCPOS2   CCPOS2I. The bit determines if the selected event has occurred via an edge        detection. The bit can be cleared by writing a 1 to the same bit        position  while writing a 0 has no effect. The dedicated edge is indicated for a selected event if          Hysteretic like Control or Capture modes are initialized in          T12MSEL.MSEL6x. If these modes are not selected  then all edges will          be indicated as an event for the inputs."]
    #[inline(always)]
    pub fn ccpos1i(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Imon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Imon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Event indication for input signal CCPOS2   CCPOS2I. The bit determines if the selected event has occurred via an edge        detection. The bit can be cleared by writing a 1 to the same bit        position  while writing a 0 has no effect. The dedicated edge is indicated for a selected event if          Hysteretic like Control or Capture modes are initialized in          T12MSEL.MSEL6x. If these modes are not selected  then all edges will          be indicated as an event for the inputs."]
    #[inline(always)]
    pub fn ccpos2i(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Imon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Imon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Event indication for input signal CC62IN   CC62INI. The bit determines if the selected event has occurred via an edge        detection. The bit can be cleared by writing a 1 to the same bit        position  while writing a 0 has no effect."]
    #[inline(always)]
    pub fn cc60ini(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Imon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Imon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Event indication for input signal CC62IN   CC62INI. The bit determines if the selected event has occurred via an edge        detection. The bit can be cleared by writing a 1 to the same bit        position  while writing a 0 has no effect."]
    #[inline(always)]
    pub fn cc61ini(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Imon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Imon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Event indication for input signal CC62IN   CC62INI. The bit determines if the selected event has occurred via an edge        detection. The bit can be cleared by writing a 1 to the same bit        position  while writing a 0 has no effect."]
    #[inline(always)]
    pub fn cc62ini(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Imon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Imon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Event indication for input signal CTRAP   CTRAPI. The bit determines if the selected event has occurred via an edge        detection. The bit can be cleared by writing a 1 to the same bit        position  while writing a 0 has no effect."]
    #[inline(always)]
    pub fn ctrapi(self) -> crate::common::RegisterFieldBool<7, 1, 0, Imon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Imon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Event indication for input signal T12HR   T12HRI. The bit determines if the selected event has occurred via an edge        detection. The bit can be cleared by writing a 1 to the same bit        position  while writing a 0 has no effect."]
    #[inline(always)]
    pub fn t12hri(self) -> crate::common::RegisterFieldBool<8, 1, 0, Imon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Imon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Event indication for input signal T13HR   T13HRI. The bit determines if the selected event has occurred via an edge        detection. The bit can be cleared by writing a 1 to the same bit        position  while writing a 0 has no effect."]
    #[inline(always)]
    pub fn t13hri(self) -> crate::common::RegisterFieldBool<9, 1, 0, Imon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Imon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Imon {
    #[inline(always)]
    fn default() -> Imon {
        <crate::RegValueT<Imon_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inp_SPEC;
impl crate::sealed::RegSpec for Inp_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Node Pointer Register\n resetvalue={Application Reset:0x3940}"]
pub type Inp = crate::RegValueT<Inp_SPEC>;

impl Inp {
    #[doc = "Interrupt Node Pointer for Channel CC6x Interrupts  INPCC6x  x 0 1 2 . This bit field defines the service request output activated due to a set        condition for bit CC6xR  if enabled by bit ENCC6xR  or for bit CC6xF  if        enabled by bit ENCC6xF ."]
    #[inline(always)]
    pub fn inpcc60(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Inp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0x3, 1, 0, u8, Inp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Node Pointer for Channel CC6x Interrupts  INPCC6x  x 0 1 2 . This bit field defines the service request output activated due to a set        condition for bit CC6xR  if enabled by bit ENCC6xR  or for bit CC6xF  if        enabled by bit ENCC6xF ."]
    #[inline(always)]
    pub fn inpcc61(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Inp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2, 0x3, 1, 0, u8, Inp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Node Pointer for Channel CC6x Interrupts  INPCC6x  x 0 1 2 . This bit field defines the service request output activated due to a set        condition for bit CC6xR  if enabled by bit ENCC6xR  or for bit CC6xF  if        enabled by bit ENCC6xF ."]
    #[inline(always)]
    pub fn inpcc62(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Inp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4, 0x3, 1, 0, u8, Inp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Node Pointer for the CHE Interrupt   INPCHE. This bit field defines the service request output activated due to a set condition for bit CHE  if enabled by bit ENCHE  of for bit STR  if enabled by bit ENSTR . Coding see INPCC6x."]
    #[inline(always)]
    pub fn inpche(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Inp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6, 0x3, 1, 0, u8, Inp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Node Pointer for Error Interrupts   INPERR. This bit field defines the service request output activated due to a set        condition for bit TRPF  if enabled by bit ENTRPF  or for bit WHE  if        enabled by bit ENWHE . Coding see INPCC6x."]
    #[inline(always)]
    pub fn inperr(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Inp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8, 0x3, 1, 0, u8, Inp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Node Pointer for Timer12 Interrupts   INPT12. This bit field defines the service request output activated due to a set condition for bit T12OM  if enabled by bit ENT12OM  or for bit T12PM  if enabled by bit ENT12PM . Coding see INPCC6x."]
    #[inline(always)]
    pub fn inpt12(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Inp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Inp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for Timer13 Interrupt   INPT13. This bit field defines the service request output activated due to a set condition for bit T13CM  if enabled by bit ENT13CM  or for bit T13PM  if enabled by bit ENT13PM . Coding see INPCC6x."]
    #[inline(always)]
    pub fn inpt13(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Inp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Inp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Inp {
    #[inline(always)]
    fn default() -> Inp {
        <crate::RegValueT<Inp_SPEC> as RegisterValue<_>>::new(14656)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Is_SPEC;
impl crate::sealed::RegSpec for Is_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Status Register\n resetvalue={Application Reset:0x0}"]
pub type Is = crate::RegValueT<Is_SPEC>;

impl Is {
    #[doc = "Capture  Compare Match Rising Edge Flag ICC6xR  x 0 1 2 . This bit indicates that event CC6x R has been detected. This event        occurs in compare mode when a compare match is detected while T12 is        counting up  CM 6x and CDIR  160    160 0  and in capture mode when a rising edge        is detected at the related input CC6xIN."]
    #[inline(always)]
    pub fn icc60r(self) -> crate::common::RegisterFieldBool<0, 1, 0, Is_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Is_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture  Compare Match Falling Edge Flag ICC6xF  x 0 1 2  . This bit indicates that event CC6x F has been detected. This event        occurs in compare mode when a compare match is detected while T12 is        counting down  CM 6x and CDIR  160    160 1  and in capture mode when a falling        edge is detected at the related input CC6xIN."]
    #[inline(always)]
    pub fn icc60f(self) -> crate::common::RegisterFieldBool<1, 1, 0, Is_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Is_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture  Compare Match Rising Edge Flag ICC6xR  x 0 1 2 . This bit indicates that event CC6x R has been detected. This event        occurs in compare mode when a compare match is detected while T12 is        counting up  CM 6x and CDIR  160    160 0  and in capture mode when a rising edge        is detected at the related input CC6xIN."]
    #[inline(always)]
    pub fn icc61r(self) -> crate::common::RegisterFieldBool<2, 1, 0, Is_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Is_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture  Compare Match Falling Edge Flag ICC6xF  x 0 1 2  . This bit indicates that event CC6x F has been detected. This event        occurs in compare mode when a compare match is detected while T12 is        counting down  CM 6x and CDIR  160    160 1  and in capture mode when a falling        edge is detected at the related input CC6xIN."]
    #[inline(always)]
    pub fn icc61f(self) -> crate::common::RegisterFieldBool<3, 1, 0, Is_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Is_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture  Compare Match Rising Edge Flag ICC6xR  x 0 1 2 . This bit indicates that event CC6x R has been detected. This event        occurs in compare mode when a compare match is detected while T12 is        counting up  CM 6x and CDIR  160    160 0  and in capture mode when a rising edge        is detected at the related input CC6xIN."]
    #[inline(always)]
    pub fn icc62r(self) -> crate::common::RegisterFieldBool<4, 1, 0, Is_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Is_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture  Compare Match Falling Edge Flag ICC6xF  x 0 1 2  . This bit indicates that event CC6x F has been detected. This event        occurs in compare mode when a compare match is detected while T12 is        counting down  CM 6x and CDIR  160    160 1  and in capture mode when a falling        edge is detected at the related input CC6xIN."]
    #[inline(always)]
    pub fn icc62f(self) -> crate::common::RegisterFieldBool<5, 1, 0, Is_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Is_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T12 One Match Flag   T12OM. This bit indicates that a timer T12 one match while counting down  T12 OM and CDIR   1  has been detected."]
    #[inline(always)]
    pub fn t12om(self) -> crate::common::RegisterFieldBool<6, 1, 0, Is_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Is_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T12 Period Match Flag   T12PM. This bit indicates that a timer T12 period match while counting up  T12 PM and CDIR   0  has been detected."]
    #[inline(always)]
    pub fn t12pm(self) -> crate::common::RegisterFieldBool<7, 1, 0, Is_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Is_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T13 Compare Match Flag   T13CM. This bit indicates that a timer T13 compare match  CM 63  has been detected."]
    #[inline(always)]
    pub fn t13cm(self) -> crate::common::RegisterFieldBool<8, 1, 0, Is_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Is_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T13 Period Match Flag   T13PM. This bit indicates that a timer T13 period match  T13 PM  has been detected."]
    #[inline(always)]
    pub fn t13pm(self) -> crate::common::RegisterFieldBool<9, 1, 0, Is_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Is_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Trap Flag   TRPF. This bit indicates if a trap condition  input CTRAP   0 or by SW  is   has been detected. If TRPM2  0  it becomes cleared automatically if CTRAP   1 or TRPPEN   0  whereas if TRPM2   1  it has to be cleared by writing RTRPF   1."]
    #[inline(always)]
    pub fn trpf(self) -> crate::common::RegisterFieldBool<10, 1, 0, Is_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Is_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Trap State   TRPS. This bit indicates the actual trap state. It is set if TRPF   1 and becomes cleared according to the mode selected in register TRPCTR."]
    #[inline(always)]
    pub fn trps(self) -> crate::common::RegisterFieldBool<11, 1, 0, Is_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Is_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Correct Hall Event   CHE. This bit indicates that a correct Hall event  CM CHE  has been detected."]
    #[inline(always)]
    pub fn che(self) -> crate::common::RegisterFieldBool<12, 1, 0, Is_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Is_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wrong Hall Event   WHE. This bit indicates that a wrong Hall event  CM WHE  has been detected."]
    #[inline(always)]
    pub fn whe(self) -> crate::common::RegisterFieldBool<13, 1, 0, Is_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Is_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "IDLE State   IDLE. If enabled by ENIDLE   1  this bit is set together with bit WHE and it has to be cleared by SW."]
    #[inline(always)]
    pub fn idle(self) -> crate::common::RegisterFieldBool<14, 1, 0, Is_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Is_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Multi Channel Mode Shadow Transfer Request   STR. This bit indicates that a shadow transfer from MCMPS to MCMP  MCM ST  has taken place."]
    #[inline(always)]
    pub fn str(self) -> crate::common::RegisterFieldBool<15, 1, 0, Is_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Is_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Is {
    #[inline(always)]
    fn default() -> Is {
        <crate::RegValueT<Is_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr_SPEC;
impl crate::sealed::RegSpec for Isr_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Status Reset Register\n resetvalue={Application Reset:0x0}"]
pub type Isr = crate::RegValueT<Isr_SPEC>;

impl Isr {
    #[doc = "Reset Capture  Compare Match Rising Edge Flag   RCC6xR  x 0 1 2"]
    #[inline(always)]
    pub fn rcc60r(self) -> crate::common::RegisterFieldBool<0, 1, 0, Isr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Isr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Capture  Compare Match Falling Edge Flag   RCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn rcc60f(self) -> crate::common::RegisterFieldBool<1, 1, 0, Isr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Isr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Capture  Compare Match Rising Edge Flag   RCC6xR  x 0 1 2"]
    #[inline(always)]
    pub fn rcc61r(self) -> crate::common::RegisterFieldBool<2, 1, 0, Isr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Isr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Capture  Compare Match Falling Edge Flag   RCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn rcc61f(self) -> crate::common::RegisterFieldBool<3, 1, 0, Isr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Isr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Capture  Compare Match Rising Edge Flag   RCC6xR  x 0 1 2"]
    #[inline(always)]
    pub fn rcc62r(self) -> crate::common::RegisterFieldBool<4, 1, 0, Isr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Isr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Capture  Compare Match Falling Edge Flag   RCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn rcc62f(self) -> crate::common::RegisterFieldBool<5, 1, 0, Isr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Isr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Timer T12 One Match Flag   RT12OM"]
    #[inline(always)]
    pub fn rt12om(self) -> crate::common::RegisterFieldBool<6, 1, 0, Isr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Isr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Timer T12 Period Match Flag   RT12PM"]
    #[inline(always)]
    pub fn rt12pm(self) -> crate::common::RegisterFieldBool<7, 1, 0, Isr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Isr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Timer T13 Compare Match Flag   RT13CM"]
    #[inline(always)]
    pub fn rt13cm(self) -> crate::common::RegisterFieldBool<8, 1, 0, Isr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Isr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Timer T13 Period Match Flag   RT13PM"]
    #[inline(always)]
    pub fn rt13pm(self) -> crate::common::RegisterFieldBool<9, 1, 0, Isr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Isr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Trap Flag   RTRPF"]
    #[inline(always)]
    pub fn rtrpf(self) -> crate::common::RegisterFieldBool<10, 1, 0, Isr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Isr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Correct Hall Event Flag   RCHE"]
    #[inline(always)]
    pub fn rche(self) -> crate::common::RegisterFieldBool<12, 1, 0, Isr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, Isr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Wrong Hall Event Flag   RWHE"]
    #[inline(always)]
    pub fn rwhe(self) -> crate::common::RegisterFieldBool<13, 1, 0, Isr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, Isr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset IDLE Flag   RIDLE"]
    #[inline(always)]
    pub fn ridle(self) -> crate::common::RegisterFieldBool<14, 1, 0, Isr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, Isr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset STR Flag   RSTR"]
    #[inline(always)]
    pub fn rstr(self) -> crate::common::RegisterFieldBool<15, 1, 0, Isr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Isr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Isr {
    #[inline(always)]
    fn default() -> Isr {
        <crate::RegValueT<Isr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iss_SPEC;
impl crate::sealed::RegSpec for Iss_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Status Set Register\n resetvalue={Application Reset:0x0}"]
pub type Iss = crate::RegValueT<Iss_SPEC>;

impl Iss {
    #[doc = "Set Capture  Compare Match Rising Edge Flag   SCC6xR  x 0 1 2"]
    #[inline(always)]
    pub fn scc60r(self) -> crate::common::RegisterFieldBool<0, 1, 0, Iss_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Iss_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Capture  Compare Match Falling Edge Flag   SCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn scc60f(self) -> crate::common::RegisterFieldBool<1, 1, 0, Iss_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Iss_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Capture  Compare Match Rising Edge Flag   SCC6xR  x 0 1 2"]
    #[inline(always)]
    pub fn scc61r(self) -> crate::common::RegisterFieldBool<2, 1, 0, Iss_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Iss_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Capture  Compare Match Falling Edge Flag   SCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn scc61f(self) -> crate::common::RegisterFieldBool<3, 1, 0, Iss_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Iss_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Capture  Compare Match Rising Edge Flag   SCC6xR  x 0 1 2"]
    #[inline(always)]
    pub fn scc62r(self) -> crate::common::RegisterFieldBool<4, 1, 0, Iss_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Iss_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Capture  Compare Match Falling Edge Flag   SCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn scc62f(self) -> crate::common::RegisterFieldBool<5, 1, 0, Iss_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Iss_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Timer T12 One Match Flag   ST12OM"]
    #[inline(always)]
    pub fn st12om(self) -> crate::common::RegisterFieldBool<6, 1, 0, Iss_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Iss_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Timer T12 Period Match Flag   ST12PM"]
    #[inline(always)]
    pub fn st12pm(self) -> crate::common::RegisterFieldBool<7, 1, 0, Iss_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Iss_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Timer T13 Compare Match Flag   ST13CM"]
    #[inline(always)]
    pub fn st13cm(self) -> crate::common::RegisterFieldBool<8, 1, 0, Iss_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Iss_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Timer T13 Period Match Flag   ST13PM"]
    #[inline(always)]
    pub fn st13pm(self) -> crate::common::RegisterFieldBool<9, 1, 0, Iss_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Iss_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Trap Flag   STRPF"]
    #[inline(always)]
    pub fn strpf(self) -> crate::common::RegisterFieldBool<10, 1, 0, Iss_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Iss_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Software Hall Compare   SWHC"]
    #[inline(always)]
    pub fn swhc(self) -> crate::common::RegisterFieldBool<11, 1, 0, Iss_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, Iss_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Correct Hall Event Flag   SCHE"]
    #[inline(always)]
    pub fn sche(self) -> crate::common::RegisterFieldBool<12, 1, 0, Iss_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, Iss_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Wrong Hall Event Flag   SWHE"]
    #[inline(always)]
    pub fn swhe(self) -> crate::common::RegisterFieldBool<13, 1, 0, Iss_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, Iss_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set IDLE Flag   SIDLE"]
    #[inline(always)]
    pub fn sidle(self) -> crate::common::RegisterFieldBool<14, 1, 0, Iss_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, Iss_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set STR Flag   SSTR"]
    #[inline(always)]
    pub fn sstr(self) -> crate::common::RegisterFieldBool<15, 1, 0, Iss_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Iss_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Iss {
    #[inline(always)]
    fn default() -> Iss {
        <crate::RegValueT<Iss_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel registers are set. The RST bit will be cleared  re set to  0   by the BPI after the kernel reset was executed."]
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel reset registers are set. The RST bit will be cleared  re set to  0   by the BPI after the kernel reset was executed."]
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
pub struct Kscsr_SPEC;
impl crate::sealed::RegSpec for Kscsr_SPEC {
    type DataType = u32;
}
#[doc = "Kernel State Control Sensitivity Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
pub type Kscsr = crate::RegValueT<Kscsr_SPEC>;

impl Kscsr {
    #[doc = "Sensitivity Block x  SBx  x 0 1 2 3 . This bit defines if block x of the CCU6 kernel is sensitive to Stop Mode 0 or Stop Mode 1. The functional        definition of the blocks is given in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn sb0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Kscsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Kscsr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Sensitivity Block x  SBx  x 0 1 2 3 . This bit defines if block x of the CCU6 kernel is sensitive to Stop Mode 0 or Stop Mode 1. The functional        definition of the blocks is given in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn sb1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Kscsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Kscsr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Sensitivity Block x  SBx  x 0 1 2 3 . This bit defines if block x of the CCU6 kernel is sensitive to Stop Mode 0 or Stop Mode 1. The functional        definition of the blocks is given in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn sb2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Kscsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Kscsr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Sensitivity Block x  SBx  x 0 1 2 3 . This bit defines if block x of the CCU6 kernel is sensitive to Stop Mode 0 or Stop Mode 1. The functional        definition of the blocks is given in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn sb3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Kscsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Kscsr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Kscsr {
    #[inline(always)]
    fn default() -> Kscsr {
        <crate::RegValueT<Kscsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Li_SPEC;
impl crate::sealed::RegSpec for Li_SPEC {
    type DataType = u32;
}
#[doc = "Lost Indicator Register\n resetvalue={Application Reset:0x0}"]
pub type Li = crate::RegValueT<Li_SPEC>;

impl Li {
    #[doc = "Lost Indicator Enable for input signal CCPOS2   CCPOS2EN. This bit determines if the monitored event at the input signal is enabled for the detection of a lost bit event."]
    #[inline(always)]
    pub fn ccpos0en(self) -> crate::common::RegisterFieldBool<1, 1, 0, Li_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Li_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Lost Indicator Enable for input signal CCPOS2   CCPOS2EN. This bit determines if the monitored event at the input signal is enabled for the detection of a lost bit event."]
    #[inline(always)]
    pub fn ccpos1en(self) -> crate::common::RegisterFieldBool<2, 1, 0, Li_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Li_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Lost Indicator Enable for input signal CCPOS2   CCPOS2EN. This bit determines if the monitored event at the input signal is enabled for the detection of a lost bit event."]
    #[inline(always)]
    pub fn ccpos2en(self) -> crate::common::RegisterFieldBool<3, 1, 0, Li_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Li_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Lost Indicator Enable for input signal CC62IN   CC62INEN. This bit determines if the monitored event at the input signal is enabled for the detection of a lost bit event."]
    #[inline(always)]
    pub fn cc60inen(self) -> crate::common::RegisterFieldBool<4, 1, 0, Li_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Li_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Lost Indicator Enable for input signal CC62IN   CC62INEN. This bit determines if the monitored event at the input signal is enabled for the detection of a lost bit event."]
    #[inline(always)]
    pub fn cc61inen(self) -> crate::common::RegisterFieldBool<5, 1, 0, Li_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Li_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Lost Indicator Enable for input signal CC62IN   CC62INEN. This bit determines if the monitored event at the input signal is enabled for the detection of a lost bit event."]
    #[inline(always)]
    pub fn cc62inen(self) -> crate::common::RegisterFieldBool<6, 1, 0, Li_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Li_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Lost Indicator Enable for input signal CTRAP   CTRAPEN. This bit determines if the monitored event at the input signal is enabled for the detection of a lost bit event."]
    #[inline(always)]
    pub fn ctrapen(self) -> crate::common::RegisterFieldBool<7, 1, 0, Li_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Li_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Lost Indicator Enable for input signal T12HR   T12HREN. This bit determines if the monitored event at the input signal is enabled for the detection of a lost bit event."]
    #[inline(always)]
    pub fn t12hren(self) -> crate::common::RegisterFieldBool<8, 1, 0, Li_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Li_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Lost Indicator Enable for input signal T13HR   T13HREN. This bit determines if the monitored event at the input signal is enabled for the detection of a lost bit event."]
    #[inline(always)]
    pub fn t13hren(self) -> crate::common::RegisterFieldBool<9, 1, 0, Li_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Li_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Enable for Lost Bit Event   LBEEN. This bit determines if a SRx line is activated if lost bit event is detected."]
    #[inline(always)]
    pub fn lbeen(self) -> crate::common::RegisterFieldBool<13, 1, 0, Li_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Li_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Node Pointer for lost bit event   INPLBE. This bit field defines which service request output line is selected to output an lost event alert for an enabled lost bit event."]
    #[inline(always)]
    pub fn inplbe(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Li_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14, 0x3, 1, 0, u8, Li_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Li {
    #[inline(always)]
    fn default() -> Li {
        <crate::RegValueT<Li_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcfg_SPEC;
impl crate::sealed::RegSpec for Mcfg_SPEC {
    type DataType = u32;
}
#[doc = "Module Configuration Register\n resetvalue={Application Reset:0x08007,Application Reset:0x7}"]
pub type Mcfg = crate::RegValueT<Mcfg_SPEC>;

impl Mcfg {
    #[doc = "T12 Available   T12. This bit indicates if the T12 block is available."]
    #[inline(always)]
    pub fn t12(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mcfg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mcfg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "T13 Available   T13. This bit indicates if the T13 block is available."]
    #[inline(always)]
    pub fn t13(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mcfg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mcfg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Multi Channel Mode Available   MCM. This bit indicates if the multi channel mode functionality is available."]
    #[inline(always)]
    pub fn mcm(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mcfg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mcfg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mcfg {
    #[inline(always)]
    fn default() -> Mcfg {
        <crate::RegValueT<Mcfg_SPEC> as RegisterValue<_>>::new(7)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcmctr_SPEC;
impl crate::sealed::RegSpec for Mcmctr_SPEC {
    type DataType = u32;
}
#[doc = "Multi Channel Mode Control Register\n resetvalue={Application Reset:0x0}"]
pub type Mcmctr = crate::RegValueT<Mcmctr_SPEC>;

impl Mcmctr {
    #[doc = "Switching Selection   SWSEL. Bit field SWSEL selects one of the following trigger request sources  next multi channel event  for the shadow transfer MCM ST from MCMPS to MCMP. The trigger request is stored in the reminder flag R until the shadow transfer is done and flag R is cleared automatically with the shadow transfer. The shadow transfer takes place synchronously with an event selected in bit field SWSYN."]
    #[inline(always)]
    pub fn swsel(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Mcmctr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Mcmctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Switching Synchronization   SWSYN. Bit field SWSYN defines the synchronization mechanism of the shadow transfer event MCM ST if it has been requested before  flag R set by an event selected by SWSEL  and if MCMEN   1. This feature permits the synchronization of the outputs to the PWM source  that is used for modulation  T12 or T13 ."]
    #[inline(always)]
    pub fn swsyn(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Mcmctr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Mcmctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Shadow Transfer Enable for T12 Upcounting   STE12U. This bit enables the shadow transfer T12 ST if flag MCMOUT.R is set or becomes set while a T12 period match is detected while counting up."]
    #[inline(always)]
    pub fn ste12u(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Mcmctr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Mcmctr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Shadow Transfer Enable for T12 Downcounting   STE12D. This bit enables the shadow transfer T12 ST if flag MCMOUT.R is set or becomes set while a T12 one match is detected while counting down."]
    #[inline(always)]
    pub fn ste12d(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Mcmctr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Mcmctr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Shadow Transfer Enable for T13 Upcounting   STE13U. This bit enables the shadow transfer T13 ST if flag MCMOUT.R is set or becomes set while a T13 period match is detected."]
    #[inline(always)]
    pub fn ste13u(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Mcmctr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Mcmctr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mcmctr {
    #[inline(always)]
    fn default() -> Mcmctr {
        <crate::RegValueT<Mcmctr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcmout_SPEC;
impl crate::sealed::RegSpec for Mcmout_SPEC {
    type DataType = u32;
}
#[doc = "Multi Channel Mode Output Register\n resetvalue={Application Reset:0x0}"]
pub type Mcmout = crate::RegValueT<Mcmout_SPEC>;

impl Mcmout {
    #[doc = "Multi Channel PWM Pattern   MCMP. Bit field MCMP defines the output pattern for the multi channel mode. If this mode is enabled by MODCTR.MCMEN   1  the output state of all T12 related PWM outputs can be modified. This bit field is 0 while IS.IDLE   1. MCMP0   MCMOUT.0 for output CC60 MCMP1   MCMOUT.1 for output COUT60 MCMP2   MCMOUT.2 for output CC61 MCMP3   MCMOUT.3 for output COUT61 MCMP4   MCMOUT.4 for output CC62 MCMP5   MCMOUT.5 for output COUT62"]
    #[inline(always)]
    pub fn mcmp(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Mcmout_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Mcmout_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reminder Flag   R. This flag indicates that the shadow transfer from MCMPS to MCMP has been requested by the selected trigger source. It is cleared when the shadow transfer takes place or while MCMEN 0."]
    #[inline(always)]
    pub fn r(self) -> crate::common::RegisterFieldBool<6, 1, 0, Mcmout_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Mcmout_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Expected Hall Pattern   EXPH. Bit field EXPH is updated by a shadow transfer HP ST from bit field EXPHS. If HCRDY   1  EXPH is compared to the sampled CCPOSx inputs in order to detect the occurrence of the next desired   expected  hall pattern or a wrong pattern. If the sampled hall pattern at the hall input pins is equal to bit field EXPH  a correct Hall event has been detected  CM CHE ."]
    #[inline(always)]
    pub fn exph(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Mcmout_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Mcmout_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Current Hall Pattern   CURH. Bit field CURH is updated by a shadow transfer HP ST from bit field CURHS. If HCRDY   1  CURH is compared to the sampled CCPOSx inputs in order to detect a spike. If the sampled Hall pattern at the Hall input pins is equal to bit field CURH  no Hall event has been detected. If the sampled Hall input pattern is neither equal to CURH nor equal to EXPH  the Hall event was not the desired one and may be due to a fatal error  e.g. blocked rotor  etc. . In this case  a wrong Hall event has been detected  CM WHE ."]
    #[inline(always)]
    pub fn curh(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Mcmout_SPEC, crate::common::R> {
        crate::common::RegisterField::<11,0x7,1,0,u8, Mcmout_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Mcmout {
    #[inline(always)]
    fn default() -> Mcmout {
        <crate::RegValueT<Mcmout_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcmouts_SPEC;
impl crate::sealed::RegSpec for Mcmouts_SPEC {
    type DataType = u32;
}
#[doc = "Multi Channel Mode Output Shadow Register\n resetvalue={Application Reset:0x0}"]
pub type Mcmouts = crate::RegValueT<Mcmouts_SPEC>;

impl Mcmouts {
    #[doc = "Multi Channel PWM Pattern Shadow   MCMPS. Bit field MCMPS is the shadow bit field for bit field MCMP. The multi channel shadow transfer is triggered by MCM ST according to the transfer conditions defined by register MCMCTR."]
    #[inline(always)]
    pub fn mcmps(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Mcmouts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Mcmouts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Shadow Transfer Request for MCMPS   STRMCM. Writing STRMCM  160    160 1 leads to an immediate activation of MCM ST to update        bit field MCMP by the value of MCMPS. When read  this bit always        delivers 0."]
    #[inline(always)]
    pub fn strmcm(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Mcmouts_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Mcmouts_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Expected Hall Pattern Shadow   EXPHS. Bit field EXPHS is the shadow bit field for bit field EXPH. The shadow transfer takes place when a correct Hall event is detected  CM CHE ."]
    #[inline(always)]
    pub fn exphs(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Mcmouts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Mcmouts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Current Hall Pattern Shadow   CURHS. Bit field CURHS is the shadow bit field for bit field CURH. The shadow transfer takes place when a correct Hall event is detected  CM CHE ."]
    #[inline(always)]
    pub fn curhs(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Mcmouts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x7,1,0,u8, Mcmouts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Shadow Transfer Request for the Hall Pattern   STRHP. Writing STRHP   1 leads to an immediate activation of HP ST to update bit fields EXPH and CURH by EXPHS and CURHS. When read  this bit always delivers 0."]
    #[inline(always)]
    pub fn strhp(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Mcmouts_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Mcmouts_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mcmouts {
    #[inline(always)]
    fn default() -> Mcmouts {
        <crate::RegValueT<Mcmouts_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modctr_SPEC;
impl crate::sealed::RegSpec for Modctr_SPEC {
    type DataType = u32;
}
#[doc = "Modulation Control Register\n resetvalue={Application Reset:0x0}"]
pub type Modctr = crate::RegValueT<Modctr_SPEC>;

impl Modctr {
    #[doc = "T12 Modulation Enable   T12MODEN. These bits enable the modulation of the corresponding output signal by a PWM pattern generated by timer T12. T12MODEN0   MODCTR.0 for output CC60 T12MODEN1   MODCTR.1 for output COUT60 T12MODEN2   MODCTR.2 for output CC61 T12MODEN3   MODCTR.3 for output COUT61 T12MODEN4   MODCTR.4 for output CC62 T12MODEN5   MODCTR.5 for output COUT62"]
    #[inline(always)]
    pub fn t12moden(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Modctr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Modctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Multi Channel Mode Enable   MCMEN"]
    #[inline(always)]
    pub fn mcmen(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Modctr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Modctr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "T13 Modulation Enable   T13MODEN. These bits enable the modulation of the corresponding output signal by the PWM pattern CC63 O generated by timer T13. T13MODEN0   MODCTR.8 for output CC60 T13MODEN1   MODCTR.9 for output COUT60 T13MODEN2   MODCTR.10 for output CC61 T13MODEN3   MODCTR.11 for output COUT61 T13MODEN4   MODCTR.12 for output CC62 T13MODEN5   MODCTR.13 for output COUT62"]
    #[inline(always)]
    pub fn t13moden(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, Modctr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3f,1,0,u8, Modctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Compare Timer T13 Output   ECT13O"]
    #[inline(always)]
    pub fn ect13o(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Modctr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Modctr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Modctr {
    #[inline(always)]
    fn default() -> Modctr {
        <crate::RegValueT<Modctr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mosel_SPEC;
impl crate::sealed::RegSpec for Mosel_SPEC {
    type DataType = u32;
}
#[doc = "CCU60 Module Output Select Register\n resetvalue={Application Reset:0x0}"]
pub type Mosel = crate::RegValueT<Mosel_SPEC>;

impl Mosel {
    #[doc = "Output Trigger Select for CCU6061 TRIG2. This bit field defines the output signal from the CCU6061 module used as        the trigger signal to the EVADC inputs."]
    #[inline(always)]
    pub fn trig0sel(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Mosel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Mosel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Output Trigger Select for CCU6061 TRIG2. This bit field defines the output signal from the CCU6061 module used as        the trigger signal to the EVADC inputs."]
    #[inline(always)]
    pub fn trig1sel(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, Mosel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x7,1,0,u8, Mosel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Output Trigger Select for CCU6061 TRIG2. This bit field defines the output signal from the CCU6061 module used as        the trigger signal to the EVADC inputs."]
    #[inline(always)]
    pub fn trig2sel(
        self,
    ) -> crate::common::RegisterField<6, 0x7, 1, 0, u8, Mosel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x7,1,0,u8, Mosel_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Mosel {
    #[inline(always)]
    fn default() -> Mosel {
        <crate::RegValueT<Mosel_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "OCDS Suspend Control   SUS. Controls the sensitivity to the suspend signal coming from the OCDS        Trigger Switch  OTGS  Effects of soft suspend options on CCU6 Functional Blocks are described        in section CROSSREFERENCE"]
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
pub struct Pisel0_SPEC;
impl crate::sealed::RegSpec for Pisel0_SPEC {
    type DataType = u32;
}
#[doc = "Port Input Select Register 0\n resetvalue={Application Reset:0x0}"]
pub type Pisel0 = crate::RegValueT<Pisel0_SPEC>;

impl Pisel0 {
    #[doc = "Input Select for CC60 ISCC6x  x 0 1 2 . This bit field defines the input signal used as CC6x capture input."]
    #[inline(always)]
    pub fn iscc60(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Pisel0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Pisel0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for CC60 ISCC6x  x 0 1 2 . This bit field defines the input signal used as CC6x capture input."]
    #[inline(always)]
    pub fn iscc61(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Pisel0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Pisel0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for CC60 ISCC6x  x 0 1 2 . This bit field defines the input signal used as CC6x capture input."]
    #[inline(always)]
    pub fn iscc62(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Pisel0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Pisel0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for CTRAP   ISTRP. This bit field defines the input signal used as CTRAP input."]
    #[inline(always)]
    pub fn istrp(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Pisel0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Pisel0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for CCPOS0 ISPOSx  x 0 1 2 . This bit field defines the input signal used as CCPOSx input."]
    #[inline(always)]
    pub fn ispos0(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Pisel0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Pisel0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for CCPOS0 ISPOSx  x 0 1 2 . This bit field defines the input signal used as CCPOSx input."]
    #[inline(always)]
    pub fn ispos1(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Pisel0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Pisel0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for CCPOS0 ISPOSx  x 0 1 2 . This bit field defines the input signal used as CCPOSx input."]
    #[inline(always)]
    pub fn ispos2(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Pisel0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Pisel0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for T12HR   IST12HR. This bit field defines the input signal used as T12HR input."]
    #[inline(always)]
    pub fn ist12hr(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Pisel0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Pisel0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pisel0 {
    #[inline(always)]
    fn default() -> Pisel0 {
        <crate::RegValueT<Pisel0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pisel2_SPEC;
impl crate::sealed::RegSpec for Pisel2_SPEC {
    type DataType = u32;
}
#[doc = "Port Input Select Register 2\n resetvalue={Application Reset:0x0}"]
pub type Pisel2 = crate::RegValueT<Pisel2_SPEC>;

impl Pisel2 {
    #[doc = "Input Select for T13HR   IST13HR. This bit field defines the input signal used as T13HR input."]
    #[inline(always)]
    pub fn ist13hr(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Pisel2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Pisel2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for T12 Counting Input   ISCNT12. This bit field defines the input event leading to a counting action of T12."]
    #[inline(always)]
    pub fn iscnt12(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Pisel2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Pisel2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for T13 Counting Input   ISCNT13. This bit field defines the input event leading to a counting action of T13."]
    #[inline(always)]
    pub fn iscnt13(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Pisel2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Pisel2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extension for T12HR Inputs   T12EXT. This bit extends the 2 bit field IST12HR."]
    #[inline(always)]
    pub fn t12ext(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Pisel2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Pisel2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Extension for T13HR Inputs   T13EXT. This bit extends the 2 bit field IST13HR."]
    #[inline(always)]
    pub fn t13ext(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Pisel2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Pisel2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Pisel2 {
    #[inline(always)]
    fn default() -> Pisel2 {
        <crate::RegValueT<Pisel2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pslr_SPEC;
impl crate::sealed::RegSpec for Pslr_SPEC {
    type DataType = u32;
}
#[doc = "Passive State Level Register\n resetvalue={Application Reset:0x0}"]
pub type Pslr = crate::RegValueT<Pslr_SPEC>;

impl Pslr {
    #[doc = "Compare Outputs Passive State Level   PSL. These bits define the passive level driven by the module outputs during the passive state. PSL0   PSLR.0 for output CC60 PSL1   PSLR.1 for output COUT60 PSL2   PSLR.2 for output CC61 PSL3   PSLR.3 for output COUT61 PSL4   PSLR.4 for output CC62 PSL5   PSLR.5 for output COUT62"]
    #[inline(always)]
    pub fn psl(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Pslr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Pslr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Passive State Level of Output COUT63   PSL63. This bit defines the passive level driven by the module output COUT63 during the passive state."]
    #[inline(always)]
    pub fn psl63(self) -> crate::common::RegisterFieldBool<7, 1, 0, Pslr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Pslr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Pslr {
    #[inline(always)]
    fn default() -> Pslr {
        <crate::RegValueT<Pslr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T12_SPEC;
impl crate::sealed::RegSpec for T12_SPEC {
    type DataType = u32;
}
#[doc = "Timer T12 Counter Register\n resetvalue={Application Reset:0x0}"]
pub type T12 = crate::RegValueT<T12_SPEC>;

impl T12 {
    #[doc = "Timer 12 Counter Value   T12CV. This register represents the 16 bit counter value of Timer12."]
    #[inline(always)]
    pub fn t12cv(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, T12_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, T12_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T12 {
    #[inline(always)]
    fn default() -> T12 {
        <crate::RegValueT<T12_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T12Dtc_SPEC;
impl crate::sealed::RegSpec for T12Dtc_SPEC {
    type DataType = u32;
}
#[doc = "Dead Time Control Register for Timer12\n resetvalue={Application Reset:0x0}"]
pub type T12Dtc = crate::RegValueT<T12Dtc_SPEC>;

impl T12Dtc {
    #[doc = "Dead Time   DTM. Bit field DTM determines the programmable delay between switching from        the passive state to the active state of the selected outputs. The        switching from the active state to the passive state is not delayed."]
    #[inline(always)]
    pub fn dtm(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, T12Dtc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, T12Dtc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Dead Time Enable Bits DTEx  x 0 1 2  . Bits DTE0  8230 DTE2 enable and disable the dead time generation for each        compare channel  0  1  2  of timer T12."]
    #[inline(always)]
    pub fn dte0(self) -> crate::common::RegisterFieldBool<8, 1, 0, T12Dtc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, T12Dtc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Dead Time Enable Bits DTEx  x 0 1 2  . Bits DTE0  8230 DTE2 enable and disable the dead time generation for each        compare channel  0  1  2  of timer T12."]
    #[inline(always)]
    pub fn dte1(self) -> crate::common::RegisterFieldBool<9, 1, 0, T12Dtc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, T12Dtc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Dead Time Enable Bits DTEx  x 0 1 2  . Bits DTE0  8230 DTE2 enable and disable the dead time generation for each        compare channel  0  1  2  of timer T12."]
    #[inline(always)]
    pub fn dte2(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, T12Dtc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, T12Dtc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Dead Time Run Indication Bits DTRx  x 1 2 3 . Bits DTR0  8230 DTR2 indicate the status of the dead time generation for each        compare channel  0  1  2  of timer T12."]
    #[inline(always)]
    pub fn dtr0(self) -> crate::common::RegisterFieldBool<12, 1, 0, T12Dtc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, T12Dtc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Dead Time Run Indication Bits DTRx  x 1 2 3 . Bits DTR0  8230 DTR2 indicate the status of the dead time generation for each        compare channel  0  1  2  of timer T12."]
    #[inline(always)]
    pub fn dtr1(self) -> crate::common::RegisterFieldBool<13, 1, 0, T12Dtc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, T12Dtc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Dead Time Run Indication Bits DTRx  x 1 2 3 . Bits DTR0  8230 DTR2 indicate the status of the dead time generation for each        compare channel  0  1  2  of timer T12."]
    #[inline(always)]
    pub fn dtr2(self) -> crate::common::RegisterFieldBool<14, 1, 0, T12Dtc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, T12Dtc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for T12Dtc {
    #[inline(always)]
    fn default() -> T12Dtc {
        <crate::RegValueT<T12Dtc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T12Msel_SPEC;
impl crate::sealed::RegSpec for T12Msel_SPEC {
    type DataType = u32;
}
#[doc = "T12 Mode Select Register\n resetvalue={Application Reset:0x0}"]
pub type T12Msel = crate::RegValueT<T12Msel_SPEC>;

impl T12Msel {
    #[doc = "Capture Compare Mode Selection MSEL6x  x 0 1 2 . These bit fields select the operating mode of the three T12        capture compare channels. Each channel  x   0  1  2  can be programmed        individually for one of these modes  except for Hall Sensor Mode .        Coding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn msel60(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, T12Msel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, T12Msel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Capture Compare Mode Selection MSEL6x  x 0 1 2 . These bit fields select the operating mode of the three T12        capture compare channels. Each channel  x   0  1  2  can be programmed        individually for one of these modes  except for Hall Sensor Mode .        Coding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn msel61(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, T12Msel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, T12Msel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Capture Compare Mode Selection MSEL6x  x 0 1 2 . These bit fields select the operating mode of the three T12        capture compare channels. Each channel  x   0  1  2  can be programmed        individually for one of these modes  except for Hall Sensor Mode .        Coding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn msel62(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, T12Msel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, T12Msel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Hall Synchronization   HSYNC. Bit field HSYNC defines the source for the sampling of the Hall input        pattern and the comparison to the current and the expected Hall pattern        bit fields. Coding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn hsync(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, T12Msel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8, T12Msel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Delay Bypass   DBYP. DBYP controls whether the source signal for the sampling of the Hall        input pattern  selected by HSYNC  is delayed by the Dead Time Counter 0."]
    #[inline(always)]
    pub fn dbyp(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, T12Msel_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, T12Msel_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for T12Msel {
    #[inline(always)]
    fn default() -> T12Msel {
        <crate::RegValueT<T12Msel_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T12Pr_SPEC;
impl crate::sealed::RegSpec for T12Pr_SPEC {
    type DataType = u32;
}
#[doc = "Timer 12 Period Register\n resetvalue={Application Reset:0x0}"]
pub type T12Pr = crate::RegValueT<T12Pr_SPEC>;

impl T12Pr {
    #[doc = "T12 Period Value   T12PV. The value T12PV defines the counter value for T12 leading to a        period match. When reaching this value  the timer T12 is set to zero         edge aligned mode  or changes its count direction to down counting         center aligned mode ."]
    #[inline(always)]
    pub fn t12pv(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, T12Pr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, T12Pr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T12Pr {
    #[inline(always)]
    fn default() -> T12Pr {
        <crate::RegValueT<T12Pr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T13_SPEC;
impl crate::sealed::RegSpec for T13_SPEC {
    type DataType = u32;
}
#[doc = "Timer T13 Counter Register\n resetvalue={Application Reset:0x0}"]
pub type T13 = crate::RegValueT<T13_SPEC>;

impl T13 {
    #[doc = "Timer 13 Counter Value   T13CV. This register represents the 16 bit counter value of Timer13."]
    #[inline(always)]
    pub fn t13cv(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, T13_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, T13_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T13 {
    #[inline(always)]
    fn default() -> T13 {
        <crate::RegValueT<T13_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T13Pr_SPEC;
impl crate::sealed::RegSpec for T13Pr_SPEC {
    type DataType = u32;
}
#[doc = "Timer 13 Period Register\n resetvalue={Application Reset:0x0}"]
pub type T13Pr = crate::RegValueT<T13Pr_SPEC>;

impl T13Pr {
    #[doc = "T13 Period Value   T13PV. The value T13PV defines the counter value for T13 leading to a period match. When reaching this value  the timer T13 is set to zero."]
    #[inline(always)]
    pub fn t13pv(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, T13Pr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, T13Pr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T13Pr {
    #[inline(always)]
    fn default() -> T13Pr {
        <crate::RegValueT<T13Pr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tctr0_SPEC;
impl crate::sealed::RegSpec for Tctr0_SPEC {
    type DataType = u32;
}
#[doc = "Timer Control Register 0\n resetvalue={Application Reset:0x0}"]
pub type Tctr0 = crate::RegValueT<Tctr0_SPEC>;

impl Tctr0 {
    #[doc = "Timer T12 Input Clock Select   T12CLK. Selects the input clock for timer T12 that is derived from the peripheral clock according to the equation f T12   f CC6   2  lt T12CLK gt  ."]
    #[inline(always)]
    pub fn t12clk(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Tctr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Tctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T12 Prescaler Bit   T12PRE. In order to support higher clock frequencies  an additional prescaler factor of 1 256 can be enabled for the prescaler for T12."]
    #[inline(always)]
    pub fn t12pre(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Tctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Tctr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T12 Run Bit   T12R. T12R starts and stops timer T12. It is set cleared by SW by setting bits T12RR or T12RS or it is cleared by HW according to the function defined by bit field T12SSC."]
    #[inline(always)]
    pub fn t12r(self) -> crate::common::RegisterFieldBool<4, 1, 0, Tctr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Tctr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T12 Shadow Transfer Enable   STE12. Bit STE12 enables or disables the shadow transfer of the T12 period value  the compare values and passive state select bits and levels from their shadow registers to the actual registers if a T12 shadow transfer event is detected. Bit STE12 is cleared by hardware after the shadow transfer. A T12 shadow transfer event is a period match while counting up or a one match while counting down."]
    #[inline(always)]
    pub fn ste12(self) -> crate::common::RegisterFieldBool<5, 1, 0, Tctr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Tctr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Count Direction of Timer T12   CDIR. This bit is set cleared according to the counting rules of T12."]
    #[inline(always)]
    pub fn cdir(self) -> crate::common::RegisterFieldBool<6, 1, 0, Tctr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Tctr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "T12 Operating Mode   CTM"]
    #[inline(always)]
    pub fn ctm(self) -> crate::common::RegisterFieldBool<7, 1, 0, Tctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Tctr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T13 Input Clock Select   T13CLK. Selects the input clock for timer T13 that is derived from the peripheral clock according to the equation f T13   f CC6   2  lt T13CLK gt  ."]
    #[inline(always)]
    pub fn t13clk(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Tctr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Tctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T13 Prescaler Bit   T13PRE. In order to support higher clock frequencies  an additional prescaler factor of 1 256 can be enabled for the prescaler for T13."]
    #[inline(always)]
    pub fn t13pre(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Tctr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Tctr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T13 Run Bit   T13R. T13R starts and stops timer T13. It is set cleared by SW by setting bits T13RR orT13RS or it is set cleared by HW according to the function defined by bit fields T13SSC  T13TEC and T13TED."]
    #[inline(always)]
    pub fn t13r(self) -> crate::common::RegisterFieldBool<12, 1, 0, Tctr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Tctr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T13 Shadow Transfer Enable   STE13. Bit STE13 enables or disables the shadow transfer of the T13 period value  the compare value and passive state select bit and level from their shadow registers to the actual registers if a T13 shadow transfer event is detected. Bit STE13 is cleared by hardware after the shadow transfer. A T13 shadow transfer event is a period match."]
    #[inline(always)]
    pub fn ste13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Tctr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Tctr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Tctr0 {
    #[inline(always)]
    fn default() -> Tctr0 {
        <crate::RegValueT<Tctr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tctr2_SPEC;
impl crate::sealed::RegSpec for Tctr2_SPEC {
    type DataType = u32;
}
#[doc = "Timer Control Register 2\n resetvalue={Application Reset:0x0}"]
pub type Tctr2 = crate::RegValueT<Tctr2_SPEC>;

impl Tctr2 {
    #[doc = "Timer T12 Single Shot Control   T12SSC. This bit controls the single shot mode of T12."]
    #[inline(always)]
    pub fn t12ssc(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Tctr2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tctr2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T13 Single Shot Control   T13SSC. This bit controls the single shot mode of T13."]
    #[inline(always)]
    pub fn t13ssc(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Tctr2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Tctr2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "T13 Trigger Event Control   T13TEC. Bit field T13TEC selects the trigger event to start T13  automatic set of T13R for synchronization to T12 compare signals  according to following combinations"]
    #[inline(always)]
    pub fn t13tec(
        self,
    ) -> crate::common::RegisterField<2, 0x7, 1, 0, u8, Tctr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x7,1,0,u8, Tctr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T13 Trigger Event Direction   T13TED. Bit field T13TED delivers additional information to control the automatic set of bit T13R in the case that the trigger action defined by T13TEC is detected."]
    #[inline(always)]
    pub fn t13ted(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, Tctr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8, Tctr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T12 External Run Selection   T12RSEL. Bit field T12RSEL defines the event of signal T12HR that can set the run bit T12R by HW."]
    #[inline(always)]
    pub fn t12rsel(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Tctr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Tctr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T13 External Run Selection   T13RSEL. Bit field T13RSEL defines the event of signal T13HR that can set the run bit T13R by HW."]
    #[inline(always)]
    pub fn t13rsel(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Tctr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Tctr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Tctr2 {
    #[inline(always)]
    fn default() -> Tctr2 {
        <crate::RegValueT<Tctr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tctr4_SPEC;
impl crate::sealed::RegSpec for Tctr4_SPEC {
    type DataType = u32;
}
#[doc = "Timer Control Register 4\n resetvalue={Application Reset:0x0}"]
pub type Tctr4 = crate::RegValueT<Tctr4_SPEC>;

impl Tctr4 {
    #[doc = "Timer T12 Run Reset   T12RR. Setting this bit clears the T12R bit."]
    #[inline(always)]
    pub fn t12rr(self) -> crate::common::RegisterFieldBool<0, 1, 0, Tctr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tctr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T12 Run Set   T12RS. Setting this bit sets the T12R bit."]
    #[inline(always)]
    pub fn t12rs(self) -> crate::common::RegisterFieldBool<1, 1, 0, Tctr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Tctr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T12 Reset   T12RES"]
    #[inline(always)]
    pub fn t12res(self) -> crate::common::RegisterFieldBool<2, 1, 0, Tctr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Tctr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Dead Time Counter Reset   DTRES"]
    #[inline(always)]
    pub fn dtres(self) -> crate::common::RegisterFieldBool<3, 1, 0, Tctr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Tctr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T12 Count Event   T12CNT"]
    #[inline(always)]
    pub fn t12cnt(self) -> crate::common::RegisterFieldBool<5, 1, 0, Tctr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Tctr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T12 Shadow Transfer Request   T12STR"]
    #[inline(always)]
    pub fn t12str(self) -> crate::common::RegisterFieldBool<6, 1, 0, Tctr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Tctr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T12 Shadow Transfer Disable   T12STD"]
    #[inline(always)]
    pub fn t12std(self) -> crate::common::RegisterFieldBool<7, 1, 0, Tctr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Tctr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T13 Run Reset   T13RR. Setting this bit clears the T13R bit."]
    #[inline(always)]
    pub fn t13rr(self) -> crate::common::RegisterFieldBool<8, 1, 0, Tctr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Tctr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T13 Run Set   T13RS. Setting this bit sets the T13R bit."]
    #[inline(always)]
    pub fn t13rs(self) -> crate::common::RegisterFieldBool<9, 1, 0, Tctr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Tctr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T13 Reset   T13RES"]
    #[inline(always)]
    pub fn t13res(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Tctr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Tctr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T13 Count Event   T13CNT"]
    #[inline(always)]
    pub fn t13cnt(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Tctr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, Tctr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T13 Shadow Transfer Request   T13STR"]
    #[inline(always)]
    pub fn t13str(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Tctr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, Tctr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer T13 Shadow Transfer Disable   T13STD"]
    #[inline(always)]
    pub fn t13std(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Tctr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Tctr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Tctr4 {
    #[inline(always)]
    fn default() -> Tctr4 {
        <crate::RegValueT<Tctr4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trpctr_SPEC;
impl crate::sealed::RegSpec for Trpctr_SPEC {
    type DataType = u32;
}
#[doc = "Trap Control Register\n resetvalue={Application Reset:0x0}"]
pub type Trpctr = crate::RegValueT<Trpctr_SPEC>;

impl Trpctr {
    #[doc = "Trap Mode Control Bit 0   TRPM0. Together with bit TRPM1  these two bits define the behavior of the        selected outputs when leaving the trap state after the trap condition        has become inactive again. A synchronization to the timer driving the        PWM pattern avoids unintended pulses when leaving the trap state. The        behavior resulting from the combination  TRPM1  TRPM0  is described in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn trpm0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Trpctr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Trpctr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Trap Mode Control Bit 1   TRPM1. Together with bit TRPM0  these two bits define the behavior of the        selected outputs when leaving the trap state after the trap condition        has become inactive again. A synchronization to the timer driving the        PWM pattern avoids unintended pulses when leaving the trap state. The        behavior resulting from the combination  TRPM1  TRPM0  is described in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn trpm1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Trpctr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Trpctr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Trap Mode Control Bit 2   TRPM2. This bit defines how the trap flag TRPF can be cleared after the trap        input condition   CTRAP   160    160 0        and TRPPEN  160    160 1  is no longer valid  either by CTRAP   160    160 1        or by TRPPEN  160    160 0 ."]
    #[inline(always)]
    pub fn trpm2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Trpctr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Trpctr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Trap Enable Control   TRPEN. Setting a bit enables the trap functionality for the following corresponding output signals  TRPEN0   TRPCTR.8 for output CC60 TRPEN1   TRPCTR.9 for output COUT60 TRPEN2   TRPCTR.10 for output CC61 TRPEN3   TRPCTR.11 for output COUT61 TRPEN4   TRPCTR.12 for output CC62 TRPEN5   TRPCTR.13 for output COUT62"]
    #[inline(always)]
    pub fn trpen(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, Trpctr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3f,1,0,u8, Trpctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trap Enable Control for Timer T13   TRPEN13"]
    #[inline(always)]
    pub fn trpen13(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Trpctr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Trpctr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Trap Pin Enable   TRPPEN. This bit enables the input  pin  function for the trap generation. An interrupt can only be generated if a falling edge is detected at pin CTRAP while TRPPEN   1."]
    #[inline(always)]
    pub fn trppen(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Trpctr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Trpctr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Trpctr {
    #[inline(always)]
    fn default() -> Trpctr {
        <crate::RegValueT<Trpctr_SPEC> as RegisterValue<_>>::new(0)
    }
}
