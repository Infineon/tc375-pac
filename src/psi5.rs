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
#[doc = r"PSI5"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psi5(pub(super) *mut u8);
unsafe impl core::marker::Send for Psi5 {}
unsafe impl core::marker::Sync for Psi5 {}
impl Psi5 {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(976usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "CRCI Overview Clear Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn crciclrx(&self) -> [crate::common::Reg<self::CrciclRx_SPEC, crate::common::W>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x59cusize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x59cusize + 0x4usize)),
            ]
        }
    }

    #[doc = "CRCI Overview Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn crciovx(&self) -> [crate::common::Reg<self::CrcioVx_SPEC, crate::common::R>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x45cusize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x45cusize + 0x4usize)),
            ]
        }
    }

    #[doc = "CRCI Overview Set Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn crcisetx(&self) -> [crate::common::Reg<self::CrciseTx_SPEC, crate::common::W>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x4fcusize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x4fcusize + 0x4usize)),
            ]
        }
    }

    #[doc = "PSI5 Fractional Divider Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fdr(&self) -> crate::common::Reg<self::Fdr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }

    #[doc = "Fractional Divider Register for Higher Bit Rate\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fdrh(&self) -> crate::common::Reg<self::Fdrh_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Fractional Divider Register for Lower Bit Rate\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fdrl(&self) -> crate::common::Reg<self::Fdrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "Fractional Divider Register for Time Stamp\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fdrt(&self) -> crate::common::Reg<self::Fdrt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "Global Control Register\n resetvalue={Application Reset:0x1F}"]
    #[inline(always)]
    pub const fn gcr(&self) -> crate::common::Reg<self::Gcr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0C3C000}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Interrupt Node Pointer Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn inpx(&self) -> [crate::common::Reg<self::InPx_SPEC, crate::common::RW>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x2fcusize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x2fcusize + 0x4usize)),
            ]
        }
    }

    #[doc = "Interrupt Clear Register A 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn intclrax(&self) -> [crate::common::Reg<self::IntclrAx_SPEC, crate::common::W>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x360usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x360usize + 0x4usize)),
            ]
        }
    }

    #[doc = "Interrupt Clear Register A 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn intclrbx(&self) -> [crate::common::Reg<self::IntclrBx_SPEC, crate::common::W>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x374usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x374usize + 0x4usize)),
            ]
        }
    }

    #[doc = "Interrupt Enable Register A 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn intenax(&self) -> [crate::common::Reg<self::IntenAx_SPEC, crate::common::RW>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x388usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x388usize + 0x4usize)),
            ]
        }
    }

    #[doc = "Interrupt Enable Register B 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn intenbx(&self) -> [crate::common::Reg<self::IntenBx_SPEC, crate::common::RW>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x39cusize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x39cusize + 0x4usize)),
            ]
        }
    }

    #[doc = "Interrupt Overview Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn intov(&self) -> crate::common::Reg<self::Intov_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(760usize)) }
    }

    #[doc = "Interrupt Set Register A 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn intsetax(&self) -> [crate::common::Reg<self::IntsetAx_SPEC, crate::common::W>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x338usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x338usize + 0x4usize)),
            ]
        }
    }

    #[doc = "Interrupt Set Register B 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn intsetbx(&self) -> [crate::common::Reg<self::IntsetBx_SPEC, crate::common::W>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x34cusize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x34cusize + 0x4usize)),
            ]
        }
    }

    #[doc = "Interrupt Status Register A 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn intstatax(
        &self,
    ) -> [crate::common::Reg<self::IntstatAx_SPEC, crate::common::R>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x310usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x310usize + 0x4usize)),
            ]
        }
    }

    #[doc = "Interrupt Status Register B 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn intstatbx(
        &self,
    ) -> [crate::common::Reg<self::IntstatBx_SPEC, crate::common::R>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x324usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x324usize + 0x4usize)),
            ]
        }
    }

    #[doc = "Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(984usize)) }
    }

    #[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(988usize)) }
    }

    #[doc = "Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(992usize)) }
    }

    #[doc = "MEI Overview Clear Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn meiclrx(&self) -> [crate::common::Reg<self::MeiclRx_SPEC, crate::common::W>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x5d8usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x5d8usize + 0x4usize)),
            ]
        }
    }

    #[doc = "MEI Overview Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn meiovx(&self) -> [crate::common::Reg<self::MeioVx_SPEC, crate::common::R>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x498usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x498usize + 0x4usize)),
            ]
        }
    }

    #[doc = "MEI Overview Set Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn meisetx(&self) -> [crate::common::Reg<self::MeiseTx_SPEC, crate::common::W>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x538usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x538usize + 0x4usize)),
            ]
        }
    }

    #[doc = "NBI Overview Clear Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn nbiclrx(&self) -> [crate::common::Reg<self::NbiclRx_SPEC, crate::common::W>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x574usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x574usize + 0x4usize)),
            ]
        }
    }

    #[doc = "NBI Overview Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn nbiovx(&self) -> [crate::common::Reg<self::NbioVx_SPEC, crate::common::R>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x434usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x434usize + 0x4usize)),
            ]
        }
    }

    #[doc = "NBI Overview Set Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn nbisetx(&self) -> [crate::common::Reg<self::NbiseTx_SPEC, crate::common::W>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x4d4usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x4d4usize + 0x4usize)),
            ]
        }
    }

    #[doc = "NFI Overview Clear Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn nficlrx(&self) -> [crate::common::Reg<self::NficlRx_SPEC, crate::common::W>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x5c4usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x5c4usize + 0x4usize)),
            ]
        }
    }

    #[doc = "NFI Overview Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn nfiovx(&self) -> [crate::common::Reg<self::NfioVx_SPEC, crate::common::R>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x484usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x484usize + 0x4usize)),
            ]
        }
    }

    #[doc = "NFI Overview Set Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn nfisetx(&self) -> [crate::common::Reg<self::NfiseTx_SPEC, crate::common::W>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x524usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x524usize + 0x4usize)),
            ]
        }
    }

    #[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(972usize)) }
    }

    #[doc = "Receive Data FIFO 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rdfx(&self) -> [crate::common::Reg<self::RdFx_SPEC, crate::common::R>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x3f8usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x3f8usize + 0x4usize)),
            ]
        }
    }

    #[doc = "RDI Overview Clear Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rdiclrx(&self) -> [crate::common::Reg<self::RdiclRx_SPEC, crate::common::W>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x5b0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x5b0usize + 0x4usize)),
            ]
        }
    }

    #[doc = "RDI Overview Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rdiovx(&self) -> [crate::common::Reg<self::RdioVx_SPEC, crate::common::R>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x470usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x470usize + 0x4usize)),
            ]
        }
    }

    #[doc = "RDI Overview Set Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rdisetx(&self) -> [crate::common::Reg<self::RdiseTx_SPEC, crate::common::W>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x510usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x510usize + 0x4usize)),
            ]
        }
    }

    #[doc = "Receive FIFO Control Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rfcx(&self) -> [crate::common::Reg<self::RfCx_SPEC, crate::common::RW>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x3e4usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x3e4usize + 0x4usize)),
            ]
        }
    }

    #[doc = "RMI Overview Clear Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rmiclrx(&self) -> [crate::common::Reg<self::RmiclRx_SPEC, crate::common::W>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x560usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x560usize + 0x4usize)),
            ]
        }
    }

    #[doc = "RMI Overview Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rmiovx(&self) -> [crate::common::Reg<self::RmioVx_SPEC, crate::common::R>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x420usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x420usize + 0x4usize)),
            ]
        }
    }

    #[doc = "RMI Overview Set Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rmisetx(&self) -> [crate::common::Reg<self::RmiseTx_SPEC, crate::common::W>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x4c0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x4c0usize + 0x4usize)),
            ]
        }
    }

    #[doc = "RSI Overview Clear Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rsiclrx(&self) -> [crate::common::Reg<self::RsiclRx_SPEC, crate::common::W>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x54cusize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x54cusize + 0x4usize)),
            ]
        }
    }

    #[doc = "RSI Overview Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rsiovx(&self) -> [crate::common::Reg<self::RsioVx_SPEC, crate::common::R>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x40cusize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x40cusize + 0x4usize)),
            ]
        }
    }

    #[doc = "RSI Overview Set Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rsisetx(&self) -> [crate::common::Reg<self::RsiseTx_SPEC, crate::common::W>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x4acusize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x4acusize + 0x4usize)),
            ]
        }
    }

    #[doc = "TEI Overview Clear Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn teiclrx(&self) -> [crate::common::Reg<self::TeiclRx_SPEC, crate::common::W>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x588usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x588usize + 0x4usize)),
            ]
        }
    }

    #[doc = "TEI Overview Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn teiovx(&self) -> [crate::common::Reg<self::TeioVx_SPEC, crate::common::R>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x448usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x448usize + 0x4usize)),
            ]
        }
    }

    #[doc = "TEI Overview Set Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn teisetx(&self) -> [crate::common::Reg<self::TeiseTx_SPEC, crate::common::W>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x4e8usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x4e8usize + 0x4usize)),
            ]
        }
    }

    #[doc = "Module Time Stamp Register A\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tsra(&self) -> crate::common::Reg<self::Tsra_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Time Stamp Register B\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tsrb(&self) -> crate::common::Reg<self::Tsrb_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Module Time Stamp Register C\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tsrc(&self) -> crate::common::Reg<self::Tsrc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "CH"]
    #[inline(always)]
    pub fn ch(self) -> [self::Ch; 2] {
        unsafe {
            [
                self::Ch(self.0.add(0x30usize + 0x0usize)),
                self::Ch(self.0.add(0x30usize + 0x90usize)),
            ]
        }
    }
    #[doc = "RDM"]
    #[inline(always)]
    pub fn rdm(self) -> [self::Rdm; 4] {
        unsafe {
            [
                self::Rdm(self.0.add(0x600usize + 0x0usize)),
                self::Rdm(self.0.add(0x600usize + 0x8usize)),
                self::Rdm(self.0.add(0x600usize + 0x10usize)),
                self::Rdm(self.0.add(0x600usize + 0x18usize)),
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
    #[doc = "External Sleep Mode Request Disable Bit   EDIS. Used to control module s sleep mode. If this bit is cleared the kernel clock f PSI5 is disabled during System Sleep Mode."]
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
pub struct CrciclRx_SPEC;
impl crate::sealed::RegSpec for CrciclRx_SPEC {
    type DataType = u32;
}
#[doc = "CRCI Overview Clear Register 0\n resetvalue={Application Reset:0x0}"]
pub type CrciclRx = crate::RegValueT<CrciclRx_SPEC>;

impl CrciclRx {
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci9(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CRCI Flag of Buffer 31   CRCI31. Setting this bit clears bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, CrciclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, CrciclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for CrciclRx {
    #[inline(always)]
    fn default() -> CrciclRx {
        <crate::RegValueT<CrciclRx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrcioVx_SPEC;
impl crate::sealed::RegSpec for CrcioVx_SPEC {
    type DataType = u32;
}
#[doc = "CRCI Overview Register 0\n resetvalue={Application Reset:0x0}"]
pub type CrcioVx = crate::RegValueT<CrcioVx_SPEC>;

impl CrcioVx {
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci9(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRCI Flag of Buffer 31   CRCI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit CRCICLRx.CRCIy. This bit can be set by bit CRCISETx.CRCIy."]
    #[inline(always)]
    pub fn crci31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, CrcioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, CrcioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for CrcioVx {
    #[inline(always)]
    fn default() -> CrcioVx {
        <crate::RegValueT<CrcioVx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CrciseTx_SPEC;
impl crate::sealed::RegSpec for CrciseTx_SPEC {
    type DataType = u32;
}
#[doc = "CRCI Overview Set Register 0\n resetvalue={Application Reset:0x0}"]
pub type CrciseTx = crate::RegValueT<CrciseTx_SPEC>;

impl CrciseTx {
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci9(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CRCI Flag of Buffer 31   CRCI31. Setting this bit sets bit CRCIOVx.CRCIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, CrciseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, CrciseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for CrciseTx {
    #[inline(always)]
    fn default() -> CrciseTx {
        <crate::RegValueT<CrciseTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdr_SPEC;
impl crate::sealed::RegSpec for Fdr_SPEC {
    type DataType = u32;
}
#[doc = "PSI5 Fractional Divider Register\n resetvalue={Application Reset:0x0}"]
pub type Fdr = crate::RegValueT<Fdr_SPEC>;

impl Fdr {
    #[doc = "Step Value   STEP. Reload or addition value for RESULT."]
    #[inline(always)]
    pub fn step(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Fdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Fdr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Divider Mode   DM. DM selects normal or fractional divider mode."]
    #[inline(always)]
    pub fn dm(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Fdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Fdr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Result Value   RESULT. Bit field for the addition result."]
    #[inline(always)]
    pub fn result(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Fdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Fdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Fdr {
    #[inline(always)]
    fn default() -> Fdr {
        <crate::RegValueT<Fdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdrh_SPEC;
impl crate::sealed::RegSpec for Fdrh_SPEC {
    type DataType = u32;
}
#[doc = "Fractional Divider Register for Higher Bit Rate\n resetvalue={Application Reset:0x0}"]
pub type Fdrh = crate::RegValueT<Fdrh_SPEC>;

impl Fdrh {
    #[doc = "Step Value   STEP. Reload or addition value for RESULT."]
    #[inline(always)]
    pub fn step(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Fdrh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Fdrh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Divider Mode   DM. DM selects normal or fractional divider mode."]
    #[inline(always)]
    pub fn dm(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Fdrh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Fdrh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Result Value   RESULT. Bit field for the addition result."]
    #[inline(always)]
    pub fn result(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Fdrh_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Fdrh_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Fdrh {
    #[inline(always)]
    fn default() -> Fdrh {
        <crate::RegValueT<Fdrh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdrl_SPEC;
impl crate::sealed::RegSpec for Fdrl_SPEC {
    type DataType = u32;
}
#[doc = "Fractional Divider Register for Lower Bit Rate\n resetvalue={Application Reset:0x0}"]
pub type Fdrl = crate::RegValueT<Fdrl_SPEC>;

impl Fdrl {
    #[doc = "Step Value   STEP. Reload or addition value for RESULT."]
    #[inline(always)]
    pub fn step(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Fdrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Fdrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Divider Mode   DM. DM selects normal or fractional divider mode."]
    #[inline(always)]
    pub fn dm(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Fdrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Fdrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Result Value   RESULT. Bit field for the addition result."]
    #[inline(always)]
    pub fn result(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Fdrl_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Fdrl_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Fdrl {
    #[inline(always)]
    fn default() -> Fdrl {
        <crate::RegValueT<Fdrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdrt_SPEC;
impl crate::sealed::RegSpec for Fdrt_SPEC {
    type DataType = u32;
}
#[doc = "Fractional Divider Register for Time Stamp\n resetvalue={Application Reset:0x0}"]
pub type Fdrt = crate::RegValueT<Fdrt_SPEC>;

impl Fdrt {
    #[doc = "Step Value   STEP. Reload or addition value for RESULT."]
    #[inline(always)]
    pub fn step(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Fdrt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Fdrt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Divider Mode   DM. DM selects normal or fractional divider mode."]
    #[inline(always)]
    pub fn dm(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Fdrt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Fdrt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Result Value   RESULT. Bit field for the addition result."]
    #[inline(always)]
    pub fn result(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Fdrt_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Fdrt_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "External Time Stamp Clear Source Select   ECS. Selects the external trigger line that clears the global time stamp        counters TSRA B C.CTS if this is enabled by ECEA ECEB ECEC. Channel must        be disabled if changed  GCR.CEN   0 ."]
    #[inline(always)]
    pub fn ecs(
        self,
    ) -> crate::common::RegisterField<26, 0x7, 1, 0, u8, Fdrt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x7,1,0,u8, Fdrt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Time Stamp Clear Enable A   ECEA. Enables the external trigger line selected by ECS to clear the global time stamp counter TSRA.CTS on rising edge of the external trigger."]
    #[inline(always)]
    pub fn ecea(self) -> crate::common::RegisterFieldBool<29, 1, 0, Fdrt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Fdrt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "External Time Stamp Clear Enable B   ECEB. Enables the external trigger line selected by ECS to clear the global time stamp counter TSRB.CTS on rising edge of the external trigger."]
    #[inline(always)]
    pub fn eceb(self) -> crate::common::RegisterFieldBool<30, 1, 0, Fdrt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Fdrt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "External Time Stamp Clear Enable C   ECEC. Enables the external trigger line selected by ECS to clear the global time stamp counter TSRC.CTS on rising edge of the external trigger."]
    #[inline(always)]
    pub fn ecec(self) -> crate::common::RegisterFieldBool<31, 1, 0, Fdrt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Fdrt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Fdrt {
    #[inline(always)]
    fn default() -> Fdrt {
        <crate::RegValueT<Fdrt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gcr_SPEC;
impl crate::sealed::RegSpec for Gcr_SPEC {
    type DataType = u32;
}
#[doc = "Global Control Register\n resetvalue={Application Reset:0x1F}"]
pub type Gcr = crate::RegValueT<Gcr_SPEC>;

impl Gcr {
    #[doc = "CRCI   CRCI. is selected if bit is set."]
    #[inline(always)]
    pub fn crci(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI   NBI. is selected if bit is set."]
    #[inline(always)]
    pub fn nbi(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI   MEI. is selected if bit is set."]
    #[inline(always)]
    pub fn mei(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI   NFI. is selected if bit is set."]
    #[inline(always)]
    pub fn nfi(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI   TEI. is selected if bit is set."]
    #[inline(always)]
    pub fn tei(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel Trigger Counter CTV4.CTC   ETC4. This bit enables CTVx.CTC. The bits ETC0 ..x can be set with one write access to synchronously start all counters. This is required for proper sync pulse staggering.  If set  CTCx counts on  starting from its current value. CTCx can be written only if ETCx is cleared  stopped . In the device only ETC0  4 are available. All other must be written with 0  and always read 0."]
    #[inline(always)]
    pub fn etc0(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel Trigger Counter CTV4.CTC   ETC4. This bit enables CTVx.CTC. The bits ETC0 ..x can be set with one write access to synchronously start all counters. This is required for proper sync pulse staggering.  If set  CTCx counts on  starting from its current value. CTCx can be written only if ETCx is cleared  stopped . In the device only ETC0  4 are available. All other must be written with 0  and always read 0."]
    #[inline(always)]
    pub fn etc1(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel Trigger Counter CTV4.CTC   ETC4. This bit enables CTVx.CTC. The bits ETC0 ..x can be set with one write access to synchronously start all counters. This is required for proper sync pulse staggering.  If set  CTCx counts on  starting from its current value. CTCx can be written only if ETCx is cleared  stopped . In the device only ETC0  4 are available. All other must be written with 0  and always read 0."]
    #[inline(always)]
    pub fn etc2(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel Trigger Counter CTV4.CTC   ETC4. This bit enables CTVx.CTC. The bits ETC0 ..x can be set with one write access to synchronously start all counters. This is required for proper sync pulse staggering.  If set  CTCx counts on  starting from its current value. CTCx can be written only if ETCx is cleared  stopped . In the device only ETC0  4 are available. All other must be written with 0  and always read 0."]
    #[inline(always)]
    pub fn etc3(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel Trigger Counter CTV4.CTC   ETC4. This bit enables CTVx.CTC. The bits ETC0 ..x can be set with one write access to synchronously start all counters. This is required for proper sync pulse staggering.  If set  CTCx counts on  starting from its current value. CTCx can be written only if ETCx is cleared  stopped . In the device only ETC0  4 are available. All other must be written with 0  and always read 0."]
    #[inline(always)]
    pub fn etc4(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel   CEN4. This bit enables PSI5 Channel x. If cleared  all internal state machines of the receiver and the sender are forced to default idle state while all registers can be read and written. Used for configuration of a channel. In the device only CEN0  4 are available. All other must be written with 0  and always read 0."]
    #[inline(always)]
    pub fn cen0(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel   CEN4. This bit enables PSI5 Channel x. If cleared  all internal state machines of the receiver and the sender are forced to default idle state while all registers can be read and written. Used for configuration of a channel. In the device only CEN0  4 are available. All other must be written with 0  and always read 0."]
    #[inline(always)]
    pub fn cen1(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel   CEN4. This bit enables PSI5 Channel x. If cleared  all internal state machines of the receiver and the sender are forced to default idle state while all registers can be read and written. Used for configuration of a channel. In the device only CEN0  4 are available. All other must be written with 0  and always read 0."]
    #[inline(always)]
    pub fn cen2(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel   CEN4. This bit enables PSI5 Channel x. If cleared  all internal state machines of the receiver and the sender are forced to default idle state while all registers can be read and written. Used for configuration of a channel. In the device only CEN0  4 are available. All other must be written with 0  and always read 0."]
    #[inline(always)]
    pub fn cen3(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel   CEN4. This bit enables PSI5 Channel x. If cleared  all internal state machines of the receiver and the sender are forced to default idle state while all registers can be read and written. Used for configuration of a channel. In the device only CEN0  4 are available. All other must be written with 0  and always read 0."]
    #[inline(always)]
    pub fn cen4(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Gcr {
    #[inline(always)]
    fn default() -> Gcr {
        <crate::RegValueT<Gcr_SPEC> as RegisterValue<_>>::new(31)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0C3C000}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MODREV. This bit field defines the module revision number. The value of a module revision starts with 01 H  first revision ."]
    #[inline(always)]
    pub fn modrev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MODTYPE. This bit field defines the module as a 32 bit module  C0 H"]
    #[inline(always)]
    pub fn modtype(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MODNUM. This bit field defines the module identification number for the PSI5  00C3 H"]
    #[inline(always)]
    pub fn modnum(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Id_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Id {
    #[inline(always)]
    fn default() -> Id {
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(12828672)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InPx_SPEC;
impl crate::sealed::RegSpec for InPx_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Node Pointer Register 0\n resetvalue={Application Reset:0x0}"]
pub type InPx = crate::RegValueT<InPx_SPEC>;

impl InPx {
    #[doc = "Interrupt Node Pointer for Interrupt RSI   RSI. This bit field defines the interrupt node  that is requested due to the        set condition for bit INTSTATAx.RSI  if enabled by bit INTENAx.RSI ."]
    #[inline(always)]
    pub fn rsi(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, InPx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for Interrupt RDI   RDI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATAx.RDI  if enabled by bit INTENAx.RDI . For bit field definition  see RSI."]
    #[inline(always)]
    pub fn rdi(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, InPx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for Interrupt RBI   RBI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATAx.RBI  if enabled by bit INTENAx.RBI . For bit field definition  see RSI."]
    #[inline(always)]
    pub fn rbi(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, InPx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for Interrupt TDI   TDI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATAx.TPI  TSI  TOI  if enabled by bit INTENAx.TPI  TSI  TOI . For bit field definition  see RSI."]
    #[inline(always)]
    pub fn tdi(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, InPx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for Interrupt TBI   TBI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATAx.TPOI  TSOI  TOOI  if enabled by bit INTENAx.TPOI  TSOI  TOOI . For bit field definition  see RSI."]
    #[inline(always)]
    pub fn tbi(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, InPx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for Interrupt ERRI   ERRI. This bit field defines the interrupt node  that is requested due to the set condition for bit TEI  NFI  NBI  MEI  CRCI  RUI  RMI  CRCIy  WSIy  SOIy  SCRIy. For bit field definition  see RSI."]
    #[inline(always)]
    pub fn erri(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, InPx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for Interrupt SDI   SDI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATx.SDI  if enabled by bit INTENx.SDI . For bit field definition  see RSI."]
    #[inline(always)]
    pub fn sdi(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, InPx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for FWI   FWI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATx.FWI. For bit field definition  see RSI."]
    #[inline(always)]
    pub fn fwi(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, InPx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for InPx {
    #[inline(always)]
    fn default() -> InPx {
        <crate::RegValueT<InPx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntclrAx_SPEC;
impl crate::sealed::RegSpec for IntclrAx_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Clear Register A 0\n resetvalue={Application Reset:0x0}"]
pub type IntclrAx = crate::RegValueT<IntclrAx_SPEC>;

impl IntclrAx {
    #[doc = "Clear Interrupt Request Flag RSI   RSI. Setting this bit clears bit INTSTATx.RSI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi(self) -> crate::common::RegisterFieldBool<0, 1, 0, IntclrAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, IntclrAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag RDI   RDI. Setting this bit clears bit INTSTATx.RDI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi(self) -> crate::common::RegisterFieldBool<1, 1, 0, IntclrAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, IntclrAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag RBI   RBI. Setting this bit clears bit INTSTATx.RBI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rbi(self) -> crate::common::RegisterFieldBool<2, 1, 0, IntclrAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, IntclrAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag TEI   TEI. Setting this bit clears bit INTSTATx.TEI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei(self) -> crate::common::RegisterFieldBool<3, 1, 0, IntclrAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, IntclrAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag NBI   NBI. Setting this bit clears bit INTSTATx.NBI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi(self) -> crate::common::RegisterFieldBool<4, 1, 0, IntclrAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, IntclrAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag MEI   MEI. Setting this bit clears bit INTSTATx.MEI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei(self) -> crate::common::RegisterFieldBool<5, 1, 0, IntclrAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, IntclrAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag CRCI   CRCI. Setting this bit clears bit INTSTATx.CRCI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, IntclrAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, IntclrAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag FWI   FWI. Setting this bit clears bit INTSTATx.FWI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn fwi(self) -> crate::common::RegisterFieldBool<7, 1, 0, IntclrAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, IntclrAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag RUI   RUI. Setting this bit clears bit INTSTATx.RUI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rui(self) -> crate::common::RegisterFieldBool<8, 1, 0, IntclrAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, IntclrAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag RMI   RMI. Setting this bit clears bit INTSTATx.RMI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi(self) -> crate::common::RegisterFieldBool<9, 1, 0, IntclrAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, IntclrAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag TPI   TPI. Setting this bit clears bit INTSTATx.TPI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tpi(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, IntclrAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, IntclrAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag TPOI   TPOI. Setting this bit clears bit INTSTATx.TPOI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tpoi(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, IntclrAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, IntclrAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag TSI   TSI. Setting this bit clears bit INTSTATx.TSI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tsi(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, IntclrAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, IntclrAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag TSOI   TSOI. Setting this bit clears bit INTSTATx.TSOI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tsoi(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, IntclrAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, IntclrAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag TOI   TOI. Setting this bit clears bit INTSTATx.TOI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn toi(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, IntclrAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, IntclrAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag TOOI   TOOI. Setting this bit clears bit INTSTATx.TOOI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tooi(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, IntclrAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, IntclrAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag NFI   NFI. Setting this bit clears bit INTSTATx.NFI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, IntclrAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, IntclrAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for IntclrAx {
    #[inline(always)]
    fn default() -> IntclrAx {
        <crate::RegValueT<IntclrAx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntclrBx_SPEC;
impl crate::sealed::RegSpec for IntclrBx_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Clear Register A 0\n resetvalue={Application Reset:0x0}"]
pub type IntclrBx = crate::RegValueT<IntclrBx_SPEC>;

impl IntclrBx {
    #[doc = "Clear Interrupt Request Flag WSI5   WSI5. Setting this bit clears bit INTSTATx.WSIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn wsi0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag WSI5   WSI5. Setting this bit clears bit INTSTATx.WSIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn wsi1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag WSI5   WSI5. Setting this bit clears bit INTSTATx.WSIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn wsi2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag WSI5   WSI5. Setting this bit clears bit INTSTATx.WSIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn wsi3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag WSI5   WSI5. Setting this bit clears bit INTSTATx.WSIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn wsi4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag WSI5   WSI5. Setting this bit clears bit INTSTATx.WSIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn wsi5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag SDI5   SDI5. Setting this bit clears bit INTSTATx.SDIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn sdi0(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag SDI5   SDI5. Setting this bit clears bit INTSTATx.SDIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn sdi1(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag SDI5   SDI5. Setting this bit clears bit INTSTATx.SDIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn sdi2(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag SDI5   SDI5. Setting this bit clears bit INTSTATx.SDIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn sdi3(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag SDI5   SDI5. Setting this bit clears bit INTSTATx.SDIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn sdi4(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag SDI5   SDI5. Setting this bit clears bit INTSTATx.SDIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn sdi5(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag SOI5   SOI5. Setting this bit clears bit INTSTATx.SOIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn soi0(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag SOI5   SOI5. Setting this bit clears bit INTSTATx.SOIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn soi1(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag SOI5   SOI5. Setting this bit clears bit INTSTATx.SOIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn soi2(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag SOI5   SOI5. Setting this bit clears bit INTSTATx.SOIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn soi3(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag SOI5   SOI5. Setting this bit clears bit INTSTATx.SOIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn soi4(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag SOI5   SOI5. Setting this bit clears bit INTSTATx.SOIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn soi5(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag SCRI5   SCRI5. Setting this bit clears bit INTSTATx.SCRIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn scri0(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag SCRI5   SCRI5. Setting this bit clears bit INTSTATx.SCRIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn scri1(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag SCRI5   SCRI5. Setting this bit clears bit INTSTATx.SCRIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn scri2(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag SCRI5   SCRI5. Setting this bit clears bit INTSTATx.SCRIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn scri3(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag SCRI5   SCRI5. Setting this bit clears bit INTSTATx.SCRIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn scri4(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag SCRI5   SCRI5. Setting this bit clears bit INTSTATx.SCRIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn scri5(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, IntclrBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, IntclrBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for IntclrBx {
    #[inline(always)]
    fn default() -> IntclrBx {
        <crate::RegValueT<IntclrBx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntenAx_SPEC;
impl crate::sealed::RegSpec for IntenAx_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Enable Register A 0\n resetvalue={Application Reset:0x0}"]
pub type IntenAx = crate::RegValueT<IntenAx_SPEC>;

impl IntenAx {
    #[doc = "Enable Interrupt Request RSI   RSI"]
    #[inline(always)]
    pub fn rsi(self) -> crate::common::RegisterFieldBool<0, 1, 0, IntenAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, IntenAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request RDI   RDI"]
    #[inline(always)]
    pub fn rdi(self) -> crate::common::RegisterFieldBool<1, 1, 0, IntenAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, IntenAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request RBI   RBI"]
    #[inline(always)]
    pub fn rbi(self) -> crate::common::RegisterFieldBool<2, 1, 0, IntenAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, IntenAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request TEI   TEI"]
    #[inline(always)]
    pub fn tei(self) -> crate::common::RegisterFieldBool<3, 1, 0, IntenAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, IntenAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request NBI   NBI"]
    #[inline(always)]
    pub fn nbi(self) -> crate::common::RegisterFieldBool<4, 1, 0, IntenAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, IntenAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request MEII   MEI"]
    #[inline(always)]
    pub fn mei(self) -> crate::common::RegisterFieldBool<5, 1, 0, IntenAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, IntenAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request CRCI   CRCI"]
    #[inline(always)]
    pub fn crci(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, IntenAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, IntenAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request FWI   FWI"]
    #[inline(always)]
    pub fn fwi(self) -> crate::common::RegisterFieldBool<7, 1, 0, IntenAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, IntenAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request RUI   RUI"]
    #[inline(always)]
    pub fn rui(self) -> crate::common::RegisterFieldBool<8, 1, 0, IntenAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, IntenAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request RMII   RMI"]
    #[inline(always)]
    pub fn rmi(self) -> crate::common::RegisterFieldBool<9, 1, 0, IntenAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, IntenAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request TPI   TPI"]
    #[inline(always)]
    pub fn tpi(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, IntenAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, IntenAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request TPOI   TPOI"]
    #[inline(always)]
    pub fn tpoi(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, IntenAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, IntenAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request TSI   TSI"]
    #[inline(always)]
    pub fn tsi(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, IntenAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, IntenAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request TSOI   TSOI"]
    #[inline(always)]
    pub fn tsoi(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, IntenAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, IntenAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request TOI   TOI"]
    #[inline(always)]
    pub fn toi(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, IntenAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, IntenAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request TOOI   TOOI"]
    #[inline(always)]
    pub fn tooi(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, IntenAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, IntenAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request NFI   NFI"]
    #[inline(always)]
    pub fn nfi(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, IntenAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, IntenAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for IntenAx {
    #[inline(always)]
    fn default() -> IntenAx {
        <crate::RegValueT<IntenAx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntenBx_SPEC;
impl crate::sealed::RegSpec for IntenBx_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Enable Register B 0\n resetvalue={Application Reset:0x0}"]
pub type IntenBx = crate::RegValueT<IntenBx_SPEC>;

impl IntenBx {
    #[doc = "Enable Interrupt Request WSI5   WSI5"]
    #[inline(always)]
    pub fn wsi0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request WSI5   WSI5"]
    #[inline(always)]
    pub fn wsi1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request WSI5   WSI5"]
    #[inline(always)]
    pub fn wsi2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request WSI5   WSI5"]
    #[inline(always)]
    pub fn wsi3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request WSI5   WSI5"]
    #[inline(always)]
    pub fn wsi4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request WSI5   WSI5"]
    #[inline(always)]
    pub fn wsi5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request SDI5   SDI5"]
    #[inline(always)]
    pub fn sdi0(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request SDI5   SDI5"]
    #[inline(always)]
    pub fn sdi1(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request SDI5   SDI5"]
    #[inline(always)]
    pub fn sdi2(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request SDI5   SDI5"]
    #[inline(always)]
    pub fn sdi3(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request SDI5   SDI5"]
    #[inline(always)]
    pub fn sdi4(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request SDI5   SDI5"]
    #[inline(always)]
    pub fn sdi5(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request SOI5   SOI5"]
    #[inline(always)]
    pub fn soi0(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request SOI5   SOI5"]
    #[inline(always)]
    pub fn soi1(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request SOI5   SOI5"]
    #[inline(always)]
    pub fn soi2(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request SOI5   SOI5"]
    #[inline(always)]
    pub fn soi3(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request SOI5   SOI5"]
    #[inline(always)]
    pub fn soi4(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request SOI5   SOI5"]
    #[inline(always)]
    pub fn soi5(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request SCRI5   SCRI5"]
    #[inline(always)]
    pub fn scri0(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request SCRI5   SCRI5"]
    #[inline(always)]
    pub fn scri1(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request SCRI5   SCRI5"]
    #[inline(always)]
    pub fn scri2(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request SCRI5   SCRI5"]
    #[inline(always)]
    pub fn scri3(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request SCRI5   SCRI5"]
    #[inline(always)]
    pub fn scri4(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request SCRI5   SCRI5"]
    #[inline(always)]
    pub fn scri5(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, IntenBx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, IntenBx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for IntenBx {
    #[inline(always)]
    fn default() -> IntenBx {
        <crate::RegValueT<IntenBx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intov_SPEC;
impl crate::sealed::RegSpec for Intov_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Overview Register\n resetvalue={Application Reset:0x0}"]
pub type Intov = crate::RegValueT<Intov_SPEC>;

impl Intov {
    #[doc = "Interrupt Pending on any Node Pointer RSI   RSI. If any interrupt requested flag is set for any Node Pointer RSI in register INTSTATx AND the referring interrupt is enabled in INTENx  then INTOV.RSI is set. It is automatically reset if all flags in INTSTATx are cleared for which the referring interrupt is enabled in INTENx."]
    #[inline(always)]
    pub fn rsi(self) -> crate::common::RegisterFieldBool<0, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on any Node Pointer RDI   RDI. If any interrupt requested flag is set for any Node Pointer RDI in register INTSTATx AND the referring interrupt is enabled in INTENx  then INTOV.RDI is set. It is automatically reset if all flags in INTSTATx are cleared for which the referring interrupt is enabled in INTENx."]
    #[inline(always)]
    pub fn rdi(self) -> crate::common::RegisterFieldBool<1, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on any Node Pointer RBI   RBI. If any interrupt requested flag is set for any Node Pointer RBI in register INTSTATx AND the referring interrupt is enabled in INTENx  then INTOV.RBI is set. It is automatically reset if all flags in INTSTATx are cleared for which the referring interrupt is enabled in INTENx."]
    #[inline(always)]
    pub fn rbi(self) -> crate::common::RegisterFieldBool<2, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on any Node Pointer TDI   TDI. If any interrupt requested flag is set for any Node Pointer TDI in register INTSTATx AND the referring interrupt is enabled in INTENx  then INTOV.TDI is set. It is automatically reset if all flags in INTSTATx are cleared for which the referring interrupt is enabled in INTENx."]
    #[inline(always)]
    pub fn tdi(self) -> crate::common::RegisterFieldBool<3, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on any Node Pointer TBI   TBI. If any interrupt requested flag is set for any Node Pointer TBI in register INTSTATx AND the referring interrupt is enabled in INTENx  then INTOV.TBI is set. It is automatically reset if all flags in INTSTATx are cleared for which the referring interrupt is enabled in INTENx."]
    #[inline(always)]
    pub fn tbi(self) -> crate::common::RegisterFieldBool<4, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on any Node Pointer ERRI   ERRI. If any interrupt requested flag is set for any Node Pointer ERRI in register INTSTATx AND the referring interrupt is enabled in INTENx  then INTOV.ERRI is set. It is automatically reset if all flags in INTSTATx are cleared for which the referring interrupt is enabled in INTENx."]
    #[inline(always)]
    pub fn erri(self) -> crate::common::RegisterFieldBool<5, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on any Node Pointer SDI   SDI. If any interrupt requested flag is set for any Node Pointer SDI in register INTSTATx AND the referring interrupt is enabled in INTENx  then INTOV.SDI is set. It is automatically reset if all flags in INTSTATx are cleared for which the referring interrupt is enabled in INTENx."]
    #[inline(always)]
    pub fn sdi(self) -> crate::common::RegisterFieldBool<6, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on any Node Pointer FWI   FWI. If any interrupt requested flag is set for any Node Pointer FWI in register INTSTATx AND the referring interrupt is enabled in INTENx  then INTOV.FWI is set. It is automatically reset if all flags in INTSTATx are cleared for which the referring interrupt is enabled in INTENx."]
    #[inline(always)]
    pub fn fwi(self) -> crate::common::RegisterFieldBool<7, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Intov {
    #[inline(always)]
    fn default() -> Intov {
        <crate::RegValueT<Intov_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntsetAx_SPEC;
impl crate::sealed::RegSpec for IntsetAx_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Set Register A 0\n resetvalue={Application Reset:0x0}"]
pub type IntsetAx = crate::RegValueT<IntsetAx_SPEC>;

impl IntsetAx {
    #[doc = "Set Interrupt Request Flag RSI   RSI. Setting this bit set bit INTSTATx.RSI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi(self) -> crate::common::RegisterFieldBool<0, 1, 0, IntsetAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, IntsetAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag RDI   RDI. Setting this bit set bit INTSTATx.RDI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi(self) -> crate::common::RegisterFieldBool<1, 1, 0, IntsetAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, IntsetAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag RBI   RBI. Setting this bit set bit INTSTATx.RBI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rbi(self) -> crate::common::RegisterFieldBool<2, 1, 0, IntsetAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, IntsetAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag TEI   TEI. Setting this bit set bit INTSTATx.TEI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei(self) -> crate::common::RegisterFieldBool<3, 1, 0, IntsetAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, IntsetAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag NBI   NBI. Setting this bit set bit INTSTATx.NBI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi(self) -> crate::common::RegisterFieldBool<4, 1, 0, IntsetAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, IntsetAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag MEI   MEI. Setting this bit set bit INTSTATx.MEI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei(self) -> crate::common::RegisterFieldBool<5, 1, 0, IntsetAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, IntsetAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag CRCI   CRCI. Setting this bit set bit INTSTATx.CRCI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, IntsetAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, IntsetAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag FWI   FWI. Setting this bit set bit INTSTATx.FWI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn fwi(self) -> crate::common::RegisterFieldBool<7, 1, 0, IntsetAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, IntsetAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag RUI   RUI. Setting this bit set bit INTSTATx.RUI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rui(self) -> crate::common::RegisterFieldBool<8, 1, 0, IntsetAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, IntsetAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag RMI   RMI. Setting this bit set bit INTSTATx.RMI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi(self) -> crate::common::RegisterFieldBool<9, 1, 0, IntsetAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, IntsetAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag TPI   TPI. Setting this bit set bit INTSTATx.TPI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tpi(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, IntsetAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, IntsetAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag TPOI   TPOI. Setting this bit set bit INTSTATx.TPOI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tpoi(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, IntsetAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, IntsetAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag TSI   TSI. Setting this bit set bit INTSTATx.TSI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tsi(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, IntsetAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, IntsetAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag TSOI   TSOI. Setting this bit set bit INTSTATx.TSOI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tsoi(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, IntsetAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, IntsetAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag TOI   TOI. Setting this bit set bit INTSTATx.TOI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn toi(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, IntsetAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, IntsetAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag TOOI   TOOI. Setting this bit set bit INTSTATx.TOOI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tooi(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, IntsetAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, IntsetAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag NFI   NFI. Setting this bit set bit INTSTATx.NFI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, IntsetAx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, IntsetAx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for IntsetAx {
    #[inline(always)]
    fn default() -> IntsetAx {
        <crate::RegValueT<IntsetAx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntsetBx_SPEC;
impl crate::sealed::RegSpec for IntsetBx_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Set Register B 0\n resetvalue={Application Reset:0x0}"]
pub type IntsetBx = crate::RegValueT<IntsetBx_SPEC>;

impl IntsetBx {
    #[doc = "Set Interrupt Request Flag WSI5   WSI5. Setting this bit set bit INTSTATx.WSIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn wsi0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag WSI5   WSI5. Setting this bit set bit INTSTATx.WSIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn wsi1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag WSI5   WSI5. Setting this bit set bit INTSTATx.WSIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn wsi2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag WSI5   WSI5. Setting this bit set bit INTSTATx.WSIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn wsi3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag WSI5   WSI5. Setting this bit set bit INTSTATx.WSIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn wsi4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag WSI5   WSI5. Setting this bit set bit INTSTATx.WSIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn wsi5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag SDI5   SDI5. Setting this bit set bit INTSTATx.SDIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn sdi0(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag SDI5   SDI5. Setting this bit set bit INTSTATx.SDIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn sdi1(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag SDI5   SDI5. Setting this bit set bit INTSTATx.SDIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn sdi2(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag SDI5   SDI5. Setting this bit set bit INTSTATx.SDIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn sdi3(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag SDI5   SDI5. Setting this bit set bit INTSTATx.SDIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn sdi4(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag SDI5   SDI5. Setting this bit set bit INTSTATx.SDIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn sdi5(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag SOI5   SOI5. Setting this bit set bit INTSTATx.SOIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn soi0(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag SOI5   SOI5. Setting this bit set bit INTSTATx.SOIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn soi1(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag SOI5   SOI5. Setting this bit set bit INTSTATx.SOIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn soi2(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag SOI5   SOI5. Setting this bit set bit INTSTATx.SOIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn soi3(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag SOI5   SOI5. Setting this bit set bit INTSTATx.SOIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn soi4(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag SOI5   SOI5. Setting this bit set bit INTSTATx.SOIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn soi5(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag SCRI5   SCRI5. Setting this bit set bit INTSTATx.SCRIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn scri0(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag SCRI5   SCRI5. Setting this bit set bit INTSTATx.SCRIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn scri1(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag SCRI5   SCRI5. Setting this bit set bit INTSTATx.SCRIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn scri2(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag SCRI5   SCRI5. Setting this bit set bit INTSTATx.SCRIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn scri3(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag SCRI5   SCRI5. Setting this bit set bit INTSTATx.SCRIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn scri4(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag SCRI5   SCRI5. Setting this bit set bit INTSTATx.SCRIy. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn scri5(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, IntsetBx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, IntsetBx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for IntsetBx {
    #[inline(always)]
    fn default() -> IntsetBx {
        <crate::RegValueT<IntsetBx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntstatAx_SPEC;
impl crate::sealed::RegSpec for IntstatAx_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Status Register A 0\n resetvalue={Application Reset:0x0}"]
pub type IntstatAx = crate::RegValueT<IntstatAx_SPEC>;

impl IntstatAx {
    #[doc = "Receive Success Interrupt Request Flag   RSI. This bit is set at the successfully received end of a frame. It indicates that this frame is free of the errors NFI  TEI  NBI  MEI  CRCI if selected to be taken into account in register GCR. This bit can be cleared by bit INTCLRAx.RSI. This bit can be set by bit INTSETAx.RSI. This bit is set independently from INTENAx."]
    #[inline(always)]
    pub fn rsi(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, IntstatAx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, IntstatAx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive Data Interrupt Request Flag   RDI. RDI is activated when a received frame is moved to a Receive Data Register RDRL Hx. Both RDI and RSI will be issued together at correct reception. This bit can be cleared by bit INTCLRAx.RDI. This bit can be set by bit INTSETAx.RDI. This bit is set independently from INTENAx."]
    #[inline(always)]
    pub fn rdi(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, IntstatAx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, IntstatAx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive Buffer Overflow Interrupt Request Flag   RBI. This bit is set after a frame has been received while the old one was not read from RDRHx. I.e. the kernel wants to set any of the two interrupts RSI and RDI and finds any of these two interrupts already set. The old data is overwritten by the new data. Thus  RBI can be ignored if the receive memory is used. This bit is NOT cleared by reading RDRx. This bit can be cleared by bit INTCLRAx.RBI. This bit can be set by bit INTSETAx.RBI. This bit is set independently from INTENAx."]
    #[inline(always)]
    pub fn rbi(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, IntstatAx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, IntstatAx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Time Slot Error Interrupt Request Flag   TEI. In Synchronous Mode  RCRAx.ASYN   0   this bit is set if the SOF and the EOF of a frame are received in different time slots. The slots are constraint by the watch dog timer  see WDTxy . In Asynchronous Mode  RCRAx.ASYN   1   this bit is set if the distance between two frames is longer than specified in WDL0. This bit can be cleared by bit INTCLRAx.TEI. This bit can be set by bit INTSETAx.TEI. This bit is set independently from INTENAx."]
    #[inline(always)]
    pub fn tei(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, IntstatAx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, IntstatAx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Number of Bits Wrong Request Flag   NBI. This bit is set if the last frame received less bits than expected but no Manchester coding error occurred until S1  S2  M0 and M1 where received. I. e. fewer bits or Manchester error after M1 received. Note that the frame after the error might be completely wrong. The frame is no longer checked  from the Manchester error on  zeros are inserted and a start of frame is looked for  no occurrence of edges for at least one bit time followed by two zero bits. This bit can be cleared by bit INTCLRAx.NBI. This bit can be set by bit INTSETAx.NBI. This bit is set independently from INTENAx."]
    #[inline(always)]
    pub fn nbi(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, IntstatAx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, IntstatAx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Error in Message Bits Flag   MEI. This bit is set if a manchester error occurred in Bit M0 or M1.  Only if configured in RCRBx.MSG  Note that the frame after the error might be completely wrong. The frame is no longer checked  from the Manchester error on  zeros are inserted and a start of frame is looked for  no occurrence of edges for at least one bit time followed by two zero bits. This bit can be cleared by bit INTCLRAx.MEI. This bit can be set by bit INTSETAx.MEI. This bit is set independently from INTENAx."]
    #[inline(always)]
    pub fn mei(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, IntstatAx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, IntstatAx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Error Request Flag   CRCI. This bit is set if the CRC fails. Set as well if CRC can not be calculated  PDL   0  CRC cut off by Manchester Error  too few bits  BOT  Sync Pulse  This bit can be cleared by bit INTCLRAx.CRCI. This bit can be set by bit INTSETAx.CRCI. This bit is set independently from INTENAx."]
    #[inline(always)]
    pub fn crci(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, IntstatAx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, IntstatAx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "FIFO Warning Level Request Flag   FWI. This bit is set after if the configured warning level of the FIFOx was reached.  See RFCx  This bit can be cleared by bit INTCLRAx.FWI. This bit can be set by bit INTSETAx.FWI. This bit is set independently from INTENAx."]
    #[inline(always)]
    pub fn fwi(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, IntstatAx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, IntstatAx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive Memory Underrun Interrupt Request Flag   RUI. This bit is set after a SPB master reads from FIFO while no new data was available.  See RFCx  This bit can be cleared by bit INTCLRAx.RUI. This bit can be set by bit INTSETAx.RUI. This bit is set independently from INTENAx."]
    #[inline(always)]
    pub fn rui(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, IntstatAx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, IntstatAx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive Memory Overflow Flag   RMI. This bit is set after a frame has been received while RDIOVx.RDI RFCx.WRP  is set.  I.e. the kernel wants to set flag RDIxWRP and finds this flag already set. The old data is overwritten by the new data in the buffer memory. This bit can be cleared by bit INTCLRAx.RMIx. This bit can be set by bit INTSETAx.RMIx. This bit is set independently from INTENAx."]
    #[inline(always)]
    pub fn rmi(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, IntstatAx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, IntstatAx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transfer Preparation Interrupt Request Flag   TPI. This bit is set after data to be transferred has been moved from SDRL H to SSRL H. Thus a new value can be written to SDRL Hx and the prepared data can be checked and modified by SW. This bit is automatically cleared by writing SDRx. This is an exception  It allows for DMA transfers without CPU activity. This bit can be cleared by bit INTCLRAx.TDI. This bit can be set by bit INTSETAx.TDI. This bit is set independently from INTENAx."]
    #[inline(always)]
    pub fn tpi(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, IntstatAx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,IntstatAx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit Preparation Overflow Interrupt Request Flag   TPOI. This bit is set after if SDRL H is written while TPF is set.  The old data is NOT overwritten. This bit can be cleared by bit INTCLRAx.TPOI. This bit can be set by bit INTSETAx.TPOI. This bit is set independently from INTENAx."]
    #[inline(always)]
    pub fn tpoi(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, IntstatAx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,IntstatAx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transfer Shift Interrupt Request Flag   TSI. This bit is set after data to be transferred has been moved from SSRL H to SORL H. Thus a new value can be written to SSRL Hx. This bit is NOT cleared by writing SSRL Hx. This bit can be cleared by bit INTCLRAx.TSI. This bit can be set by bit INTSETAx.TSI. This bit is set independently from INTENAx."]
    #[inline(always)]
    pub fn tsi(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, IntstatAx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,IntstatAx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit Shift Overflow Interrupt Request Flag   TSOI. This bit is set after if SSRL H is written while TSF is set.  The old data is NOT overwritten. This bit can be cleared by bit INTCLRAx.TSOI. This bit can be set by bit INTSETAx.TSOI. This bit is set independently from INTENAx."]
    #[inline(always)]
    pub fn tsoi(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, IntstatAx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,IntstatAx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transfer Output Interrupt Request Flag   TOI. This bit is set after last bit to be transferred has been moved from SORL H to the pulse generator. Thus a new value can be written to SORL Hx. This bit is NOT cleared by writing SORL Hx. This bit can be cleared by bit INTCLRAx.TOI. This bit can be set by bit INTSETAx.TOI. This bit is set independently from INTENAx."]
    #[inline(always)]
    pub fn toi(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, IntstatAx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,IntstatAx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit Shift Overflow Interrupt Request Flag   TOOI. This bit is set after if SORL H is written while TOF is set.  The old data is NOT overwritten. This bit can be cleared by bit INTCLRAx.TOOI. This bit can be set by bit INTSETAx.TOOI. This bit is set independently from INTENAx."]
    #[inline(always)]
    pub fn tooi(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, IntstatAx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,IntstatAx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "No Frame Received Interrupt Flag   NFI. This bit is set at the end of a slot if a frame is expected in the slot and SOF was not received in the slot. A special case is where an SOF is received in slot 0. In this scenario  the NFI bit is only set at the end of slot 1 if a frame is expected in slot 1 and an EOF was not received in slot 1. If only the two start bits are received and no further data  this   x201c SOF  x201d  is discarded completely and treated like no frame received as well. If the   x201c SOF only frame  x201d  crosses the WDL  NFI occurs after WDL and EOF detection. If RCRBx.VBS is set  an empty frame is stored anyhow. In this case RDIx is issued as well. The only valid information is SC  TS  captured at the end of the slot  and this bit  NFI . This bit can be cleared by bit INTCLRAx.NFI. This bit can be set by bit INTSETAx.NFI. This bit is set independently from INTENAx."]
    #[inline(always)]
    pub fn nfi(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, IntstatAx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16,1,0,IntstatAx_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for IntstatAx {
    #[inline(always)]
    fn default() -> IntstatAx {
        <crate::RegValueT<IntstatAx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntstatBx_SPEC;
impl crate::sealed::RegSpec for IntstatBx_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Status Register B 0\n resetvalue={Application Reset:0x0}"]
pub type IntstatBx = crate::RegValueT<IntstatBx_SPEC>;

impl IntstatBx {
    #[doc = "Wrong Serial Protocol Error Request Flag   WSI5. This bit is set if the Messaging bits are configured and do not show a start sequence for more than 18 frames of slot y. This bit can be cleared by bit INTCLRBx.WSIy. This bit can be set by bit INTSETBx.WSIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn wsi0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, IntstatBx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wrong Serial Protocol Error Request Flag   WSI5. This bit is set if the Messaging bits are configured and do not show a start sequence for more than 18 frames of slot y. This bit can be cleared by bit INTCLRBx.WSIy. This bit can be set by bit INTSETBx.WSIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn wsi1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, IntstatBx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wrong Serial Protocol Error Request Flag   WSI5. This bit is set if the Messaging bits are configured and do not show a start sequence for more than 18 frames of slot y. This bit can be cleared by bit INTCLRBx.WSIy. This bit can be set by bit INTSETBx.WSIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn wsi2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, IntstatBx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wrong Serial Protocol Error Request Flag   WSI5. This bit is set if the Messaging bits are configured and do not show a start sequence for more than 18 frames of slot y. This bit can be cleared by bit INTCLRBx.WSIy. This bit can be set by bit INTSETBx.WSIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn wsi3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, IntstatBx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wrong Serial Protocol Error Request Flag   WSI5. This bit is set if the Messaging bits are configured and do not show a start sequence for more than 18 frames of slot y. This bit can be cleared by bit INTCLRBx.WSIy. This bit can be set by bit INTSETBx.WSIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn wsi4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, IntstatBx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wrong Serial Protocol Error Request Flag   WSI5. This bit is set if the Messaging bits are configured and do not show a start sequence for more than 18 frames of slot y. This bit can be cleared by bit INTCLRBx.WSIy. This bit can be set by bit INTSETBx.WSIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn wsi5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, IntstatBx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Serial Data Receive Interrupt Request Flag   SDI5. This bit is set after all serial data bits have been received via the Messaging bit field of slot y. This does NOT indicates a successful check of the CRC. This bit can be cleared by bit INTCLRBx.SDIy. This bit can be set by bit INTSETBx.SDIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn sdi0(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, IntstatBx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Serial Data Receive Interrupt Request Flag   SDI5. This bit is set after all serial data bits have been received via the Messaging bit field of slot y. This does NOT indicates a successful check of the CRC. This bit can be cleared by bit INTCLRBx.SDIy. This bit can be set by bit INTSETBx.SDIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn sdi1(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, IntstatBx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Serial Data Receive Interrupt Request Flag   SDI5. This bit is set after all serial data bits have been received via the Messaging bit field of slot y. This does NOT indicates a successful check of the CRC. This bit can be cleared by bit INTCLRBx.SDIy. This bit can be set by bit INTSETBx.SDIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn sdi2(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, IntstatBx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Serial Data Receive Interrupt Request Flag   SDI5. This bit is set after all serial data bits have been received via the Messaging bit field of slot y. This does NOT indicates a successful check of the CRC. This bit can be cleared by bit INTCLRBx.SDIy. This bit can be set by bit INTSETBx.SDIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn sdi3(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, IntstatBx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Serial Data Receive Interrupt Request Flag   SDI5. This bit is set after all serial data bits have been received via the Messaging bit field of slot y. This does NOT indicates a successful check of the CRC. This bit can be cleared by bit INTCLRBx.SDIy. This bit can be set by bit INTSETBx.SDIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn sdi4(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,IntstatBx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Serial Data Receive Interrupt Request Flag   SDI5. This bit is set after all serial data bits have been received via the Messaging bit field of slot y. This does NOT indicates a successful check of the CRC. This bit can be cleared by bit INTCLRBx.SDIy. This bit can be set by bit INTSETBx.SDIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn sdi5(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,IntstatBx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Serial Data Buffer Overrun Interrupt Request Flag   SOI5. This bit is set if the HW wants to set SDIy while SDIy is still set. This bit can be cleared by bit INTCLRBx.SOIy. This bit can be set by bit INTSETBx.SOIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn soi0(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,IntstatBx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Serial Data Buffer Overrun Interrupt Request Flag   SOI5. This bit is set if the HW wants to set SDIy while SDIy is still set. This bit can be cleared by bit INTCLRBx.SOIy. This bit can be set by bit INTSETBx.SOIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn soi1(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,IntstatBx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Serial Data Buffer Overrun Interrupt Request Flag   SOI5. This bit is set if the HW wants to set SDIy while SDIy is still set. This bit can be cleared by bit INTCLRBx.SOIy. This bit can be set by bit INTSETBx.SOIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn soi2(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,IntstatBx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Serial Data Buffer Overrun Interrupt Request Flag   SOI5. This bit is set if the HW wants to set SDIy while SDIy is still set. This bit can be cleared by bit INTCLRBx.SOIy. This bit can be set by bit INTSETBx.SOIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn soi3(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,IntstatBx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Serial Data Buffer Overrun Interrupt Request Flag   SOI5. This bit is set if the HW wants to set SDIy while SDIy is still set. This bit can be cleared by bit INTCLRBx.SOIy. This bit can be set by bit INTSETBx.SOIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn soi4(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16,1,0,IntstatBx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Serial Data Buffer Overrun Interrupt Request Flag   SOI5. This bit is set if the HW wants to set SDIy while SDIy is still set. This bit can be cleared by bit INTCLRBx.SOIy. This bit can be set by bit INTSETBx.SOIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn soi5(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17,1,0,IntstatBx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Serial Data CRC Error Request Flag   SCRI5. This bit is set if the CRC of the serial message fails. This includes a check of the Messaging bit field for correct 0 values of bit 1 in frames 7  13 and 18. This bit can be cleared by bit INTCLRBx.SCRIy. This bit can be set by bit INTSETx.SCRIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn scri0(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18,1,0,IntstatBx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Serial Data CRC Error Request Flag   SCRI5. This bit is set if the CRC of the serial message fails. This includes a check of the Messaging bit field for correct 0 values of bit 1 in frames 7  13 and 18. This bit can be cleared by bit INTCLRBx.SCRIy. This bit can be set by bit INTSETx.SCRIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn scri1(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19,1,0,IntstatBx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Serial Data CRC Error Request Flag   SCRI5. This bit is set if the CRC of the serial message fails. This includes a check of the Messaging bit field for correct 0 values of bit 1 in frames 7  13 and 18. This bit can be cleared by bit INTCLRBx.SCRIy. This bit can be set by bit INTSETx.SCRIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn scri2(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20,1,0,IntstatBx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Serial Data CRC Error Request Flag   SCRI5. This bit is set if the CRC of the serial message fails. This includes a check of the Messaging bit field for correct 0 values of bit 1 in frames 7  13 and 18. This bit can be cleared by bit INTCLRBx.SCRIy. This bit can be set by bit INTSETx.SCRIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn scri3(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21,1,0,IntstatBx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Serial Data CRC Error Request Flag   SCRI5. This bit is set if the CRC of the serial message fails. This includes a check of the Messaging bit field for correct 0 values of bit 1 in frames 7  13 and 18. This bit can be cleared by bit INTCLRBx.SCRIy. This bit can be set by bit INTSETx.SCRIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn scri4(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22,1,0,IntstatBx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Serial Data CRC Error Request Flag   SCRI5. This bit is set if the CRC of the serial message fails. This includes a check of the Messaging bit field for correct 0 values of bit 1 in frames 7  13 and 18. This bit can be cleared by bit INTCLRBx.SCRIy. This bit can be set by bit INTSETx.SCRIy. This bit is set independently from INTENBx."]
    #[inline(always)]
    pub fn scri5(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, IntstatBx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23,1,0,IntstatBx_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for IntstatBx {
    #[inline(always)]
    fn default() -> IntstatBx {
        <crate::RegValueT<IntstatBx_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel registers are set. The RST bit will be cleared  re set to  0   by the BPI FPI after the kernel reset was executed."]
    #[inline(always)]
    pub fn rst(self) -> crate::common::RegisterFieldBool<0, 1, 0, Krst0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Krst0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Kernel Reset Status   RSTSTAT. This bit indicates wether a kernel reset was executed or not. This bit is set by the BPI FPI after the execution of a kernel reset in the same clock cycle both reset bits. This bit can be cleared by writing with  1  to the CLR bit in the related KRSTCLR register."]
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel reset registers is set. The RST bit will be cleared  re set to  0   by the BPI FPI after the kernel reset was executed."]
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
pub struct MeiclRx_SPEC;
impl crate::sealed::RegSpec for MeiclRx_SPEC {
    type DataType = u32;
}
#[doc = "MEI Overview Clear Register 0\n resetvalue={Application Reset:0x0}"]
pub type MeiclRx = crate::RegValueT<MeiclRx_SPEC>;

impl MeiclRx {
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei0(self) -> crate::common::RegisterFieldBool<0, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei1(self) -> crate::common::RegisterFieldBool<1, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei2(self) -> crate::common::RegisterFieldBool<2, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei3(self) -> crate::common::RegisterFieldBool<3, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei4(self) -> crate::common::RegisterFieldBool<4, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei5(self) -> crate::common::RegisterFieldBool<5, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei6(self) -> crate::common::RegisterFieldBool<6, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei7(self) -> crate::common::RegisterFieldBool<7, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei8(self) -> crate::common::RegisterFieldBool<8, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei9(self) -> crate::common::RegisterFieldBool<9, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MEI Flag of Buffer 31   MEI31. Setting this bit clears bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, MeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, MeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for MeiclRx {
    #[inline(always)]
    fn default() -> MeiclRx {
        <crate::RegValueT<MeiclRx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MeioVx_SPEC;
impl crate::sealed::RegSpec for MeioVx_SPEC {
    type DataType = u32;
}
#[doc = "MEI Overview Register 0\n resetvalue={Application Reset:0x0}"]
pub type MeioVx = crate::RegValueT<MeioVx_SPEC>;

impl MeioVx {
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei0(self) -> crate::common::RegisterFieldBool<0, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei1(self) -> crate::common::RegisterFieldBool<1, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei2(self) -> crate::common::RegisterFieldBool<2, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei3(self) -> crate::common::RegisterFieldBool<3, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei4(self) -> crate::common::RegisterFieldBool<4, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei5(self) -> crate::common::RegisterFieldBool<5, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei6(self) -> crate::common::RegisterFieldBool<6, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei7(self) -> crate::common::RegisterFieldBool<7, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei8(self) -> crate::common::RegisterFieldBool<8, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei9(self) -> crate::common::RegisterFieldBool<9, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MEI Flag of Buffer 31   MEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit MEICLRx.MEIy. This bit can be set by bit MEISETx.MEIy."]
    #[inline(always)]
    pub fn mei31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, MeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, MeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for MeioVx {
    #[inline(always)]
    fn default() -> MeioVx {
        <crate::RegValueT<MeioVx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MeiseTx_SPEC;
impl crate::sealed::RegSpec for MeiseTx_SPEC {
    type DataType = u32;
}
#[doc = "MEI Overview Set Register 0\n resetvalue={Application Reset:0x0}"]
pub type MeiseTx = crate::RegValueT<MeiseTx_SPEC>;

impl MeiseTx {
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei0(self) -> crate::common::RegisterFieldBool<0, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei1(self) -> crate::common::RegisterFieldBool<1, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei2(self) -> crate::common::RegisterFieldBool<2, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei3(self) -> crate::common::RegisterFieldBool<3, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei4(self) -> crate::common::RegisterFieldBool<4, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei5(self) -> crate::common::RegisterFieldBool<5, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei6(self) -> crate::common::RegisterFieldBool<6, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei7(self) -> crate::common::RegisterFieldBool<7, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei8(self) -> crate::common::RegisterFieldBool<8, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei9(self) -> crate::common::RegisterFieldBool<9, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MEI Flag of Buffer 31   MEI31. Setting this bit sets bit MEIOVx.MEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn mei31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, MeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, MeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for MeiseTx {
    #[inline(always)]
    fn default() -> MeiseTx {
        <crate::RegValueT<MeiseTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NbiclRx_SPEC;
impl crate::sealed::RegSpec for NbiclRx_SPEC {
    type DataType = u32;
}
#[doc = "NBI Overview Clear Register 0\n resetvalue={Application Reset:0x0}"]
pub type NbiclRx = crate::RegValueT<NbiclRx_SPEC>;

impl NbiclRx {
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi0(self) -> crate::common::RegisterFieldBool<0, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi1(self) -> crate::common::RegisterFieldBool<1, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi2(self) -> crate::common::RegisterFieldBool<2, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi3(self) -> crate::common::RegisterFieldBool<3, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi4(self) -> crate::common::RegisterFieldBool<4, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi5(self) -> crate::common::RegisterFieldBool<5, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi6(self) -> crate::common::RegisterFieldBool<6, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi7(self) -> crate::common::RegisterFieldBool<7, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi8(self) -> crate::common::RegisterFieldBool<8, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi9(self) -> crate::common::RegisterFieldBool<9, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NBI Flag of Buffer 31   NBI31. Setting this bit clears bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, NbiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, NbiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for NbiclRx {
    #[inline(always)]
    fn default() -> NbiclRx {
        <crate::RegValueT<NbiclRx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NbioVx_SPEC;
impl crate::sealed::RegSpec for NbioVx_SPEC {
    type DataType = u32;
}
#[doc = "NBI Overview Register 0\n resetvalue={Application Reset:0x0}"]
pub type NbioVx = crate::RegValueT<NbioVx_SPEC>;

impl NbioVx {
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi0(self) -> crate::common::RegisterFieldBool<0, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi1(self) -> crate::common::RegisterFieldBool<1, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi2(self) -> crate::common::RegisterFieldBool<2, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi3(self) -> crate::common::RegisterFieldBool<3, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi4(self) -> crate::common::RegisterFieldBool<4, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi5(self) -> crate::common::RegisterFieldBool<5, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi6(self) -> crate::common::RegisterFieldBool<6, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi7(self) -> crate::common::RegisterFieldBool<7, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi8(self) -> crate::common::RegisterFieldBool<8, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi9(self) -> crate::common::RegisterFieldBool<9, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NBI Flag of Buffer 31   NBI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NBICLRx.NBIy. This bit can be set by bit NBISETx.NBIy."]
    #[inline(always)]
    pub fn nbi31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, NbioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, NbioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for NbioVx {
    #[inline(always)]
    fn default() -> NbioVx {
        <crate::RegValueT<NbioVx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NbiseTx_SPEC;
impl crate::sealed::RegSpec for NbiseTx_SPEC {
    type DataType = u32;
}
#[doc = "NBI Overview Set Register 0\n resetvalue={Application Reset:0x0}"]
pub type NbiseTx = crate::RegValueT<NbiseTx_SPEC>;

impl NbiseTx {
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi0(self) -> crate::common::RegisterFieldBool<0, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi1(self) -> crate::common::RegisterFieldBool<1, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi2(self) -> crate::common::RegisterFieldBool<2, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi3(self) -> crate::common::RegisterFieldBool<3, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi4(self) -> crate::common::RegisterFieldBool<4, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi5(self) -> crate::common::RegisterFieldBool<5, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi6(self) -> crate::common::RegisterFieldBool<6, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi7(self) -> crate::common::RegisterFieldBool<7, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi8(self) -> crate::common::RegisterFieldBool<8, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi9(self) -> crate::common::RegisterFieldBool<9, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NBI Flag of Buffer 31   NBI31. Setting this bit sets bit NBIOVx.NBIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nbi31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, NbiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, NbiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for NbiseTx {
    #[inline(always)]
    fn default() -> NbiseTx {
        <crate::RegValueT<NbiseTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NficlRx_SPEC;
impl crate::sealed::RegSpec for NficlRx_SPEC {
    type DataType = u32;
}
#[doc = "NFI Overview Clear Register 0\n resetvalue={Application Reset:0x0}"]
pub type NficlRx = crate::RegValueT<NficlRx_SPEC>;

impl NficlRx {
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi0(self) -> crate::common::RegisterFieldBool<0, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi1(self) -> crate::common::RegisterFieldBool<1, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi2(self) -> crate::common::RegisterFieldBool<2, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi3(self) -> crate::common::RegisterFieldBool<3, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi4(self) -> crate::common::RegisterFieldBool<4, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi5(self) -> crate::common::RegisterFieldBool<5, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi6(self) -> crate::common::RegisterFieldBool<6, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi7(self) -> crate::common::RegisterFieldBool<7, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi8(self) -> crate::common::RegisterFieldBool<8, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi9(self) -> crate::common::RegisterFieldBool<9, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear NFI Flag of Buffer 31   NFI31. Setting this bit clears bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, NficlRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, NficlRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for NficlRx {
    #[inline(always)]
    fn default() -> NficlRx {
        <crate::RegValueT<NficlRx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NfioVx_SPEC;
impl crate::sealed::RegSpec for NfioVx_SPEC {
    type DataType = u32;
}
#[doc = "NFI Overview Register 0\n resetvalue={Application Reset:0x0}"]
pub type NfioVx = crate::RegValueT<NfioVx_SPEC>;

impl NfioVx {
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi0(self) -> crate::common::RegisterFieldBool<0, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi1(self) -> crate::common::RegisterFieldBool<1, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi2(self) -> crate::common::RegisterFieldBool<2, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi3(self) -> crate::common::RegisterFieldBool<3, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi4(self) -> crate::common::RegisterFieldBool<4, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi5(self) -> crate::common::RegisterFieldBool<5, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi6(self) -> crate::common::RegisterFieldBool<6, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi7(self) -> crate::common::RegisterFieldBool<7, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi8(self) -> crate::common::RegisterFieldBool<8, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi9(self) -> crate::common::RegisterFieldBool<9, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NFI Flag of Buffer 31   NFI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit NFICLRx.NFIy. This bit can be set by bit NFISETx.NFIy."]
    #[inline(always)]
    pub fn nfi31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, NfioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, NfioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for NfioVx {
    #[inline(always)]
    fn default() -> NfioVx {
        <crate::RegValueT<NfioVx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NfiseTx_SPEC;
impl crate::sealed::RegSpec for NfiseTx_SPEC {
    type DataType = u32;
}
#[doc = "NFI Overview Set Register 0\n resetvalue={Application Reset:0x0}"]
pub type NfiseTx = crate::RegValueT<NfiseTx_SPEC>;

impl NfiseTx {
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi0(self) -> crate::common::RegisterFieldBool<0, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi1(self) -> crate::common::RegisterFieldBool<1, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi2(self) -> crate::common::RegisterFieldBool<2, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi3(self) -> crate::common::RegisterFieldBool<3, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi4(self) -> crate::common::RegisterFieldBool<4, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi5(self) -> crate::common::RegisterFieldBool<5, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi6(self) -> crate::common::RegisterFieldBool<6, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi7(self) -> crate::common::RegisterFieldBool<7, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi8(self) -> crate::common::RegisterFieldBool<8, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi9(self) -> crate::common::RegisterFieldBool<9, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set NFI Flag of Buffer 31   NFI31. Setting this bit sets bit NFIOVx.NFIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn nfi31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, NfiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, NfiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for NfiseTx {
    #[inline(always)]
    fn default() -> NfiseTx {
        <crate::RegValueT<NfiseTx_SPEC> as RegisterValue<_>>::new(0)
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

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RdFx_SPEC;
impl crate::sealed::RegSpec for RdFx_SPEC {
    type DataType = u32;
}
#[doc = "Receive Data FIFO 0\n resetvalue={Application Reset:0x0}"]
pub type RdFx = crate::RegValueT<RdFx_SPEC>;

impl RdFx {
    #[doc = "RD   RD. Shows the content of RDML Hxy  y   RFCx.REP 5 1   L H   RFCx.REP 0 . Reading this register triggers incrementation of REP and presentation of next FIFO value at next read access  Once a complete buffer is read  64 bit  2 accesses to RDF  all overview flags for this very buffer are cleared automatically."]
    #[inline(always)]
    pub fn rd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, RdFx_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, RdFx_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for RdFx {
    #[inline(always)]
    fn default() -> RdFx {
        <crate::RegValueT<RdFx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RdiclRx_SPEC;
impl crate::sealed::RegSpec for RdiclRx_SPEC {
    type DataType = u32;
}
#[doc = "RDI Overview Clear Register 0\n resetvalue={Application Reset:0x0}"]
pub type RdiclRx = crate::RegValueT<RdiclRx_SPEC>;

impl RdiclRx {
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi0(self) -> crate::common::RegisterFieldBool<0, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi1(self) -> crate::common::RegisterFieldBool<1, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi2(self) -> crate::common::RegisterFieldBool<2, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi3(self) -> crate::common::RegisterFieldBool<3, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi4(self) -> crate::common::RegisterFieldBool<4, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi5(self) -> crate::common::RegisterFieldBool<5, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi6(self) -> crate::common::RegisterFieldBool<6, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi7(self) -> crate::common::RegisterFieldBool<7, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi8(self) -> crate::common::RegisterFieldBool<8, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi9(self) -> crate::common::RegisterFieldBool<9, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RDI Flag of Buffer 31   RDI31. Setting this bit clears bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, RdiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, RdiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for RdiclRx {
    #[inline(always)]
    fn default() -> RdiclRx {
        <crate::RegValueT<RdiclRx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RdioVx_SPEC;
impl crate::sealed::RegSpec for RdioVx_SPEC {
    type DataType = u32;
}
#[doc = "RDI Overview Register 0\n resetvalue={Application Reset:0x0}"]
pub type RdioVx = crate::RegValueT<RdioVx_SPEC>;

impl RdioVx {
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi0(self) -> crate::common::RegisterFieldBool<0, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi1(self) -> crate::common::RegisterFieldBool<1, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi2(self) -> crate::common::RegisterFieldBool<2, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi3(self) -> crate::common::RegisterFieldBool<3, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi4(self) -> crate::common::RegisterFieldBool<4, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi5(self) -> crate::common::RegisterFieldBool<5, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi6(self) -> crate::common::RegisterFieldBool<6, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi7(self) -> crate::common::RegisterFieldBool<7, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi8(self) -> crate::common::RegisterFieldBool<8, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi9(self) -> crate::common::RegisterFieldBool<9, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RDI Flag of Buffer 31   RDI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RDICLRx.RDIy. This bit can be set by bit RDISETx.RDIy."]
    #[inline(always)]
    pub fn rdi31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, RdioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, RdioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for RdioVx {
    #[inline(always)]
    fn default() -> RdioVx {
        <crate::RegValueT<RdioVx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RdiseTx_SPEC;
impl crate::sealed::RegSpec for RdiseTx_SPEC {
    type DataType = u32;
}
#[doc = "RDI Overview Set Register 0\n resetvalue={Application Reset:0x0}"]
pub type RdiseTx = crate::RegValueT<RdiseTx_SPEC>;

impl RdiseTx {
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi0(self) -> crate::common::RegisterFieldBool<0, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi1(self) -> crate::common::RegisterFieldBool<1, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi2(self) -> crate::common::RegisterFieldBool<2, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi3(self) -> crate::common::RegisterFieldBool<3, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi4(self) -> crate::common::RegisterFieldBool<4, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi5(self) -> crate::common::RegisterFieldBool<5, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi6(self) -> crate::common::RegisterFieldBool<6, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi7(self) -> crate::common::RegisterFieldBool<7, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi8(self) -> crate::common::RegisterFieldBool<8, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi9(self) -> crate::common::RegisterFieldBool<9, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RDI Flag of Buffer 31   RDI31. Setting this bit sets bit RDIOVx.RDIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, RdiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, RdiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for RdiseTx {
    #[inline(always)]
    fn default() -> RdiseTx {
        <crate::RegValueT<RdiseTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfCx_SPEC;
impl crate::sealed::RegSpec for RfCx_SPEC {
    type DataType = u32;
}
#[doc = "Receive FIFO Control Register 0\n resetvalue={Application Reset:0x0}"]
pub type RfCx = crate::RegValueT<RfCx_SPEC>;

impl RfCx {
    #[doc = "FIFO Read Pointer   REP. points to the buffer to be read next.  Incremented after read. The last bit bit indicates if RDMxH or RDMxL is read. This LSB is ignored for FWL comparison WRP   REP collision  RBI  RUI"]
    #[inline(always)]
    pub fn rep(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, RfCx_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, RfCx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "FIFO Ring Buffer Write Pointer   WRP. points to the buffer written last.  Incremented before write. The last bit indicates if RDMxH or RDMxL is written. This LSB is ignored for FWL comparison WRP   REP collision  RBI  RUI ."]
    #[inline(always)]
    pub fn wrp(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, RfCx_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3f,1,0,u8, RfCx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "FIFO Warning Level   FWL. Number of new entries at which the FIFO Warning Interrupt FWIx is set automatically.  For calculation WRP and REP are used. I.e. only if the memory is read via RDFx  this works properly."]
    #[inline(always)]
    pub fn fwl(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, RfCx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, RfCx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Write Pointer WRAP Indicator   WRAP. If set  the Write Pointer is one wrap around ahead of the Read Pointer. It is cleared  if the Read Pointer wraps around too or when FLU is set by SW."]
    #[inline(always)]
    pub fn wrap(self) -> crate::common::RegisterFieldBool<29, 1, 0, RfCx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, RfCx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Flush Request   FRQ. if set   FIFO read and write pointers are not incremented any more due to excessive RMI.  Too many overrun conditions .  This bit is cleared when FLU is set."]
    #[inline(always)]
    pub fn frq(self) -> crate::common::RegisterFieldBool<30, 1, 0, RfCx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, RfCx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Flush   FLU. if set   FIFO read and write pointers are cleared. Bits and WRAP are cleared. SW needs to clear interrupts as needed. This bit can only be set and is cleared by HW. Always read as 0."]
    #[inline(always)]
    pub fn flu(self) -> crate::common::RegisterFieldBool<31, 1, 0, RfCx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, RfCx_SPEC, crate::common::W>::from_register(
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
pub struct RmiclRx_SPEC;
impl crate::sealed::RegSpec for RmiclRx_SPEC {
    type DataType = u32;
}
#[doc = "RMI Overview Clear Register 0\n resetvalue={Application Reset:0x0}"]
pub type RmiclRx = crate::RegValueT<RmiclRx_SPEC>;

impl RmiclRx {
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi0(self) -> crate::common::RegisterFieldBool<0, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi1(self) -> crate::common::RegisterFieldBool<1, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi2(self) -> crate::common::RegisterFieldBool<2, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi3(self) -> crate::common::RegisterFieldBool<3, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi4(self) -> crate::common::RegisterFieldBool<4, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi5(self) -> crate::common::RegisterFieldBool<5, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi6(self) -> crate::common::RegisterFieldBool<6, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi7(self) -> crate::common::RegisterFieldBool<7, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi8(self) -> crate::common::RegisterFieldBool<8, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi9(self) -> crate::common::RegisterFieldBool<9, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RMI Flag of Buffer 31   RMI31. Setting this bit clears bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, RmiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, RmiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for RmiclRx {
    #[inline(always)]
    fn default() -> RmiclRx {
        <crate::RegValueT<RmiclRx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmioVx_SPEC;
impl crate::sealed::RegSpec for RmioVx_SPEC {
    type DataType = u32;
}
#[doc = "RMI Overview Register 0\n resetvalue={Application Reset:0x0}"]
pub type RmioVx = crate::RegValueT<RmioVx_SPEC>;

impl RmioVx {
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi0(self) -> crate::common::RegisterFieldBool<0, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi1(self) -> crate::common::RegisterFieldBool<1, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi2(self) -> crate::common::RegisterFieldBool<2, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi3(self) -> crate::common::RegisterFieldBool<3, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi4(self) -> crate::common::RegisterFieldBool<4, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi5(self) -> crate::common::RegisterFieldBool<5, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi6(self) -> crate::common::RegisterFieldBool<6, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi7(self) -> crate::common::RegisterFieldBool<7, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi8(self) -> crate::common::RegisterFieldBool<8, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi9(self) -> crate::common::RegisterFieldBool<9, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RMI Flag of Buffer 31   RMI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RMICLRx.RMIy. This bit can be set by bit RMISETx.RMIy."]
    #[inline(always)]
    pub fn rmi31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, RmioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, RmioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for RmioVx {
    #[inline(always)]
    fn default() -> RmioVx {
        <crate::RegValueT<RmioVx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RmiseTx_SPEC;
impl crate::sealed::RegSpec for RmiseTx_SPEC {
    type DataType = u32;
}
#[doc = "RMI Overview Set Register 0\n resetvalue={Application Reset:0x0}"]
pub type RmiseTx = crate::RegValueT<RmiseTx_SPEC>;

impl RmiseTx {
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi0(self) -> crate::common::RegisterFieldBool<0, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi1(self) -> crate::common::RegisterFieldBool<1, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi2(self) -> crate::common::RegisterFieldBool<2, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi3(self) -> crate::common::RegisterFieldBool<3, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi4(self) -> crate::common::RegisterFieldBool<4, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi5(self) -> crate::common::RegisterFieldBool<5, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi6(self) -> crate::common::RegisterFieldBool<6, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi7(self) -> crate::common::RegisterFieldBool<7, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi8(self) -> crate::common::RegisterFieldBool<8, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi9(self) -> crate::common::RegisterFieldBool<9, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RMI Flag of Buffer 31   RMI31. Setting this bit sets bit RMIOVx.RMIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rmi31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, RmiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, RmiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for RmiseTx {
    #[inline(always)]
    fn default() -> RmiseTx {
        <crate::RegValueT<RmiseTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RsiclRx_SPEC;
impl crate::sealed::RegSpec for RsiclRx_SPEC {
    type DataType = u32;
}
#[doc = "RSI Overview Clear Register 0\n resetvalue={Application Reset:0x0}"]
pub type RsiclRx = crate::RegValueT<RsiclRx_SPEC>;

impl RsiclRx {
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi0(self) -> crate::common::RegisterFieldBool<0, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi1(self) -> crate::common::RegisterFieldBool<1, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi2(self) -> crate::common::RegisterFieldBool<2, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi3(self) -> crate::common::RegisterFieldBool<3, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi4(self) -> crate::common::RegisterFieldBool<4, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi5(self) -> crate::common::RegisterFieldBool<5, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi6(self) -> crate::common::RegisterFieldBool<6, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi7(self) -> crate::common::RegisterFieldBool<7, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi8(self) -> crate::common::RegisterFieldBool<8, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi9(self) -> crate::common::RegisterFieldBool<9, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RSI Flag of Buffer 31   RSI31. Setting this bit clears bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, RsiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, RsiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for RsiclRx {
    #[inline(always)]
    fn default() -> RsiclRx {
        <crate::RegValueT<RsiclRx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RsioVx_SPEC;
impl crate::sealed::RegSpec for RsioVx_SPEC {
    type DataType = u32;
}
#[doc = "RSI Overview Register 0\n resetvalue={Application Reset:0x0}"]
pub type RsioVx = crate::RegValueT<RsioVx_SPEC>;

impl RsioVx {
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi0(self) -> crate::common::RegisterFieldBool<0, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi1(self) -> crate::common::RegisterFieldBool<1, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi2(self) -> crate::common::RegisterFieldBool<2, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi3(self) -> crate::common::RegisterFieldBool<3, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi4(self) -> crate::common::RegisterFieldBool<4, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi5(self) -> crate::common::RegisterFieldBool<5, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi6(self) -> crate::common::RegisterFieldBool<6, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi7(self) -> crate::common::RegisterFieldBool<7, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi8(self) -> crate::common::RegisterFieldBool<8, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi9(self) -> crate::common::RegisterFieldBool<9, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RSI Flag of Buffer 31   RSI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit RSICLRx.RSIy. This bit can be set by bit RSISETx.RSIy."]
    #[inline(always)]
    pub fn rsi31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, RsioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, RsioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for RsioVx {
    #[inline(always)]
    fn default() -> RsioVx {
        <crate::RegValueT<RsioVx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RsiseTx_SPEC;
impl crate::sealed::RegSpec for RsiseTx_SPEC {
    type DataType = u32;
}
#[doc = "RSI Overview Set Register 0\n resetvalue={Application Reset:0x0}"]
pub type RsiseTx = crate::RegValueT<RsiseTx_SPEC>;

impl RsiseTx {
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi0(self) -> crate::common::RegisterFieldBool<0, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi1(self) -> crate::common::RegisterFieldBool<1, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi2(self) -> crate::common::RegisterFieldBool<2, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi3(self) -> crate::common::RegisterFieldBool<3, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi4(self) -> crate::common::RegisterFieldBool<4, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi5(self) -> crate::common::RegisterFieldBool<5, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi6(self) -> crate::common::RegisterFieldBool<6, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi7(self) -> crate::common::RegisterFieldBool<7, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi8(self) -> crate::common::RegisterFieldBool<8, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi9(self) -> crate::common::RegisterFieldBool<9, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set RSI Flag of Buffer 31   RSI31. Setting this bit sets bit RSIOVx.RSIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, RsiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, RsiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for RsiseTx {
    #[inline(always)]
    fn default() -> RsiseTx {
        <crate::RegValueT<RsiseTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TeiclRx_SPEC;
impl crate::sealed::RegSpec for TeiclRx_SPEC {
    type DataType = u32;
}
#[doc = "TEI Overview Clear Register 0\n resetvalue={Application Reset:0x0}"]
pub type TeiclRx = crate::RegValueT<TeiclRx_SPEC>;

impl TeiclRx {
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei0(self) -> crate::common::RegisterFieldBool<0, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei1(self) -> crate::common::RegisterFieldBool<1, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei2(self) -> crate::common::RegisterFieldBool<2, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei3(self) -> crate::common::RegisterFieldBool<3, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei4(self) -> crate::common::RegisterFieldBool<4, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei5(self) -> crate::common::RegisterFieldBool<5, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei6(self) -> crate::common::RegisterFieldBool<6, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei7(self) -> crate::common::RegisterFieldBool<7, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei8(self) -> crate::common::RegisterFieldBool<8, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei9(self) -> crate::common::RegisterFieldBool<9, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear TEI Flag of Buffer 31   TEI31. Setting this bit clears bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, TeiclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, TeiclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for TeiclRx {
    #[inline(always)]
    fn default() -> TeiclRx {
        <crate::RegValueT<TeiclRx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TeioVx_SPEC;
impl crate::sealed::RegSpec for TeioVx_SPEC {
    type DataType = u32;
}
#[doc = "TEI Overview Register 0\n resetvalue={Application Reset:0x0}"]
pub type TeioVx = crate::RegValueT<TeioVx_SPEC>;

impl TeioVx {
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei0(self) -> crate::common::RegisterFieldBool<0, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei1(self) -> crate::common::RegisterFieldBool<1, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei2(self) -> crate::common::RegisterFieldBool<2, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei3(self) -> crate::common::RegisterFieldBool<3, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei4(self) -> crate::common::RegisterFieldBool<4, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei5(self) -> crate::common::RegisterFieldBool<5, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei6(self) -> crate::common::RegisterFieldBool<6, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei7(self) -> crate::common::RegisterFieldBool<7, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei8(self) -> crate::common::RegisterFieldBool<8, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei9(self) -> crate::common::RegisterFieldBool<9, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI Flag of Buffer 31   TEI31. Copied from INTSTATAx at frame end while RFCx.WRP pointed to buffer y. This bit can be cleared by bit TEICLRx.TEIy. This bit can be set by bit TEISETx.TEIy."]
    #[inline(always)]
    pub fn tei31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, TeioVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, TeioVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for TeioVx {
    #[inline(always)]
    fn default() -> TeioVx {
        <crate::RegValueT<TeioVx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TeiseTx_SPEC;
impl crate::sealed::RegSpec for TeiseTx_SPEC {
    type DataType = u32;
}
#[doc = "TEI Overview Set Register 0\n resetvalue={Application Reset:0x0}"]
pub type TeiseTx = crate::RegValueT<TeiseTx_SPEC>;

impl TeiseTx {
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei0(self) -> crate::common::RegisterFieldBool<0, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei1(self) -> crate::common::RegisterFieldBool<1, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei2(self) -> crate::common::RegisterFieldBool<2, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei3(self) -> crate::common::RegisterFieldBool<3, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei4(self) -> crate::common::RegisterFieldBool<4, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei5(self) -> crate::common::RegisterFieldBool<5, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei6(self) -> crate::common::RegisterFieldBool<6, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei7(self) -> crate::common::RegisterFieldBool<7, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei8(self) -> crate::common::RegisterFieldBool<8, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei9(self) -> crate::common::RegisterFieldBool<9, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set TEI Flag of Buffer 31   TEI31. Setting this bit sets bit TEIOVx.TEIy Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, TeiseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, TeiseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for TeiseTx {
    #[inline(always)]
    fn default() -> TeiseTx {
        <crate::RegValueT<TeiseTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsra_SPEC;
impl crate::sealed::RegSpec for Tsra_SPEC {
    type DataType = u32;
}
#[doc = "Module Time Stamp Register A\n resetvalue={Application Reset:0x0}"]
pub type Tsra = crate::RegValueT<Tsra_SPEC>;

impl Tsra {
    #[doc = "Current Time Stamp for the Module   CTS. This bit field shows the current time stamp."]
    #[inline(always)]
    pub fn cts(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, Tsra_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffff,1,0,u32, Tsra_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "External Time Base Select   ETB. Selects the external clock line for counter CTS. Channel must be        disabled if changed  GCR.CEN   0 ."]
    #[inline(always)]
    pub fn etb(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Tsra_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Tsra_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Time Base Select   TBS. This bit selects the clock source for CTS"]
    #[inline(always)]
    pub fn tbs(self) -> crate::common::RegisterFieldBool<27, 1, 0, Tsra_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Tsra_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear All Current Time Stamp Counters   ACLR. If set  this bit clears TSRA B C.CTS. TSRA B C.CTS count on  starting from 0. This bit is automatically cleared by HW and will always be read as zero."]
    #[inline(always)]
    pub fn aclr(self) -> crate::common::RegisterFieldBool<30, 1, 0, Tsra_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Tsra_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Current Time Stamp for the Module   CLR. If set  this bit clears CTS. CTS counts on  starting from 0. This bit is automatically cleared by HW and will always be read as zero."]
    #[inline(always)]
    pub fn clr(self) -> crate::common::RegisterFieldBool<31, 1, 0, Tsra_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Tsra_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Tsra {
    #[inline(always)]
    fn default() -> Tsra {
        <crate::RegValueT<Tsra_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsrb_SPEC;
impl crate::sealed::RegSpec for Tsrb_SPEC {
    type DataType = u32;
}
#[doc = "Time Stamp Register B\n resetvalue={Application Reset:0x0}"]
pub type Tsrb = crate::RegValueT<Tsrb_SPEC>;

impl Tsrb {
    #[doc = "Current Time Stamp for the Module   CTS. This bit field shows the current time stamp."]
    #[inline(always)]
    pub fn cts(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, Tsrb_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffff,1,0,u32, Tsrb_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "External Time Base Select   ETB. Selects the external clock line for counter CTS. Channel must be        disabled if changed  GCR.CEN   0 ."]
    #[inline(always)]
    pub fn etb(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Tsrb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Tsrb_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Time Base Select   TBS. This bit selects the clock source for CTS"]
    #[inline(always)]
    pub fn tbs(self) -> crate::common::RegisterFieldBool<27, 1, 0, Tsrb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Tsrb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear All Current Time Stamp Counters   ACLR. If set  this bit clears TSRA B C.CTS. TSRA B C.CTS count on  starting from 0. This bit is automatically cleared by HW and will always be read as zero."]
    #[inline(always)]
    pub fn aclr(self) -> crate::common::RegisterFieldBool<30, 1, 0, Tsrb_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Tsrb_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Current Time Stamp for the Module   CLR. If set  this bit clears CTS. CTS counts on  starting from 0. This bit is automatically cleared by HW and will always be read as zero."]
    #[inline(always)]
    pub fn clr(self) -> crate::common::RegisterFieldBool<31, 1, 0, Tsrb_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Tsrb_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Tsrb {
    #[inline(always)]
    fn default() -> Tsrb {
        <crate::RegValueT<Tsrb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsrc_SPEC;
impl crate::sealed::RegSpec for Tsrc_SPEC {
    type DataType = u32;
}
#[doc = "Module Time Stamp Register C\n resetvalue={Application Reset:0x0}"]
pub type Tsrc = crate::RegValueT<Tsrc_SPEC>;

impl Tsrc {
    #[doc = "Current Time Stamp for the Module   CTS. This bit field shows the current time stamp."]
    #[inline(always)]
    pub fn cts(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, Tsrc_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffff,1,0,u32, Tsrc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "External Time Base Select   ETB. Selects the external clock line for counter CTS. Channel must be        disabled if changed  GCR.CEN   0 ."]
    #[inline(always)]
    pub fn etb(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Tsrc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Tsrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Time Base Select   TBS. This bit selects the clock source for CTS"]
    #[inline(always)]
    pub fn tbs(self) -> crate::common::RegisterFieldBool<27, 1, 0, Tsrc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Tsrc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear All Current Time Stamp Counters   ACLR. If set  this bit clears TSRA B C.CTS. TSRA B C.CTS count on  starting from 0. This bit is automatically cleared by HW and will always be read as zero."]
    #[inline(always)]
    pub fn aclr(self) -> crate::common::RegisterFieldBool<30, 1, 0, Tsrc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Tsrc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Current Time Stamp for the Module   CLR. If set  this bit clears CTS. CTS counts on  starting from 0. This bit is automatically cleared by HW and will always be read as zero."]
    #[inline(always)]
    pub fn clr(self) -> crate::common::RegisterFieldBool<31, 1, 0, Tsrc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Tsrc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Tsrc {
    #[inline(always)]
    fn default() -> Tsrc {
        <crate::RegValueT<Tsrc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc = "CH"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch(pub(super) *mut u8);
unsafe impl core::marker::Send for Ch {}
unsafe impl core::marker::Sync for Ch {}
impl Ch {
    #[doc = "Channel Trigger Value Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ctvx(&self) -> crate::common::Reg<ch::CtVx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(92usize)) }
    }
    #[doc = "Input and Output Control Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn iocrx(&self) -> crate::common::Reg<ch::IocRx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Pulse Generation Control Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pgcx(&self) -> crate::common::Reg<ch::PgCx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(88usize)) }
    }
    #[doc = "Receiver Control Register A 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rcrax(&self) -> crate::common::Reg<ch::RcrAx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Receiver Control Register B 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rcrbx(&self) -> crate::common::Reg<ch::RcrBx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Receiver Control Register C 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rcrcx(&self) -> crate::common::Reg<ch::RcrCx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Receive Data Register High 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rdrhx(&self) -> crate::common::Reg<ch::RdrHx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(84usize)) }
    }
    #[doc = "Receive Data Register Low 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rdrlx(&self) -> crate::common::Reg<ch::RdrLx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }
    #[doc = "Receive Status Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rsrx(&self) -> crate::common::Reg<ch::RsRx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }
    #[doc = "Send Control Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn scrx(&self) -> crate::common::Reg<ch::ScRx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }
    #[doc = "Send Data Register High 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sdrhx(&self) -> crate::common::Reg<ch::SdrHx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(104usize)) }
    }
    #[doc = "Send Data Register Low 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sdrlx(&self) -> crate::common::Reg<ch::SdrLx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(100usize)) }
    }
    #[doc = "Serial Data and Status Register 00\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sdsxz(&self) -> [crate::common::Reg<ch::SdSxz_SPEC, crate::common::R>; 6] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x14usize)),
            ]
        }
    }
    #[doc = "Start of Frame Time Stamp Capture Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sftscx(&self) -> crate::common::Reg<ch::SftsCx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(76usize)) }
    }
    #[doc = "Send Output Register High 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sorhx(&self) -> crate::common::Reg<ch::SorHx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(120usize)) }
    }
    #[doc = "Send Output Register Low 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sorlx(&self) -> crate::common::Reg<ch::SorLx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(116usize)) }
    }
    #[doc = "Start of Pulse Time Stamp Capture Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sptscx(&self) -> crate::common::Reg<ch::SptsCx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }
    #[doc = "Send Shift Register High 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ssrhx(&self) -> crate::common::Reg<ch::SsrHx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(112usize)) }
    }
    #[doc = "Send Shift Register Low 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ssrlx(&self) -> crate::common::Reg<ch::SsrLx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(108usize)) }
    }
    #[doc = "Watch Dog Timer Register 00\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn wdtxw(&self) -> [crate::common::Reg<ch::WdTxw_SPEC, crate::common::RW>; 7] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x10usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x10usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x10usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x10usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x10usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x10usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x10usize + 0x18usize)),
            ]
        }
    }
}
pub mod ch {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtVx_SPEC;
    impl crate::sealed::RegSpec for CtVx_SPEC {
        type DataType = u32;
    }
    #[doc = "Channel Trigger Value Register 0\n resetvalue={Application Reset:0x0}"]
    pub type CtVx = crate::RegValueT<CtVx_SPEC>;

    impl CtVx {
        #[doc = "Channel Trigger Value CTV   CTV. Contains the compare value  exact match  of Channel Trigger CTC at which        a sync pulse is triggered for channel x and the counter CTC is cleared.        If cleared  CTC is stopped and no pulse is generated."]
        #[inline(always)]
        pub fn ctv(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, CtVx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, CtVx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Channel Trigger Counter   CTC. This bit field allows to read the current counter value of the reset        timer cell CTVx. If GCR.ETCx is cleared  CTC can be written."]
        #[inline(always)]
        pub fn ctc(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, CtVx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, CtVx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for CtVx {
        #[inline(always)]
        fn default() -> CtVx {
            <crate::RegValueT<CtVx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IocRx_SPEC;
    impl crate::sealed::RegSpec for IocRx_SPEC {
        type DataType = u32;
    }
    #[doc = "Input and Output Control Register 0\n resetvalue={Application Reset:0x0}"]
    pub type IocRx = crate::RegValueT<IocRx_SPEC>;

    impl IocRx {
        #[doc = "Alternate Input Select   ALTI. Selects the alternate input for channel x"]
        #[inline(always)]
        pub fn alti(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, IocRx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<0,0x3,1,0,u8, IocRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Digital Glitch Filter Depth   DEPTH. DEPTH determines the number of port input samples clocked with f PSI5        that are taken into account for the calculation of the floating average.        The higher DEPTH is chosen to be  the longer the glitches that are        suppressed and the longer the delay of the input signal introduced by        this filter."]
        #[inline(always)]
        pub fn depth(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, IocRx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<4,0xf,1,0,u8, IocRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Output Inverter Enable Channel x   OIE. Selects the Pulse Polarity of the output of channel x"]
        #[inline(always)]
        pub fn oie(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, IocRx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,IocRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Input Inverter Enable Channel x   IIE. Selects the Pulse Polarity of the input of channel x"]
        #[inline(always)]
        pub fn iie(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, IocRx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9,1,0,IocRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Rising Edge Glitch Flag for Channel x   REG. Shows the status of the glitch detection of channel x REG is cleared by setting CREG."]
        #[inline(always)]
        pub fn reg(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, IocRx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<12,1,0,IocRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Falling Edge Glitch Flag for Channel x   FEG. Shows the status of the glitch detection of channel x FEG is cleared by setting CFEG."]
        #[inline(always)]
        pub fn feg(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, IocRx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<13,1,0,IocRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Clear Rising Edge Glitch Flag for Channel x   CREG. Clears the status flag REG CREG always read zero."]
        #[inline(always)]
        pub fn creg(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, IocRx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<14,1,0,IocRx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Falling Edge Glitch Flag for Channel x   CFEG. Clears the status flag FEG CFEG always read zero."]
        #[inline(always)]
        pub fn cfeg(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, IocRx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<15,1,0,IocRx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Receive Monitor for Channel x   RXM. Shows the status of the receive signal of channel x after glitch        filtering and inverted as specified by IIE."]
        #[inline(always)]
        pub fn rxm(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, IocRx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<30,1,0,IocRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Transmit Monitor for Channel x   TXM. Shows the status of the transmit signal of channel x inverted as        specified by OIE."]
        #[inline(always)]
        pub fn txm(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, IocRx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<31,1,0,IocRx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for IocRx {
        #[inline(always)]
        fn default() -> IocRx {
            <crate::RegValueT<IocRx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PgCx_SPEC;
    impl crate::sealed::RegSpec for PgCx_SPEC {
        type DataType = u32;
    }
    #[doc = "Pulse Generation Control Register 0\n resetvalue={Application Reset:0x0}"]
    pub type PgCx = crate::RegValueT<PgCx_SPEC>;

    impl PgCx {
        #[doc = "Pulse Length   PLEN. Defines the length of the pulse in TTS times  defined by FDRT . This is        the standard pulse width without data coding into the pulse width or for        coding a   8216 0  8217 ."]
        #[inline(always)]
        pub fn plen(
            self,
        ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, PgCx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<0,0x3f,1,0,u8, PgCx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Delay Length   DEL. In case data is coded into the pulse length  this defines the ADDITONAL        length of the pulse in TTS times. The resulting sum length is the pulse        width for coding a   8216 1  8217  into the pulse width."]
        #[inline(always)]
        pub fn del(
            self,
        ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, PgCx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<8,0x3f,1,0,u8, PgCx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Time Base Select   TBS. This bit selects the clock source for CTVx"]
        #[inline(always)]
        pub fn tbs(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, PgCx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,PgCx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "External Time Base Select   ETB. Selects the external clock line for counter CTVx."]
        #[inline(always)]
        pub fn etb(
            self,
        ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, PgCx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<16,0x7,1,0,u8, PgCx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Periodic Trigger Enable   PTE. Periodic trigger is defined by CTVx. Should be 0 if ETE or BYP is set."]
        #[inline(always)]
        pub fn pte(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, PgCx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<19,1,0,PgCx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "External Trigger Select   ETS. Selects the external trigger line for pulse generation  e.g. angle        synchronous ."]
        #[inline(always)]
        pub fn ets(
            self,
        ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, PgCx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<20,0x7,1,0,u8, PgCx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "External Trigger Enable   ETE.   8220 Angle sync. trigger  8221   external line is selected by ETS. Should be 0 if        PTE or BYP is set."]
        #[inline(always)]
        pub fn ete(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, PgCx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<23,1,0,PgCx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Bypass Enable   BYP. An external signal  selected by ETS directly drives the output of        channel x. Should be 0 if PTE or ETE is set."]
        #[inline(always)]
        pub fn byp(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, PgCx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<24,1,0,PgCx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Blank Out Time   BOT. BOT defines the length of the blank out time in TTS times  defined by        FDRT  starting from the end of a Sync Pulse. The receiver is always        switched off starting from the beginning of a Sync Pulse. BOT keeps the        receiver switched off additionally after the Sync Pulse. This suppresses        potential noise on the receive line after the pulse and thus eases the        design of the transceiver."]
        #[inline(always)]
        pub fn bot(
            self,
        ) -> crate::common::RegisterField<25, 0x7f, 1, 0, u8, PgCx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<25,0x7f,1,0,u8, PgCx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for PgCx {
        #[inline(always)]
        fn default() -> PgCx {
            <crate::RegValueT<PgCx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RcrAx_SPEC;
    impl crate::sealed::RegSpec for RcrAx_SPEC {
        type DataType = u32;
    }
    #[doc = "Receiver Control Register A 0\n resetvalue={Application Reset:0x0}"]
    pub type RcrAx = crate::RegValueT<RcrAx_SPEC>;

    impl RcrAx {
        #[doc = "Payload Data Length   PDL5. PDL determines the number of data bits per frame that the PSI5 channel x        is setup for in slot y. PDL does not include the start bits and the        Parity CRC bits. PDL includes the Messaging bits  the Frame Control bits        and the Status bits."]
        #[inline(always)]
        pub fn pdl0(
            self,
        ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, RcrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1f,1,0,u8, RcrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Payload Data Length   PDL5. PDL determines the number of data bits per frame that the PSI5 channel x        is setup for in slot y. PDL does not include the start bits and the        Parity CRC bits. PDL includes the Messaging bits  the Frame Control bits        and the Status bits."]
        #[inline(always)]
        pub fn pdl1(
            self,
        ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, RcrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x1f,1,0,u8, RcrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Payload Data Length   PDL5. PDL determines the number of data bits per frame that the PSI5 channel x        is setup for in slot y. PDL does not include the start bits and the        Parity CRC bits. PDL includes the Messaging bits  the Frame Control bits        and the Status bits."]
        #[inline(always)]
        pub fn pdl2(
            self,
        ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, RcrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<10,0x1f,1,0,u8, RcrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Payload Data Length   PDL5. PDL determines the number of data bits per frame that the PSI5 channel x        is setup for in slot y. PDL does not include the start bits and the        Parity CRC bits. PDL includes the Messaging bits  the Frame Control bits        and the Status bits."]
        #[inline(always)]
        pub fn pdl3(
            self,
        ) -> crate::common::RegisterField<15, 0x1f, 1, 0, u8, RcrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<15,0x1f,1,0,u8, RcrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Payload Data Length   PDL5. PDL determines the number of data bits per frame that the PSI5 channel x        is setup for in slot y. PDL does not include the start bits and the        Parity CRC bits. PDL includes the Messaging bits  the Frame Control bits        and the Status bits."]
        #[inline(always)]
        pub fn pdl4(
            self,
        ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, RcrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0x1f,1,0,u8, RcrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Payload Data Length   PDL5. PDL determines the number of data bits per frame that the PSI5 channel x        is setup for in slot y. PDL does not include the start bits and the        Parity CRC bits. PDL includes the Messaging bits  the Frame Control bits        and the Status bits."]
        #[inline(always)]
        pub fn pdl5(
            self,
        ) -> crate::common::RegisterField<25, 0x1f, 1, 0, u8, RcrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<25,0x1f,1,0,u8, RcrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Asynchronous Mode   ASYN. If set  the watch dog timers for the PSI5 channel x  slot WDL1 .. 6 are          disabled If WDL0 is not cleared TEI is issued if the distance between two          frames is longer than specified in WDL0 Slot Counter SC is incremented with each received frame  works as          frame counter  with roll over after 6. No Sync Pulses generated  TX path is inactive Note that in Asynchronous Mode all slow channel data is still          collected in Slot 1  SDSx0 ."]
        #[inline(always)]
        pub fn asyn(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, RcrAx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<30,1,0,RcrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Verbose Mode for Asynchronous Mode   AVBS. If set  and ASYN is set and WDL0 is  gt  0 an empty frame is stored in        RDRL Hx and the referring RDML H each time  the watch dog timer for the        PSI5 channel x without reception of a frame."]
        #[inline(always)]
        pub fn avbs(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, RcrAx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<31,1,0,RcrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for RcrAx {
        #[inline(always)]
        fn default() -> RcrAx {
            <crate::RegValueT<RcrAx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RcrBx_SPEC;
    impl crate::sealed::RegSpec for RcrBx_SPEC {
        type DataType = u32;
    }
    #[doc = "Receiver Control Register B 0\n resetvalue={Application Reset:0x0}"]
    pub type RcrBx = crate::RegValueT<RcrBx_SPEC>;

    impl RcrBx {
        #[doc = "Messaging Bits   MSG5. If set  the 2 Messaging bits are configured for the PSI5 channel x in slot y. If set  Enhanced Serial Message processing on bits D 1 0  is activated.  18 frames  4 or 8 bit ID  12 or 16 bit data  6 bit CRC  If Messaging bits are transmitted they are always presented in RDRx independently from status of MSGy. I.e. RDRx always stores exactly the data that was received."]
        #[inline(always)]
        pub fn msg0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Messaging Bits   MSG5. If set  the 2 Messaging bits are configured for the PSI5 channel x in slot y. If set  Enhanced Serial Message processing on bits D 1 0  is activated.  18 frames  4 or 8 bit ID  12 or 16 bit data  6 bit CRC  If Messaging bits are transmitted they are always presented in RDRx independently from status of MSGy. I.e. RDRx always stores exactly the data that was received."]
        #[inline(always)]
        pub fn msg1(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Messaging Bits   MSG5. If set  the 2 Messaging bits are configured for the PSI5 channel x in slot y. If set  Enhanced Serial Message processing on bits D 1 0  is activated.  18 frames  4 or 8 bit ID  12 or 16 bit data  6 bit CRC  If Messaging bits are transmitted they are always presented in RDRx independently from status of MSGy. I.e. RDRx always stores exactly the data that was received."]
        #[inline(always)]
        pub fn msg2(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Messaging Bits   MSG5. If set  the 2 Messaging bits are configured for the PSI5 channel x in slot y. If set  Enhanced Serial Message processing on bits D 1 0  is activated.  18 frames  4 or 8 bit ID  12 or 16 bit data  6 bit CRC  If Messaging bits are transmitted they are always presented in RDRx independently from status of MSGy. I.e. RDRx always stores exactly the data that was received."]
        #[inline(always)]
        pub fn msg3(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Messaging Bits   MSG5. If set  the 2 Messaging bits are configured for the PSI5 channel x in slot y. If set  Enhanced Serial Message processing on bits D 1 0  is activated.  18 frames  4 or 8 bit ID  12 or 16 bit data  6 bit CRC  If Messaging bits are transmitted they are always presented in RDRx independently from status of MSGy. I.e. RDRx always stores exactly the data that was received."]
        #[inline(always)]
        pub fn msg4(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<16,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Messaging Bits   MSG5. If set  the 2 Messaging bits are configured for the PSI5 channel x in slot y. If set  Enhanced Serial Message processing on bits D 1 0  is activated.  18 frames  4 or 8 bit ID  12 or 16 bit data  6 bit CRC  If Messaging bits are transmitted they are always presented in RDRx independently from status of MSGy. I.e. RDRx always stores exactly the data that was received."]
        #[inline(always)]
        pub fn msg5(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<20,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "CRC or Parit5 Selection   CRC5. If set  a 3 bit CRC checksum is expected for the PSI5 channel x in slot y. Else  1 bit Parity is assumed."]
        #[inline(always)]
        pub fn crc0(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "CRC or Parit5 Selection   CRC5. If set  a 3 bit CRC checksum is expected for the PSI5 channel x in slot y. Else  1 bit Parity is assumed."]
        #[inline(always)]
        pub fn crc1(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "CRC or Parit5 Selection   CRC5. If set  a 3 bit CRC checksum is expected for the PSI5 channel x in slot y. Else  1 bit Parity is assumed."]
        #[inline(always)]
        pub fn crc2(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "CRC or Parit5 Selection   CRC5. If set  a 3 bit CRC checksum is expected for the PSI5 channel x in slot y. Else  1 bit Parity is assumed."]
        #[inline(always)]
        pub fn crc3(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "CRC or Parit5 Selection   CRC5. If set  a 3 bit CRC checksum is expected for the PSI5 channel x in slot y. Else  1 bit Parity is assumed."]
        #[inline(always)]
        pub fn crc4(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<17,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "CRC or Parit5 Selection   CRC5. If set  a 3 bit CRC checksum is expected for the PSI5 channel x in slot y. Else  1 bit Parity is assumed."]
        #[inline(always)]
        pub fn crc5(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<21,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Frame Expectation Control   FEC5. If set  a frame is expected for the PSI5 channel x in slot y. A No Frame Received error interrupt NFI is issued each time  the watch dog timer for the PSI5 channel x  slot y expires without reception of a SOF or EOF."]
        #[inline(always)]
        pub fn fec0(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Frame Expectation Control   FEC5. If set  a frame is expected for the PSI5 channel x in slot y. A No Frame Received error interrupt NFI is issued each time  the watch dog timer for the PSI5 channel x  slot y expires without reception of a SOF or EOF."]
        #[inline(always)]
        pub fn fec1(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Frame Expectation Control   FEC5. If set  a frame is expected for the PSI5 channel x in slot y. A No Frame Received error interrupt NFI is issued each time  the watch dog timer for the PSI5 channel x  slot y expires without reception of a SOF or EOF."]
        #[inline(always)]
        pub fn fec2(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Frame Expectation Control   FEC5. If set  a frame is expected for the PSI5 channel x in slot y. A No Frame Received error interrupt NFI is issued each time  the watch dog timer for the PSI5 channel x  slot y expires without reception of a SOF or EOF."]
        #[inline(always)]
        pub fn fec3(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Frame Expectation Control   FEC5. If set  a frame is expected for the PSI5 channel x in slot y. A No Frame Received error interrupt NFI is issued each time  the watch dog timer for the PSI5 channel x  slot y expires without reception of a SOF or EOF."]
        #[inline(always)]
        pub fn fec4(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<18,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Frame Expectation Control   FEC5. If set  a frame is expected for the PSI5 channel x in slot y. A No Frame Received error interrupt NFI is issued each time  the watch dog timer for the PSI5 channel x  slot y expires without reception of a SOF or EOF."]
        #[inline(always)]
        pub fn fec5(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<22,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Verbose Mode   VBS5. If set  and a message is expected for the PSI5 channel x in slot y  FECy  an empty frame is stored in RDRL Hx and the referring RDML H each time  the watch dog timer for the PSI5 channel x  slot y expires without reception of a frame."]
        #[inline(always)]
        pub fn vbs0(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Verbose Mode   VBS5. If set  and a message is expected for the PSI5 channel x in slot y  FECy  an empty frame is stored in RDRL Hx and the referring RDML H each time  the watch dog timer for the PSI5 channel x  slot y expires without reception of a frame."]
        #[inline(always)]
        pub fn vbs1(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Verbose Mode   VBS5. If set  and a message is expected for the PSI5 channel x in slot y  FECy  an empty frame is stored in RDRL Hx and the referring RDML H each time  the watch dog timer for the PSI5 channel x  slot y expires without reception of a frame."]
        #[inline(always)]
        pub fn vbs2(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<11,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Verbose Mode   VBS5. If set  and a message is expected for the PSI5 channel x in slot y  FECy  an empty frame is stored in RDRL Hx and the referring RDML H each time  the watch dog timer for the PSI5 channel x  slot y expires without reception of a frame."]
        #[inline(always)]
        pub fn vbs3(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Verbose Mode   VBS5. If set  and a message is expected for the PSI5 channel x in slot y  FECy  an empty frame is stored in RDRL Hx and the referring RDML H each time  the watch dog timer for the PSI5 channel x  slot y expires without reception of a frame."]
        #[inline(always)]
        pub fn vbs4(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<19,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Verbose Mode   VBS5. If set  and a message is expected for the PSI5 channel x in slot y  FECy  an empty frame is stored in RDRL Hx and the referring RDML H each time  the watch dog timer for the PSI5 channel x  slot y expires without reception of a frame."]
        #[inline(always)]
        pub fn vbs5(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, RcrBx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<23,1,0,RcrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for RcrBx {
        #[inline(always)]
        fn default() -> RcrBx {
            <crate::RegValueT<RcrBx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RcrCx_SPEC;
    impl crate::sealed::RegSpec for RcrCx_SPEC {
        type DataType = u32;
    }
    #[doc = "Receiver Control Register C 0\n resetvalue={Application Reset:0x0}"]
    pub type RcrCx = crate::RegValueT<RcrCx_SPEC>;

    impl RcrCx {
        #[doc = "Bit Rate Select   BRS"]
        #[inline(always)]
        pub fn brs(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, RcrCx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,RcrCx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Time Stamp Select for Pulses   TSP"]
        #[inline(always)]
        pub fn tsp(
            self,
        ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, RcrCx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<1,0x3,1,0,u8, RcrCx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Time Stamp Select for Start of Frame  SOF    TSF"]
        #[inline(always)]
        pub fn tsf(
            self,
        ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, RcrCx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<3,0x3,1,0,u8, RcrCx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Time Stamp Select for Receive Data Registers   TSR"]
        #[inline(always)]
        pub fn tsr(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, RcrCx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,RcrCx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for RcrCx {
        #[inline(always)]
        fn default() -> RcrCx {
            <crate::RegValueT<RcrCx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RdrHx_SPEC;
    impl crate::sealed::RegSpec for RdrHx_SPEC {
        type DataType = u32;
    }
    #[doc = "Receive Data Register High 0\n resetvalue={Application Reset:0x0}"]
    pub type RdrHx = crate::RegValueT<RdrHx_SPEC>;

    impl RdrHx {
        #[doc = "Time Stamp   TS. of the last received PSI5 frame. RCRC.TSR determines if SPTSCx or SFTSCx        is stored in RDRHx.  It can be selected if the time stamp of the last        sync pulse is stored or the time stamp for the start of frame ."]
        #[inline(always)]
        pub fn ts(
            self,
        ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, RdrHx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffff,1,0,u32, RdrHx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Slot Counter   SC. In Synchronous Mode  RCRAx.ASYN 0   Number of the slot the SOF of the        frame was received in. In Asynchronous Mode  RCRAx.ASYN 1   revolving        frame number between 1 and 6. Note        that in Asynchronous Mode all slow channel data is still collected in        Slot 1  SDSx0 ."]
        #[inline(always)]
        pub fn sc(
            self,
        ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, RdrHx_SPEC, crate::common::R> {
            crate::common::RegisterField::<24,0x7,1,0,u8, RdrHx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Time Slot Error Flag   TEI. In Synchronous Mode  RCRAx.ASYN   0   this bit is set if the SOF and the        EOF of a frame are received in different time slots. The slots are        constraint by the watch dog timer  see WDTxy . In Asynchronous Mode         RCRAx.ASYN   1   this bit is set if the distance between two frames is        longer than specified in WDL0."]
        #[inline(always)]
        pub fn tei(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, RdrHx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<27,1,0,RdrHx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Number of bits Error Flag   NBI. This bit is set if the last frame received less bits than expected but        no manchester coding error occurred until S1  S2  M0 and M1 where        received. Note that the frame after the error might be completely wrong."]
        #[inline(always)]
        pub fn nbi(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, RdrHx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<28,1,0,RdrHx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Error in Message Bits Flag   MEI. This bit is set if a manchester error occurred in Bit M0 or M1.  Only if        configured in RCRBx.MSG  Note that the frame after the error might be completely wrong."]
        #[inline(always)]
        pub fn mei(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, RdrHx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<29,1,0,RdrHx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "No Frame Received Flag   NFI. This bit is set when the NFI condition as described in the first paragraph of INTSTATA.NFI occurs. If only the two start bits are received and no further data  this   x201c SOF  x201d  is discarded completely and treated like no frame received as well. If RCRBx.VBS is set  an empty frame is stored anyhow and RDIx is issued as well. The content of RDRL Hx   RDML Hx is valid for SC and for this bit  NFI . Bit fields RD and CRC are cleared  all zero . If RCRC.VBSx is 0 then TS is cleared. If RCRC.VBSx is 1 then TS is set to either SPTSCx.TS or SFTSCx.TS depending on the value of RCRC.TSR."]
        #[inline(always)]
        pub fn nfi(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, RdrHx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<30,1,0,RdrHx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Receive Buffer Overflow Flag   RBI. This bit is set after a frame has been received while the old one was        not read from RDRHx. I.e. the kernel wants to set any of the two        interrupts RSI and RDI and finds any of these two interrupts already        set. The old data is overwritten by the new data."]
        #[inline(always)]
        pub fn rbi(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, RdrHx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<31,1,0,RdrHx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for RdrHx {
        #[inline(always)]
        fn default() -> RdrHx {
            <crate::RegValueT<RdrHx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RdrLx_SPEC;
    impl crate::sealed::RegSpec for RdrLx_SPEC {
        type DataType = u32;
    }
    #[doc = "Receive Data Register Low 0\n resetvalue={Application Reset:0x0}"]
    pub type RdrLx = crate::RegValueT<RdrLx_SPEC>;

    impl RdrLx {
        #[doc = "CRC Error Flag   CRCI. This bit is set if the CRC or Parity check fails. Set as well if CRC can        not be calculated  PDL   0  CRC cut off by Manchester Error  too few        bits  BOT  Sync Pulse"]
        #[inline(always)]
        pub fn crci(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, RdrLx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0, 1, 0, RdrLx_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "CRC   CRC. CRC parity bit of last PSI5 frame. CRC0 or Parity bit is on bit position        1. In case of NFI  this field is cleared."]
        #[inline(always)]
        pub fn crc(
            self,
        ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, RdrLx_SPEC, crate::common::R> {
            crate::common::RegisterField::<1,0x7,1,0,u8, RdrLx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "RD   RD. Receive data of last PSI5 frame. D0 is on bit position 4. In case of        NFI  this field is cleared."]
        #[inline(always)]
        pub fn rd(
            self,
        ) -> crate::common::RegisterField<4, 0xfffffff, 1, 0, u32, RdrLx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<4,0xfffffff,1,0,u32, RdrLx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for RdrLx {
        #[inline(always)]
        fn default() -> RdrLx {
            <crate::RegValueT<RdrLx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RsRx_SPEC;
    impl crate::sealed::RegSpec for RsRx_SPEC {
        type DataType = u32;
    }
    #[doc = "Receive Status Register 0\n resetvalue={Application Reset:0x0}"]
    pub type RsRx = crate::RegValueT<RsRx_SPEC>;

    impl RsRx {
        #[doc = "CRC   CRC. of last frame. CRC0 is on bit position 0."]
        #[inline(always)]
        pub fn crc(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, RsRx_SPEC, crate::common::R> {
            crate::common::RegisterField::<0,0x7,1,0,u8, RsRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Messaging Bits   MSG. of last frame. MSG0 is on bit position 8."]
        #[inline(always)]
        pub fn msg(
            self,
        ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, RsRx_SPEC, crate::common::R> {
            crate::common::RegisterField::<8,0x3,1,0,u8, RsRx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for RsRx {
        #[inline(always)]
        fn default() -> RsRx {
            <crate::RegValueT<RsRx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ScRx_SPEC;
    impl crate::sealed::RegSpec for ScRx_SPEC {
        type DataType = u32;
    }
    #[doc = "Send Control Register 0\n resetvalue={Application Reset:0x0}"]
    pub type ScRx = crate::RegValueT<ScRx_SPEC>;

    impl ScRx {
        #[doc = "Pay Load Length of Registers SDRL H   PLL. If PLL  gt  31  SDRH needs to be written to trigger the HW for automatic        STA  BSC or CRC generation or just moving data to SSRL H. Else writing        to SDRL is sufficient. PLL needs to written before SDRL H is used for        proper operation. If insertion of STA  BSC and CRC results in more than        64 bits  the MSBs are truncated in SSR. Defines the length that is taken        into account"]
        #[inline(always)]
        pub fn pll(
            self,
        ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, ScRx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<0,0x3f,1,0,u8, ScRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enhanced Protocol Selection   EPS"]
        #[inline(always)]
        pub fn eps(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, ScRx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6, 1, 0, ScRx_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Bit Stuffing Control   BSC. Depending from bit EPS after 3 bits a   8216 1  8217  is inserted  V1.3  or after 6        bits a   8216 0  8217  is inserted  enhanced Power Train Mode  Note that this makes the frame longer."]
        #[inline(always)]
        pub fn bsc(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, ScRx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7, 1, 0, ScRx_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Pay Load Length of Registers SSRL H   SSL. If SSL  gt  31  SSRH needs to be written to trigger the HW for automatic        moving data to SORL H. Else writing to SSRL is sufficient. SSL needs to        be written before SSRL H is used for proper operation. Defines the        length that is taken into account. Start Sequence  BSC and CRC need to        be added to PLL by SW if SSL is calculated based on PLL."]
        #[inline(always)]
        pub fn ssl(
            self,
        ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, ScRx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<8,0x3f,1,0,u8, ScRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Flush SSRH Lx   FLUS. Setting this bit stops the shifting process and flushes  clears  SSRH Lx        and TSF. TPIx is issued at the end of successful flushing. Reads always        as zero."]
        #[inline(always)]
        pub fn flus(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, ScRx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<14, 1, 0, ScRx_SPEC, crate::common::W>::from_register(
                self, 0,
            )
        }
        #[doc = "Flush SORH Lx   FLUO. Setting this bit stops the sending process and flushes  clears  SORH Lx        and TOF. TSIx is issued at the end of successful flushing. Reads always        as zero."]
        #[inline(always)]
        pub fn fluo(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, ScRx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<15, 1, 0, ScRx_SPEC, crate::common::W>::from_register(
                self, 0,
            )
        }
        #[doc = "Pay Load Length of Registers SORL H   SOL. If SOL  gt  31  SORH needs to be written to trigger the HW for automatic        moving data to the pulse generator. Else writing to SORL is sufficient.        SOL needs to written before SORL H is used for proper operation. Defines        the length that is taken into account. Start Sequence  CRC and Stuffing        needs to be added by SW to PLL if SOL is calculated based on PLL."]
        #[inline(always)]
        pub fn sol(
            self,
        ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, ScRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x3f,1,0,u8, ScRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "CRC Generation Control   CRC"]
        #[inline(always)]
        pub fn crc(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, ScRx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<22,1,0,ScRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Start Sequence Generation Control   STA"]
        #[inline(always)]
        pub fn sta(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, ScRx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<23,1,0,ScRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Inhibit Transfer   INH. This inhibits the automatic transfer from the shift registers SSRL H to        SORL H after the preparation of all automatic bit fields  STA  BSC         CRC . After a write access to SDRL Hx  the automatic preparation is done        by HW if selected. If automatic preparation of the start sequence is not        selected  data is moved by HW without change to SSRL H. When the HW is        done with the transfer to the internal shift registers  interrupt TPI is        activated  flag TSF is set and flag TPF is cleared automatically.        Depending on INH  GO must be set after TPI before transfer to SORL H        takes place  After the transfer to SORL H  TSI is activated  TOF is set        and and TSF is cleared automatically. . INH defines the start mode"]
        #[inline(always)]
        pub fn inh(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, ScRx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<24,1,0,ScRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Release prepared Send data   GO. This is only relevant if INH is set It allows for manual control of the        transfer from SSRH L to SORL H. It is always read as zero and is        automatically cleared."]
        #[inline(always)]
        pub fn go(self) -> crate::common::RegisterFieldBool<25, 1, 0, ScRx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<25, 1, 0, ScRx_SPEC, crate::common::W>::from_register(
                self, 0,
            )
        }
        #[doc = "Transmit Preparation Flag   TPF. If set  preparation is in progress  start sequence and  or CRC and or        stuffing and or at least copying data from SDRL H. If set  write access        to SDRL H will not change any data and issue TPOI. It is cleared automatically after preparation is finished."]
        #[inline(always)]
        pub fn tpf(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, ScRx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<26, 1, 0, ScRx_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Transmit Shift Flag   TSF. If set  data in SSRL H is waiting to be shifted to SORL H. This can be        caused by an automatic transfer from SDRL H or from a CPU write access        to SSRL H. If SSL  lt  32 it is sufficient to write SSRL. Else  only        writing SSRH will set TSF. If set  write access to SSRL H will not        change any data and issue TSOI. It is cleared automatically after data was shifted to SORL H or after        flushing the register by setting bit FLUS."]
        #[inline(always)]
        pub fn tsf(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, ScRx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<27, 1, 0, ScRx_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Transmit Output Flag   TOF. If set  data in SORL H is waiting to be transmitted to the sensor. This        can be caused by an automatic transfer from SSRL H or from a CPU write        access to SORL H. If SOL  lt  32 it is sufficient to write SORL. Else  only        writing SDRH will set TSF. If set  write access to SORL H will not        change any data and issue TOOI. It is cleared automatically after data was transmitted to the sensor or        after flushing the register by setting bit FLUO."]
        #[inline(always)]
        pub fn tof(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, ScRx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<28, 1, 0, ScRx_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Transfer Request in Progress   TRQ. While a transfer is being sent this bit is set. Write access is ignored."]
        #[inline(always)]
        pub fn trq(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, ScRx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<31, 1, 0, ScRx_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
    }
    impl core::default::Default for ScRx {
        #[inline(always)]
        fn default() -> ScRx {
            <crate::RegValueT<ScRx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdrHx_SPEC;
    impl crate::sealed::RegSpec for SdrHx_SPEC {
        type DataType = u32;
    }
    #[doc = "Send Data Register High 0\n resetvalue={Application Reset:0x0}"]
    pub type SdrHx = crate::RegValueT<SdrHx_SPEC>;

    impl SdrHx {
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd32(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd33(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd34(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd35(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd36(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd37(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd38(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd39(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd40(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd41(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd42(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd43(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<11,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd44(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd45(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd46(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd47(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd48(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<16,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd49(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<17,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd50(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<18,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd51(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<19,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd52(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<20,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd53(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<21,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd54(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<22,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd55(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<23,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd56(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<24,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd57(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<25,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd58(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<26,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd59(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<27,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd60(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<28,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd61(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<29,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd62(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<30,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with          8216 0  8217 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd63(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, SdrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<31,1,0,SdrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for SdrHx {
        #[inline(always)]
        fn default() -> SdrHx {
            <crate::RegValueT<SdrHx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdrLx_SPEC;
    impl crate::sealed::RegSpec for SdrLx_SPEC {
        type DataType = u32;
    }
    #[doc = "Send Data Register Low 0\n resetvalue={Application Reset:0x0}"]
    pub type SdrLx = crate::RegValueT<SdrLx_SPEC>;

    impl SdrLx {
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd3(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd4(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd5(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd6(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd7(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd8(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd9(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd10(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd11(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<11,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd12(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd13(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd14(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd15(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd16(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<16,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd17(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<17,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd18(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<18,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd19(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<19,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd20(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<20,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd21(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<21,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd22(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<22,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd23(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<23,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd24(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<24,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd25(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<25,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd26(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<26,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd27(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<27,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd28(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<28,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd29(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<29,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd30(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<30,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0 . If PLL is  lt  32  SDRHx must not be written."]
        #[inline(always)]
        pub fn sd31(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, SdrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<31,1,0,SdrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for SdrLx {
        #[inline(always)]
        fn default() -> SdrLx {
            <crate::RegValueT<SdrLx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdSxz_SPEC;
    impl crate::sealed::RegSpec for SdSxz_SPEC {
        type DataType = u32;
    }
    #[doc = "Serial Data and Status Register 00\n resetvalue={Application Reset:0x0}"]
    pub type SdSxz = crate::RegValueT<SdSxz_SPEC>;

    impl SdSxz {
        #[doc = "Serial Data   SD. of last serial data frame on channel x slot z. SD0 is on bit position 0.        If SDS.CON is not set  bits  15 12  are zero."]
        #[inline(always)]
        pub fn sd(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, SdSxz_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, SdSxz_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Message ID   MID. of last serial data frame. ID0 is on bit position 16. If SDS.CON is set         bits  23 20  are zero."]
        #[inline(always)]
        pub fn mid(
            self,
        ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, SdSxz_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0xff,1,0,u8, SdSxz_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "SCRC   SCRC. CRC of last serial data frame. CRC0 is on position 24."]
        #[inline(always)]
        pub fn scrc(
            self,
        ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, SdSxz_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<24,0x3f,1,0,u8, SdSxz_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "CRC of Serial Message failed Interrupt Flag.   SCRI. This bit is set if the CRC of the serial message fails. This includes a        check of the Messaging bit field for correct 0 values of bit 1 in frames        7  13 and 18. See INTSTATx."]
        #[inline(always)]
        pub fn scri(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, SdSxz_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<30,1,0,SdSxz_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Configuration bit   CON. of last serial frame."]
        #[inline(always)]
        pub fn con(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, SdSxz_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<31,1,0,SdSxz_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for SdSxz {
        #[inline(always)]
        fn default() -> SdSxz {
            <crate::RegValueT<SdSxz_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SftsCx_SPEC;
    impl crate::sealed::RegSpec for SftsCx_SPEC {
        type DataType = u32;
    }
    #[doc = "Start of Frame Time Stamp Capture Register 0\n resetvalue={Application Reset:0x0}"]
    pub type SftsCx = crate::RegValueT<SftsCx_SPEC>;

    impl SftsCx {
        #[doc = "Time Stamp   TS. of the last frame received on channel x. The Time Stamp is taken at the        rising edge of the first start bit S1 that was qualified by the receiver."]
        #[inline(always)]
        pub fn ts(
            self,
        ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, SftsCx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffff,1,0,u32, SftsCx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for SftsCx {
        #[inline(always)]
        fn default() -> SftsCx {
            <crate::RegValueT<SftsCx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SorHx_SPEC;
    impl crate::sealed::RegSpec for SorHx_SPEC {
        type DataType = u32;
    }
    #[doc = "Send Output Register High 0\n resetvalue={Application Reset:0x0}"]
    pub type SorHx = crate::RegValueT<SorHx_SPEC>;

    impl SorHx {
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd32(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd33(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd34(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd35(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd36(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd37(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd38(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd39(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd40(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd41(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd42(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd43(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<11,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd44(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd45(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd46(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd47(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd48(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<16,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd49(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<17,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd50(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<18,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd51(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<19,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd52(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<20,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd53(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<21,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd54(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<22,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd55(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<23,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd56(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<24,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd57(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<25,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd58(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<26,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd59(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<27,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd60(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<28,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd61(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<29,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd62(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<30,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SORLx."]
        #[inline(always)]
        pub fn sd63(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, SorHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<31,1,0,SorHx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for SorHx {
        #[inline(always)]
        fn default() -> SorHx {
            <crate::RegValueT<SorHx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SorLx_SPEC;
    impl crate::sealed::RegSpec for SorLx_SPEC {
        type DataType = u32;
    }
    #[doc = "Send Output Register Low 0\n resetvalue={Application Reset:0x0}"]
    pub type SorLx = crate::RegValueT<SorLx_SPEC>;

    impl SorLx {
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd3(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd4(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd5(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd6(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd7(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd8(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd9(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd10(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd11(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<11,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd12(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd13(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd14(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd15(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd16(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<16,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd17(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<17,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd18(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<18,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd19(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<19,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd20(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<20,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd21(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<21,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd22(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<22,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd23(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<23,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd24(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<24,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd25(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<25,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd26(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<26,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd27(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<27,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd28(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<28,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd29(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<29,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd30(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<30,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SORHx."]
        #[inline(always)]
        pub fn sd31(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, SorLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<31,1,0,SorLx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for SorLx {
        #[inline(always)]
        fn default() -> SorLx {
            <crate::RegValueT<SorLx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SptsCx_SPEC;
    impl crate::sealed::RegSpec for SptsCx_SPEC {
        type DataType = u32;
    }
    #[doc = "Start of Pulse Time Stamp Capture Register 0\n resetvalue={Application Reset:0x0}"]
    pub type SptsCx = crate::RegValueT<SptsCx_SPEC>;

    impl SptsCx {
        #[doc = "Time Stamp   TS. of the last sync pulse sent on channel x. The Time Stamp is taken at the        rising edge of the pulse."]
        #[inline(always)]
        pub fn ts(
            self,
        ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, SptsCx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffff,1,0,u32, SptsCx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for SptsCx {
        #[inline(always)]
        fn default() -> SptsCx {
            <crate::RegValueT<SptsCx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SsrHx_SPEC;
    impl crate::sealed::RegSpec for SsrHx_SPEC {
        type DataType = u32;
    }
    #[doc = "Send Shift Register High 0\n resetvalue={Application Reset:0x0}"]
    pub type SsrHx = crate::RegValueT<SsrHx_SPEC>;

    impl SsrHx {
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd32(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd33(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd34(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd35(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd36(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd37(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd38(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd39(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd40(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd41(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd42(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd43(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<11,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd44(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd45(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd46(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd47(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd48(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<16,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd49(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<17,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd50(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<18,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd51(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<19,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd52(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<20,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd53(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<21,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd54(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<22,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd55(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<23,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd56(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<24,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd57(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<25,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd58(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<26,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd59(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<27,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd60(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<28,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd61(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<29,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd62(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<30,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD63   SD63. Continues the bit stream as described in SSRLx."]
        #[inline(always)]
        pub fn sd63(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, SsrHx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<31,1,0,SsrHx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for SsrHx {
        #[inline(always)]
        fn default() -> SsrHx {
            <crate::RegValueT<SsrHx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SsrLx_SPEC;
    impl crate::sealed::RegSpec for SsrLx_SPEC {
        type DataType = u32;
    }
    #[doc = "Send Shift Register Low 0\n resetvalue={Application Reset:0x0}"]
    pub type SsrLx = crate::RegValueT<SsrLx_SPEC>;

    impl SsrLx {
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd3(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd4(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd5(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd6(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd7(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd8(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd9(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd10(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd11(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<11,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd12(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd13(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd14(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd15(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd16(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<16,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd17(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<17,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd18(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<18,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd19(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<19,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd20(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<20,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd21(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<21,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd22(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<22,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd23(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<23,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd24(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<24,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd25(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<25,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd26(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<26,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd27(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<27,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd28(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<28,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd29(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<29,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd30(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<30,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SD31   SD31. Send data of next ECU to Sensor frame. The sequence is  STS  AD  SD  CRC. Depending on individual field length  the bit stream is continued in SSRHx."]
        #[inline(always)]
        pub fn sd31(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, SsrLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<31,1,0,SsrLx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for SsrLx {
        #[inline(always)]
        fn default() -> SsrLx {
            <crate::RegValueT<SsrLx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdTxw_SPEC;
    impl crate::sealed::RegSpec for WdTxw_SPEC {
        type DataType = u32;
    }
    #[doc = "Watch Dog Timer Register 00\n resetvalue={Application Reset:0x0}"]
    pub type WdTxw = crate::RegValueT<WdTxw_SPEC>;

    impl WdTxw {
        #[doc = "Watch Dog Timer Limit   WDL. for channel x limit w."]
        #[inline(always)]
        pub fn wdl(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, WdTxw_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, WdTxw_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for WdTxw {
        #[inline(always)]
        fn default() -> WdTxw {
            <crate::RegValueT<WdTxw_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "RDM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdm(pub(super) *mut u8);
unsafe impl core::marker::Send for Rdm {}
unsafe impl core::marker::Sync for Rdm {}
impl Rdm {
    #[doc = ""]
    #[inline(always)]
    pub fn rdmx(self) -> [rdm::RdMx; 32] {
        unsafe {
            [
                rdm::RdMx(self.0.add(0x0usize + 0x0usize)),
                rdm::RdMx(self.0.add(0x0usize + 0x8usize)),
                rdm::RdMx(self.0.add(0x0usize + 0x10usize)),
                rdm::RdMx(self.0.add(0x0usize + 0x18usize)),
                rdm::RdMx(self.0.add(0x0usize + 0x20usize)),
                rdm::RdMx(self.0.add(0x0usize + 0x28usize)),
                rdm::RdMx(self.0.add(0x0usize + 0x30usize)),
                rdm::RdMx(self.0.add(0x0usize + 0x38usize)),
                rdm::RdMx(self.0.add(0x0usize + 0x40usize)),
                rdm::RdMx(self.0.add(0x0usize + 0x48usize)),
                rdm::RdMx(self.0.add(0x0usize + 0x50usize)),
                rdm::RdMx(self.0.add(0x0usize + 0x58usize)),
                rdm::RdMx(self.0.add(0x0usize + 0x60usize)),
                rdm::RdMx(self.0.add(0x0usize + 0x68usize)),
                rdm::RdMx(self.0.add(0x0usize + 0x70usize)),
                rdm::RdMx(self.0.add(0x0usize + 0x78usize)),
                rdm::RdMx(self.0.add(0x0usize + 0x80usize)),
                rdm::RdMx(self.0.add(0x0usize + 0x88usize)),
                rdm::RdMx(self.0.add(0x0usize + 0x90usize)),
                rdm::RdMx(self.0.add(0x0usize + 0x98usize)),
                rdm::RdMx(self.0.add(0x0usize + 0xa0usize)),
                rdm::RdMx(self.0.add(0x0usize + 0xa8usize)),
                rdm::RdMx(self.0.add(0x0usize + 0xb0usize)),
                rdm::RdMx(self.0.add(0x0usize + 0xb8usize)),
                rdm::RdMx(self.0.add(0x0usize + 0xc0usize)),
                rdm::RdMx(self.0.add(0x0usize + 0xc8usize)),
                rdm::RdMx(self.0.add(0x0usize + 0xd0usize)),
                rdm::RdMx(self.0.add(0x0usize + 0xd8usize)),
                rdm::RdMx(self.0.add(0x0usize + 0xe0usize)),
                rdm::RdMx(self.0.add(0x0usize + 0xe8usize)),
                rdm::RdMx(self.0.add(0x0usize + 0xf0usize)),
                rdm::RdMx(self.0.add(0x0usize + 0xf8usize)),
            ]
        }
    }
}
pub mod rdm {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = ""]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RdMx(pub(super) *mut u8);
    unsafe impl core::marker::Send for RdMx {}
    unsafe impl core::marker::Sync for RdMx {}
    impl RdMx {
        #[doc = "Receive Data Memory High 00\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn rdmhxy(&self) -> crate::common::Reg<rdmx::RdmHxy_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "Receive Data Memory Low 00\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn rdmlxy(&self) -> crate::common::Reg<rdmx::RdmLxy_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
    }
    pub mod rdmx {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RdmHxy_SPEC;
        impl crate::sealed::RegSpec for RdmHxy_SPEC {
            type DataType = u32;
        }
        #[doc = "Receive Data Memory High 00\n resetvalue={Application Reset:0x0}"]
        pub type RdmHxy = crate::RegValueT<RdmHxy_SPEC>;

        impl RdmHxy {
            #[doc = "Time Stamp   TS. Copied from RDRHx at frame end."]
            #[inline(always)]
            pub fn ts(
                self,
            ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, RdmHxy_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0xffffff,1,0,u32, RdmHxy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Slot Counter   SC. Copied from RDRHx at frame end."]
            #[inline(always)]
            pub fn sc(
                self,
            ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, RdmHxy_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<24,0x7,1,0,u8, RdmHxy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Time Slot Error Flag   TEI. Copied from RDRHx at frame end."]
            #[inline(always)]
            pub fn tei(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, RdmHxy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,RdmHxy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Number of bits Error Flag   NBI. Copied from RDRHx at frame end."]
            #[inline(always)]
            pub fn nbi(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, RdmHxy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<28,1,0,RdmHxy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Error in Messaging Bits Flag   MEI. Copied from RDRHx at frame end."]
            #[inline(always)]
            pub fn mei(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, RdmHxy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,RdmHxy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "No Frame Received Flag   NFI. Copied from RDRHx at frame end."]
            #[inline(always)]
            pub fn nfi(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, RdmHxy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<30,1,0,RdmHxy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Receive Memory Overflow Flag   RMI. Copied from INTSTATAx at frame end."]
            #[inline(always)]
            pub fn rmi(
                self,
            ) -> crate::common::RegisterFieldBool<31, 1, 0, RdmHxy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<31,1,0,RdmHxy_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for RdmHxy {
            #[inline(always)]
            fn default() -> RdmHxy {
                <crate::RegValueT<RdmHxy_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RdmLxy_SPEC;
        impl crate::sealed::RegSpec for RdmLxy_SPEC {
            type DataType = u32;
        }
        #[doc = "Receive Data Memory Low 00\n resetvalue={Application Reset:0x0}"]
        pub type RdmLxy = crate::RegValueT<RdmLxy_SPEC>;

        impl RdmLxy {
            #[doc = "CRC Error Flag   CRCI. Copied from RDRLx at frame end."]
            #[inline(always)]
            pub fn crci(
                self,
            ) -> crate::common::RegisterFieldBool<0, 1, 0, RdmLxy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<0,1,0,RdmLxy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "CRC   CRC. Copied from RDRLx at frame end."]
            #[inline(always)]
            pub fn crc(
                self,
            ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, RdmLxy_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<1,0x7,1,0,u8, RdmLxy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "RD   RD. Copied from RDRLx at frame end."]
            #[inline(always)]
            pub fn rd(
                self,
            ) -> crate::common::RegisterField<4, 0xfffffff, 1, 0, u32, RdmLxy_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<4,0xfffffff,1,0,u32, RdmLxy_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for RdmLxy {
            #[inline(always)]
            fn default() -> RdmLxy {
                <crate::RegValueT<RdmLxy_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
