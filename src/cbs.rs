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
#[doc = r"CBS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbs(pub(super) *mut u8);
unsafe impl core::marker::Send for Cbs {}
unsafe impl core::marker::Sync for Cbs {}
impl Cbs {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(508usize)) }
    }

    #[doc = "Communication Mode Data Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn comdata(&self) -> crate::common::Reg<self::Comdata_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(104usize)) }
    }

    #[doc = "Internally Controlled Trace Source Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn ictsa(&self) -> crate::common::Reg<self::Ictsa_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }

    #[doc = "Internally Controlled Trace Destination Register\n resetvalue={PowerOn Reset:0x10F068,Application Reset:0x10F068,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn ictta(&self) -> crate::common::Reg<self::Ictta_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(140usize)) }
    }

    #[doc = "IFS Address Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ifsa(&self) -> crate::common::Reg<self::Ifsa_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(240usize)) }
    }

    #[doc = "IFS Control Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ifsc(&self) -> crate::common::Reg<self::Ifsc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(244usize)) }
    }

    #[doc = "Internal Mode Status and Control Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn intmod(&self) -> crate::common::Reg<self::Intmod_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }

    #[doc = "IOClientStatus and Control Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn iosr(&self) -> crate::common::Reg<self::Iosr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(108usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={PowerOn Reset:0x6360,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn jdpid(&self) -> crate::common::Reg<self::Jdpid_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "JTAGDevice Identification Register\n resetvalue={PowerOn Reset:0x0,CFS Value:0x10207083,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn jtagid(&self) -> crate::common::Reg<self::Jtagid_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(100usize)) }
    }

    #[doc = "OSCU Control Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn ocntrl(&self) -> crate::common::Reg<self::Ocntrl_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(124usize)) }
    }

    #[doc = "OCDS Enable Control Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn oec(&self) -> crate::common::Reg<self::Oec_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(120usize)) }
    }

    #[doc = "OCDS Interface Mode Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn oifm(&self) -> crate::common::Reg<self::Oifm_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }

    #[doc = "OSCUStatus Register\n resetvalue={PowerOn Reset:0x10000,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn ostate(&self) -> crate::common::Reg<self::Ostate_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "TG Capture for Cores   BRKOUT\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tccb(&self) -> crate::common::Reg<self::Tccb_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(176usize)) }
    }

    #[doc = "TG Capture for Cores   HALT\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tcch(&self) -> crate::common::Reg<self::Tcch_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(180usize)) }
    }

    #[doc = "TG Capture for TG Input Pins\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tcip(&self) -> crate::common::Reg<self::Tcip_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "TG Capture for MCDS\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tcm(&self) -> crate::common::Reg<self::Tcm_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(188usize)) }
    }

    #[doc = "TG Capture for OTGB0 1\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tctgb(&self) -> crate::common::Reg<self::Tctgb_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(184usize)) }
    }

    #[doc = "TG Capture for TG Lines\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tctl(&self) -> crate::common::Reg<self::Tctl_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(116usize)) }
    }

    #[doc = "TG Input Pins Routing\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tipr(&self) -> crate::common::Reg<self::Tipr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "TG Line 1 Suspend Targets\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tl1st(&self) -> crate::common::Reg<self::Tl1St_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }

    #[doc = "TG Line Control\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tlc(&self) -> crate::common::Reg<self::Tlc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }

    #[doc = "TG Line Counter Control\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tlccx(&self) -> [crate::common::Reg<self::TlcCx_SPEC, crate::common::RW>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x40usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x40usize + 0x4usize)),
            ]
        }
    }

    #[doc = "TG Line Capture and Hold Enable\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tlche(&self) -> crate::common::Reg<self::Tlche_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(152usize)) }
    }

    #[doc = "TG Line Capture and Hold Clear\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tlchs(&self) -> crate::common::Reg<self::Tlchs_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(156usize)) }
    }

    #[doc = "TG Line Counter Value\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tlcvx(&self) -> [crate::common::Reg<self::TlcVx_SPEC, crate::common::R>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x50usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x50usize + 0x4usize)),
            ]
        }
    }

    #[doc = "TG Line State\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tls(&self) -> crate::common::Reg<self::Tls_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(112usize)) }
    }

    #[doc = "TG Line Timer\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tlt(&self) -> crate::common::Reg<self::Tlt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(168usize)) }
    }

    #[doc = "TG Lines for Trigger to Host\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tltth(&self) -> crate::common::Reg<self::Tltth_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(172usize)) }
    }

    #[doc = "TG Output Pins Pulse Stretcher\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn topps(&self) -> crate::common::Reg<self::Topps_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "TG Output Pins Routing\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn topr(&self) -> crate::common::Reg<self::Topr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "TG Routing for CPU0\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x2,Debug Reset:0x0,Application Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn trcx(&self) -> [crate::common::Reg<self::TrCx_SPEC, crate::common::RW>; 6] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x14usize)),
            ]
        }
    }

    #[doc = "TG Routing Events of CPU0\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn trecx(&self) -> [crate::common::Reg<self::TreCx_SPEC, crate::common::RW>; 6] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x14usize)),
            ]
        }
    }

    #[doc = "TG Routing for HSMControl\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn trhsm(&self) -> crate::common::Reg<self::Trhsm_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }

    #[doc = "Clear Trigger to Host Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn trigc(&self) -> crate::common::Reg<self::Trigc_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(164usize)) }
    }

    #[doc = "Set Trigger to Host Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn trigs(&self) -> crate::common::Reg<self::Trigs_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }

    #[doc = "Trigger to Host Register 0\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn trigx(&self) -> [crate::common::Reg<self::TriGx_SPEC, crate::common::R>; 6] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x14usize)),
            ]
        }
    }

    #[doc = "TG Routing for MCDS Control\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn trmc(&self) -> crate::common::Reg<self::Trmc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "TG Routing for MCDS Triggers\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn trmt(&self) -> crate::common::Reg<self::Trmt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(220usize)) }
    }

    #[doc = "TG Routing for Special Signals\n resetvalue={Debug Reset:0x0EF0000}"]
    #[inline(always)]
    pub const fn trss(&self) -> crate::common::Reg<self::Trss_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }
    #[doc = "TRTGB"]
    #[inline(always)]
    pub fn trtgb(self) -> [self::Trtgb; 2] {
        unsafe {
            [
                self::Trtgb(self.0.add(0xe0usize + 0x0usize)),
                self::Trtgb(self.0.add(0xe0usize + 0x8usize)),
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
pub struct Comdata_SPEC;
impl crate::sealed::RegSpec for Comdata_SPEC {
    type DataType = u32;
}
#[doc = "Communication Mode Data Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Comdata = crate::RegValueT<Comdata_SPEC>;

impl Comdata {
    #[doc = "Read Write Data   DATA. Data transferred by read write access executed by Cerberus in Communication mode."]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Comdata_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Comdata_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Comdata {
    #[inline(always)]
    fn default() -> Comdata {
        <crate::RegValueT<Comdata_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ictsa_SPEC;
impl crate::sealed::RegSpec for Ictsa_SPEC {
    type DataType = u32;
}
#[doc = "Internally Controlled Trace Source Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Ictsa = crate::RegValueT<Ictsa_SPEC>;

impl Ictsa {
    #[doc = "Source Address   ADDR. This address is used by Cerberus to read data  size depending on INTMOD . TRC  MOD          when a triggered transfer takes place in internal mode."]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Ictsa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Ictsa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ictsa {
    #[inline(always)]
    fn default() -> Ictsa {
        <crate::RegValueT<Ictsa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ictta_SPEC;
impl crate::sealed::RegSpec for Ictta_SPEC {
    type DataType = u32;
}
#[doc = "Internally Controlled Trace Destination Register\n resetvalue={PowerOn Reset:0x10F068,Application Reset:0x10F068,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Ictta = crate::RegValueT<Ictta_SPEC>;

impl Ictta {
    #[doc = "Destination Address   ADDR. This address is used by Cerberus to write data  size depending on INTMOD . TRC  MOD   when a triggered transfer takes place in internal mode."]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Ictta_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Ictta_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ictta {
    #[inline(always)]
    fn default() -> Ictta {
        <crate::RegValueT<Ictta_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifsa_SPEC;
impl crate::sealed::RegSpec for Ifsa_SPEC {
    type DataType = u32;
}
#[doc = "IFS Address Register\n resetvalue={Debug Reset:0x0}"]
pub type Ifsa = crate::RegValueT<Ifsa_SPEC>;

impl Ifsa {
    #[doc = "Address for FI SI Accesses   ADDR. In case of FI the lowest two bits address the byte within the 32  160 bit        word which is used for the RMW access. In case of SI the lowest two bits        are ignored."]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Ifsa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Ifsa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ifsa {
    #[inline(always)]
    fn default() -> Ifsa {
        <crate::RegValueT<Ifsa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifsc_SPEC;
impl crate::sealed::RegSpec for Ifsc_SPEC {
    type DataType = u32;
}
#[doc = "IFS Control Register\n resetvalue={Debug Reset:0x0}"]
pub type Ifsc = crate::RegValueT<Ifsc_SPEC>;

impl Ifsc {
    #[doc = "Injection Trigger   GO. This bit shall be gated by OCDS enable."]
    #[inline(always)]
    pub fn go(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ifsc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ifsc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Injection Trigger by OTGS   OTGS"]
    #[inline(always)]
    pub fn otgs(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ifsc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Stress or Fault Injection Mode   MODE"]
    #[inline(always)]
    pub fn mode(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ifsc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Bus Master Priority   PRIO"]
    #[inline(always)]
    pub fn prio(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ifsc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Read Stress Repetitions. The value 0 means just one SI read  the value 15 means 16 reads overall"]
    #[inline(always)]
    pub fn rsrep(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Ifsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault Injection Bit Manipulation   BM7. Controls the manipulation of bit x within the byte addressed by IFSA . The lowest two bits of IFSA determine the byte within the 32 bit word of the RMW operation. All other bytes of the word are unchanged."]
    #[inline(always)]
    pub fn bm0(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Ifsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault Injection Bit Manipulation   BM7. Controls the manipulation of bit x within the byte addressed by IFSA . The lowest two bits of IFSA determine the byte within the 32 bit word of the RMW operation. All other bytes of the word are unchanged."]
    #[inline(always)]
    pub fn bm1(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, Ifsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault Injection Bit Manipulation   BM7. Controls the manipulation of bit x within the byte addressed by IFSA . The lowest two bits of IFSA determine the byte within the 32 bit word of the RMW operation. All other bytes of the word are unchanged."]
    #[inline(always)]
    pub fn bm2(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Ifsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault Injection Bit Manipulation   BM7. Controls the manipulation of bit x within the byte addressed by IFSA . The lowest two bits of IFSA determine the byte within the 32 bit word of the RMW operation. All other bytes of the word are unchanged."]
    #[inline(always)]
    pub fn bm3(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8, Ifsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault Injection Bit Manipulation   BM7. Controls the manipulation of bit x within the byte addressed by IFSA . The lowest two bits of IFSA determine the byte within the 32 bit word of the RMW operation. All other bytes of the word are unchanged."]
    #[inline(always)]
    pub fn bm4(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, Ifsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault Injection Bit Manipulation   BM7. Controls the manipulation of bit x within the byte addressed by IFSA . The lowest two bits of IFSA determine the byte within the 32 bit word of the RMW operation. All other bytes of the word are unchanged."]
    #[inline(always)]
    pub fn bm5(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3,1,0,u8, Ifsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault Injection Bit Manipulation   BM7. Controls the manipulation of bit x within the byte addressed by IFSA . The lowest two bits of IFSA determine the byte within the 32 bit word of the RMW operation. All other bytes of the word are unchanged."]
    #[inline(always)]
    pub fn bm6(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, Ifsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault Injection Bit Manipulation   BM7. Controls the manipulation of bit x within the byte addressed by IFSA . The lowest two bits of IFSA determine the byte within the 32 bit word of the RMW operation. All other bytes of the word are unchanged."]
    #[inline(always)]
    pub fn bm7(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, Ifsc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ifsc {
    #[inline(always)]
    fn default() -> Ifsc {
        <crate::RegValueT<Ifsc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intmod_SPEC;
impl crate::sealed::RegSpec for Intmod_SPEC {
    type DataType = u32;
}
#[doc = "Internal Mode Status and Control Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Intmod = crate::RegValueT<Intmod_SPEC>;

impl Intmod {
    #[doc = "Set Read Sync Flag   SET CRS. Used by the monitor program to set the Read Sync Flag in Internal Mode."]
    #[inline(always)]
    pub fn set_crs(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Intmod_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Intmod_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Write Sync Flag   SET CWS. Used by the monitor program to set the Write Sync Flag in Internal Mode."]
    #[inline(always)]
    pub fn set_cws(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Intmod_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Intmod_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Communication Synchronization Flag   SET CS. Used by the monitor program to set the higher level Communication Mode sync bit in Internal Mode."]
    #[inline(always)]
    pub fn set_cs(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Intmod_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Intmod_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Communication Synchronization Flag   CLR CS. Used by the monitor program to clear the higher level Communication Mode sync bit in Internal Mode."]
    #[inline(always)]
    pub fn clr_cs(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Intmod_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Intmod_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "CHANNEL Write Protection   CHANNEL P. Protect CHA NNEL against unintended changes."]
    #[inline(always)]
    pub fn channel_p(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Intmod_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Intmod_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel Indication   CHANNEL. These bits are just written into the IOSR . CHAN NEL bit field in Internal Mode when CHANNEL P is held  160   8217 1  8217  concurrently 1  . Upon reading the current setting of IOSR . CHAN NEL is returned. If INT MOD is de asserted this is the value present in CROSSREFERENCE . CHAN NEL"]
    #[inline(always)]
    pub fn channel(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, Intmod_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8, Intmod_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enter Internal Mode   SET INT MOD. This bit is the only way to enter Internal Mode."]
    #[inline(always)]
    pub fn set_int_mod(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Intmod_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Intmod_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Internally Controlled Triggered Transfer   SET INT TRC. See CROSSREFERENCE for a description of the Triggered Transfer modes."]
    #[inline(always)]
    pub fn set_int_trc(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Intmod_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Intmod_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable Internally Controlled Triggered Transfer   CLR INT TRC"]
    #[inline(always)]
    pub fn clr_int_trc(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Intmod_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, Intmod_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "TRC MOD Write Protection   TRC MOD P. Protect TRC MOD against unintended changes."]
    #[inline(always)]
    pub fn trc_mod_p(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Intmod_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, Intmod_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Data Size Definition for Triggered Transfer   TRC MOD. The kind of bus access used during an Internally Controlled Triggered Transfer is defined by this bit field 1    If INT MOD is de asserted this bit field is set to 00 . This value will also be present whenever Internal Mode is entered."]
    #[inline(always)]
    pub fn trc_mod(
        self,
    ) -> crate::common::RegisterField<21, 0x3, 1, 0, u8, Intmod_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x3,1,0,u8, Intmod_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Internal Mode Enabled Flag   INT MOD. This bit reflects whether the Internal Mode is currently active. Set by SET IN T MOD   Cleared by Application Reset   OCDS disable or any IOClient access"]
    #[inline(always)]
    pub fn int_mod(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Intmod_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Intmod_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Internally Controlled Triggered Transfer Enable   INT TRC. This bit tells whether an Internally Controlled Triggered Transfer is currently enabled. Set by SET INT TRC  Cleared by CLR INT TRC. This bit is just CROSSREFERENCE . EX B US TRC if INT MOD is de asserted."]
    #[inline(always)]
    pub fn int_trc(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Intmod_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Intmod_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Intmod {
    #[inline(always)]
    fn default() -> Intmod {
        <crate::RegValueT<Intmod_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iosr_SPEC;
impl crate::sealed::RegSpec for Iosr_SPEC {
    type DataType = u32;
}
#[doc = "IOClientStatus and Control Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Iosr = crate::RegValueT<Iosr_SPEC>;

impl Iosr {
    #[doc = "Communication Mode Read Sync Flag   CRSYNC. Reflects the protocol state  namely whether the host is waiting for data to be sent. While INT MOD is de asserted this bit is set by reading the COMDATA register via the IO READ  WORD instruction. While INT MOD is asserted this bit is set by INTMOD . SET  CRS . It is cleared by writing to COMDATA through the bus slave interface or by CROSSREFERENCE . COM  RST The flag seen when INT MOD is asserted is not affected by CROSSREFERENCE . COM  RST . . Whenever this bit is set  independent of INT MOD   the OTGS is notified to optionally request an interrupt. This bit is not affected by changes of INT MOD itself."]
    #[inline(always)]
    pub fn crsync(self) -> crate::common::RegisterFieldBool<4, 1, 0, Iosr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Iosr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Communication Mode Write Sync Flag   CWSYNC. Reflects the protocol state  namely whether the host has posted data to be fetched. While INT MOD is de asserted this bit is set by writing the COMDATA register via the IO WRI TE WORD instruction. While INT MOD is asserted this bit is set by INTMOD . SET  CWS . It is cleared by writing a  1  to CW  ACK or by CROSSREFERENCE . COM  RST 1  . Whenever this bit is set  independent of INT MOD   the OTGS is notified to optionally request an interrupt. This bit is not affected by changes of INT MOD itself."]
    #[inline(always)]
    pub fn cwsync(self) -> crate::common::RegisterFieldBool<5, 1, 0, Iosr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Iosr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Communication Mode Write Acknowledge   CW ACK. The on chip software uses this bit to clear CW SYNC after the data was taken from COMDATA . There is no automatic clear of CWS YNC as each read may potentially be cancelled by pipelining or bus re arbitration   speculative  reads ."]
    #[inline(always)]
    pub fn cw_ack(self) -> crate::common::RegisterFieldBool<6, 1, 0, Iosr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Iosr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Communication Mode Synchronization Flag   COM SYNC. Can be used by the monitor program for higher level synchronization. While INT MOD is asserted this bit is set by INTMOD . SET  CS and cleared by INTMOD . CLR  CS . If INT MOD is de asserted this bit represents CROSSREFERENCE . COM  SYNC directly."]
    #[inline(always)]
    pub fn com_sync(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Iosr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Iosr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Tool Interface in Use   HOSTED. This bit is active when the DAP interface has received a CROSSREFERENCE ."]
    #[inline(always)]
    pub fn hosted(self) -> crate::common::RegisterFieldBool<8, 1, 0, Iosr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Iosr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel Indication   CHANNEL. These bits can be used by the tool software to facilitate multiple users of the tool interface in Communication Mode While INT MOD is asserted this represents INTMOD . CHAN NEL   CROSSREFERENCE . CHAN NEL otherwise."]
    #[inline(always)]
    pub fn channel(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, Iosr_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x7,1,0,u8, Iosr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Iosr {
    #[inline(always)]
    fn default() -> Iosr {
        <crate::RegValueT<Iosr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jdpid_SPEC;
impl crate::sealed::RegSpec for Jdpid_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={PowerOn Reset:0x6360,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Jdpid = crate::RegValueT<Jdpid_SPEC>;

impl Jdpid {
    #[doc = "Module Revision   MOD REV. This bit field indicates the revision number of the module implementation. It is just CROSSREFERENCE  7 0 ."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Jdpid_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Jdpid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Module Type   MOD TYPE"]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Jdpid_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Jdpid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Module Number   MOD NUMBER"]
    #[inline(always)]
    pub fn mod_number(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Jdpid_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Jdpid_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Jdpid {
    #[inline(always)]
    fn default() -> Jdpid {
        <crate::RegValueT<Jdpid_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jtagid_SPEC;
impl crate::sealed::RegSpec for Jtagid_SPEC {
    type DataType = u32;
}
#[doc = "JTAGDevice Identification Register\n resetvalue={PowerOn Reset:0x0,CFS Value:0x10207083,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Jtagid = crate::RegValueT<Jtagid_SPEC>;

impl Jtagid {
    #[doc = "JTAGDevice ID   JTAG ID"]
    #[inline(always)]
    pub fn jtag_id(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Jtagid_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Jtagid_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Jtagid {
    #[inline(always)]
    fn default() -> Jtagid {
        <crate::RegValueT<Jtagid_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ocntrl_SPEC;
impl crate::sealed::RegSpec for Ocntrl_SPEC {
    type DataType = u32;
}
#[doc = "OSCU Control Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Ocntrl = crate::RegValueT<Ocntrl_SPEC>;

impl Ocntrl {
    #[doc = "OC5 Write Protection   OC5 P. Protect OSTATE .OCx against unintended changes."]
    #[inline(always)]
    pub fn oc0_p(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "OC5 Write Protection   OC5 P. Protect OSTATE .OCx against unintended changes."]
    #[inline(always)]
    pub fn oc1_p(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "OC5 Write Protection   OC5 P. Protect OSTATE .OCx against unintended changes."]
    #[inline(always)]
    pub fn oc2_p(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "OC5 Write Protection   OC5 P. Protect OSTATE .OCx against unintended changes."]
    #[inline(always)]
    pub fn oc3_p(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "OC5 Write Protection   OC5 P. Protect OSTATE .OCx against unintended changes."]
    #[inline(always)]
    pub fn oc4_p(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "OC5 Write Protection   OC5 P. Protect OSTATE .OCx against unintended changes."]
    #[inline(always)]
    pub fn oc5_p(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Clear OCDS Control Bits Bus Domain   OC5. These bits allow to change the associated OSTATE .OCx bits by software. OSTATE .OC3 is also called OSTATE . ENIDIS  OEN protected . OSTATE .OC4 is also called OSTATE . EECTRC  OEN protected . OSTATE .OC5 is also called OSTATE . EECDIS  OEN protected ."]
    #[inline(always)]
    pub fn oc0(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Clear OCDS Control Bits Bus Domain   OC5. These bits allow to change the associated OSTATE .OCx bits by software. OSTATE .OC3 is also called OSTATE . ENIDIS  OEN protected . OSTATE .OC4 is also called OSTATE . EECTRC  OEN protected . OSTATE .OC5 is also called OSTATE . EECDIS  OEN protected ."]
    #[inline(always)]
    pub fn oc1(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Clear OCDS Control Bits Bus Domain   OC5. These bits allow to change the associated OSTATE .OCx bits by software. OSTATE .OC3 is also called OSTATE . ENIDIS  OEN protected . OSTATE .OC4 is also called OSTATE . EECTRC  OEN protected . OSTATE .OC5 is also called OSTATE . EECDIS  OEN protected ."]
    #[inline(always)]
    pub fn oc2(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Clear OCDS Control Bits Bus Domain   OC5. These bits allow to change the associated OSTATE .OCx bits by software. OSTATE .OC3 is also called OSTATE . ENIDIS  OEN protected . OSTATE .OC4 is also called OSTATE . EECTRC  OEN protected . OSTATE .OC5 is also called OSTATE . EECDIS  OEN protected ."]
    #[inline(always)]
    pub fn oc3(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Clear OCDS Control Bits Bus Domain   OC5. These bits allow to change the associated OSTATE .OCx bits by software. OSTATE .OC3 is also called OSTATE . ENIDIS  OEN protected . OSTATE .OC4 is also called OSTATE . EECTRC  OEN protected . OSTATE .OC5 is also called OSTATE . EECDIS  OEN protected ."]
    #[inline(always)]
    pub fn oc4(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Clear OCDS Control Bits Bus Domain   OC5. These bits allow to change the associated OSTATE .OCx bits by software. OSTATE .OC3 is also called OSTATE . ENIDIS  OEN protected . OSTATE .OC4 is also called OSTATE . EECTRC  OEN protected . OSTATE .OC5 is also called OSTATE . EECDIS  OEN protected ."]
    #[inline(always)]
    pub fn oc5(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "WDTSUS Write Protection   WDTSUS P. Protect OSTATE . W DTSUS against unintended changes."]
    #[inline(always)]
    pub fn wdtsus_p(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Clear Watchdog Timer Suspension Control   WDTSUS. This bit  OEN protected  is the only way to change the OSTATE . WD TSUS by software."]
    #[inline(always)]
    pub fn wdtsus(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "STABLE Write Protection   STABLE P. Protect OSTATE . STA BLE against unintended changes."]
    #[inline(always)]
    pub fn stable_p(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "InitializeApplication ResetIndication   STABLE. This bit is the only way to change the OSTATE . STA BLE bit by software."]
    #[inline(always)]
    pub fn stable(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "OJC7 Write Protection   OJC7 P. Protect OSTATE .OJCx against unintended changes."]
    #[inline(always)]
    pub fn ojc0_p(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "OJC7 Write Protection   OJC7 P. Protect OSTATE .OJCx against unintended changes."]
    #[inline(always)]
    pub fn ojc1_p(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "OJC7 Write Protection   OJC7 P. Protect OSTATE .OJCx against unintended changes."]
    #[inline(always)]
    pub fn ojc2_p(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "OJC7 Write Protection   OJC7 P. Protect OSTATE .OJCx against unintended changes."]
    #[inline(always)]
    pub fn ojc3_p(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "OJC7 Write Protection   OJC7 P. Protect OSTATE .OJCx against unintended changes."]
    #[inline(always)]
    pub fn ojc4_p(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "OJC7 Write Protection   OJC7 P. Protect OSTATE .OJCx against unintended changes."]
    #[inline(always)]
    pub fn ojc5_p(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "OJC7 Write Protection   OJC7 P. Protect OSTATE .OJCx against unintended changes."]
    #[inline(always)]
    pub fn ojc6_p(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "OJC7 Write Protection   OJC7 P. Protect OSTATE .OJCx against unintended changes."]
    #[inline(always)]
    pub fn ojc7_p(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Clear OCDS Control Bits IOClient Domain   OJC7. These bits allow to change the associated OSTATE .OJCx bits by software. OSTATE .OJC0 is also called OSTATE . H ARR  OEN protected  . OSTATE .OJC4 is also called OSTATE . RSTCL0  OEN protected  . OSTATE .OJC5 is also called OSTATE . RSTCL1 . OSTATE .OJC7 is also called OSTATE . RSTCL3  OEN protected  . OSTATE .OJCx can also be set cleared by the IOClient via the CROSSREFERENCE register. The IOClient has the higher priority."]
    #[inline(always)]
    pub fn ojc0(self) -> crate::common::RegisterFieldBool<17, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Clear OCDS Control Bits IOClient Domain   OJC7. These bits allow to change the associated OSTATE .OJCx bits by software. OSTATE .OJC0 is also called OSTATE . H ARR  OEN protected  . OSTATE .OJC4 is also called OSTATE . RSTCL0  OEN protected  . OSTATE .OJC5 is also called OSTATE . RSTCL1 . OSTATE .OJC7 is also called OSTATE . RSTCL3  OEN protected  . OSTATE .OJCx can also be set cleared by the IOClient via the CROSSREFERENCE register. The IOClient has the higher priority."]
    #[inline(always)]
    pub fn ojc1(self) -> crate::common::RegisterFieldBool<19, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Clear OCDS Control Bits IOClient Domain   OJC7. These bits allow to change the associated OSTATE .OJCx bits by software. OSTATE .OJC0 is also called OSTATE . H ARR  OEN protected  . OSTATE .OJC4 is also called OSTATE . RSTCL0  OEN protected  . OSTATE .OJC5 is also called OSTATE . RSTCL1 . OSTATE .OJC7 is also called OSTATE . RSTCL3  OEN protected  . OSTATE .OJCx can also be set cleared by the IOClient via the CROSSREFERENCE register. The IOClient has the higher priority."]
    #[inline(always)]
    pub fn ojc2(self) -> crate::common::RegisterFieldBool<21, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Clear OCDS Control Bits IOClient Domain   OJC7. These bits allow to change the associated OSTATE .OJCx bits by software. OSTATE .OJC0 is also called OSTATE . H ARR  OEN protected  . OSTATE .OJC4 is also called OSTATE . RSTCL0  OEN protected  . OSTATE .OJC5 is also called OSTATE . RSTCL1 . OSTATE .OJC7 is also called OSTATE . RSTCL3  OEN protected  . OSTATE .OJCx can also be set cleared by the IOClient via the CROSSREFERENCE register. The IOClient has the higher priority."]
    #[inline(always)]
    pub fn ojc3(self) -> crate::common::RegisterFieldBool<23, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Clear OCDS Control Bits IOClient Domain   OJC7. These bits allow to change the associated OSTATE .OJCx bits by software. OSTATE .OJC0 is also called OSTATE . H ARR  OEN protected  . OSTATE .OJC4 is also called OSTATE . RSTCL0  OEN protected  . OSTATE .OJC5 is also called OSTATE . RSTCL1 . OSTATE .OJC7 is also called OSTATE . RSTCL3  OEN protected  . OSTATE .OJCx can also be set cleared by the IOClient via the CROSSREFERENCE register. The IOClient has the higher priority."]
    #[inline(always)]
    pub fn ojc4(self) -> crate::common::RegisterFieldBool<25, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Clear OCDS Control Bits IOClient Domain   OJC7. These bits allow to change the associated OSTATE .OJCx bits by software. OSTATE .OJC0 is also called OSTATE . H ARR  OEN protected  . OSTATE .OJC4 is also called OSTATE . RSTCL0  OEN protected  . OSTATE .OJC5 is also called OSTATE . RSTCL1 . OSTATE .OJC7 is also called OSTATE . RSTCL3  OEN protected  . OSTATE .OJCx can also be set cleared by the IOClient via the CROSSREFERENCE register. The IOClient has the higher priority."]
    #[inline(always)]
    pub fn ojc5(self) -> crate::common::RegisterFieldBool<27, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Clear OCDS Control Bits IOClient Domain   OJC7. These bits allow to change the associated OSTATE .OJCx bits by software. OSTATE .OJC0 is also called OSTATE . H ARR  OEN protected  . OSTATE .OJC4 is also called OSTATE . RSTCL0  OEN protected  . OSTATE .OJC5 is also called OSTATE . RSTCL1 . OSTATE .OJC7 is also called OSTATE . RSTCL3  OEN protected  . OSTATE .OJCx can also be set cleared by the IOClient via the CROSSREFERENCE register. The IOClient has the higher priority."]
    #[inline(always)]
    pub fn ojc6(self) -> crate::common::RegisterFieldBool<29, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Clear OCDS Control Bits IOClient Domain   OJC7. These bits allow to change the associated OSTATE .OJCx bits by software. OSTATE .OJC0 is also called OSTATE . H ARR  OEN protected  . OSTATE .OJC4 is also called OSTATE . RSTCL0  OEN protected  . OSTATE .OJC5 is also called OSTATE . RSTCL1 . OSTATE .OJC7 is also called OSTATE . RSTCL3  OEN protected  . OSTATE .OJCx can also be set cleared by the IOClient via the CROSSREFERENCE register. The IOClient has the higher priority."]
    #[inline(always)]
    pub fn ojc7(self) -> crate::common::RegisterFieldBool<31, 1, 0, Ocntrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ocntrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ocntrl {
    #[inline(always)]
    fn default() -> Ocntrl {
        <crate::RegValueT<Ocntrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oec_SPEC;
impl crate::sealed::RegSpec for Oec_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Enable Control Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Oec = crate::RegValueT<Oec_SPEC>;

impl Oec {
    #[doc = "OCDS Enabling Pattern   PAT. The byte sequence described in CROSSREFERENCE must be written to this bit field sequentially to enable the OCDS subsystem."]
    #[inline(always)]
    pub fn pat(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Oec_SPEC, crate::common::W> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Oec_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable OCDS   DS. This bit allows the on chip software to disable the OCDS subsystem at any time. This is also good practice to do at the end of any debug session. To enable OCDS again the tool needs to repeat writing the enabling sequence into P AT ."]
    #[inline(always)]
    pub fn ds(self) -> crate::common::RegisterFieldBool<8, 1, 0, Oec_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Oec_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "OCDS Clock Off   OCO. If OCDS is disabled again  the OCDS clock can be optional switched off as well for power saving reasons  e.g. for power measurements . The OCDS clock is automatically enabled with the OCDS. OSTATE . OCO shows the status. The OCDS clock can only be disabled  if OCDS was already disabled before by another OEC register write."]
    #[inline(always)]
    pub fn oco(self) -> crate::common::RegisterFieldBool<9, 1, 0, Oec_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Oec_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "IF LCK Write Protection   IF LCK P. Protect IF  LCK against unintended changes."]
    #[inline(always)]
    pub fn if_lck_p(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Oec_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Oec_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Clear Interface Locked Indication   IF LCK. This bit is the only way to change the OSTATE . IF  LCK by software."]
    #[inline(always)]
    pub fn if_lck(self) -> crate::common::RegisterFieldBool<17, 1, 0, Oec_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Oec_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "AUT OK Write Protection   AUT OK P. Protect AUT  OK against unintended changes."]
    #[inline(always)]
    pub fn aut_ok_p(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Oec_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Oec_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Clear the Authorization OK Indication   AUT OK. This bit is the only way to change the OSTATE . AUT  OK by software."]
    #[inline(always)]
    pub fn aut_ok(self) -> crate::common::RegisterFieldBool<19, 1, 0, Oec_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, Oec_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Oec {
    #[inline(always)]
    fn default() -> Oec {
        <crate::RegValueT<Oec_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oifm_SPEC;
impl crate::sealed::RegSpec for Oifm_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Interface Mode Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Oifm = crate::RegValueT<Oifm_SPEC>;

impl Oifm {
    #[doc = "DAPInterface Mode   DAPMODE. DAPMODE mainly determines the number of pins allocated for the DAP        Interface. The selected mode becomes active with the next received CROSSREFERENCE .        The minimum number of dedicated DAP pins is device specific. If two pins        are dedicated  no SPD  Single Pin DAP  mode is needed offered. Please        request DAP RST in parallel only if it is really needed and intended to force the        selected mode and cut by this an already established tool connection.        The underlying assumption is that the tool user is in control and the        device should not sabotage tool access."]
    #[inline(always)]
    pub fn dapmode(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Oifm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Oifm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DAPProtocol Clear   DAPRST. Synchronous clear for the DAP state machine. See CROSSREFERENCE . Once set this bit is cleared by the hardware as soon as the DAP module has processed the reset request. As this is done with the DAP0 clock the time it takes is not predictable."]
    #[inline(always)]
    pub fn daprst(self) -> crate::common::RegisterFieldBool<3, 1, 0, Oifm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Oifm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Forced JTAG Mode.  dap bypass o"]
    #[inline(always)]
    pub fn f_jtag(self) -> crate::common::RegisterFieldBool<8, 1, 0, Oifm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Oifm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "No Switch to JTAG.  jtag disable o"]
    #[inline(always)]
    pub fn n_jtag(self) -> crate::common::RegisterFieldBool<9, 1, 0, Oifm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Oifm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pad Control for Debug Interface Pins.  padctl o  Please consult the Data Sheet for the proper setting relative to the        data rate. Selection valid for DAP and DAPE pads."]
    #[inline(always)]
    pub fn padctl(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Oifm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Oifm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad input threshold control for Debug Interface Pins. DAP pins always use TTL levels. This bits allows to set correct TTL        levels even for 3.3V supply. Selection valid for DAP and DAPE pads."]
    #[inline(always)]
    pub fn padctli(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Oifm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Oifm_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Oifm {
    #[inline(always)]
    fn default() -> Oifm {
        <crate::RegValueT<Oifm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ostate_SPEC;
impl crate::sealed::RegSpec for Ostate_SPEC {
    type DataType = u32;
}
#[doc = "OSCUStatus Register\n resetvalue={PowerOn Reset:0x10000,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Ostate = crate::RegValueT<Ostate_SPEC>;

impl Ostate {
    #[doc = "OCDS Enabled Flag   OEN. This bit enables the OCDS functionality of the complete device. Set and clear is done through the OEC register. This flag does not show any overruling by the HSM ."]
    #[inline(always)]
    pub fn oen(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ostate_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ostate_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "OCDS Control Bits System Bus Domain   OC2. Are used for device specific OCDS configuration purposes. They can be set and cleared with the associated OCNTRL .OCx bits."]
    #[inline(always)]
    pub fn oc0(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ostate_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ostate_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "OCDS Control Bits System Bus Domain   OC2. Are used for device specific OCDS configuration purposes. They can be set and cleared with the associated OCNTRL .OCx bits."]
    #[inline(always)]
    pub fn oc1(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ostate_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ostate_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "OCDS Control Bits System Bus Domain   OC2. Are used for device specific OCDS configuration purposes. They can be set and cleared with the associated OCNTRL .OCx bits."]
    #[inline(always)]
    pub fn oc2(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ostate_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ostate_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "OCDS ENDINIT Protection Override   ENIDIS. To modify ENDINIT protected register bits via Cerberus it would be dangerous to tamper with the protection logic controlled by a watchdog timer. This control bit allows to overrule the SCU.  Can be set and cleared with OCNTRL .OC3 only."]
    #[inline(always)]
    pub fn enidis(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ostate_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ostate_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "On Chip Trace Enable   EECTRC. The trace ports of all cores are normally turned off for security and power consumption reasons. Only if tracing is desired the ports are globally enabled with this control bit.  Can be set and cleared with OCNTRL .OC4 only. Specific enables for single trace interfaces will be added inside MCDS."]
    #[inline(always)]
    pub fn eectrc(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ostate_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ostate_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Emulation Logic Disable   EECDIS. If the features of the EEC are not needed  the complete EEC can be switched off with this control bit.  Can be set and cleared with OCNTRL .OC5 only. Both tracing and calibration  overlay  require the EEC to be turned on."]
    #[inline(always)]
    pub fn eecdis(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ostate_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ostate_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Control of Watchdog Timer  WDT  Suspension   WDTSUS. This bit determines the behavior of the watchdog timers when the system is running with OCDS enabled. Set and clear is done by OCNTRL . WD TSUS ."]
    #[inline(always)]
    pub fn wdtsus(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ostate_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ostate_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Halt after Reset Request   HARR. This bit tells the Startup Software to stop prior to entering the application code.  It can be set cleared with either OCNTRL .OJC0  via the Bus Slave Interface  or CROSSREFERENCE .OJC0  via the IOClient  . The IOClient however has the higher priority. The Kernel clock must be running to change this bit. A request from the IOClient however is stored until this is the case."]
    #[inline(always)]
    pub fn harr(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ostate_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ostate_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "OCDS Control Bits IOClient Domain OJC1. Are used for device specific OCDS configuration purposes. As these bits can be set by the IOClient regardless of IF  LCK and O EN make sure not to create a security hole in usage. They can be set and cleared either with the associated OCNTRL .OJCx         via the system bus  or CROSSREFERENCE .OJCx        bits. The IOClient however has the higher priority. The Kernel clock must be running to change these bits. A request from          the IOClient however is stored until this is the case."]
    #[inline(always)]
    pub fn ojc1(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ostate_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ostate_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "OCDS Control Bits IOClient Domain OJC2. Are used for device specific OCDS configuration purposes. As these bits can be set by the IOClient regardless of IF  LCK and O EN make sure not to create a security hole in usage. They can be set and cleared either with the associated OCNTRL .OJCx         via the system bus  or CROSSREFERENCE .OJCx        bits. The IOClient however has the higher priority. The Kernel clock must be running to change these bits. A request from          the IOClient however is stored until this is the case."]
    #[inline(always)]
    pub fn ojc2(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ostate_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ostate_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "OCDS Control Bits IOClient Domain OJC3. Are used for device specific OCDS configuration purposes. As these bits can be set by the IOClient regardless of IF  LCK and O EN make sure not to create a security hole in usage. They can be set and cleared either with the associated OCNTRL .OJCx         via the system bus  or CROSSREFERENCE .OJCx        bits. The IOClient however has the higher priority. The Kernel clock must be running to change these bits. A request from          the IOClient however is stored until this is the case."]
    #[inline(always)]
    pub fn ojc3(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ostate_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ostate_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "OCDS System Reset Request   RSTCL0. Can be set either with the associated OCNTRL .OJC4  via the system bus  or CROSSREFERENCE .OJC4 bits. The IOClient has the higher priority. The Kernel clock must be running to change this bit. A request from the IOClient however is stored until this is the case. This bit is automatically cleared by the next System Reset."]
    #[inline(always)]
    pub fn rstcl0(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Ostate_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ostate_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "OCDS Debug Reset Request   RSTCL1. Can be set and cleared either with the associated OCNTRL .OJC5  via the system bus  or CROSSREFERENCE .OJC5 bits. The IOClient has the higher priority. The Kernel clock must be running to change this bit. A request from the IOClient however is stored until this is the case. If the last set operation was done via OCNTRL this bit is automatically cleared by the next Application Reset . The requested Debug Reset will not clear this request automatically  a second write to OCNTRL is required to terminate the Debug Reset ."]
    #[inline(always)]
    pub fn rstcl1(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Ostate_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ostate_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "OCDS Control Bits IOClient Domain OJC6. Are used for device specific OCDS configuration purposes. As these bits can be set by the IOClient regardless of IF  LCK and O EN make sure not to create a security hole in usage. They can be set and cleared either with the associated OCNTRL .OJCx         via the system bus  or CROSSREFERENCE .OJCx        bits. The IOClient however has the higher priority. The Kernel clock must be running to change these bits. A request from          the IOClient however is stored until this is the case."]
    #[inline(always)]
    pub fn ojc6(self) -> crate::common::RegisterFieldBool<14, 1, 0, Ostate_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ostate_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "OCDS Application Reset Request   RSTCL3. Can be set and cleared either with the associated OCNTRL .OJC7  via the system bus  or CROSSREFERENCE .OJC7 bits. The IOClient has the higher priority. The Kernel clock must be running to change this bit. A request from the IOClient however is stored until this is the case. If the last set operation was done via OCNTRL this bit is automatically cleared by the next Application Reset ."]
    #[inline(always)]
    pub fn rstcl3(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Ostate_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ostate_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interface Locked Indication   IF LCK. This bit is used by the software inside the device to control the access right of the IOClient. Set and clear is done through the OEC register. Overruling by HSM is not visible in this bit."]
    #[inline(always)]
    pub fn if_lck(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Ostate_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ostate_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Authorization OK Indication   AUT OK. This bit is used by the software inside the device to store the higher        level   CROSSREFERENCE          access right status of the IOClient through Application        Reset s. It has no direct hardware effects. Set and clear is done through the OEC register."]
    #[inline(always)]
    pub fn aut_ok(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Ostate_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ostate_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Application Reset Indication   STABLE. This bit is used by the tool to detect Application        Reset s. It has no direct hardware effects. Set and clear is done through the OCNTRL register."]
    #[inline(always)]
    pub fn stable(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Ostate_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ostate_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "OCDS debug resource Clock On Indication   OCO. This bit is used by the tool to detect the status of the OCDS debug        resource clock on control register. Set and clear is done through the OEC . OCO register."]
    #[inline(always)]
    pub fn oco(self) -> crate::common::RegisterFieldBool<19, 1, 0, Ostate_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ostate_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ostate {
    #[inline(always)]
    fn default() -> Ostate {
        <crate::RegValueT<Ostate_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tccb_SPEC;
impl crate::sealed::RegSpec for Tccb_SPEC {
    type DataType = u32;
}
#[doc = "TG Capture for Cores   BRKOUT\n resetvalue={Debug Reset:0x0}"]
pub type Tccb = crate::RegValueT<Tccb_SPEC>;

impl Tccb {
    #[doc = "Capture of BRKOUT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Tccb_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tccb_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of BRKOUT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Tccb_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Tccb_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of BRKOUT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Tccb_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Tccb_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of BRKOUT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Tccb_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Tccb_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of BRKOUT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Tccb_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Tccb_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of BRKOUT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Tccb_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Tccb_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of BRKOUT Signal ofHSM   HSM"]
    #[inline(always)]
    pub fn hsm(self) -> crate::common::RegisterFieldBool<31, 1, 0, Tccb_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Tccb_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Tccb {
    #[inline(always)]
    fn default() -> Tccb {
        <crate::RegValueT<Tccb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcch_SPEC;
impl crate::sealed::RegSpec for Tcch_SPEC {
    type DataType = u32;
}
#[doc = "TG Capture for Cores   HALT\n resetvalue={Debug Reset:0x0}"]
pub type Tcch = crate::RegValueT<Tcch_SPEC>;

impl Tcch {
    #[doc = "Capture of HALT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Tcch_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tcch_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of HALT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Tcch_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Tcch_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of HALT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Tcch_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Tcch_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of HALT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Tcch_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Tcch_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of HALT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Tcch_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Tcch_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of HALT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Tcch_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Tcch_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of HALT Signal ofHSM   HSM"]
    #[inline(always)]
    pub fn hsm(self) -> crate::common::RegisterFieldBool<31, 1, 0, Tcch_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Tcch_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Tcch {
    #[inline(always)]
    fn default() -> Tcch {
        <crate::RegValueT<Tcch_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcip_SPEC;
impl crate::sealed::RegSpec for Tcip_SPEC {
    type DataType = u32;
}
#[doc = "TG Capture for TG Input Pins\n resetvalue={Debug Reset:0x0}"]
pub type Tcip = crate::RegValueT<Tcip_SPEC>;

impl Tcip {
    #[doc = "Capture of Trigger Input Pin 7   P7"]
    #[inline(always)]
    pub fn p0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Tcip_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tcip_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of Trigger Input Pin 7   P7"]
    #[inline(always)]
    pub fn p1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Tcip_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Tcip_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of Trigger Input Pin 7   P7"]
    #[inline(always)]
    pub fn p2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Tcip_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Tcip_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of Trigger Input Pin 7   P7"]
    #[inline(always)]
    pub fn p3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Tcip_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Tcip_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of Trigger Input Pin 7   P7"]
    #[inline(always)]
    pub fn p4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Tcip_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Tcip_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of Trigger Input Pin 7   P7"]
    #[inline(always)]
    pub fn p5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Tcip_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Tcip_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of Trigger Input Pin 7   P7"]
    #[inline(always)]
    pub fn p6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Tcip_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Tcip_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of Trigger Input Pin 7   P7"]
    #[inline(always)]
    pub fn p7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Tcip_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Tcip_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Tcip {
    #[inline(always)]
    fn default() -> Tcip {
        <crate::RegValueT<Tcip_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcm_SPEC;
impl crate::sealed::RegSpec for Tcm_SPEC {
    type DataType = u32;
}
#[doc = "TG Capture for MCDS\n resetvalue={Debug Reset:0x0}"]
pub type Tcm = crate::RegValueT<Tcm_SPEC>;

impl Tcm {
    #[doc = "Capture of MCDS break out   BRK"]
    #[inline(always)]
    pub fn brk(self) -> crate::common::RegisterFieldBool<0, 1, 0, Tcm_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tcm_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of MCDS suspend out   SUS"]
    #[inline(always)]
    pub fn sus(self) -> crate::common::RegisterFieldBool<1, 1, 0, Tcm_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Tcm_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of MCDS trig out 3   T3"]
    #[inline(always)]
    pub fn t0(self) -> crate::common::RegisterFieldBool<8, 1, 0, Tcm_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Tcm_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of MCDS trig out 3   T3"]
    #[inline(always)]
    pub fn t1(self) -> crate::common::RegisterFieldBool<9, 1, 0, Tcm_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Tcm_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of MCDS trig out 3   T3"]
    #[inline(always)]
    pub fn t2(self) -> crate::common::RegisterFieldBool<10, 1, 0, Tcm_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Tcm_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of MCDS trig out 3   T3"]
    #[inline(always)]
    pub fn t3(self) -> crate::common::RegisterFieldBool<11, 1, 0, Tcm_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Tcm_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Tcm {
    #[inline(always)]
    fn default() -> Tcm {
        <crate::RegValueT<Tcm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tctgb_SPEC;
impl crate::sealed::RegSpec for Tctgb_SPEC {
    type DataType = u32;
}
#[doc = "TG Capture for OTGB0 1\n resetvalue={Debug Reset:0x0}"]
pub type Tctgb = crate::RegValueT<Tctgb_SPEC>;

impl Tctgb {
    #[doc = "Capture Bits for OTGB0   OTGB0. If a bit is set  the associated OTGB0 signal was active high since last read."]
    #[inline(always)]
    pub fn otgb0(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Tctgb_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Tctgb_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture Bits for OTGB1   OTGB1. If a bit is set  the associated OTGB1 signal was active high since last read."]
    #[inline(always)]
    pub fn otgb1(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Tctgb_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Tctgb_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tctgb {
    #[inline(always)]
    fn default() -> Tctgb {
        <crate::RegValueT<Tctgb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tctl_SPEC;
impl crate::sealed::RegSpec for Tctl_SPEC {
    type DataType = u32;
}
#[doc = "TG Capture for TG Lines\n resetvalue={Debug Reset:0x0}"]
pub type Tctl = crate::RegValueT<Tctl_SPEC>;

impl Tctl {
    #[doc = "Capture of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Tctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Tctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Tctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Tctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Tctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Tctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Tctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Tctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Tctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Tctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Tctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Tctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Tctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Tctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Tctl {
    #[inline(always)]
    fn default() -> Tctl {
        <crate::RegValueT<Tctl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tipr_SPEC;
impl crate::sealed::RegSpec for Tipr_SPEC {
    type DataType = u32;
}
#[doc = "TG Input Pins Routing\n resetvalue={Debug Reset:0x0}"]
pub type Tipr = crate::RegValueT<Tipr_SPEC>;

impl Tipr {
    #[doc = "Trigger Pin 7 to Trigger Line Routing"]
    #[inline(always)]
    pub fn pin0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Tipr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Tipr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Pin 7 to Trigger Line Routing"]
    #[inline(always)]
    pub fn pin1(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Tipr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Tipr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Pin 7 to Trigger Line Routing"]
    #[inline(always)]
    pub fn pin2(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Tipr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Tipr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Pin 7 to Trigger Line Routing"]
    #[inline(always)]
    pub fn pin3(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Tipr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Tipr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Pin 7 to Trigger Line Routing"]
    #[inline(always)]
    pub fn pin4(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Tipr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Tipr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Pin 7 to Trigger Line Routing"]
    #[inline(always)]
    pub fn pin5(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Tipr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Tipr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Pin 7 to Trigger Line Routing"]
    #[inline(always)]
    pub fn pin6(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Tipr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Tipr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Pin 7 to Trigger Line Routing"]
    #[inline(always)]
    pub fn pin7(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Tipr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Tipr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Tipr {
    #[inline(always)]
    fn default() -> Tipr {
        <crate::RegValueT<Tipr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tl1St_SPEC;
impl crate::sealed::RegSpec for Tl1St_SPEC {
    type DataType = u32;
}
#[doc = "TG Line 1 Suspend Targets\n resetvalue={Debug Reset:0x0}"]
pub type Tl1St = crate::RegValueT<Tl1St_SPEC>;

impl Tl1St {
    #[doc = "CPU5 as Suspend Target   C5"]
    #[inline(always)]
    pub fn c0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tl1St_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 as Suspend Target   C5"]
    #[inline(always)]
    pub fn c1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Tl1St_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 as Suspend Target   C5"]
    #[inline(always)]
    pub fn c2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Tl1St_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 as Suspend Target   C5"]
    #[inline(always)]
    pub fn c3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Tl1St_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 as Suspend Target   C5"]
    #[inline(always)]
    pub fn c4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Tl1St_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 as Suspend Target   C5"]
    #[inline(always)]
    pub fn c5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Tl1St_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Master 0 as Suspend Target. Prepared for not yet known bus masters on future SoCs  which need        suspend control. Once used in the future the bit will get the name of        the master."]
    #[inline(always)]
    pub fn m0(self) -> crate::common::RegisterFieldBool<24, 1, 0, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Tl1St_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Master 1 as Suspend Target. Prepared for not yet known bus masters on future SoCs  which need        suspend control. Once used in the future the bit will get the name of        the master."]
    #[inline(always)]
    pub fn m1(self) -> crate::common::RegisterFieldBool<25, 1, 0, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Tl1St_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Master 2 as Suspend Target. Prepared for not yet known bus masters on future SoCs  which need        suspend control. Once used in the future the bit will get the name of        the master."]
    #[inline(always)]
    pub fn m2(self) -> crate::common::RegisterFieldBool<26, 1, 0, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Tl1St_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "HSSL1 as Suspend Target   HSS1"]
    #[inline(always)]
    pub fn hss1(self) -> crate::common::RegisterFieldBool<27, 1, 0, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Tl1St_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "HSSL0 as Suspend Target   HSS0"]
    #[inline(always)]
    pub fn hss0(self) -> crate::common::RegisterFieldBool<28, 1, 0, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Tl1St_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "DMA as Suspend Target   DMA"]
    #[inline(always)]
    pub fn dma(self) -> crate::common::RegisterFieldBool<29, 1, 0, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Tl1St_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "DMA as Suspend Target   DMA1. Prepared Reserved for future use with DMA"]
    #[inline(always)]
    pub fn dma1(self) -> crate::common::RegisterFieldBool<30, 1, 0, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Tl1St_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "HSM as Suspend Target   HSM"]
    #[inline(always)]
    pub fn hsm(self) -> crate::common::RegisterFieldBool<31, 1, 0, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Tl1St_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Tl1St {
    #[inline(always)]
    fn default() -> Tl1St {
        <crate::RegValueT<Tl1St_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tlc_SPEC;
impl crate::sealed::RegSpec for Tlc_SPEC {
    type DataType = u32;
}
#[doc = "TG Line Control\n resetvalue={Debug Reset:0x0}"]
pub type Tlc = crate::RegValueT<Tlc_SPEC>;

impl Tlc {
    #[doc = "TG Line Signal Processing   TLSP7"]
    #[inline(always)]
    pub fn tlsp1(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Tlc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, Tlc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "TG Line Signal Processing   TLSP7"]
    #[inline(always)]
    pub fn tlsp2(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Tlc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8, 0xf, 1, 0, u8, Tlc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "TG Line Signal Processing   TLSP7"]
    #[inline(always)]
    pub fn tlsp3(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Tlc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Tlc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Signal Processing   TLSP7"]
    #[inline(always)]
    pub fn tlsp4(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Tlc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Tlc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Signal Processing   TLSP7"]
    #[inline(always)]
    pub fn tlsp5(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Tlc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Tlc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Signal Processing   TLSP7"]
    #[inline(always)]
    pub fn tlsp6(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Tlc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Tlc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Signal Processing   TLSP7"]
    #[inline(always)]
    pub fn tlsp7(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Tlc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Tlc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Tlc {
    #[inline(always)]
    fn default() -> Tlc {
        <crate::RegValueT<Tlc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TlcCx_SPEC;
impl crate::sealed::RegSpec for TlcCx_SPEC {
    type DataType = u32;
}
#[doc = "TG Line Counter Control\n resetvalue={Debug Reset:0x0}"]
pub type TlcCx = crate::RegValueT<TlcCx_SPEC>;

impl TlcCx {
    #[doc = "Trigger Line to Counter Routing   TGL"]
    #[inline(always)]
    pub fn tgl(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, TlcCx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, TlcCx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Level or Edge Counting   LE"]
    #[inline(always)]
    pub fn le(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, TlcCx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8, TlcCx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear and Enable Counter s    CLR. Clears the counter value and enables a stopped counter. Counter reset state is stopped."]
    #[inline(always)]
    pub fn clr(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, TlcCx_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0x3,1,0,u8, TlcCx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Stop Counter s    STOP"]
    #[inline(always)]
    pub fn stop(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, TlcCx_SPEC, crate::common::W> {
        crate::common::RegisterField::<12,0x3,1,0,u8, TlcCx_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for TlcCx {
    #[inline(always)]
    fn default() -> TlcCx {
        <crate::RegValueT<TlcCx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tlche_SPEC;
impl crate::sealed::RegSpec for Tlche_SPEC {
    type DataType = u32;
}
#[doc = "TG Line Capture and Hold Enable\n resetvalue={Debug Reset:0x0}"]
pub type Tlche = crate::RegValueT<Tlche_SPEC>;

impl Tlche {
    #[doc = "Capture and Hold Enable for Trigger Line 3   TL3"]
    #[inline(always)]
    pub fn tl1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Tlche_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Tlche_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture and Hold Enable for Trigger Line 3   TL3"]
    #[inline(always)]
    pub fn tl2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Tlche_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Tlche_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture and Hold Enable for Trigger Line 3   TL3"]
    #[inline(always)]
    pub fn tl3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Tlche_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Tlche_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Tlche {
    #[inline(always)]
    fn default() -> Tlche {
        <crate::RegValueT<Tlche_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tlchs_SPEC;
impl crate::sealed::RegSpec for Tlchs_SPEC {
    type DataType = u32;
}
#[doc = "TG Line Capture and Hold Clear\n resetvalue={Debug Reset:0x0}"]
pub type Tlchs = crate::RegValueT<Tlchs_SPEC>;

impl Tlchs {
    #[doc = "Capture and Hold Clear for Trigger Line 3   TL3"]
    #[inline(always)]
    pub fn tl1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Tlchs_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Tlchs_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture and Hold Clear for Trigger Line 3   TL3"]
    #[inline(always)]
    pub fn tl2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Tlchs_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Tlchs_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture and Hold Clear for Trigger Line 3   TL3"]
    #[inline(always)]
    pub fn tl3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Tlchs_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Tlchs_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Tlchs {
    #[inline(always)]
    fn default() -> Tlchs {
        <crate::RegValueT<Tlchs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TlcVx_SPEC;
impl crate::sealed::RegSpec for TlcVx_SPEC {
    type DataType = u32;
}
#[doc = "TG Line Counter Value\n resetvalue={Debug Reset:0x0}"]
pub type TlcVx = crate::RegValueT<TlcVx_SPEC>;

impl TlcVx {
    #[doc = "Count Value   CV"]
    #[inline(always)]
    pub fn cv(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffffff, 1, 0, u32, TlcVx_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7fffffff,1,0,u32, TlcVx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Sticky Overflow Bit   SO. This bit is set by hardware when Count Value  30 0  is 0x7FFFFFFF. It is cleared by software like CV with TLCVx.CLR."]
    #[inline(always)]
    pub fn so(self) -> crate::common::RegisterFieldBool<31, 1, 0, TlcVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, TlcVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for TlcVx {
    #[inline(always)]
    fn default() -> TlcVx {
        <crate::RegValueT<TlcVx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tls_SPEC;
impl crate::sealed::RegSpec for Tls_SPEC {
    type DataType = u32;
}
#[doc = "TG Line State\n resetvalue={Debug Reset:0x0}"]
pub type Tls = crate::RegValueT<Tls_SPEC>;

impl Tls {
    #[doc = "Current State of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Tls_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Tls_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Current State of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Tls_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Tls_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Current State of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Tls_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Tls_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Current State of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Tls_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Tls_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Current State of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Tls_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Tls_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Current State of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Tls_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Tls_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Current State of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Tls_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Tls_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Tls {
    #[inline(always)]
    fn default() -> Tls {
        <crate::RegValueT<Tls_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tlt_SPEC;
impl crate::sealed::RegSpec for Tlt_SPEC {
    type DataType = u32;
}
#[doc = "TG Line Timer\n resetvalue={Debug Reset:0x0}"]
pub type Tlt = crate::RegValueT<Tlt_SPEC>;

impl Tlt {
    #[doc = "Timer to Trigger Line Routing   TGL"]
    #[inline(always)]
    pub fn tgl(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Tlt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0xf, 1, 0, u8, Tlt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "TG Line Value when Timer Value is Zero   VTZ"]
    #[inline(always)]
    pub fn vtz(self) -> crate::common::RegisterFieldBool<4, 1, 0, Tlt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Tlt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Reload Timer   RL"]
    #[inline(always)]
    pub fn rl(self) -> crate::common::RegisterFieldBool<5, 1, 0, Tlt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Tlt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer Value   TIM. Timer value which is automatically decremented if not 0. The timer frequency is fixed to f SPB divided by 4."]
    #[inline(always)]
    pub fn tim(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Tlt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Tlt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Tlt {
    #[inline(always)]
    fn default() -> Tlt {
        <crate::RegValueT<Tlt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tltth_SPEC;
impl crate::sealed::RegSpec for Tltth_SPEC {
    type DataType = u32;
}
#[doc = "TG Lines for Trigger to Host\n resetvalue={Debug Reset:0x0}"]
pub type Tltth = crate::RegValueT<Tltth_SPEC>;

impl Tltth {
    #[doc = "TG Line Enabling for Trigger to Host  TRIG    TL7"]
    #[inline(always)]
    pub fn tl1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Tltth_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Tltth_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Enabling for Trigger to Host  TRIG    TL7"]
    #[inline(always)]
    pub fn tl2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Tltth_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Tltth_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Enabling for Trigger to Host  TRIG    TL7"]
    #[inline(always)]
    pub fn tl3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Tltth_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Tltth_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Enabling for Trigger to Host  TRIG    TL7"]
    #[inline(always)]
    pub fn tl4(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Tltth_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Tltth_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Enabling for Trigger to Host  TRIG    TL7"]
    #[inline(always)]
    pub fn tl5(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Tltth_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Tltth_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Enabling for Trigger to Host  TRIG    TL7"]
    #[inline(always)]
    pub fn tl6(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Tltth_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Tltth_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Enabling for Trigger to Host  TRIG    TL7"]
    #[inline(always)]
    pub fn tl7(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Tltth_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Tltth_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Tltth {
    #[inline(always)]
    fn default() -> Tltth {
        <crate::RegValueT<Tltth_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Topps_SPEC;
impl crate::sealed::RegSpec for Topps_SPEC {
    type DataType = u32;
}
#[doc = "TG Output Pins Pulse Stretcher\n resetvalue={Debug Reset:0x0}"]
pub type Topps = crate::RegValueT<Topps_SPEC>;

impl Topps {
    #[doc = "Pulse Stretch Control for Trigger Pin 7   PIN7"]
    #[inline(always)]
    pub fn pin0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Topps_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Topps_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pulse Stretch Control for Trigger Pin 7   PIN7"]
    #[inline(always)]
    pub fn pin1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Topps_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Topps_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pulse Stretch Control for Trigger Pin 7   PIN7"]
    #[inline(always)]
    pub fn pin2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Topps_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Topps_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pulse Stretch Control for Trigger Pin 7   PIN7"]
    #[inline(always)]
    pub fn pin3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Topps_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Topps_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pulse Stretch Control for Trigger Pin 7   PIN7"]
    #[inline(always)]
    pub fn pin4(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Topps_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Topps_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pulse Stretch Control for Trigger Pin 7   PIN7"]
    #[inline(always)]
    pub fn pin5(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Topps_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Topps_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pulse Stretch Control for Trigger Pin 7   PIN7"]
    #[inline(always)]
    pub fn pin6(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Topps_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Topps_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pulse Stretch Control for Trigger Pin 7   PIN7"]
    #[inline(always)]
    pub fn pin7(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Topps_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Topps_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Topps {
    #[inline(always)]
    fn default() -> Topps {
        <crate::RegValueT<Topps_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Topr_SPEC;
impl crate::sealed::RegSpec for Topr_SPEC {
    type DataType = u32;
}
#[doc = "TG Output Pins Routing\n resetvalue={Debug Reset:0x0}"]
pub type Topr = crate::RegValueT<Topr_SPEC>;

impl Topr {
    #[doc = "Trigger Line to Trigger Pin 7 Routing"]
    #[inline(always)]
    pub fn pin0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Topr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Topr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to Trigger Pin 7 Routing"]
    #[inline(always)]
    pub fn pin1(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Topr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Topr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to Trigger Pin 7 Routing"]
    #[inline(always)]
    pub fn pin2(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Topr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Topr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to Trigger Pin 7 Routing"]
    #[inline(always)]
    pub fn pin3(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Topr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Topr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to Trigger Pin 7 Routing"]
    #[inline(always)]
    pub fn pin4(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Topr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Topr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to Trigger Pin 7 Routing"]
    #[inline(always)]
    pub fn pin5(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Topr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Topr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to Trigger Pin 7 Routing"]
    #[inline(always)]
    pub fn pin6(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Topr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Topr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to Trigger Pin 7 Routing"]
    #[inline(always)]
    pub fn pin7(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Topr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Topr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Topr {
    #[inline(always)]
    fn default() -> Topr {
        <crate::RegValueT<Topr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrCx_SPEC;
impl crate::sealed::RegSpec for TrCx_SPEC {
    type DataType = u32;
}
#[doc = "TG Routing for CPU0\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x2,Debug Reset:0x0,Application Reset:0x0,IOClient Reset:0x0}"]
pub type TrCx = crate::RegValueT<TrCx_SPEC>;

impl TrCx {
    #[doc = "HALT to Trigger Line Routing   HALT"]
    #[inline(always)]
    pub fn halt(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, TrCx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, TrCx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "BRKOUT to Trigger Line Routing   BRKOUT"]
    #[inline(always)]
    pub fn brkout(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, TrCx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, TrCx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "BRKOUT to Trigger Line 1 Routing   BT1. This option is available independent of the routing with BRKOUT .        The use case is that BRKOUT is  in addition  a source for the suspend        generation with Trigger Line 1."]
    #[inline(always)]
    pub fn bt1(self) -> crate::common::RegisterFieldBool<8, 1, 0, TrCx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, TrCx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Trigger Line to BRKIN Routing   BRKIN"]
    #[inline(always)]
    pub fn brkin(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, TrCx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, TrCx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to SUSIN Routing   SUSIN. Note  Use TL1ST in case of Trigger Line 1."]
    #[inline(always)]
    pub fn susin(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, TrCx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, TrCx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for TrCx {
    #[inline(always)]
    fn default() -> TrCx {
        <crate::RegValueT<TrCx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TreCx_SPEC;
impl crate::sealed::RegSpec for TreCx_SPEC {
    type DataType = u32;
}
#[doc = "TG Routing Events of CPU0\n resetvalue={Debug Reset:0x0}"]
pub type TreCx = crate::RegValueT<TreCx_SPEC>;

impl TreCx {
    #[doc = "TRxEVT to Trigger Line Routing TR0EV"]
    #[inline(always)]
    pub fn tr0ev(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, TreCx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, TreCx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TRxEVT to Trigger Line Routing TR2EV"]
    #[inline(always)]
    pub fn tr2ev(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, TreCx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, TreCx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TRxEVT to Trigger Line Routing TR4EV"]
    #[inline(always)]
    pub fn tr4ev(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, TreCx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, TreCx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TRxEVT to Trigger Line Routing TR6EV"]
    #[inline(always)]
    pub fn tr6ev(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, TreCx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, TreCx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for TreCx {
    #[inline(always)]
    fn default() -> TreCx {
        <crate::RegValueT<TreCx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trhsm_SPEC;
impl crate::sealed::RegSpec for Trhsm_SPEC {
    type DataType = u32;
}
#[doc = "TG Routing for HSMControl\n resetvalue={Debug Reset:0x0}"]
pub type Trhsm = crate::RegValueT<Trhsm_SPEC>;

impl Trhsm {
    #[doc = "HALT to Trigger Line Routing   HALT"]
    #[inline(always)]
    pub fn halt(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Trhsm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Trhsm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "BRKOUT to Trigger Line Routing   BRKOUT"]
    #[inline(always)]
    pub fn brkout(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Trhsm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Trhsm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "BRKOUT to Trigger Line 1 Routing   BT1. BRKOUT signal is used in addition for suspend generation with Trigger Line 1."]
    #[inline(always)]
    pub fn bt1(self) -> crate::common::RegisterFieldBool<8, 1, 0, Trhsm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Trhsm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Trigger Line to BRKIN Routing   BRKIN"]
    #[inline(always)]
    pub fn brkin(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Trhsm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Trhsm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to SUSIN Routing   SUSIN. Note  Use TL1ST in case of Trigger Line 1."]
    #[inline(always)]
    pub fn susin(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Trhsm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Trhsm_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Trhsm {
    #[inline(always)]
    fn default() -> Trhsm {
        <crate::RegValueT<Trhsm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trigc_SPEC;
impl crate::sealed::RegSpec for Trigc_SPEC {
    type DataType = u32;
}
#[doc = "Clear Trigger to Host Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Trigc = crate::RegValueT<Trigc_SPEC>;

impl Trigc {
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Trigc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Trigc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Trigc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Trigc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Trigc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Trigc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Trigc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Trigc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Trigc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Trigc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Trigc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Trigc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Trigc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Trigc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Trigc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Trigc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Trigc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Trigc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Trigc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Trigc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Trigc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Trigc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Trigc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Trigc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Trigc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Trigc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Trigc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Trigc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Trigc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Trigc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Trigc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Trigc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Index of most important register TRIGx   x. For TRIGx the highest x available x   5 ."]
    #[inline(always)]
    pub fn x(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Trigc_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Trigc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Trigc {
    #[inline(always)]
    fn default() -> Trigc {
        <crate::RegValueT<Trigc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trigs_SPEC;
impl crate::sealed::RegSpec for Trigs_SPEC {
    type DataType = u32;
}
#[doc = "Set Trigger to Host Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Trigs = crate::RegValueT<Trigs_SPEC>;

impl Trigs {
    #[doc = "Service Request Bit Number to Set   BITNUM. The bit TRGx y of register TRIGx is set  with  x   BITNUM div 16   and  y   BITNUM mod 16  . LOST SEQUENCE DEFINITION"]
    #[inline(always)]
    pub fn bitnum(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Trigs_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Trigs_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Trigs {
    #[inline(always)]
    fn default() -> Trigs {
        <crate::RegValueT<Trigs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TriGx_SPEC;
impl crate::sealed::RegSpec for TriGx_SPEC {
    type DataType = u32;
}
#[doc = "Trigger to Host Register 0\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type TriGx = crate::RegValueT<TriGx_SPEC>;

impl TriGx {
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_0(self) -> crate::common::RegisterFieldBool<0, 1, 0, TriGx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, TriGx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_1(self) -> crate::common::RegisterFieldBool<1, 1, 0, TriGx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, TriGx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_2(self) -> crate::common::RegisterFieldBool<2, 1, 0, TriGx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, TriGx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_3(self) -> crate::common::RegisterFieldBool<3, 1, 0, TriGx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, TriGx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_4(self) -> crate::common::RegisterFieldBool<4, 1, 0, TriGx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, TriGx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_5(self) -> crate::common::RegisterFieldBool<5, 1, 0, TriGx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, TriGx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_6(self) -> crate::common::RegisterFieldBool<6, 1, 0, TriGx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, TriGx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_7(self) -> crate::common::RegisterFieldBool<7, 1, 0, TriGx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, TriGx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_8(self) -> crate::common::RegisterFieldBool<8, 1, 0, TriGx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, TriGx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_9(self) -> crate::common::RegisterFieldBool<9, 1, 0, TriGx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, TriGx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, TriGx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, TriGx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, TriGx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, TriGx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, TriGx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, TriGx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, TriGx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, TriGx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, TriGx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, TriGx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, TriGx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, TriGx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TRIG register number   x"]
    #[inline(always)]
    pub fn x(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, TriGx_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, TriGx_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for TriGx {
    #[inline(always)]
    fn default() -> TriGx {
        <crate::RegValueT<TriGx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trmc_SPEC;
impl crate::sealed::RegSpec for Trmc_SPEC {
    type DataType = u32;
}
#[doc = "TG Routing for MCDS Control\n resetvalue={Debug Reset:0x0}"]
pub type Trmc = crate::RegValueT<Trmc_SPEC>;

impl Trmc {
    #[doc = "MCDS break out to Trigger Line Routing   BRKOUT"]
    #[inline(always)]
    pub fn brkout(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Trmc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Trmc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MCDS suspend out to Trigger Line Routing   SUSOUT"]
    #[inline(always)]
    pub fn susout(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Trmc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Trmc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to MCDS break in Routing   BRKIN"]
    #[inline(always)]
    pub fn brkin(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Trmc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Trmc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Trmc {
    #[inline(always)]
    fn default() -> Trmc {
        <crate::RegValueT<Trmc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trmt_SPEC;
impl crate::sealed::RegSpec for Trmt_SPEC {
    type DataType = u32;
}
#[doc = "TG Routing for MCDS Triggers\n resetvalue={Debug Reset:0x0}"]
pub type Trmt = crate::RegValueT<Trmt_SPEC>;

impl Trmt {
    #[doc = "MCDS trig out 3 to Trigger Line Routing   TG3"]
    #[inline(always)]
    pub fn tg0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Trmt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Trmt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MCDS trig out 3 to Trigger Line Routing   TG3"]
    #[inline(always)]
    pub fn tg1(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Trmt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Trmt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MCDS trig out 3 to Trigger Line Routing   TG3"]
    #[inline(always)]
    pub fn tg2(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Trmt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Trmt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MCDS trig out 3 to Trigger Line Routing   TG3"]
    #[inline(always)]
    pub fn tg3(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Trmt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Trmt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Trmt {
    #[inline(always)]
    fn default() -> Trmt {
        <crate::RegValueT<Trmt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trss_SPEC;
impl crate::sealed::RegSpec for Trss_SPEC {
    type DataType = u32;
}
#[doc = "TG Routing for Special Signals\n resetvalue={Debug Reset:0x0EF0000}"]
pub type Trss = crate::RegValueT<Trss_SPEC>;

impl Trss {
    #[doc = "Trigger Line to Cerberus  Triggered Transfer Routing   TT"]
    #[inline(always)]
    pub fn tt(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Trss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Trss_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to Fault and Stress Injection Routing   IFS. This bit field is not availavle in TC39x A Step."]
    #[inline(always)]
    pub fn ifs(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Trss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Trss_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to SRC1 Interrupt Routing   SRC1"]
    #[inline(always)]
    pub fn src0(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Trss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Trss_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to SRC1 Interrupt Routing   SRC1"]
    #[inline(always)]
    pub fn src1(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Trss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Trss_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Trss {
    #[inline(always)]
    fn default() -> Trss {
        <crate::RegValueT<Trss_SPEC> as RegisterValue<_>>::new(15663104)
    }
}

#[doc = "TRTGB"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trtgb(pub(super) *mut u8);
unsafe impl core::marker::Send for Trtgb {}
unsafe impl core::marker::Sync for Trtgb {}
impl Trtgb {
    #[doc = "TG Routing for OTGBi Bits  15 8 \n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn trtgbih(&self) -> crate::common::Reg<trtgb::TrtgBiH_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "TG Routing for OTGBi Bits  7 0 \n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn trtgbil(&self) -> crate::common::Reg<trtgb::TrtgBiL_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod trtgb {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrtgBiH_SPEC;
    impl crate::sealed::RegSpec for TrtgBiH_SPEC {
        type DataType = u32;
    }
    #[doc = "TG Routing for OTGBi Bits  15 8 \n resetvalue={Debug Reset:0x0}"]
    pub type TrtgBiH = crate::RegValueT<TrtgBiH_SPEC>;

    impl TrtgBiH {
        #[doc = "OTGBi Bit 15 to Trigger Line Routing   TG15"]
        #[inline(always)]
        pub fn tg8(
            self,
        ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, TrtgBiH_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xf,1,0,u8, TrtgBiH_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "OTGBi Bit 15 to Trigger Line Routing   TG15"]
        #[inline(always)]
        pub fn tg9(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, TrtgBiH_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0xf,1,0,u8, TrtgBiH_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "OTGBi Bit 15 to Trigger Line Routing   TG15"]
        #[inline(always)]
        pub fn tg10(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, TrtgBiH_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xf,1,0,u8, TrtgBiH_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "OTGBi Bit 15 to Trigger Line Routing   TG15"]
        #[inline(always)]
        pub fn tg11(
            self,
        ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, TrtgBiH_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0xf,1,0,u8, TrtgBiH_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "OTGBi Bit 15 to Trigger Line Routing   TG15"]
        #[inline(always)]
        pub fn tg12(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, TrtgBiH_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, TrtgBiH_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "OTGBi Bit 15 to Trigger Line Routing   TG15"]
        #[inline(always)]
        pub fn tg13(
            self,
        ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, TrtgBiH_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0xf,1,0,u8, TrtgBiH_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "OTGBi Bit 15 to Trigger Line Routing   TG15"]
        #[inline(always)]
        pub fn tg14(
            self,
        ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, TrtgBiH_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0xf,1,0,u8, TrtgBiH_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "OTGBi Bit 15 to Trigger Line Routing   TG15"]
        #[inline(always)]
        pub fn tg15(
            self,
        ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, TrtgBiH_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0xf,1,0,u8, TrtgBiH_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for TrtgBiH {
        #[inline(always)]
        fn default() -> TrtgBiH {
            <crate::RegValueT<TrtgBiH_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrtgBiL_SPEC;
    impl crate::sealed::RegSpec for TrtgBiL_SPEC {
        type DataType = u32;
    }
    #[doc = "TG Routing for OTGBi Bits  7 0 \n resetvalue={Debug Reset:0x0}"]
    pub type TrtgBiL = crate::RegValueT<TrtgBiL_SPEC>;

    impl TrtgBiL {
        #[doc = "OTGBi Bit 7 to Trigger Line Routing   TG7"]
        #[inline(always)]
        pub fn tg0(
            self,
        ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, TrtgBiL_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xf,1,0,u8, TrtgBiL_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "OTGBi Bit 7 to Trigger Line Routing   TG7"]
        #[inline(always)]
        pub fn tg1(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, TrtgBiL_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0xf,1,0,u8, TrtgBiL_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "OTGBi Bit 7 to Trigger Line Routing   TG7"]
        #[inline(always)]
        pub fn tg2(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, TrtgBiL_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xf,1,0,u8, TrtgBiL_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "OTGBi Bit 7 to Trigger Line Routing   TG7"]
        #[inline(always)]
        pub fn tg3(
            self,
        ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, TrtgBiL_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0xf,1,0,u8, TrtgBiL_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "OTGBi Bit 7 to Trigger Line Routing   TG7"]
        #[inline(always)]
        pub fn tg4(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, TrtgBiL_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, TrtgBiL_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "OTGBi Bit 7 to Trigger Line Routing   TG7"]
        #[inline(always)]
        pub fn tg5(
            self,
        ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, TrtgBiL_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0xf,1,0,u8, TrtgBiL_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "OTGBi Bit 7 to Trigger Line Routing   TG7"]
        #[inline(always)]
        pub fn tg6(
            self,
        ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, TrtgBiL_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0xf,1,0,u8, TrtgBiL_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "OTGBi Bit 7 to Trigger Line Routing   TG7"]
        #[inline(always)]
        pub fn tg7(
            self,
        ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, TrtgBiL_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0xf,1,0,u8, TrtgBiL_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for TrtgBiL {
        #[inline(always)]
        fn default() -> TrtgBiL {
            <crate::RegValueT<TrtgBiL_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
