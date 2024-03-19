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
#[doc = r"SRC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Src(pub(super) *mut u8);
unsafe impl core::marker::Send for Src {}
unsafe impl core::marker::Sync for Src {}
impl Src {
    #[doc = "SBCU Service Request  SPB Bus Control Unit \n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn bcuspb(&self) -> crate::common::Reg<self::Bcuspb_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "DMU FSI Service Request\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dmufsi(&self) -> crate::common::Reg<self::Dmufsi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2148usize)) }
    }

    #[doc = "DMU Host Service Request\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dmuhost(&self) -> crate::common::Reg<self::Dmuhost_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2144usize)) }
    }

    #[doc = "AEI Shared Service Request\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gtmaeiirq(&self) -> crate::common::Reg<self::Gtmaeiirq_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2672usize)) }
    }

    #[doc = "ARU Shared Service Request 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gtmaruirqw(
        &self,
    ) -> [crate::common::Reg<self::GtmaruirQw_SPEC, crate::common::RW>; 3] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0xa74usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0xa74usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0xa74usize + 0x8usize)),
            ]
        }
    }

    #[doc = "BRC Shared Service Request\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gtmbrcirq(&self) -> crate::common::Reg<self::Gtmbrcirq_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2688usize)) }
    }

    #[doc = "CMP Shared Service Request\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gtmcmpirq(&self) -> crate::common::Reg<self::Gtmcmpirq_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2692usize)) }
    }

    #[doc = "DPLL Service Request 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gtmdpllw(
        &self,
    ) -> [crate::common::Reg<self::GtmdplLw_SPEC, crate::common::RW>; 27] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x5cusize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x60usize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x64usize)),
                crate::common::Reg::from_ptr(self.0.add(0xb00usize + 0x68usize)),
            ]
        }
    }

    #[doc = "Error Service Request\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gtmerr(&self) -> crate::common::Reg<self::Gtmerr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2928usize)) }
    }

    #[doc = "GTM Multi Channel Sequencer Service Request 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gtmmcsww(
        &self,
    ) -> [crate::common::Reg<self::GtmmcsWw_SPEC, crate::common::RW>; 10] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0xfd0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0xfd0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0xfd0usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0xfd0usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0xfd0usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0xfd0usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0xfd0usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0xfd0usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0xfd0usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0xfd0usize + 0x24usize)),
            ]
        }
    }

    #[doc = "PSM0 Shared Service Request 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gtmpsmwx(
        &self,
    ) -> [crate::common::Reg<self::GtmpsMwx_SPEC, crate::common::RW>; 8] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0xaa0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0xaa0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0xaa0usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0xaa0usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0xaa0usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0xaa0usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0xaa0usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0xaa0usize + 0x1cusize)),
            ]
        }
    }

    #[doc = "SPE0 Shared Service Request\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gtmspewirq(
        &self,
    ) -> [crate::common::Reg<self::GtmspEwIrq_SPEC, crate::common::RW>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0xa88usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0xa88usize + 0x4usize)),
            ]
        }
    }

    #[doc = "MTU Done Service Request\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mtudone(&self) -> crate::common::Reg<self::Mtudone_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(236usize)) }
    }

    #[doc = "PMS DTS Service Request\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pmsdts(&self) -> crate::common::Reg<self::Pmsdts_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2220usize)) }
    }

    #[doc = "Stand By Controller Service Request\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn scr(&self) -> crate::common::Reg<self::Scr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2240usize)) }
    }

    #[doc = "SRI Domain 0 Service Request\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn xbar0(&self) -> crate::common::Reg<self::Xbar0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }
    #[doc = "ASCLIN"]
    #[inline(always)]
    pub fn asclin(self) -> self::Asclin {
        unsafe { self::Asclin(self.0.add(80usize)) }
    }
    #[doc = "CAN"]
    #[inline(always)]
    pub fn can(self) -> self::Can {
        unsafe { self::Can(self.0.add(1456usize)) }
    }
    #[doc = "CCU6"]
    #[inline(always)]
    pub fn ccu6(self) -> self::Ccu6 {
        unsafe { self::Ccu6(self.0.add(704usize)) }
    }
    #[doc = "CERBERUS"]
    #[inline(always)]
    pub fn cerberus(self) -> self::Cerberus {
        unsafe { self::Cerberus(self.0.add(64usize)) }
    }
    #[doc = "CPU"]
    #[inline(always)]
    pub fn cpu(self) -> self::Cpu {
        unsafe { self::Cpu(self.0.add(0usize)) }
    }
    #[doc = "DAM"]
    #[inline(always)]
    pub fn dam(self) -> self::Dam {
        unsafe { self::Dam(self.0.add(2320usize)) }
    }
    #[doc = "DMA"]
    #[inline(always)]
    pub fn dma(self) -> self::Dma {
        unsafe { self::Dma(self.0.add(832usize)) }
    }
    #[doc = "DSADC"]
    #[inline(always)]
    pub fn dsadc(self) -> self::Dsadc {
        unsafe { self::Dsadc(self.0.add(1904usize)) }
    }
    #[doc = "ERAY"]
    #[inline(always)]
    pub fn eray(self) -> self::Eray {
        unsafe { self::Eray(self.0.add(2048usize)) }
    }
    #[doc = "FCE"]
    #[inline(always)]
    pub fn fce(self) -> self::Fce {
        unsafe { self::Fce(self.0.add(816usize)) }
    }
    #[doc = "GETH"]
    #[inline(always)]
    pub fn geth(self) -> self::Geth {
        unsafe { self::Geth(self.0.add(1408usize)) }
    }
    #[doc = "GPSR"]
    #[inline(always)]
    pub fn gpsr(self) -> self::Gpsr {
        unsafe { self::Gpsr(self.0.add(2448usize)) }
    }
    #[doc = "GPT12"]
    #[inline(always)]
    pub fn gpt12(self) -> self::Gpt12 {
        unsafe { self::Gpt12(self.0.add(736usize)) }
    }
    #[doc = "ATOM"]
    #[inline(always)]
    pub fn gtmatomwx(self) -> [self::GtmatoMwx; 6] {
        unsafe {
            [
                self::GtmatoMwx(self.0.add(0xef0usize + 0x0usize)),
                self::GtmatoMwx(self.0.add(0xef0usize + 0x4usize)),
                self::GtmatoMwx(self.0.add(0xef0usize + 0x8usize)),
                self::GtmatoMwx(self.0.add(0xef0usize + 0xcusize)),
                self::GtmatoMwx(self.0.add(0xef0usize + 0x10usize)),
                self::GtmatoMwx(self.0.add(0xef0usize + 0x14usize)),
            ]
        }
    }
    #[doc = "MCS"]
    #[inline(always)]
    pub fn gtmmcswx(self) -> [self::GtmmcSwx; 5] {
        unsafe {
            [
                self::GtmmcSwx(self.0.add(0xcb0usize + 0x0usize)),
                self::GtmmcSwx(self.0.add(0xcb0usize + 0x4usize)),
                self::GtmmcSwx(self.0.add(0xcb0usize + 0x8usize)),
                self::GtmmcSwx(self.0.add(0xcb0usize + 0xcusize)),
                self::GtmmcSwx(self.0.add(0xcb0usize + 0x10usize)),
            ]
        }
    }
    #[doc = "TIM"]
    #[inline(always)]
    pub fn gtmtimwx(self) -> [self::GtmtiMwx; 6] {
        unsafe {
            [
                self::GtmtiMwx(self.0.add(0xb90usize + 0x0usize)),
                self::GtmtiMwx(self.0.add(0xb90usize + 0x4usize)),
                self::GtmtiMwx(self.0.add(0xb90usize + 0x8usize)),
                self::GtmtiMwx(self.0.add(0xb90usize + 0xcusize)),
                self::GtmtiMwx(self.0.add(0xb90usize + 0x10usize)),
                self::GtmtiMwx(self.0.add(0xb90usize + 0x14usize)),
            ]
        }
    }
    #[doc = "TOM"]
    #[inline(always)]
    pub fn gtmtomwx(self) -> [self::GtmtoMwx; 3] {
        unsafe {
            [
                self::GtmtoMwx(self.0.add(0xe10usize + 0x0usize)),
                self::GtmtoMwx(self.0.add(0xe10usize + 0x4usize)),
                self::GtmtoMwx(self.0.add(0xe10usize + 0x8usize)),
            ]
        }
    }
    #[doc = "HSCT"]
    #[inline(always)]
    pub fn hsct(self) -> self::Hsct {
        unsafe { self::Hsct(self.0.add(384usize)) }
    }
    #[doc = "HSM"]
    #[inline(always)]
    pub fn hsm(self) -> self::Hsm {
        unsafe { self::Hsm(self.0.add(2160usize)) }
    }
    #[doc = "HSSL"]
    #[inline(always)]
    pub fn hssl(self) -> self::Hssl {
        unsafe { self::Hssl(self.0.add(400usize)) }
    }
    #[doc = "I2C"]
    #[inline(always)]
    pub fn i2c(self) -> self::I2C {
        unsafe { self::I2C(self.0.add(544usize)) }
    }
    #[doc = "MSC"]
    #[inline(always)]
    pub fn msc(self) -> self::Msc {
        unsafe { self::Msc(self.0.add(624usize)) }
    }
    #[doc = "PMS"]
    #[inline(always)]
    pub fn pms(self) -> self::Pms {
        unsafe { self::Pms(self.0.add(2224usize)) }
    }
    #[doc = "PSI5"]
    #[inline(always)]
    pub fn psi5(self) -> self::Psi5 {
        unsafe { self::Psi5(self.0.add(2272usize)) }
    }
    #[doc = "PSI5S"]
    #[inline(always)]
    pub fn psi5s(self) -> self::Psi5S {
        unsafe { self::Psi5S(self.0.add(2384usize)) }
    }
    #[doc = "QSPI"]
    #[inline(always)]
    pub fn qspi(self) -> self::Qspi {
        unsafe { self::Qspi(self.0.add(240usize)) }
    }
    #[doc = "SCU"]
    #[inline(always)]
    pub fn scu(self) -> self::Scu {
        unsafe { self::Scu(self.0.add(2176usize)) }
    }
    #[doc = "SENT"]
    #[inline(always)]
    pub fn sent(self) -> self::Sent {
        unsafe { self::Sent(self.0.add(576usize)) }
    }
    #[doc = "SMU"]
    #[inline(always)]
    pub fn smu(self) -> self::Smu {
        unsafe { self::Smu(self.0.add(2256usize)) }
    }
    #[doc = "STM"]
    #[inline(always)]
    pub fn stm(self) -> self::Stm {
        unsafe { self::Stm(self.0.add(768usize)) }
    }
    #[doc = "VADC"]
    #[inline(always)]
    pub fn vadc(self) -> self::Vadc {
        unsafe { self::Vadc(self.0.add(1648usize)) }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bcuspb_SPEC;
impl crate::sealed::RegSpec for Bcuspb_SPEC {
    type DataType = u32;
}
#[doc = "SBCU Service Request  SPB Bus Control Unit \n resetvalue={Debug Reset:0x0}"]
pub type Bcuspb = crate::RegValueT<Bcuspb_SPEC>;

impl Bcuspb {
    #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
    #[inline(always)]
    pub fn srpn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Bcuspb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Bcuspb_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Enable"]
    #[inline(always)]
    pub fn sre(self) -> crate::common::RegisterFieldBool<10, 1, 0, Bcuspb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Bcuspb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
    #[inline(always)]
    pub fn tos(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Bcuspb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x7,1,0,u8, Bcuspb_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
    #[inline(always)]
    pub fn ecc(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Bcuspb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Bcuspb_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
    #[inline(always)]
    pub fn srr(self) -> crate::common::RegisterFieldBool<24, 1, 0, Bcuspb_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Bcuspb_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
    #[inline(always)]
    pub fn clrr(self) -> crate::common::RegisterFieldBool<25, 1, 0, Bcuspb_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Bcuspb_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
    #[inline(always)]
    pub fn setr(self) -> crate::common::RegisterFieldBool<26, 1, 0, Bcuspb_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Bcuspb_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
    #[inline(always)]
    pub fn iov(self) -> crate::common::RegisterFieldBool<27, 1, 0, Bcuspb_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Bcuspb_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
    #[inline(always)]
    pub fn iovclr(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Bcuspb_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Bcuspb_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
    #[inline(always)]
    pub fn sws(self) -> crate::common::RegisterFieldBool<29, 1, 0, Bcuspb_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Bcuspb_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
    #[inline(always)]
    pub fn swsclr(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Bcuspb_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Bcuspb_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Bcuspb {
    #[inline(always)]
    fn default() -> Bcuspb {
        <crate::RegValueT<Bcuspb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmufsi_SPEC;
impl crate::sealed::RegSpec for Dmufsi_SPEC {
    type DataType = u32;
}
#[doc = "DMU FSI Service Request\n resetvalue={Application Reset:0x0}"]
pub type Dmufsi = crate::RegValueT<Dmufsi_SPEC>;

impl Dmufsi {
    #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
    #[inline(always)]
    pub fn srpn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Dmufsi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Dmufsi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Enable"]
    #[inline(always)]
    pub fn sre(self) -> crate::common::RegisterFieldBool<10, 1, 0, Dmufsi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Dmufsi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
    #[inline(always)]
    pub fn tos(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Dmufsi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x7,1,0,u8, Dmufsi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
    #[inline(always)]
    pub fn ecc(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Dmufsi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Dmufsi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
    #[inline(always)]
    pub fn srr(self) -> crate::common::RegisterFieldBool<24, 1, 0, Dmufsi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Dmufsi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
    #[inline(always)]
    pub fn clrr(self) -> crate::common::RegisterFieldBool<25, 1, 0, Dmufsi_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Dmufsi_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
    #[inline(always)]
    pub fn setr(self) -> crate::common::RegisterFieldBool<26, 1, 0, Dmufsi_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Dmufsi_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
    #[inline(always)]
    pub fn iov(self) -> crate::common::RegisterFieldBool<27, 1, 0, Dmufsi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Dmufsi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
    #[inline(always)]
    pub fn iovclr(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Dmufsi_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Dmufsi_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
    #[inline(always)]
    pub fn sws(self) -> crate::common::RegisterFieldBool<29, 1, 0, Dmufsi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Dmufsi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
    #[inline(always)]
    pub fn swsclr(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Dmufsi_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Dmufsi_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Dmufsi {
    #[inline(always)]
    fn default() -> Dmufsi {
        <crate::RegValueT<Dmufsi_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmuhost_SPEC;
impl crate::sealed::RegSpec for Dmuhost_SPEC {
    type DataType = u32;
}
#[doc = "DMU Host Service Request\n resetvalue={Application Reset:0x0}"]
pub type Dmuhost = crate::RegValueT<Dmuhost_SPEC>;

impl Dmuhost {
    #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
    #[inline(always)]
    pub fn srpn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Dmuhost_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Dmuhost_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Enable"]
    #[inline(always)]
    pub fn sre(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Dmuhost_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Dmuhost_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
    #[inline(always)]
    pub fn tos(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Dmuhost_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x7,1,0,u8, Dmuhost_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
    #[inline(always)]
    pub fn ecc(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Dmuhost_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Dmuhost_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
    #[inline(always)]
    pub fn srr(self) -> crate::common::RegisterFieldBool<24, 1, 0, Dmuhost_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Dmuhost_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
    #[inline(always)]
    pub fn clrr(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Dmuhost_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Dmuhost_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
    #[inline(always)]
    pub fn setr(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Dmuhost_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Dmuhost_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
    #[inline(always)]
    pub fn iov(self) -> crate::common::RegisterFieldBool<27, 1, 0, Dmuhost_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Dmuhost_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
    #[inline(always)]
    pub fn iovclr(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Dmuhost_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Dmuhost_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
    #[inline(always)]
    pub fn sws(self) -> crate::common::RegisterFieldBool<29, 1, 0, Dmuhost_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Dmuhost_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
    #[inline(always)]
    pub fn swsclr(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Dmuhost_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Dmuhost_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Dmuhost {
    #[inline(always)]
    fn default() -> Dmuhost {
        <crate::RegValueT<Dmuhost_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtmaeiirq_SPEC;
impl crate::sealed::RegSpec for Gtmaeiirq_SPEC {
    type DataType = u32;
}
#[doc = "AEI Shared Service Request\n resetvalue={Application Reset:0x0}"]
pub type Gtmaeiirq = crate::RegValueT<Gtmaeiirq_SPEC>;

impl Gtmaeiirq {
    #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
    #[inline(always)]
    pub fn srpn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Gtmaeiirq_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Gtmaeiirq_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Enable"]
    #[inline(always)]
    pub fn sre(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Gtmaeiirq_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Gtmaeiirq_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
    #[inline(always)]
    pub fn tos(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Gtmaeiirq_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x7,1,0,u8, Gtmaeiirq_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
    #[inline(always)]
    pub fn ecc(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Gtmaeiirq_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Gtmaeiirq_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
    #[inline(always)]
    pub fn srr(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Gtmaeiirq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24,1,0,Gtmaeiirq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
    #[inline(always)]
    pub fn clrr(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Gtmaeiirq_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25,1,0,Gtmaeiirq_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
    #[inline(always)]
    pub fn setr(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Gtmaeiirq_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26,1,0,Gtmaeiirq_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
    #[inline(always)]
    pub fn iov(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Gtmaeiirq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27,1,0,Gtmaeiirq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
    #[inline(always)]
    pub fn iovclr(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Gtmaeiirq_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28,1,0,Gtmaeiirq_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
    #[inline(always)]
    pub fn sws(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Gtmaeiirq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29,1,0,Gtmaeiirq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
    #[inline(always)]
    pub fn swsclr(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Gtmaeiirq_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30,1,0,Gtmaeiirq_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Gtmaeiirq {
    #[inline(always)]
    fn default() -> Gtmaeiirq {
        <crate::RegValueT<Gtmaeiirq_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GtmaruirQw_SPEC;
impl crate::sealed::RegSpec for GtmaruirQw_SPEC {
    type DataType = u32;
}
#[doc = "ARU Shared Service Request 0\n resetvalue={Application Reset:0x0}"]
pub type GtmaruirQw = crate::RegValueT<GtmaruirQw_SPEC>;

impl GtmaruirQw {
    #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
    #[inline(always)]
    pub fn srpn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, GtmaruirQw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, GtmaruirQw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Enable"]
    #[inline(always)]
    pub fn sre(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, GtmaruirQw_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,GtmaruirQw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
    #[inline(always)]
    pub fn tos(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, GtmaruirQw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x7,1,0,u8, GtmaruirQw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
    #[inline(always)]
    pub fn ecc(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, GtmaruirQw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, GtmaruirQw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
    #[inline(always)]
    pub fn srr(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, GtmaruirQw_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24,1,0,GtmaruirQw_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
    #[inline(always)]
    pub fn clrr(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, GtmaruirQw_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25,1,0,GtmaruirQw_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
    #[inline(always)]
    pub fn setr(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, GtmaruirQw_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26,1,0,GtmaruirQw_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
    #[inline(always)]
    pub fn iov(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, GtmaruirQw_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27,1,0,GtmaruirQw_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
    #[inline(always)]
    pub fn iovclr(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, GtmaruirQw_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28,1,0,GtmaruirQw_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
    #[inline(always)]
    pub fn sws(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, GtmaruirQw_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29,1,0,GtmaruirQw_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
    #[inline(always)]
    pub fn swsclr(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, GtmaruirQw_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30,1,0,GtmaruirQw_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for GtmaruirQw {
    #[inline(always)]
    fn default() -> GtmaruirQw {
        <crate::RegValueT<GtmaruirQw_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtmbrcirq_SPEC;
impl crate::sealed::RegSpec for Gtmbrcirq_SPEC {
    type DataType = u32;
}
#[doc = "BRC Shared Service Request\n resetvalue={Application Reset:0x0}"]
pub type Gtmbrcirq = crate::RegValueT<Gtmbrcirq_SPEC>;

impl Gtmbrcirq {
    #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
    #[inline(always)]
    pub fn srpn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Gtmbrcirq_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Gtmbrcirq_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Enable"]
    #[inline(always)]
    pub fn sre(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Gtmbrcirq_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Gtmbrcirq_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
    #[inline(always)]
    pub fn tos(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Gtmbrcirq_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x7,1,0,u8, Gtmbrcirq_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
    #[inline(always)]
    pub fn ecc(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Gtmbrcirq_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Gtmbrcirq_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
    #[inline(always)]
    pub fn srr(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Gtmbrcirq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24,1,0,Gtmbrcirq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
    #[inline(always)]
    pub fn clrr(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Gtmbrcirq_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25,1,0,Gtmbrcirq_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
    #[inline(always)]
    pub fn setr(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Gtmbrcirq_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26,1,0,Gtmbrcirq_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
    #[inline(always)]
    pub fn iov(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Gtmbrcirq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27,1,0,Gtmbrcirq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
    #[inline(always)]
    pub fn iovclr(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Gtmbrcirq_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28,1,0,Gtmbrcirq_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
    #[inline(always)]
    pub fn sws(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Gtmbrcirq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29,1,0,Gtmbrcirq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
    #[inline(always)]
    pub fn swsclr(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Gtmbrcirq_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30,1,0,Gtmbrcirq_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Gtmbrcirq {
    #[inline(always)]
    fn default() -> Gtmbrcirq {
        <crate::RegValueT<Gtmbrcirq_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtmcmpirq_SPEC;
impl crate::sealed::RegSpec for Gtmcmpirq_SPEC {
    type DataType = u32;
}
#[doc = "CMP Shared Service Request\n resetvalue={Application Reset:0x0}"]
pub type Gtmcmpirq = crate::RegValueT<Gtmcmpirq_SPEC>;

impl Gtmcmpirq {
    #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
    #[inline(always)]
    pub fn srpn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Gtmcmpirq_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Gtmcmpirq_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Enable"]
    #[inline(always)]
    pub fn sre(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Gtmcmpirq_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Gtmcmpirq_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
    #[inline(always)]
    pub fn tos(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Gtmcmpirq_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x7,1,0,u8, Gtmcmpirq_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
    #[inline(always)]
    pub fn ecc(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Gtmcmpirq_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Gtmcmpirq_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
    #[inline(always)]
    pub fn srr(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Gtmcmpirq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24,1,0,Gtmcmpirq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
    #[inline(always)]
    pub fn clrr(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Gtmcmpirq_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25,1,0,Gtmcmpirq_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
    #[inline(always)]
    pub fn setr(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Gtmcmpirq_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26,1,0,Gtmcmpirq_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
    #[inline(always)]
    pub fn iov(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Gtmcmpirq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27,1,0,Gtmcmpirq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
    #[inline(always)]
    pub fn iovclr(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Gtmcmpirq_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28,1,0,Gtmcmpirq_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
    #[inline(always)]
    pub fn sws(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Gtmcmpirq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29,1,0,Gtmcmpirq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
    #[inline(always)]
    pub fn swsclr(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Gtmcmpirq_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30,1,0,Gtmcmpirq_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Gtmcmpirq {
    #[inline(always)]
    fn default() -> Gtmcmpirq {
        <crate::RegValueT<Gtmcmpirq_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GtmdplLw_SPEC;
impl crate::sealed::RegSpec for GtmdplLw_SPEC {
    type DataType = u32;
}
#[doc = "DPLL Service Request 0\n resetvalue={Application Reset:0x0}"]
pub type GtmdplLw = crate::RegValueT<GtmdplLw_SPEC>;

impl GtmdplLw {
    #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
    #[inline(always)]
    pub fn srpn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, GtmdplLw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, GtmdplLw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Enable"]
    #[inline(always)]
    pub fn sre(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, GtmdplLw_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,GtmdplLw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
    #[inline(always)]
    pub fn tos(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, GtmdplLw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x7,1,0,u8, GtmdplLw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
    #[inline(always)]
    pub fn ecc(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, GtmdplLw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, GtmdplLw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
    #[inline(always)]
    pub fn srr(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, GtmdplLw_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, GtmdplLw_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
    #[inline(always)]
    pub fn clrr(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, GtmdplLw_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, GtmdplLw_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
    #[inline(always)]
    pub fn setr(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, GtmdplLw_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, GtmdplLw_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
    #[inline(always)]
    pub fn iov(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, GtmdplLw_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, GtmdplLw_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
    #[inline(always)]
    pub fn iovclr(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, GtmdplLw_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, GtmdplLw_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
    #[inline(always)]
    pub fn sws(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, GtmdplLw_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, GtmdplLw_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
    #[inline(always)]
    pub fn swsclr(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, GtmdplLw_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, GtmdplLw_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for GtmdplLw {
    #[inline(always)]
    fn default() -> GtmdplLw {
        <crate::RegValueT<GtmdplLw_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtmerr_SPEC;
impl crate::sealed::RegSpec for Gtmerr_SPEC {
    type DataType = u32;
}
#[doc = "Error Service Request\n resetvalue={Application Reset:0x0}"]
pub type Gtmerr = crate::RegValueT<Gtmerr_SPEC>;

impl Gtmerr {
    #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
    #[inline(always)]
    pub fn srpn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Gtmerr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Gtmerr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Enable"]
    #[inline(always)]
    pub fn sre(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gtmerr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gtmerr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
    #[inline(always)]
    pub fn tos(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Gtmerr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x7,1,0,u8, Gtmerr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
    #[inline(always)]
    pub fn ecc(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Gtmerr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Gtmerr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
    #[inline(always)]
    pub fn srr(self) -> crate::common::RegisterFieldBool<24, 1, 0, Gtmerr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Gtmerr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
    #[inline(always)]
    pub fn clrr(self) -> crate::common::RegisterFieldBool<25, 1, 0, Gtmerr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Gtmerr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
    #[inline(always)]
    pub fn setr(self) -> crate::common::RegisterFieldBool<26, 1, 0, Gtmerr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Gtmerr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
    #[inline(always)]
    pub fn iov(self) -> crate::common::RegisterFieldBool<27, 1, 0, Gtmerr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Gtmerr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
    #[inline(always)]
    pub fn iovclr(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Gtmerr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Gtmerr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
    #[inline(always)]
    pub fn sws(self) -> crate::common::RegisterFieldBool<29, 1, 0, Gtmerr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Gtmerr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
    #[inline(always)]
    pub fn swsclr(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Gtmerr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Gtmerr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Gtmerr {
    #[inline(always)]
    fn default() -> Gtmerr {
        <crate::RegValueT<Gtmerr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GtmmcsWw_SPEC;
impl crate::sealed::RegSpec for GtmmcsWw_SPEC {
    type DataType = u32;
}
#[doc = "GTM Multi Channel Sequencer Service Request 0\n resetvalue={Application Reset:0x0}"]
pub type GtmmcsWw = crate::RegValueT<GtmmcsWw_SPEC>;

impl GtmmcsWw {
    #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
    #[inline(always)]
    pub fn srpn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, GtmmcsWw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, GtmmcsWw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Enable"]
    #[inline(always)]
    pub fn sre(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, GtmmcsWw_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,GtmmcsWw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
    #[inline(always)]
    pub fn tos(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, GtmmcsWw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x7,1,0,u8, GtmmcsWw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
    #[inline(always)]
    pub fn ecc(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, GtmmcsWw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, GtmmcsWw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
    #[inline(always)]
    pub fn srr(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, GtmmcsWw_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, GtmmcsWw_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
    #[inline(always)]
    pub fn clrr(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, GtmmcsWw_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, GtmmcsWw_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
    #[inline(always)]
    pub fn setr(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, GtmmcsWw_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, GtmmcsWw_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
    #[inline(always)]
    pub fn iov(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, GtmmcsWw_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, GtmmcsWw_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
    #[inline(always)]
    pub fn iovclr(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, GtmmcsWw_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, GtmmcsWw_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
    #[inline(always)]
    pub fn sws(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, GtmmcsWw_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, GtmmcsWw_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
    #[inline(always)]
    pub fn swsclr(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, GtmmcsWw_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, GtmmcsWw_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for GtmmcsWw {
    #[inline(always)]
    fn default() -> GtmmcsWw {
        <crate::RegValueT<GtmmcsWw_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GtmpsMwx_SPEC;
impl crate::sealed::RegSpec for GtmpsMwx_SPEC {
    type DataType = u32;
}
#[doc = "PSM0 Shared Service Request 0\n resetvalue={Application Reset:0x0}"]
pub type GtmpsMwx = crate::RegValueT<GtmpsMwx_SPEC>;

impl GtmpsMwx {
    #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
    #[inline(always)]
    pub fn srpn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, GtmpsMwx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, GtmpsMwx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Enable"]
    #[inline(always)]
    pub fn sre(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, GtmpsMwx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,GtmpsMwx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
    #[inline(always)]
    pub fn tos(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, GtmpsMwx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x7,1,0,u8, GtmpsMwx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
    #[inline(always)]
    pub fn ecc(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, GtmpsMwx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, GtmpsMwx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
    #[inline(always)]
    pub fn srr(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, GtmpsMwx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, GtmpsMwx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
    #[inline(always)]
    pub fn clrr(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, GtmpsMwx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, GtmpsMwx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
    #[inline(always)]
    pub fn setr(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, GtmpsMwx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, GtmpsMwx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
    #[inline(always)]
    pub fn iov(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, GtmpsMwx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, GtmpsMwx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
    #[inline(always)]
    pub fn iovclr(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, GtmpsMwx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, GtmpsMwx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
    #[inline(always)]
    pub fn sws(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, GtmpsMwx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, GtmpsMwx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
    #[inline(always)]
    pub fn swsclr(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, GtmpsMwx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, GtmpsMwx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for GtmpsMwx {
    #[inline(always)]
    fn default() -> GtmpsMwx {
        <crate::RegValueT<GtmpsMwx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GtmspEwIrq_SPEC;
impl crate::sealed::RegSpec for GtmspEwIrq_SPEC {
    type DataType = u32;
}
#[doc = "SPE0 Shared Service Request\n resetvalue={Application Reset:0x0}"]
pub type GtmspEwIrq = crate::RegValueT<GtmspEwIrq_SPEC>;

impl GtmspEwIrq {
    #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
    #[inline(always)]
    pub fn srpn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, GtmspEwIrq_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, GtmspEwIrq_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Enable"]
    #[inline(always)]
    pub fn sre(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, GtmspEwIrq_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,GtmspEwIrq_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
    #[inline(always)]
    pub fn tos(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, GtmspEwIrq_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x7,1,0,u8, GtmspEwIrq_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
    #[inline(always)]
    pub fn ecc(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, GtmspEwIrq_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, GtmspEwIrq_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
    #[inline(always)]
    pub fn srr(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, GtmspEwIrq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24,1,0,GtmspEwIrq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
    #[inline(always)]
    pub fn clrr(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, GtmspEwIrq_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25,1,0,GtmspEwIrq_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
    #[inline(always)]
    pub fn setr(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, GtmspEwIrq_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26,1,0,GtmspEwIrq_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
    #[inline(always)]
    pub fn iov(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, GtmspEwIrq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27,1,0,GtmspEwIrq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
    #[inline(always)]
    pub fn iovclr(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, GtmspEwIrq_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28,1,0,GtmspEwIrq_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
    #[inline(always)]
    pub fn sws(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, GtmspEwIrq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29,1,0,GtmspEwIrq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
    #[inline(always)]
    pub fn swsclr(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, GtmspEwIrq_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30,1,0,GtmspEwIrq_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for GtmspEwIrq {
    #[inline(always)]
    fn default() -> GtmspEwIrq {
        <crate::RegValueT<GtmspEwIrq_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtudone_SPEC;
impl crate::sealed::RegSpec for Mtudone_SPEC {
    type DataType = u32;
}
#[doc = "MTU Done Service Request\n resetvalue={Application Reset:0x0}"]
pub type Mtudone = crate::RegValueT<Mtudone_SPEC>;

impl Mtudone {
    #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
    #[inline(always)]
    pub fn srpn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Mtudone_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Mtudone_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Enable"]
    #[inline(always)]
    pub fn sre(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Mtudone_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Mtudone_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
    #[inline(always)]
    pub fn tos(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Mtudone_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x7,1,0,u8, Mtudone_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
    #[inline(always)]
    pub fn ecc(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Mtudone_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Mtudone_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
    #[inline(always)]
    pub fn srr(self) -> crate::common::RegisterFieldBool<24, 1, 0, Mtudone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Mtudone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
    #[inline(always)]
    pub fn clrr(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Mtudone_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Mtudone_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
    #[inline(always)]
    pub fn setr(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Mtudone_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Mtudone_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
    #[inline(always)]
    pub fn iov(self) -> crate::common::RegisterFieldBool<27, 1, 0, Mtudone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Mtudone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
    #[inline(always)]
    pub fn iovclr(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Mtudone_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Mtudone_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
    #[inline(always)]
    pub fn sws(self) -> crate::common::RegisterFieldBool<29, 1, 0, Mtudone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Mtudone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
    #[inline(always)]
    pub fn swsclr(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Mtudone_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Mtudone_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mtudone {
    #[inline(always)]
    fn default() -> Mtudone {
        <crate::RegValueT<Mtudone_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmsdts_SPEC;
impl crate::sealed::RegSpec for Pmsdts_SPEC {
    type DataType = u32;
}
#[doc = "PMS DTS Service Request\n resetvalue={Application Reset:0x0}"]
pub type Pmsdts = crate::RegValueT<Pmsdts_SPEC>;

impl Pmsdts {
    #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
    #[inline(always)]
    pub fn srpn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Pmsdts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Pmsdts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Enable"]
    #[inline(always)]
    pub fn sre(self) -> crate::common::RegisterFieldBool<10, 1, 0, Pmsdts_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Pmsdts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
    #[inline(always)]
    pub fn tos(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Pmsdts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x7,1,0,u8, Pmsdts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
    #[inline(always)]
    pub fn ecc(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Pmsdts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Pmsdts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
    #[inline(always)]
    pub fn srr(self) -> crate::common::RegisterFieldBool<24, 1, 0, Pmsdts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Pmsdts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
    #[inline(always)]
    pub fn clrr(self) -> crate::common::RegisterFieldBool<25, 1, 0, Pmsdts_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Pmsdts_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
    #[inline(always)]
    pub fn setr(self) -> crate::common::RegisterFieldBool<26, 1, 0, Pmsdts_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Pmsdts_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
    #[inline(always)]
    pub fn iov(self) -> crate::common::RegisterFieldBool<27, 1, 0, Pmsdts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Pmsdts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
    #[inline(always)]
    pub fn iovclr(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Pmsdts_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Pmsdts_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
    #[inline(always)]
    pub fn sws(self) -> crate::common::RegisterFieldBool<29, 1, 0, Pmsdts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Pmsdts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
    #[inline(always)]
    pub fn swsclr(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Pmsdts_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Pmsdts_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Pmsdts {
    #[inline(always)]
    fn default() -> Pmsdts {
        <crate::RegValueT<Pmsdts_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scr_SPEC;
impl crate::sealed::RegSpec for Scr_SPEC {
    type DataType = u32;
}
#[doc = "Stand By Controller Service Request\n resetvalue={Application Reset:0x0}"]
pub type Scr = crate::RegValueT<Scr_SPEC>;

impl Scr {
    #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
    #[inline(always)]
    pub fn srpn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Scr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Scr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Enable"]
    #[inline(always)]
    pub fn sre(self) -> crate::common::RegisterFieldBool<10, 1, 0, Scr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Scr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
    #[inline(always)]
    pub fn tos(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Scr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x7,1,0,u8, Scr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
    #[inline(always)]
    pub fn ecc(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Scr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Scr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
    #[inline(always)]
    pub fn srr(self) -> crate::common::RegisterFieldBool<24, 1, 0, Scr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Scr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
    #[inline(always)]
    pub fn clrr(self) -> crate::common::RegisterFieldBool<25, 1, 0, Scr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Scr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
    #[inline(always)]
    pub fn setr(self) -> crate::common::RegisterFieldBool<26, 1, 0, Scr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Scr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
    #[inline(always)]
    pub fn iov(self) -> crate::common::RegisterFieldBool<27, 1, 0, Scr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Scr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
    #[inline(always)]
    pub fn iovclr(self) -> crate::common::RegisterFieldBool<28, 1, 0, Scr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Scr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
    #[inline(always)]
    pub fn sws(self) -> crate::common::RegisterFieldBool<29, 1, 0, Scr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Scr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
    #[inline(always)]
    pub fn swsclr(self) -> crate::common::RegisterFieldBool<30, 1, 0, Scr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Scr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Scr {
    #[inline(always)]
    fn default() -> Scr {
        <crate::RegValueT<Scr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xbar0_SPEC;
impl crate::sealed::RegSpec for Xbar0_SPEC {
    type DataType = u32;
}
#[doc = "SRI Domain 0 Service Request\n resetvalue={Debug Reset:0x0}"]
pub type Xbar0 = crate::RegValueT<Xbar0_SPEC>;

impl Xbar0 {
    #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
    #[inline(always)]
    pub fn srpn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Xbar0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Xbar0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Enable"]
    #[inline(always)]
    pub fn sre(self) -> crate::common::RegisterFieldBool<10, 1, 0, Xbar0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Xbar0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
    #[inline(always)]
    pub fn tos(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Xbar0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x7,1,0,u8, Xbar0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
    #[inline(always)]
    pub fn ecc(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Xbar0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Xbar0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
    #[inline(always)]
    pub fn srr(self) -> crate::common::RegisterFieldBool<24, 1, 0, Xbar0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Xbar0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
    #[inline(always)]
    pub fn clrr(self) -> crate::common::RegisterFieldBool<25, 1, 0, Xbar0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Xbar0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
    #[inline(always)]
    pub fn setr(self) -> crate::common::RegisterFieldBool<26, 1, 0, Xbar0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Xbar0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
    #[inline(always)]
    pub fn iov(self) -> crate::common::RegisterFieldBool<27, 1, 0, Xbar0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Xbar0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
    #[inline(always)]
    pub fn iovclr(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Xbar0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Xbar0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
    #[inline(always)]
    pub fn sws(self) -> crate::common::RegisterFieldBool<29, 1, 0, Xbar0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Xbar0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
    #[inline(always)]
    pub fn swsclr(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Xbar0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Xbar0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Xbar0 {
    #[inline(always)]
    fn default() -> Xbar0 {
        <crate::RegValueT<Xbar0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc = "ASCLIN"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asclin(pub(super) *mut u8);
unsafe impl core::marker::Send for Asclin {}
unsafe impl core::marker::Sync for Asclin {}
impl Asclin {
    #[doc = "ASCLIN"]
    #[inline(always)]
    pub fn asclin(self) -> [asclin::AsclinAsclin; 12] {
        unsafe {
            [
                asclin::AsclinAsclin(self.0.add(0x0usize + 0x0usize)),
                asclin::AsclinAsclin(self.0.add(0x0usize + 0xcusize)),
                asclin::AsclinAsclin(self.0.add(0x0usize + 0x18usize)),
                asclin::AsclinAsclin(self.0.add(0x0usize + 0x24usize)),
                asclin::AsclinAsclin(self.0.add(0x0usize + 0x30usize)),
                asclin::AsclinAsclin(self.0.add(0x0usize + 0x3cusize)),
                asclin::AsclinAsclin(self.0.add(0x0usize + 0x48usize)),
                asclin::AsclinAsclin(self.0.add(0x0usize + 0x54usize)),
                asclin::AsclinAsclin(self.0.add(0x0usize + 0x60usize)),
                asclin::AsclinAsclin(self.0.add(0x0usize + 0x6cusize)),
                asclin::AsclinAsclin(self.0.add(0x0usize + 0x78usize)),
                asclin::AsclinAsclin(self.0.add(0x0usize + 0x84usize)),
            ]
        }
    }
}
pub mod asclin {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "ASCLIN"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AsclinAsclin(pub(super) *mut u8);
    unsafe impl core::marker::Send for AsclinAsclin {}
    unsafe impl core::marker::Sync for AsclinAsclin {}
    impl AsclinAsclin {
        #[doc = "ASCLIN0 Error Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn asclinxerr(
            &self,
        ) -> crate::common::Reg<asclin_asclin::AscliNxErr_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "ASCLIN0 Receive Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn asclinxrx(
            &self,
        ) -> crate::common::Reg<asclin_asclin::AscliNxRx_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "ASCLIN0 Transmit Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn asclinxtx(
            &self,
        ) -> crate::common::Reg<asclin_asclin::AscliNxTx_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
    }
    pub mod asclin_asclin {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct AscliNxErr_SPEC;
        impl crate::sealed::RegSpec for AscliNxErr_SPEC {
            type DataType = u32;
        }
        #[doc = "ASCLIN0 Error Service Request\n resetvalue={Application Reset:0x0}"]
        pub type AscliNxErr = crate::RegValueT<AscliNxErr_SPEC>;

        impl AscliNxErr {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, AscliNxErr_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, AscliNxErr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, AscliNxErr_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,AscliNxErr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, AscliNxErr_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, AscliNxErr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, AscliNxErr_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, AscliNxErr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, AscliNxErr_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,AscliNxErr_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, AscliNxErr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,AscliNxErr_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, AscliNxErr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,AscliNxErr_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, AscliNxErr_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,AscliNxErr_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, AscliNxErr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,AscliNxErr_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, AscliNxErr_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,AscliNxErr_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, AscliNxErr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,AscliNxErr_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for AscliNxErr {
            #[inline(always)]
            fn default() -> AscliNxErr {
                <crate::RegValueT<AscliNxErr_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct AscliNxRx_SPEC;
        impl crate::sealed::RegSpec for AscliNxRx_SPEC {
            type DataType = u32;
        }
        #[doc = "ASCLIN0 Receive Service Request\n resetvalue={Application Reset:0x0}"]
        pub type AscliNxRx = crate::RegValueT<AscliNxRx_SPEC>;

        impl AscliNxRx {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, AscliNxRx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, AscliNxRx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, AscliNxRx_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,AscliNxRx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, AscliNxRx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, AscliNxRx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, AscliNxRx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, AscliNxRx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, AscliNxRx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,AscliNxRx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, AscliNxRx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,AscliNxRx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, AscliNxRx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,AscliNxRx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, AscliNxRx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,AscliNxRx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, AscliNxRx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,AscliNxRx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, AscliNxRx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,AscliNxRx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, AscliNxRx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,AscliNxRx_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for AscliNxRx {
            #[inline(always)]
            fn default() -> AscliNxRx {
                <crate::RegValueT<AscliNxRx_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct AscliNxTx_SPEC;
        impl crate::sealed::RegSpec for AscliNxTx_SPEC {
            type DataType = u32;
        }
        #[doc = "ASCLIN0 Transmit Service Request\n resetvalue={Application Reset:0x0}"]
        pub type AscliNxTx = crate::RegValueT<AscliNxTx_SPEC>;

        impl AscliNxTx {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, AscliNxTx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, AscliNxTx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, AscliNxTx_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,AscliNxTx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, AscliNxTx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, AscliNxTx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, AscliNxTx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, AscliNxTx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, AscliNxTx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,AscliNxTx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, AscliNxTx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,AscliNxTx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, AscliNxTx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,AscliNxTx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, AscliNxTx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,AscliNxTx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, AscliNxTx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,AscliNxTx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, AscliNxTx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,AscliNxTx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, AscliNxTx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,AscliNxTx_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for AscliNxTx {
            #[inline(always)]
            fn default() -> AscliNxTx {
                <crate::RegValueT<AscliNxTx_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "CAN"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can(pub(super) *mut u8);
unsafe impl core::marker::Send for Can {}
unsafe impl core::marker::Sync for Can {}
impl Can {
    #[doc = "CAN"]
    #[inline(always)]
    pub fn can(self) -> [can::CanCan; 2] {
        unsafe {
            [
                can::CanCan(self.0.add(0x0usize + 0x0usize)),
                can::CanCan(self.0.add(0x0usize + 0x4usize)),
            ]
        }
    }
}
pub mod can {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "CAN"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CanCan(pub(super) *mut u8);
    unsafe impl core::marker::Send for CanCan {}
    unsafe impl core::marker::Sync for CanCan {}
    impl CanCan {
        #[doc = "CAN0 Service Request 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn canxinty(
            &self,
        ) -> [crate::common::Reg<can_can::CaNxInTy_SPEC, crate::common::RW>; 16] {
            unsafe {
                [
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x8usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0xcusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x10usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x14usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x18usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x1cusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x20usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x24usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x28usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x2cusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x30usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x34usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x38usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x3cusize)),
                ]
            }
        }
    }
    pub mod can_can {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct CaNxInTy_SPEC;
        impl crate::sealed::RegSpec for CaNxInTy_SPEC {
            type DataType = u32;
        }
        #[doc = "CAN0 Service Request 0\n resetvalue={Application Reset:0x0}"]
        pub type CaNxInTy = crate::RegValueT<CaNxInTy_SPEC>;

        impl CaNxInTy {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, CaNxInTy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, CaNxInTy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, CaNxInTy_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,CaNxInTy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, CaNxInTy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, CaNxInTy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, CaNxInTy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, CaNxInTy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, CaNxInTy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,CaNxInTy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, CaNxInTy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,CaNxInTy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, CaNxInTy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,CaNxInTy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, CaNxInTy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,CaNxInTy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, CaNxInTy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,CaNxInTy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, CaNxInTy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,CaNxInTy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, CaNxInTy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,CaNxInTy_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for CaNxInTy {
            #[inline(always)]
            fn default() -> CaNxInTy {
                <crate::RegValueT<CaNxInTy_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "CCU6"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccu6(pub(super) *mut u8);
unsafe impl core::marker::Send for Ccu6 {}
unsafe impl core::marker::Sync for Ccu6 {}
impl Ccu6 {
    #[doc = "CCU"]
    #[inline(always)]
    pub fn ccu(self) -> [ccu6::Ccu6Ccu; 2] {
        unsafe {
            [
                ccu6::Ccu6Ccu(self.0.add(0x0usize + 0x0usize)),
                ccu6::Ccu6Ccu(self.0.add(0x0usize + 0x4usize)),
            ]
        }
    }
}
pub mod ccu6 {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "CCU"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccu6Ccu(pub(super) *mut u8);
    unsafe impl core::marker::Send for Ccu6Ccu {}
    unsafe impl core::marker::Sync for Ccu6Ccu {}
    impl Ccu6Ccu {
        #[doc = "CCU0 Service Request 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ccu6xsry(
            &self,
        ) -> [crate::common::Reg<ccu6_ccu::Ccu6XSRy_SPEC, crate::common::RW>; 4] {
            unsafe {
                [
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x8usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0xcusize)),
                ]
            }
        }
    }
    pub mod ccu6_ccu {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Ccu6XSRy_SPEC;
        impl crate::sealed::RegSpec for Ccu6XSRy_SPEC {
            type DataType = u32;
        }
        #[doc = "CCU0 Service Request 0\n resetvalue={Application Reset:0x0}"]
        pub type Ccu6XSRy = crate::RegValueT<Ccu6XSRy_SPEC>;

        impl Ccu6XSRy {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Ccu6XSRy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, Ccu6XSRy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, Ccu6XSRy_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,Ccu6XSRy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Ccu6XSRy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, Ccu6XSRy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Ccu6XSRy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, Ccu6XSRy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, Ccu6XSRy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,Ccu6XSRy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, Ccu6XSRy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,Ccu6XSRy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, Ccu6XSRy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,Ccu6XSRy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, Ccu6XSRy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,Ccu6XSRy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, Ccu6XSRy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,Ccu6XSRy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, Ccu6XSRy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,Ccu6XSRy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, Ccu6XSRy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,Ccu6XSRy_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for Ccu6XSRy {
            #[inline(always)]
            fn default() -> Ccu6XSRy {
                <crate::RegValueT<Ccu6XSRy_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "CERBERUS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cerberus(pub(super) *mut u8);
unsafe impl core::marker::Send for Cerberus {}
unsafe impl core::marker::Sync for Cerberus {}
impl Cerberus {
    #[doc = "CERBERUS"]
    #[inline(always)]
    pub fn cerberus(self) -> cerberus::CerberusCerberus {
        unsafe { cerberus::CerberusCerberus(self.0.add(0usize)) }
    }
}
pub mod cerberus {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "CERBERUS"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CerberusCerberus(pub(super) *mut u8);
    unsafe impl core::marker::Send for CerberusCerberus {}
    unsafe impl core::marker::Sync for CerberusCerberus {}
    impl CerberusCerberus {
        #[doc = "Cerberus Service Request 0\n resetvalue={Debug Reset:0x0}"]
        #[inline(always)]
        pub const fn cerberusy(
            &self,
        ) -> [crate::common::Reg<cerberus_cerberus::CerberuSy_SPEC, crate::common::RW>; 2] {
            unsafe {
                [
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
                ]
            }
        }
    }
    pub mod cerberus_cerberus {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct CerberuSy_SPEC;
        impl crate::sealed::RegSpec for CerberuSy_SPEC {
            type DataType = u32;
        }
        #[doc = "Cerberus Service Request 0\n resetvalue={Debug Reset:0x0}"]
        pub type CerberuSy = crate::RegValueT<CerberuSy_SPEC>;

        impl CerberuSy {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, CerberuSy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, CerberuSy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, CerberuSy_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,CerberuSy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, CerberuSy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, CerberuSy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, CerberuSy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, CerberuSy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, CerberuSy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,CerberuSy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, CerberuSy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,CerberuSy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, CerberuSy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,CerberuSy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, CerberuSy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,CerberuSy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, CerberuSy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,CerberuSy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, CerberuSy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,CerberuSy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, CerberuSy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,CerberuSy_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for CerberuSy {
            #[inline(always)]
            fn default() -> CerberuSy {
                <crate::RegValueT<CerberuSy_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "CPU"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu(pub(super) *mut u8);
unsafe impl core::marker::Send for Cpu {}
unsafe impl core::marker::Sync for Cpu {}
impl Cpu {
    #[doc = "CPU"]
    #[inline(always)]
    pub fn cpu(self) -> [cpu::CpuCpu; 3] {
        unsafe {
            [
                cpu::CpuCpu(self.0.add(0x0usize + 0x0usize)),
                cpu::CpuCpu(self.0.add(0x0usize + 0x4usize)),
                cpu::CpuCpu(self.0.add(0x0usize + 0x8usize)),
            ]
        }
    }
}
pub mod cpu {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "CPU"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpuCpu(pub(super) *mut u8);
    unsafe impl core::marker::Send for CpuCpu {}
    unsafe impl core::marker::Sync for CpuCpu {}
    impl CpuCpu {
        #[doc = "CPU0 Software Breakpoint Service Request\n resetvalue={Debug Reset:0x0}"]
        #[inline(always)]
        pub const fn cpuxsb(&self) -> crate::common::Reg<cpu_cpu::CpUxSb_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
    }
    pub mod cpu_cpu {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct CpUxSb_SPEC;
        impl crate::sealed::RegSpec for CpUxSb_SPEC {
            type DataType = u32;
        }
        #[doc = "CPU0 Software Breakpoint Service Request\n resetvalue={Debug Reset:0x0}"]
        pub type CpUxSb = crate::RegValueT<CpUxSb_SPEC>;

        impl CpUxSb {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, CpUxSb_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, CpUxSb_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, CpUxSb_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,CpUxSb_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, CpUxSb_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, CpUxSb_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, CpUxSb_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, CpUxSb_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, CpUxSb_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,CpUxSb_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, CpUxSb_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,CpUxSb_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, CpUxSb_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,CpUxSb_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, CpUxSb_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,CpUxSb_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, CpUxSb_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,CpUxSb_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, CpUxSb_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,CpUxSb_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, CpUxSb_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,CpUxSb_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for CpUxSb {
            #[inline(always)]
            fn default() -> CpUxSb {
                <crate::RegValueT<CpUxSb_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "DAM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dam(pub(super) *mut u8);
unsafe impl core::marker::Send for Dam {}
unsafe impl core::marker::Sync for Dam {}
impl Dam {
    #[doc = "dAM"]
    #[inline(always)]
    pub fn dam(self) -> dam::DamDAm {
        unsafe { dam::DamDAm(self.0.add(0usize)) }
    }
}
pub mod dam {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "dAM"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DamDAm(pub(super) *mut u8);
    unsafe impl core::marker::Send for DamDAm {}
    unsafe impl core::marker::Sync for DamDAm {}
    impl DamDAm {
        #[doc = "DAM0 DMA Ready Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn damxdr(&self) -> crate::common::Reg<dam_dam::DaMxDr_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "DAM0 Error Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn damxerr(
            &self,
        ) -> crate::common::Reg<dam_dam::DaMxErr_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "DAM0 Limit 0 Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn damxli0(
            &self,
        ) -> crate::common::Reg<dam_dam::DaMxLi0_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "DAM0 Limit 1 Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn damxli1(
            &self,
        ) -> crate::common::Reg<dam_dam::DaMxLi1_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "DAM0 Ready 0 Service Reques\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn damxri0(
            &self,
        ) -> crate::common::Reg<dam_dam::DaMxRi0_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "DAM0 Ready 1 Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn damxri1(
            &self,
        ) -> crate::common::Reg<dam_dam::DaMxRi1_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
        }
    }
    pub mod dam_dam {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DaMxDr_SPEC;
        impl crate::sealed::RegSpec for DaMxDr_SPEC {
            type DataType = u32;
        }
        #[doc = "DAM0 DMA Ready Service Request\n resetvalue={Application Reset:0x0}"]
        pub type DaMxDr = crate::RegValueT<DaMxDr_SPEC>;

        impl DaMxDr {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, DaMxDr_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, DaMxDr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, DaMxDr_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,DaMxDr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, DaMxDr_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, DaMxDr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, DaMxDr_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, DaMxDr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, DaMxDr_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,DaMxDr_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, DaMxDr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,DaMxDr_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, DaMxDr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,DaMxDr_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, DaMxDr_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,DaMxDr_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, DaMxDr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,DaMxDr_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, DaMxDr_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,DaMxDr_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, DaMxDr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,DaMxDr_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for DaMxDr {
            #[inline(always)]
            fn default() -> DaMxDr {
                <crate::RegValueT<DaMxDr_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DaMxErr_SPEC;
        impl crate::sealed::RegSpec for DaMxErr_SPEC {
            type DataType = u32;
        }
        #[doc = "DAM0 Error Service Request\n resetvalue={Application Reset:0x0}"]
        pub type DaMxErr = crate::RegValueT<DaMxErr_SPEC>;

        impl DaMxErr {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, DaMxErr_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, DaMxErr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, DaMxErr_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,DaMxErr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, DaMxErr_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, DaMxErr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, DaMxErr_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, DaMxErr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, DaMxErr_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,DaMxErr_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, DaMxErr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,DaMxErr_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, DaMxErr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,DaMxErr_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, DaMxErr_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,DaMxErr_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, DaMxErr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,DaMxErr_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, DaMxErr_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,DaMxErr_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, DaMxErr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,DaMxErr_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for DaMxErr {
            #[inline(always)]
            fn default() -> DaMxErr {
                <crate::RegValueT<DaMxErr_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DaMxLi0_SPEC;
        impl crate::sealed::RegSpec for DaMxLi0_SPEC {
            type DataType = u32;
        }
        #[doc = "DAM0 Limit 0 Service Request\n resetvalue={Application Reset:0x0}"]
        pub type DaMxLi0 = crate::RegValueT<DaMxLi0_SPEC>;

        impl DaMxLi0 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, DaMxLi0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, DaMxLi0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, DaMxLi0_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,DaMxLi0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, DaMxLi0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, DaMxLi0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, DaMxLi0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, DaMxLi0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, DaMxLi0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,DaMxLi0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, DaMxLi0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,DaMxLi0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, DaMxLi0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,DaMxLi0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, DaMxLi0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,DaMxLi0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, DaMxLi0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,DaMxLi0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, DaMxLi0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,DaMxLi0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, DaMxLi0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,DaMxLi0_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for DaMxLi0 {
            #[inline(always)]
            fn default() -> DaMxLi0 {
                <crate::RegValueT<DaMxLi0_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DaMxLi1_SPEC;
        impl crate::sealed::RegSpec for DaMxLi1_SPEC {
            type DataType = u32;
        }
        #[doc = "DAM0 Limit 1 Service Request\n resetvalue={Application Reset:0x0}"]
        pub type DaMxLi1 = crate::RegValueT<DaMxLi1_SPEC>;

        impl DaMxLi1 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, DaMxLi1_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, DaMxLi1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, DaMxLi1_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,DaMxLi1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, DaMxLi1_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, DaMxLi1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, DaMxLi1_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, DaMxLi1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, DaMxLi1_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,DaMxLi1_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, DaMxLi1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,DaMxLi1_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, DaMxLi1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,DaMxLi1_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, DaMxLi1_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,DaMxLi1_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, DaMxLi1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,DaMxLi1_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, DaMxLi1_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,DaMxLi1_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, DaMxLi1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,DaMxLi1_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for DaMxLi1 {
            #[inline(always)]
            fn default() -> DaMxLi1 {
                <crate::RegValueT<DaMxLi1_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DaMxRi0_SPEC;
        impl crate::sealed::RegSpec for DaMxRi0_SPEC {
            type DataType = u32;
        }
        #[doc = "DAM0 Ready 0 Service Reques\n resetvalue={Application Reset:0x0}"]
        pub type DaMxRi0 = crate::RegValueT<DaMxRi0_SPEC>;

        impl DaMxRi0 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, DaMxRi0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, DaMxRi0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, DaMxRi0_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,DaMxRi0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, DaMxRi0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, DaMxRi0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, DaMxRi0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, DaMxRi0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, DaMxRi0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,DaMxRi0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, DaMxRi0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,DaMxRi0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, DaMxRi0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,DaMxRi0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, DaMxRi0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,DaMxRi0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, DaMxRi0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,DaMxRi0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, DaMxRi0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,DaMxRi0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, DaMxRi0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,DaMxRi0_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for DaMxRi0 {
            #[inline(always)]
            fn default() -> DaMxRi0 {
                <crate::RegValueT<DaMxRi0_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DaMxRi1_SPEC;
        impl crate::sealed::RegSpec for DaMxRi1_SPEC {
            type DataType = u32;
        }
        #[doc = "DAM0 Ready 1 Service Request\n resetvalue={Application Reset:0x0}"]
        pub type DaMxRi1 = crate::RegValueT<DaMxRi1_SPEC>;

        impl DaMxRi1 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, DaMxRi1_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, DaMxRi1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, DaMxRi1_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,DaMxRi1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, DaMxRi1_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, DaMxRi1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, DaMxRi1_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, DaMxRi1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, DaMxRi1_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,DaMxRi1_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, DaMxRi1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,DaMxRi1_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, DaMxRi1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,DaMxRi1_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, DaMxRi1_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,DaMxRi1_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, DaMxRi1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,DaMxRi1_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, DaMxRi1_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,DaMxRi1_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, DaMxRi1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,DaMxRi1_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for DaMxRi1 {
            #[inline(always)]
            fn default() -> DaMxRi1 {
                <crate::RegValueT<DaMxRi1_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "DMA"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma(pub(super) *mut u8);
unsafe impl core::marker::Send for Dma {}
unsafe impl core::marker::Sync for Dma {}
impl Dma {
    #[doc = "DMA"]
    #[inline(always)]
    pub fn dma(self) -> dma::DmaDma {
        unsafe { dma::DmaDma(self.0.add(0usize)) }
    }
}
pub mod dma {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "DMA"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaDma(pub(super) *mut u8);
    unsafe impl core::marker::Send for DmaDma {}
    unsafe impl core::marker::Sync for DmaDma {}
    impl DmaDma {
        #[doc = "DMA Channel  0 Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn dmachy(
            &self,
        ) -> [crate::common::Reg<dma_dma::DmacHy_SPEC, crate::common::RW>; 128] {
            unsafe {
                [
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x4usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x8usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xcusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x10usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x14usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x18usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1cusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x20usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x24usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x28usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x2cusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x30usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x34usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x38usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x3cusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x40usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x44usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x48usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x4cusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x50usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x54usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x58usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x5cusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x60usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x64usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x68usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x6cusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x70usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x74usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x78usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x7cusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x80usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x84usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x88usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x8cusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x90usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x94usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x98usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x9cusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xa0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xa4usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xa8usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xacusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xb0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xb4usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xb8usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xbcusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xc0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xc4usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xc8usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xccusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xd0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xd4usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xd8usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xdcusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xe0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xe4usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xe8usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xecusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xf0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xf4usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xf8usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xfcusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x100usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x104usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x108usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x10cusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x110usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x114usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x118usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x11cusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x120usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x124usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x128usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x12cusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x130usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x134usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x138usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x13cusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x140usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x144usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x148usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x14cusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x150usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x154usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x158usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x15cusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x160usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x164usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x168usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x16cusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x170usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x174usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x178usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x17cusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x180usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x184usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x188usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x18cusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x190usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x194usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x198usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x19cusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1a0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1a4usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1a8usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1acusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1b0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1b4usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1b8usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1bcusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1c0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1c4usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1c8usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1ccusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1d0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1d4usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1d8usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1dcusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1e0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1e4usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1e8usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1ecusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1f0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1f4usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1f8usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1fcusize)),
                ]
            }
        }
        #[doc = "DMA Error Service Request 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn dmaerry(
            &self,
        ) -> [crate::common::Reg<dma_dma::DmaerRy_SPEC, crate::common::RW>; 4] {
            unsafe {
                [
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x8usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0xcusize)),
                ]
            }
        }
    }
    pub mod dma_dma {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmacHy_SPEC;
        impl crate::sealed::RegSpec for DmacHy_SPEC {
            type DataType = u32;
        }
        #[doc = "DMA Channel  0 Service Request\n resetvalue={Application Reset:0x0}"]
        pub type DmacHy = crate::RegValueT<DmacHy_SPEC>;

        impl DmacHy {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, DmacHy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, DmacHy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, DmacHy_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,DmacHy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, DmacHy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, DmacHy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, DmacHy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, DmacHy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, DmacHy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,DmacHy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, DmacHy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,DmacHy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, DmacHy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,DmacHy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, DmacHy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,DmacHy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, DmacHy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,DmacHy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, DmacHy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,DmacHy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, DmacHy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,DmacHy_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for DmacHy {
            #[inline(always)]
            fn default() -> DmacHy {
                <crate::RegValueT<DmacHy_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DmaerRy_SPEC;
        impl crate::sealed::RegSpec for DmaerRy_SPEC {
            type DataType = u32;
        }
        #[doc = "DMA Error Service Request 0\n resetvalue={Application Reset:0x0}"]
        pub type DmaerRy = crate::RegValueT<DmaerRy_SPEC>;

        impl DmaerRy {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, DmaerRy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, DmaerRy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, DmaerRy_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,DmaerRy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, DmaerRy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, DmaerRy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, DmaerRy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, DmaerRy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, DmaerRy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,DmaerRy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, DmaerRy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,DmaerRy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, DmaerRy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,DmaerRy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, DmaerRy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,DmaerRy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, DmaerRy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,DmaerRy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, DmaerRy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,DmaerRy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, DmaerRy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,DmaerRy_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for DmaerRy {
            #[inline(always)]
            fn default() -> DmaerRy {
                <crate::RegValueT<DmaerRy_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "DSADC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsadc(pub(super) *mut u8);
unsafe impl core::marker::Send for Dsadc {}
unsafe impl core::marker::Sync for Dsadc {}
impl Dsadc {
    #[doc = "DSADC"]
    #[inline(always)]
    pub fn dsadc(self) -> [dsadc::DsadcDsadc; 6] {
        unsafe {
            [
                dsadc::DsadcDsadc(self.0.add(0x0usize + 0x0usize)),
                dsadc::DsadcDsadc(self.0.add(0x0usize + 0x8usize)),
                dsadc::DsadcDsadc(self.0.add(0x0usize + 0x10usize)),
                dsadc::DsadcDsadc(self.0.add(0x0usize + 0x18usize)),
                dsadc::DsadcDsadc(self.0.add(0x0usize + 0x20usize)),
                dsadc::DsadcDsadc(self.0.add(0x0usize + 0x28usize)),
            ]
        }
    }
}
pub mod dsadc {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "DSADC"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DsadcDsadc(pub(super) *mut u8);
    unsafe impl core::marker::Send for DsadcDsadc {}
    unsafe impl core::marker::Sync for DsadcDsadc {}
    impl DsadcDsadc {
        #[doc = "DSADC SRA0 Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn dsadcsrax(
            &self,
        ) -> crate::common::Reg<dsadc_dsadc::DsadcsrAx_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "DSADC SRM0 Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn dsadcsrmx(
            &self,
        ) -> crate::common::Reg<dsadc_dsadc::DsadcsrMx_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
    }
    pub mod dsadc_dsadc {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DsadcsrAx_SPEC;
        impl crate::sealed::RegSpec for DsadcsrAx_SPEC {
            type DataType = u32;
        }
        #[doc = "DSADC SRA0 Service Request\n resetvalue={Application Reset:0x0}"]
        pub type DsadcsrAx = crate::RegValueT<DsadcsrAx_SPEC>;

        impl DsadcsrAx {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, DsadcsrAx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, DsadcsrAx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, DsadcsrAx_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,DsadcsrAx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, DsadcsrAx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, DsadcsrAx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, DsadcsrAx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, DsadcsrAx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, DsadcsrAx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,DsadcsrAx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, DsadcsrAx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,DsadcsrAx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, DsadcsrAx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,DsadcsrAx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, DsadcsrAx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,DsadcsrAx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, DsadcsrAx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,DsadcsrAx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, DsadcsrAx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,DsadcsrAx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, DsadcsrAx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,DsadcsrAx_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for DsadcsrAx {
            #[inline(always)]
            fn default() -> DsadcsrAx {
                <crate::RegValueT<DsadcsrAx_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct DsadcsrMx_SPEC;
        impl crate::sealed::RegSpec for DsadcsrMx_SPEC {
            type DataType = u32;
        }
        #[doc = "DSADC SRM0 Service Request\n resetvalue={Application Reset:0x0}"]
        pub type DsadcsrMx = crate::RegValueT<DsadcsrMx_SPEC>;

        impl DsadcsrMx {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, DsadcsrMx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, DsadcsrMx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, DsadcsrMx_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,DsadcsrMx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, DsadcsrMx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, DsadcsrMx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, DsadcsrMx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, DsadcsrMx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, DsadcsrMx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,DsadcsrMx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, DsadcsrMx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,DsadcsrMx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, DsadcsrMx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,DsadcsrMx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, DsadcsrMx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,DsadcsrMx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, DsadcsrMx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,DsadcsrMx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, DsadcsrMx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,DsadcsrMx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, DsadcsrMx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,DsadcsrMx_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for DsadcsrMx {
            #[inline(always)]
            fn default() -> DsadcsrMx {
                <crate::RegValueT<DsadcsrMx_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "ERAY"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eray(pub(super) *mut u8);
unsafe impl core::marker::Send for Eray {}
unsafe impl core::marker::Sync for Eray {}
impl Eray {
    #[doc = "ERAY"]
    #[inline(always)]
    pub fn eray(self) -> eray::ErayEray {
        unsafe { eray::ErayEray(self.0.add(0usize)) }
    }
}
pub mod eray {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "ERAY"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ErayEray(pub(super) *mut u8);
    unsafe impl core::marker::Send for ErayEray {}
    unsafe impl core::marker::Sync for ErayEray {}
    impl ErayEray {
        #[doc = "E RAY 0 Input Buffer Busy\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn erayxibusy(
            &self,
        ) -> crate::common::Reg<eray_eray::EraYxIbusy_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
        }
        #[doc = "E RAY 0 Service Request 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn erayxint0(
            &self,
        ) -> crate::common::Reg<eray_eray::EraYxInt0_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "E RAY 0 Service Request 1\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn erayxint1(
            &self,
        ) -> crate::common::Reg<eray_eray::EraYxInt1_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "E RAY 0 Message Buffer Status Changed 0 Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn erayxmbsc0(
            &self,
        ) -> crate::common::Reg<eray_eray::EraYxMbsc0_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
        }
        #[doc = "E RAY 0 Message Buffer Status Changed 1 Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn erayxmbsc1(
            &self,
        ) -> crate::common::Reg<eray_eray::EraYxMbsc1_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
        }
        #[doc = "E RAY 0 New Data 0 Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn erayxndat0(
            &self,
        ) -> crate::common::Reg<eray_eray::EraYxNdat0_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "E RAY 0 New Data 1 Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn erayxndat1(
            &self,
        ) -> crate::common::Reg<eray_eray::EraYxNdat1_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "E RAY 0 Output Buffer Busy\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn erayxobusy(
            &self,
        ) -> crate::common::Reg<eray_eray::EraYxObusy_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
        }
        #[doc = "E RAY 0 Timer Interrupt 0 Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn erayxtint0(
            &self,
        ) -> crate::common::Reg<eray_eray::EraYxTint0_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "E RAY 0 Timer Interrupt 1 Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn erayxtint1(
            &self,
        ) -> crate::common::Reg<eray_eray::EraYxTint1_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
        }
    }
    pub mod eray_eray {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct EraYxIbusy_SPEC;
        impl crate::sealed::RegSpec for EraYxIbusy_SPEC {
            type DataType = u32;
        }
        #[doc = "E RAY 0 Input Buffer Busy\n resetvalue={Application Reset:0x0}"]
        pub type EraYxIbusy = crate::RegValueT<EraYxIbusy_SPEC>;

        impl EraYxIbusy {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, EraYxIbusy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, EraYxIbusy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, EraYxIbusy_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,EraYxIbusy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, EraYxIbusy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, EraYxIbusy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, EraYxIbusy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, EraYxIbusy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, EraYxIbusy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,EraYxIbusy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, EraYxIbusy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,EraYxIbusy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, EraYxIbusy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,EraYxIbusy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, EraYxIbusy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,EraYxIbusy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, EraYxIbusy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,EraYxIbusy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, EraYxIbusy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,EraYxIbusy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, EraYxIbusy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,EraYxIbusy_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for EraYxIbusy {
            #[inline(always)]
            fn default() -> EraYxIbusy {
                <crate::RegValueT<EraYxIbusy_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct EraYxInt0_SPEC;
        impl crate::sealed::RegSpec for EraYxInt0_SPEC {
            type DataType = u32;
        }
        #[doc = "E RAY 0 Service Request 0\n resetvalue={Application Reset:0x0}"]
        pub type EraYxInt0 = crate::RegValueT<EraYxInt0_SPEC>;

        impl EraYxInt0 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, EraYxInt0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, EraYxInt0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, EraYxInt0_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,EraYxInt0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, EraYxInt0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, EraYxInt0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, EraYxInt0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, EraYxInt0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, EraYxInt0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,EraYxInt0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, EraYxInt0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,EraYxInt0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, EraYxInt0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,EraYxInt0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, EraYxInt0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,EraYxInt0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, EraYxInt0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,EraYxInt0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, EraYxInt0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,EraYxInt0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, EraYxInt0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,EraYxInt0_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for EraYxInt0 {
            #[inline(always)]
            fn default() -> EraYxInt0 {
                <crate::RegValueT<EraYxInt0_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct EraYxInt1_SPEC;
        impl crate::sealed::RegSpec for EraYxInt1_SPEC {
            type DataType = u32;
        }
        #[doc = "E RAY 0 Service Request 1\n resetvalue={Application Reset:0x0}"]
        pub type EraYxInt1 = crate::RegValueT<EraYxInt1_SPEC>;

        impl EraYxInt1 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, EraYxInt1_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, EraYxInt1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, EraYxInt1_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,EraYxInt1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, EraYxInt1_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, EraYxInt1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, EraYxInt1_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, EraYxInt1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, EraYxInt1_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,EraYxInt1_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, EraYxInt1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,EraYxInt1_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, EraYxInt1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,EraYxInt1_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, EraYxInt1_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,EraYxInt1_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, EraYxInt1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,EraYxInt1_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, EraYxInt1_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,EraYxInt1_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, EraYxInt1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,EraYxInt1_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for EraYxInt1 {
            #[inline(always)]
            fn default() -> EraYxInt1 {
                <crate::RegValueT<EraYxInt1_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct EraYxMbsc0_SPEC;
        impl crate::sealed::RegSpec for EraYxMbsc0_SPEC {
            type DataType = u32;
        }
        #[doc = "E RAY 0 Message Buffer Status Changed 0 Service Request\n resetvalue={Application Reset:0x0}"]
        pub type EraYxMbsc0 = crate::RegValueT<EraYxMbsc0_SPEC>;

        impl EraYxMbsc0 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, EraYxMbsc0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, EraYxMbsc0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, EraYxMbsc0_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,EraYxMbsc0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, EraYxMbsc0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, EraYxMbsc0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, EraYxMbsc0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, EraYxMbsc0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, EraYxMbsc0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,EraYxMbsc0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, EraYxMbsc0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,EraYxMbsc0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, EraYxMbsc0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,EraYxMbsc0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, EraYxMbsc0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,EraYxMbsc0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, EraYxMbsc0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,EraYxMbsc0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, EraYxMbsc0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,EraYxMbsc0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, EraYxMbsc0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,EraYxMbsc0_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for EraYxMbsc0 {
            #[inline(always)]
            fn default() -> EraYxMbsc0 {
                <crate::RegValueT<EraYxMbsc0_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct EraYxMbsc1_SPEC;
        impl crate::sealed::RegSpec for EraYxMbsc1_SPEC {
            type DataType = u32;
        }
        #[doc = "E RAY 0 Message Buffer Status Changed 1 Service Request\n resetvalue={Application Reset:0x0}"]
        pub type EraYxMbsc1 = crate::RegValueT<EraYxMbsc1_SPEC>;

        impl EraYxMbsc1 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, EraYxMbsc1_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, EraYxMbsc1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, EraYxMbsc1_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,EraYxMbsc1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, EraYxMbsc1_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, EraYxMbsc1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, EraYxMbsc1_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, EraYxMbsc1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, EraYxMbsc1_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,EraYxMbsc1_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, EraYxMbsc1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,EraYxMbsc1_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, EraYxMbsc1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,EraYxMbsc1_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, EraYxMbsc1_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,EraYxMbsc1_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, EraYxMbsc1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,EraYxMbsc1_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, EraYxMbsc1_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,EraYxMbsc1_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, EraYxMbsc1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,EraYxMbsc1_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for EraYxMbsc1 {
            #[inline(always)]
            fn default() -> EraYxMbsc1 {
                <crate::RegValueT<EraYxMbsc1_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct EraYxNdat0_SPEC;
        impl crate::sealed::RegSpec for EraYxNdat0_SPEC {
            type DataType = u32;
        }
        #[doc = "E RAY 0 New Data 0 Service Request\n resetvalue={Application Reset:0x0}"]
        pub type EraYxNdat0 = crate::RegValueT<EraYxNdat0_SPEC>;

        impl EraYxNdat0 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, EraYxNdat0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, EraYxNdat0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, EraYxNdat0_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,EraYxNdat0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, EraYxNdat0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, EraYxNdat0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, EraYxNdat0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, EraYxNdat0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, EraYxNdat0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,EraYxNdat0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, EraYxNdat0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,EraYxNdat0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, EraYxNdat0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,EraYxNdat0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, EraYxNdat0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,EraYxNdat0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, EraYxNdat0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,EraYxNdat0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, EraYxNdat0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,EraYxNdat0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, EraYxNdat0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,EraYxNdat0_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for EraYxNdat0 {
            #[inline(always)]
            fn default() -> EraYxNdat0 {
                <crate::RegValueT<EraYxNdat0_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct EraYxNdat1_SPEC;
        impl crate::sealed::RegSpec for EraYxNdat1_SPEC {
            type DataType = u32;
        }
        #[doc = "E RAY 0 New Data 1 Service Request\n resetvalue={Application Reset:0x0}"]
        pub type EraYxNdat1 = crate::RegValueT<EraYxNdat1_SPEC>;

        impl EraYxNdat1 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, EraYxNdat1_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, EraYxNdat1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, EraYxNdat1_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,EraYxNdat1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, EraYxNdat1_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, EraYxNdat1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, EraYxNdat1_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, EraYxNdat1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, EraYxNdat1_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,EraYxNdat1_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, EraYxNdat1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,EraYxNdat1_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, EraYxNdat1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,EraYxNdat1_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, EraYxNdat1_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,EraYxNdat1_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, EraYxNdat1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,EraYxNdat1_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, EraYxNdat1_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,EraYxNdat1_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, EraYxNdat1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,EraYxNdat1_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for EraYxNdat1 {
            #[inline(always)]
            fn default() -> EraYxNdat1 {
                <crate::RegValueT<EraYxNdat1_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct EraYxObusy_SPEC;
        impl crate::sealed::RegSpec for EraYxObusy_SPEC {
            type DataType = u32;
        }
        #[doc = "E RAY 0 Output Buffer Busy\n resetvalue={Application Reset:0x0}"]
        pub type EraYxObusy = crate::RegValueT<EraYxObusy_SPEC>;

        impl EraYxObusy {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, EraYxObusy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, EraYxObusy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, EraYxObusy_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,EraYxObusy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, EraYxObusy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, EraYxObusy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, EraYxObusy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, EraYxObusy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, EraYxObusy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,EraYxObusy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, EraYxObusy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,EraYxObusy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, EraYxObusy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,EraYxObusy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, EraYxObusy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,EraYxObusy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, EraYxObusy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,EraYxObusy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, EraYxObusy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,EraYxObusy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, EraYxObusy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,EraYxObusy_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for EraYxObusy {
            #[inline(always)]
            fn default() -> EraYxObusy {
                <crate::RegValueT<EraYxObusy_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct EraYxTint0_SPEC;
        impl crate::sealed::RegSpec for EraYxTint0_SPEC {
            type DataType = u32;
        }
        #[doc = "E RAY 0 Timer Interrupt 0 Service Request\n resetvalue={Application Reset:0x0}"]
        pub type EraYxTint0 = crate::RegValueT<EraYxTint0_SPEC>;

        impl EraYxTint0 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, EraYxTint0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, EraYxTint0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, EraYxTint0_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,EraYxTint0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, EraYxTint0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, EraYxTint0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, EraYxTint0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, EraYxTint0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, EraYxTint0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,EraYxTint0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, EraYxTint0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,EraYxTint0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, EraYxTint0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,EraYxTint0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, EraYxTint0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,EraYxTint0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, EraYxTint0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,EraYxTint0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, EraYxTint0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,EraYxTint0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, EraYxTint0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,EraYxTint0_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for EraYxTint0 {
            #[inline(always)]
            fn default() -> EraYxTint0 {
                <crate::RegValueT<EraYxTint0_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct EraYxTint1_SPEC;
        impl crate::sealed::RegSpec for EraYxTint1_SPEC {
            type DataType = u32;
        }
        #[doc = "E RAY 0 Timer Interrupt 1 Service Request\n resetvalue={Application Reset:0x0}"]
        pub type EraYxTint1 = crate::RegValueT<EraYxTint1_SPEC>;

        impl EraYxTint1 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, EraYxTint1_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, EraYxTint1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, EraYxTint1_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,EraYxTint1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, EraYxTint1_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, EraYxTint1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, EraYxTint1_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, EraYxTint1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, EraYxTint1_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,EraYxTint1_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, EraYxTint1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,EraYxTint1_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, EraYxTint1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,EraYxTint1_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, EraYxTint1_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,EraYxTint1_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, EraYxTint1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,EraYxTint1_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, EraYxTint1_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,EraYxTint1_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, EraYxTint1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,EraYxTint1_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for EraYxTint1 {
            #[inline(always)]
            fn default() -> EraYxTint1 {
                <crate::RegValueT<EraYxTint1_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "FCE"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fce(pub(super) *mut u8);
unsafe impl core::marker::Send for Fce {}
unsafe impl core::marker::Sync for Fce {}
impl Fce {
    #[doc = "FCE0"]
    #[inline(always)]
    pub fn fce0(self) -> fce::FceFce0 {
        unsafe { fce::FceFce0(self.0.add(0usize)) }
    }
}
pub mod fce {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "FCE0"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FceFce0(pub(super) *mut u8);
    unsafe impl core::marker::Send for FceFce0 {}
    unsafe impl core::marker::Sync for FceFce0 {}
    impl FceFce0 {
        #[doc = "FCE0 Error Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn fce0(&self) -> crate::common::Reg<fce_fce0::Fce0_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
    }
    pub mod fce_fce0 {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Fce0_SPEC;
        impl crate::sealed::RegSpec for Fce0_SPEC {
            type DataType = u32;
        }
        #[doc = "FCE0 Error Service Request\n resetvalue={Application Reset:0x0}"]
        pub type Fce0 = crate::RegValueT<Fce0_SPEC>;

        impl Fce0 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Fce0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, Fce0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, Fce0_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,Fce0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Fce0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, Fce0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Fce0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, Fce0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, Fce0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,Fce0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, Fce0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,Fce0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, Fce0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,Fce0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, Fce0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,Fce0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, Fce0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,Fce0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, Fce0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,Fce0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, Fce0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,Fce0_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for Fce0 {
            #[inline(always)]
            fn default() -> Fce0 {
                <crate::RegValueT<Fce0_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "GETH"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Geth(pub(super) *mut u8);
unsafe impl core::marker::Send for Geth {}
unsafe impl core::marker::Sync for Geth {}
impl Geth {
    #[doc = "GETH"]
    #[inline(always)]
    pub fn geth(self) -> geth::GethGeth {
        unsafe { geth::GethGeth(self.0.add(0usize)) }
    }
}
pub mod geth {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "GETH"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GethGeth(pub(super) *mut u8);
    unsafe impl core::marker::Send for GethGeth {}
    unsafe impl core::marker::Sync for GethGeth {}
    impl GethGeth {
        #[doc = "GETH Service Request 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn gethy(
            &self,
        ) -> [crate::common::Reg<geth_geth::GetHy_SPEC, crate::common::RW>; 10] {
            unsafe {
                [
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x8usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0xcusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x10usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x14usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x18usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x1cusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x20usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x24usize)),
                ]
            }
        }
    }
    pub mod geth_geth {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GetHy_SPEC;
        impl crate::sealed::RegSpec for GetHy_SPEC {
            type DataType = u32;
        }
        #[doc = "GETH Service Request 0\n resetvalue={Application Reset:0x0}"]
        pub type GetHy = crate::RegValueT<GetHy_SPEC>;

        impl GetHy {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, GetHy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, GetHy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, GetHy_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,GetHy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, GetHy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, GetHy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, GetHy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, GetHy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, GetHy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,GetHy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, GetHy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,GetHy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, GetHy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,GetHy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, GetHy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,GetHy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, GetHy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,GetHy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, GetHy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,GetHy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, GetHy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,GetHy_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for GetHy {
            #[inline(always)]
            fn default() -> GetHy {
                <crate::RegValueT<GetHy_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "GPSR"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpsr(pub(super) *mut u8);
unsafe impl core::marker::Send for Gpsr {}
unsafe impl core::marker::Sync for Gpsr {}
impl Gpsr {
    #[doc = "GPSR"]
    #[inline(always)]
    pub fn gpsr(self) -> [gpsr::GpsrGpsr; 3] {
        unsafe {
            [
                gpsr::GpsrGpsr(self.0.add(0x0usize + 0x0usize)),
                gpsr::GpsrGpsr(self.0.add(0x0usize + 0x4usize)),
                gpsr::GpsrGpsr(self.0.add(0x0usize + 0x8usize)),
            ]
        }
    }
}
pub mod gpsr {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "GPSR"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GpsrGpsr(pub(super) *mut u8);
    unsafe impl core::marker::Send for GpsrGpsr {}
    unsafe impl core::marker::Sync for GpsrGpsr {}
    impl GpsrGpsr {
        #[doc = "General Purpose Group 0 Service Request 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn gpsrxy(
            &self,
        ) -> [crate::common::Reg<gpsr_gpsr::GpsRxy_SPEC, crate::common::RW>; 8] {
            unsafe {
                [
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x8usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0xcusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x10usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x14usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x18usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x1cusize)),
                ]
            }
        }
    }
    pub mod gpsr_gpsr {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GpsRxy_SPEC;
        impl crate::sealed::RegSpec for GpsRxy_SPEC {
            type DataType = u32;
        }
        #[doc = "General Purpose Group 0 Service Request 0\n resetvalue={Application Reset:0x0}"]
        pub type GpsRxy = crate::RegValueT<GpsRxy_SPEC>;

        impl GpsRxy {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, GpsRxy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, GpsRxy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, GpsRxy_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,GpsRxy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, GpsRxy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, GpsRxy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, GpsRxy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, GpsRxy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, GpsRxy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,GpsRxy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, GpsRxy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,GpsRxy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, GpsRxy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,GpsRxy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, GpsRxy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,GpsRxy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, GpsRxy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,GpsRxy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, GpsRxy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,GpsRxy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, GpsRxy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,GpsRxy_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for GpsRxy {
            #[inline(always)]
            fn default() -> GpsRxy {
                <crate::RegValueT<GpsRxy_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "GPT12"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt12(pub(super) *mut u8);
unsafe impl core::marker::Send for Gpt12 {}
unsafe impl core::marker::Sync for Gpt12 {}
impl Gpt12 {
    #[doc = "GPT12"]
    #[inline(always)]
    pub fn gpt12(self) -> gpt12::Gpt12Gpt12 {
        unsafe { gpt12::Gpt12Gpt12(self.0.add(0usize)) }
    }
}
pub mod gpt12 {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "GPT12"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gpt12Gpt12(pub(super) *mut u8);
    unsafe impl core::marker::Send for Gpt12Gpt12 {}
    unsafe impl core::marker::Sync for Gpt12Gpt12 {}
    impl Gpt12Gpt12 {
        #[doc = "GPT120 CAPREL Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn gpt120cirq(
            &self,
        ) -> crate::common::Reg<gpt12_gpt12::Gpt120Cirq_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "GPT120 Timer 2 Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn gpt120t2(
            &self,
        ) -> crate::common::Reg<gpt12_gpt12::Gpt120T2_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "GPT120 Timer 3 Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn gpt120t3(
            &self,
        ) -> crate::common::Reg<gpt12_gpt12::Gpt120T3_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "GPT120 Timer 4 Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn gpt120t4(
            &self,
        ) -> crate::common::Reg<gpt12_gpt12::Gpt120T4_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "GPT120 Timer 5 Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn gpt120t5(
            &self,
        ) -> crate::common::Reg<gpt12_gpt12::Gpt120T5_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "GPT120 Timer 6 Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn gpt120t6(
            &self,
        ) -> crate::common::Reg<gpt12_gpt12::Gpt120T6_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
        }
    }
    pub mod gpt12_gpt12 {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Gpt120Cirq_SPEC;
        impl crate::sealed::RegSpec for Gpt120Cirq_SPEC {
            type DataType = u32;
        }
        #[doc = "GPT120 CAPREL Service Request\n resetvalue={Application Reset:0x0}"]
        pub type Gpt120Cirq = crate::RegValueT<Gpt120Cirq_SPEC>;

        impl Gpt120Cirq {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Gpt120Cirq_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, Gpt120Cirq_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, Gpt120Cirq_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,Gpt120Cirq_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Gpt120Cirq_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, Gpt120Cirq_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Gpt120Cirq_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, Gpt120Cirq_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, Gpt120Cirq_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,Gpt120Cirq_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, Gpt120Cirq_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,Gpt120Cirq_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, Gpt120Cirq_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,Gpt120Cirq_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, Gpt120Cirq_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,Gpt120Cirq_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, Gpt120Cirq_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,Gpt120Cirq_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, Gpt120Cirq_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,Gpt120Cirq_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, Gpt120Cirq_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,Gpt120Cirq_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for Gpt120Cirq {
            #[inline(always)]
            fn default() -> Gpt120Cirq {
                <crate::RegValueT<Gpt120Cirq_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Gpt120T2_SPEC;
        impl crate::sealed::RegSpec for Gpt120T2_SPEC {
            type DataType = u32;
        }
        #[doc = "GPT120 Timer 2 Service Request\n resetvalue={Application Reset:0x0}"]
        pub type Gpt120T2 = crate::RegValueT<Gpt120T2_SPEC>;

        impl Gpt120T2 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Gpt120T2_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, Gpt120T2_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, Gpt120T2_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,Gpt120T2_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Gpt120T2_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, Gpt120T2_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Gpt120T2_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, Gpt120T2_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, Gpt120T2_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,Gpt120T2_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, Gpt120T2_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,Gpt120T2_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, Gpt120T2_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,Gpt120T2_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, Gpt120T2_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,Gpt120T2_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, Gpt120T2_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,Gpt120T2_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, Gpt120T2_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,Gpt120T2_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, Gpt120T2_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,Gpt120T2_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for Gpt120T2 {
            #[inline(always)]
            fn default() -> Gpt120T2 {
                <crate::RegValueT<Gpt120T2_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Gpt120T3_SPEC;
        impl crate::sealed::RegSpec for Gpt120T3_SPEC {
            type DataType = u32;
        }
        #[doc = "GPT120 Timer 3 Service Request\n resetvalue={Application Reset:0x0}"]
        pub type Gpt120T3 = crate::RegValueT<Gpt120T3_SPEC>;

        impl Gpt120T3 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Gpt120T3_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, Gpt120T3_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, Gpt120T3_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,Gpt120T3_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Gpt120T3_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, Gpt120T3_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Gpt120T3_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, Gpt120T3_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, Gpt120T3_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,Gpt120T3_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, Gpt120T3_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,Gpt120T3_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, Gpt120T3_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,Gpt120T3_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, Gpt120T3_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,Gpt120T3_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, Gpt120T3_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,Gpt120T3_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, Gpt120T3_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,Gpt120T3_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, Gpt120T3_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,Gpt120T3_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for Gpt120T3 {
            #[inline(always)]
            fn default() -> Gpt120T3 {
                <crate::RegValueT<Gpt120T3_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Gpt120T4_SPEC;
        impl crate::sealed::RegSpec for Gpt120T4_SPEC {
            type DataType = u32;
        }
        #[doc = "GPT120 Timer 4 Service Request\n resetvalue={Application Reset:0x0}"]
        pub type Gpt120T4 = crate::RegValueT<Gpt120T4_SPEC>;

        impl Gpt120T4 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Gpt120T4_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, Gpt120T4_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, Gpt120T4_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,Gpt120T4_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Gpt120T4_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, Gpt120T4_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Gpt120T4_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, Gpt120T4_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, Gpt120T4_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,Gpt120T4_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, Gpt120T4_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,Gpt120T4_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, Gpt120T4_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,Gpt120T4_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, Gpt120T4_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,Gpt120T4_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, Gpt120T4_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,Gpt120T4_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, Gpt120T4_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,Gpt120T4_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, Gpt120T4_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,Gpt120T4_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for Gpt120T4 {
            #[inline(always)]
            fn default() -> Gpt120T4 {
                <crate::RegValueT<Gpt120T4_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Gpt120T5_SPEC;
        impl crate::sealed::RegSpec for Gpt120T5_SPEC {
            type DataType = u32;
        }
        #[doc = "GPT120 Timer 5 Service Request\n resetvalue={Application Reset:0x0}"]
        pub type Gpt120T5 = crate::RegValueT<Gpt120T5_SPEC>;

        impl Gpt120T5 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Gpt120T5_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, Gpt120T5_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, Gpt120T5_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,Gpt120T5_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Gpt120T5_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, Gpt120T5_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Gpt120T5_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, Gpt120T5_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, Gpt120T5_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,Gpt120T5_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, Gpt120T5_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,Gpt120T5_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, Gpt120T5_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,Gpt120T5_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, Gpt120T5_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,Gpt120T5_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, Gpt120T5_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,Gpt120T5_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, Gpt120T5_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,Gpt120T5_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, Gpt120T5_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,Gpt120T5_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for Gpt120T5 {
            #[inline(always)]
            fn default() -> Gpt120T5 {
                <crate::RegValueT<Gpt120T5_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Gpt120T6_SPEC;
        impl crate::sealed::RegSpec for Gpt120T6_SPEC {
            type DataType = u32;
        }
        #[doc = "GPT120 Timer 6 Service Request\n resetvalue={Application Reset:0x0}"]
        pub type Gpt120T6 = crate::RegValueT<Gpt120T6_SPEC>;

        impl Gpt120T6 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Gpt120T6_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, Gpt120T6_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, Gpt120T6_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,Gpt120T6_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Gpt120T6_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, Gpt120T6_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Gpt120T6_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, Gpt120T6_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, Gpt120T6_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,Gpt120T6_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, Gpt120T6_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,Gpt120T6_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, Gpt120T6_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,Gpt120T6_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, Gpt120T6_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,Gpt120T6_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, Gpt120T6_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,Gpt120T6_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, Gpt120T6_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,Gpt120T6_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, Gpt120T6_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,Gpt120T6_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for Gpt120T6 {
            #[inline(always)]
            fn default() -> Gpt120T6 {
                <crate::RegValueT<Gpt120T6_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "ATOM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GtmatoMwx(pub(super) *mut u8);
unsafe impl core::marker::Send for GtmatoMwx {}
unsafe impl core::marker::Sync for GtmatoMwx {}
impl GtmatoMwx {
    #[doc = "ATOM0 Shared Service Request 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gtmatomwx_(
        &self,
    ) -> [crate::common::Reg<gtmatomwx::GtmatoMwx_SPEC, crate::common::RW>; 4] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0xcusize)),
            ]
        }
    }
}
pub mod gtmatomwx {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GtmatoMwx_SPEC;
    impl crate::sealed::RegSpec for GtmatoMwx_SPEC {
        type DataType = u32;
    }
    #[doc = "ATOM0 Shared Service Request 0\n resetvalue={Application Reset:0x0}"]
    pub type GtmatoMwx = crate::RegValueT<GtmatoMwx_SPEC>;

    impl GtmatoMwx {
        #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
        #[inline(always)]
        pub fn srpn(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, GtmatoMwx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xff,1,0,u8, GtmatoMwx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Enable"]
        #[inline(always)]
        pub fn sre(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, GtmatoMwx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,GtmatoMwx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
        #[inline(always)]
        pub fn tos(
            self,
        ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, GtmatoMwx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<11,0x7,1,0,u8, GtmatoMwx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
        #[inline(always)]
        pub fn ecc(
            self,
        ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, GtmatoMwx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1f,1,0,u8, GtmatoMwx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
        #[inline(always)]
        pub fn srr(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, GtmatoMwx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<24,1,0,GtmatoMwx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
        #[inline(always)]
        pub fn clrr(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, GtmatoMwx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<25,1,0,GtmatoMwx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
        #[inline(always)]
        pub fn setr(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, GtmatoMwx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<26,1,0,GtmatoMwx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
        #[inline(always)]
        pub fn iov(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, GtmatoMwx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<27,1,0,GtmatoMwx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
        #[inline(always)]
        pub fn iovclr(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, GtmatoMwx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<28,1,0,GtmatoMwx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
        #[inline(always)]
        pub fn sws(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, GtmatoMwx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<29,1,0,GtmatoMwx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
        #[inline(always)]
        pub fn swsclr(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, GtmatoMwx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<30,1,0,GtmatoMwx_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for GtmatoMwx {
        #[inline(always)]
        fn default() -> GtmatoMwx {
            <crate::RegValueT<GtmatoMwx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "MCS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GtmmcSwx(pub(super) *mut u8);
unsafe impl core::marker::Send for GtmmcSwx {}
unsafe impl core::marker::Sync for GtmmcSwx {}
impl GtmmcSwx {
    #[doc = "MCS0 Shared Service Request 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gtmmcswx_(
        &self,
    ) -> [crate::common::Reg<gtmmcswx::GtmmcSwx_SPEC, crate::common::RW>; 8] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x1cusize)),
            ]
        }
    }
}
pub mod gtmmcswx {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GtmmcSwx_SPEC;
    impl crate::sealed::RegSpec for GtmmcSwx_SPEC {
        type DataType = u32;
    }
    #[doc = "MCS0 Shared Service Request 0\n resetvalue={Application Reset:0x0}"]
    pub type GtmmcSwx = crate::RegValueT<GtmmcSwx_SPEC>;

    impl GtmmcSwx {
        #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
        #[inline(always)]
        pub fn srpn(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, GtmmcSwx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xff,1,0,u8, GtmmcSwx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Enable"]
        #[inline(always)]
        pub fn sre(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, GtmmcSwx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,GtmmcSwx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
        #[inline(always)]
        pub fn tos(
            self,
        ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, GtmmcSwx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<11,0x7,1,0,u8, GtmmcSwx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
        #[inline(always)]
        pub fn ecc(
            self,
        ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, GtmmcSwx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1f,1,0,u8, GtmmcSwx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
        #[inline(always)]
        pub fn srr(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, GtmmcSwx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<24,1,0,GtmmcSwx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
        #[inline(always)]
        pub fn clrr(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, GtmmcSwx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<25,1,0,GtmmcSwx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
        #[inline(always)]
        pub fn setr(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, GtmmcSwx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<26,1,0,GtmmcSwx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
        #[inline(always)]
        pub fn iov(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, GtmmcSwx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<27,1,0,GtmmcSwx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
        #[inline(always)]
        pub fn iovclr(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, GtmmcSwx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<28,1,0,GtmmcSwx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
        #[inline(always)]
        pub fn sws(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, GtmmcSwx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<29,1,0,GtmmcSwx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
        #[inline(always)]
        pub fn swsclr(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, GtmmcSwx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<30,1,0,GtmmcSwx_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for GtmmcSwx {
        #[inline(always)]
        fn default() -> GtmmcSwx {
            <crate::RegValueT<GtmmcSwx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "TIM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GtmtiMwx(pub(super) *mut u8);
unsafe impl core::marker::Send for GtmtiMwx {}
unsafe impl core::marker::Sync for GtmtiMwx {}
impl GtmtiMwx {
    #[doc = "TIM0 Shared Service Request 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gtmtimwx_(
        &self,
    ) -> [crate::common::Reg<gtmtimwx::GtmtiMwx_SPEC, crate::common::RW>; 8] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x1cusize)),
            ]
        }
    }
}
pub mod gtmtimwx {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GtmtiMwx_SPEC;
    impl crate::sealed::RegSpec for GtmtiMwx_SPEC {
        type DataType = u32;
    }
    #[doc = "TIM0 Shared Service Request 0\n resetvalue={Application Reset:0x0}"]
    pub type GtmtiMwx = crate::RegValueT<GtmtiMwx_SPEC>;

    impl GtmtiMwx {
        #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
        #[inline(always)]
        pub fn srpn(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, GtmtiMwx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xff,1,0,u8, GtmtiMwx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Enable"]
        #[inline(always)]
        pub fn sre(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, GtmtiMwx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,GtmtiMwx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
        #[inline(always)]
        pub fn tos(
            self,
        ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, GtmtiMwx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<11,0x7,1,0,u8, GtmtiMwx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
        #[inline(always)]
        pub fn ecc(
            self,
        ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, GtmtiMwx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1f,1,0,u8, GtmtiMwx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
        #[inline(always)]
        pub fn srr(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, GtmtiMwx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<24,1,0,GtmtiMwx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
        #[inline(always)]
        pub fn clrr(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, GtmtiMwx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<25,1,0,GtmtiMwx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
        #[inline(always)]
        pub fn setr(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, GtmtiMwx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<26,1,0,GtmtiMwx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
        #[inline(always)]
        pub fn iov(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, GtmtiMwx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<27,1,0,GtmtiMwx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
        #[inline(always)]
        pub fn iovclr(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, GtmtiMwx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<28,1,0,GtmtiMwx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
        #[inline(always)]
        pub fn sws(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, GtmtiMwx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<29,1,0,GtmtiMwx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
        #[inline(always)]
        pub fn swsclr(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, GtmtiMwx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<30,1,0,GtmtiMwx_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for GtmtiMwx {
        #[inline(always)]
        fn default() -> GtmtiMwx {
            <crate::RegValueT<GtmtiMwx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "TOM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GtmtoMwx(pub(super) *mut u8);
unsafe impl core::marker::Send for GtmtoMwx {}
unsafe impl core::marker::Sync for GtmtoMwx {}
impl GtmtoMwx {
    #[doc = "TOM0 Shared Service Request 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gtmtomwx_(
        &self,
    ) -> [crate::common::Reg<gtmtomwx::GtmtoMwx_SPEC, crate::common::RW>; 8] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x1cusize)),
            ]
        }
    }
}
pub mod gtmtomwx {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GtmtoMwx_SPEC;
    impl crate::sealed::RegSpec for GtmtoMwx_SPEC {
        type DataType = u32;
    }
    #[doc = "TOM0 Shared Service Request 0\n resetvalue={Application Reset:0x0}"]
    pub type GtmtoMwx = crate::RegValueT<GtmtoMwx_SPEC>;

    impl GtmtoMwx {
        #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
        #[inline(always)]
        pub fn srpn(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, GtmtoMwx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xff,1,0,u8, GtmtoMwx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Enable"]
        #[inline(always)]
        pub fn sre(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, GtmtoMwx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,GtmtoMwx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
        #[inline(always)]
        pub fn tos(
            self,
        ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, GtmtoMwx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<11,0x7,1,0,u8, GtmtoMwx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
        #[inline(always)]
        pub fn ecc(
            self,
        ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, GtmtoMwx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1f,1,0,u8, GtmtoMwx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
        #[inline(always)]
        pub fn srr(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, GtmtoMwx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<24,1,0,GtmtoMwx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
        #[inline(always)]
        pub fn clrr(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, GtmtoMwx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<25,1,0,GtmtoMwx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
        #[inline(always)]
        pub fn setr(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, GtmtoMwx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<26,1,0,GtmtoMwx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
        #[inline(always)]
        pub fn iov(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, GtmtoMwx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<27,1,0,GtmtoMwx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
        #[inline(always)]
        pub fn iovclr(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, GtmtoMwx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<28,1,0,GtmtoMwx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
        #[inline(always)]
        pub fn sws(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, GtmtoMwx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<29,1,0,GtmtoMwx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
        #[inline(always)]
        pub fn swsclr(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, GtmtoMwx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<30,1,0,GtmtoMwx_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for GtmtoMwx {
        #[inline(always)]
        fn default() -> GtmtoMwx {
            <crate::RegValueT<GtmtoMwx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "HSCT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsct(pub(super) *mut u8);
unsafe impl core::marker::Send for Hsct {}
unsafe impl core::marker::Sync for Hsct {}
impl Hsct {
    #[doc = "HSCT"]
    #[inline(always)]
    pub fn hsct(self) -> hsct::HsctHsct {
        unsafe { hsct::HsctHsct(self.0.add(0usize)) }
    }
}
pub mod hsct {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "HSCT"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HsctHsct(pub(super) *mut u8);
    unsafe impl core::marker::Send for HsctHsct {}
    unsafe impl core::marker::Sync for HsctHsct {}
    impl HsctHsct {
        #[doc = "HSCT0 Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn hsctx(&self) -> crate::common::Reg<hsct_hsct::HscTx_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
    }
    pub mod hsct_hsct {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct HscTx_SPEC;
        impl crate::sealed::RegSpec for HscTx_SPEC {
            type DataType = u32;
        }
        #[doc = "HSCT0 Service Request\n resetvalue={Application Reset:0x0}"]
        pub type HscTx = crate::RegValueT<HscTx_SPEC>;

        impl HscTx {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, HscTx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, HscTx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, HscTx_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,HscTx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, HscTx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, HscTx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, HscTx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, HscTx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, HscTx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,HscTx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, HscTx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,HscTx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, HscTx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,HscTx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, HscTx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,HscTx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, HscTx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,HscTx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, HscTx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,HscTx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, HscTx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,HscTx_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for HscTx {
            #[inline(always)]
            fn default() -> HscTx {
                <crate::RegValueT<HscTx_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "HSM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsm(pub(super) *mut u8);
unsafe impl core::marker::Send for Hsm {}
unsafe impl core::marker::Sync for Hsm {}
impl Hsm {
    #[doc = "HSM"]
    #[inline(always)]
    pub fn hsm(self) -> hsm::HsmHsm {
        unsafe { hsm::HsmHsm(self.0.add(0usize)) }
    }
}
pub mod hsm {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "HSM"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HsmHsm(pub(super) *mut u8);
    unsafe impl core::marker::Send for HsmHsm {}
    unsafe impl core::marker::Sync for HsmHsm {}
    impl HsmHsm {
        #[doc = "HSM Service Request 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn hsmy(&self) -> [crate::common::Reg<hsm_hsm::HsMy_SPEC, crate::common::RW>; 2] {
            unsafe {
                [
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
                ]
            }
        }
    }
    pub mod hsm_hsm {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct HsMy_SPEC;
        impl crate::sealed::RegSpec for HsMy_SPEC {
            type DataType = u32;
        }
        #[doc = "HSM Service Request 0\n resetvalue={Application Reset:0x0}"]
        pub type HsMy = crate::RegValueT<HsMy_SPEC>;

        impl HsMy {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, HsMy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, HsMy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, HsMy_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,HsMy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, HsMy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, HsMy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, HsMy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, HsMy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, HsMy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,HsMy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, HsMy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,HsMy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, HsMy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,HsMy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, HsMy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,HsMy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, HsMy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,HsMy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, HsMy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,HsMy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, HsMy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,HsMy_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for HsMy {
            #[inline(always)]
            fn default() -> HsMy {
                <crate::RegValueT<HsMy_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "HSSL"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hssl(pub(super) *mut u8);
unsafe impl core::marker::Send for Hssl {}
unsafe impl core::marker::Sync for Hssl {}
impl Hssl {
    #[doc = "HSSL"]
    #[inline(always)]
    pub fn hssl(self) -> hssl::HsslHssl {
        unsafe { hssl::HsslHssl(self.0.add(0usize)) }
    }
}
pub mod hssl {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "HSSL"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HsslHssl(pub(super) *mut u8);
    unsafe impl core::marker::Send for HsslHssl {}
    unsafe impl core::marker::Sync for HsslHssl {}
    impl HsslHssl {
        #[doc = "HSSL0 Exception Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn hsslxexi(
            &self,
        ) -> crate::common::Reg<hssl_hssl::HssLxExi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
        }
        #[doc = "CH"]
        #[inline(always)]
        pub fn ch(self) -> [hssl_hssl::HsslHsslCh; 4] {
            unsafe {
                [
                    hssl_hssl::HsslHsslCh(self.0.add(0x0usize + 0x0usize)),
                    hssl_hssl::HsslHsslCh(self.0.add(0x0usize + 0x10usize)),
                    hssl_hssl::HsslHsslCh(self.0.add(0x0usize + 0x20usize)),
                    hssl_hssl::HsslHsslCh(self.0.add(0x0usize + 0x30usize)),
                ]
            }
        }
    }
    pub mod hssl_hssl {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct HssLxExi_SPEC;
        impl crate::sealed::RegSpec for HssLxExi_SPEC {
            type DataType = u32;
        }
        #[doc = "HSSL0 Exception Service Request\n resetvalue={Application Reset:0x0}"]
        pub type HssLxExi = crate::RegValueT<HssLxExi_SPEC>;

        impl HssLxExi {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, HssLxExi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, HssLxExi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, HssLxExi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,HssLxExi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, HssLxExi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, HssLxExi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, HssLxExi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, HssLxExi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, HssLxExi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,HssLxExi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, HssLxExi_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,HssLxExi_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, HssLxExi_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,HssLxExi_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, HssLxExi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,HssLxExi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, HssLxExi_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,HssLxExi_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, HssLxExi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,HssLxExi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, HssLxExi_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,HssLxExi_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for HssLxExi {
            #[inline(always)]
            fn default() -> HssLxExi {
                <crate::RegValueT<HssLxExi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc = "CH"]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct HsslHsslCh(pub(super) *mut u8);
        unsafe impl core::marker::Send for HsslHsslCh {}
        unsafe impl core::marker::Sync for HsslHsslCh {}
        impl HsslHsslCh {
            #[doc = "HSSL0 Channel 0 OK Service Request\n resetvalue={Application Reset:0x0}"]
            #[inline(always)]
            pub const fn hsslxcoky(
                &self,
            ) -> crate::common::Reg<hssl_hssl_ch::HssLxCoKy_SPEC, crate::common::RW> {
                unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
            }
            #[doc = "HSSL0 Channel 0 Error Service Request\n resetvalue={Application Reset:0x0}"]
            #[inline(always)]
            pub const fn hsslxerry(
                &self,
            ) -> crate::common::Reg<hssl_hssl_ch::HssLxErRy_SPEC, crate::common::RW> {
                unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
            }
            #[doc = "HSSL0 Channel 0 Read Data Service Request\n resetvalue={Application Reset:0x0}"]
            #[inline(always)]
            pub const fn hsslxrdiy(
                &self,
            ) -> crate::common::Reg<hssl_hssl_ch::HssLxRdIy_SPEC, crate::common::RW> {
                unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
            }
            #[doc = "HSSL0 Channel 0 Trigger Interrupt Service Request\n resetvalue={Application Reset:0x0}"]
            #[inline(always)]
            pub const fn hsslxtrgy(
                &self,
            ) -> crate::common::Reg<hssl_hssl_ch::HssLxTrGy_SPEC, crate::common::RW> {
                unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
            }
        }
        pub mod hssl_hssl_ch {
            #[allow(unused_imports)]
            use crate::common::*;
            #[doc(hidden)]
            #[derive(Copy, Clone, Eq, PartialEq)]
            pub struct HssLxCoKy_SPEC;
            impl crate::sealed::RegSpec for HssLxCoKy_SPEC {
                type DataType = u32;
            }
            #[doc = "HSSL0 Channel 0 OK Service Request\n resetvalue={Application Reset:0x0}"]
            pub type HssLxCoKy = crate::RegValueT<HssLxCoKy_SPEC>;

            impl HssLxCoKy {
                #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
                #[inline(always)]
                pub fn srpn(
                    self,
                ) -> crate::common::RegisterField<
                    0,
                    0xff,
                    1,
                    0,
                    u8,
                    HssLxCoKy_SPEC,
                    crate::common::RW,
                > {
                    crate::common::RegisterField::<
                        0,
                        0xff,
                        1,
                        0,
                        u8,
                        HssLxCoKy_SPEC,
                        crate::common::RW,
                    >::from_register(self, 0)
                }
                #[doc = "Service Request Enable"]
                #[inline(always)]
                pub fn sre(
                    self,
                ) -> crate::common::RegisterFieldBool<10, 1, 0, HssLxCoKy_SPEC, crate::common::RW>
                {
                    crate::common::RegisterFieldBool::<10,1,0,HssLxCoKy_SPEC,crate::common::RW>::from_register(self,0)
                }
                #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
                #[inline(always)]
                pub fn tos(
                    self,
                ) -> crate::common::RegisterField<
                    11,
                    0x7,
                    1,
                    0,
                    u8,
                    HssLxCoKy_SPEC,
                    crate::common::RW,
                > {
                    crate::common::RegisterField::<
                        11,
                        0x7,
                        1,
                        0,
                        u8,
                        HssLxCoKy_SPEC,
                        crate::common::RW,
                    >::from_register(self, 0)
                }
                #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
                #[inline(always)]
                pub fn ecc(
                    self,
                ) -> crate::common::RegisterField<
                    16,
                    0x1f,
                    1,
                    0,
                    u8,
                    HssLxCoKy_SPEC,
                    crate::common::RW,
                > {
                    crate::common::RegisterField::<
                        16,
                        0x1f,
                        1,
                        0,
                        u8,
                        HssLxCoKy_SPEC,
                        crate::common::RW,
                    >::from_register(self, 0)
                }
                #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
                #[inline(always)]
                pub fn srr(
                    self,
                ) -> crate::common::RegisterFieldBool<24, 1, 0, HssLxCoKy_SPEC, crate::common::R>
                {
                    crate::common::RegisterFieldBool::<24,1,0,HssLxCoKy_SPEC,crate::common::R>::from_register(self,0)
                }
                #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
                #[inline(always)]
                pub fn clrr(
                    self,
                ) -> crate::common::RegisterFieldBool<25, 1, 0, HssLxCoKy_SPEC, crate::common::W>
                {
                    crate::common::RegisterFieldBool::<25,1,0,HssLxCoKy_SPEC,crate::common::W>::from_register(self,0)
                }
                #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
                #[inline(always)]
                pub fn setr(
                    self,
                ) -> crate::common::RegisterFieldBool<26, 1, 0, HssLxCoKy_SPEC, crate::common::W>
                {
                    crate::common::RegisterFieldBool::<26,1,0,HssLxCoKy_SPEC,crate::common::W>::from_register(self,0)
                }
                #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
                #[inline(always)]
                pub fn iov(
                    self,
                ) -> crate::common::RegisterFieldBool<27, 1, 0, HssLxCoKy_SPEC, crate::common::R>
                {
                    crate::common::RegisterFieldBool::<27,1,0,HssLxCoKy_SPEC,crate::common::R>::from_register(self,0)
                }
                #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
                #[inline(always)]
                pub fn iovclr(
                    self,
                ) -> crate::common::RegisterFieldBool<28, 1, 0, HssLxCoKy_SPEC, crate::common::W>
                {
                    crate::common::RegisterFieldBool::<28,1,0,HssLxCoKy_SPEC,crate::common::W>::from_register(self,0)
                }
                #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
                #[inline(always)]
                pub fn sws(
                    self,
                ) -> crate::common::RegisterFieldBool<29, 1, 0, HssLxCoKy_SPEC, crate::common::R>
                {
                    crate::common::RegisterFieldBool::<29,1,0,HssLxCoKy_SPEC,crate::common::R>::from_register(self,0)
                }
                #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
                #[inline(always)]
                pub fn swsclr(
                    self,
                ) -> crate::common::RegisterFieldBool<30, 1, 0, HssLxCoKy_SPEC, crate::common::W>
                {
                    crate::common::RegisterFieldBool::<30,1,0,HssLxCoKy_SPEC,crate::common::W>::from_register(self,0)
                }
            }
            impl core::default::Default for HssLxCoKy {
                #[inline(always)]
                fn default() -> HssLxCoKy {
                    <crate::RegValueT<HssLxCoKy_SPEC> as RegisterValue<_>>::new(0)
                }
            }

            #[doc(hidden)]
            #[derive(Copy, Clone, Eq, PartialEq)]
            pub struct HssLxErRy_SPEC;
            impl crate::sealed::RegSpec for HssLxErRy_SPEC {
                type DataType = u32;
            }
            #[doc = "HSSL0 Channel 0 Error Service Request\n resetvalue={Application Reset:0x0}"]
            pub type HssLxErRy = crate::RegValueT<HssLxErRy_SPEC>;

            impl HssLxErRy {
                #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
                #[inline(always)]
                pub fn srpn(
                    self,
                ) -> crate::common::RegisterField<
                    0,
                    0xff,
                    1,
                    0,
                    u8,
                    HssLxErRy_SPEC,
                    crate::common::RW,
                > {
                    crate::common::RegisterField::<
                        0,
                        0xff,
                        1,
                        0,
                        u8,
                        HssLxErRy_SPEC,
                        crate::common::RW,
                    >::from_register(self, 0)
                }
                #[doc = "Service Request Enable"]
                #[inline(always)]
                pub fn sre(
                    self,
                ) -> crate::common::RegisterFieldBool<10, 1, 0, HssLxErRy_SPEC, crate::common::RW>
                {
                    crate::common::RegisterFieldBool::<10,1,0,HssLxErRy_SPEC,crate::common::RW>::from_register(self,0)
                }
                #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
                #[inline(always)]
                pub fn tos(
                    self,
                ) -> crate::common::RegisterField<
                    11,
                    0x7,
                    1,
                    0,
                    u8,
                    HssLxErRy_SPEC,
                    crate::common::RW,
                > {
                    crate::common::RegisterField::<
                        11,
                        0x7,
                        1,
                        0,
                        u8,
                        HssLxErRy_SPEC,
                        crate::common::RW,
                    >::from_register(self, 0)
                }
                #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
                #[inline(always)]
                pub fn ecc(
                    self,
                ) -> crate::common::RegisterField<
                    16,
                    0x1f,
                    1,
                    0,
                    u8,
                    HssLxErRy_SPEC,
                    crate::common::RW,
                > {
                    crate::common::RegisterField::<
                        16,
                        0x1f,
                        1,
                        0,
                        u8,
                        HssLxErRy_SPEC,
                        crate::common::RW,
                    >::from_register(self, 0)
                }
                #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
                #[inline(always)]
                pub fn srr(
                    self,
                ) -> crate::common::RegisterFieldBool<24, 1, 0, HssLxErRy_SPEC, crate::common::R>
                {
                    crate::common::RegisterFieldBool::<24,1,0,HssLxErRy_SPEC,crate::common::R>::from_register(self,0)
                }
                #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
                #[inline(always)]
                pub fn clrr(
                    self,
                ) -> crate::common::RegisterFieldBool<25, 1, 0, HssLxErRy_SPEC, crate::common::W>
                {
                    crate::common::RegisterFieldBool::<25,1,0,HssLxErRy_SPEC,crate::common::W>::from_register(self,0)
                }
                #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
                #[inline(always)]
                pub fn setr(
                    self,
                ) -> crate::common::RegisterFieldBool<26, 1, 0, HssLxErRy_SPEC, crate::common::W>
                {
                    crate::common::RegisterFieldBool::<26,1,0,HssLxErRy_SPEC,crate::common::W>::from_register(self,0)
                }
                #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
                #[inline(always)]
                pub fn iov(
                    self,
                ) -> crate::common::RegisterFieldBool<27, 1, 0, HssLxErRy_SPEC, crate::common::R>
                {
                    crate::common::RegisterFieldBool::<27,1,0,HssLxErRy_SPEC,crate::common::R>::from_register(self,0)
                }
                #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
                #[inline(always)]
                pub fn iovclr(
                    self,
                ) -> crate::common::RegisterFieldBool<28, 1, 0, HssLxErRy_SPEC, crate::common::W>
                {
                    crate::common::RegisterFieldBool::<28,1,0,HssLxErRy_SPEC,crate::common::W>::from_register(self,0)
                }
                #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
                #[inline(always)]
                pub fn sws(
                    self,
                ) -> crate::common::RegisterFieldBool<29, 1, 0, HssLxErRy_SPEC, crate::common::R>
                {
                    crate::common::RegisterFieldBool::<29,1,0,HssLxErRy_SPEC,crate::common::R>::from_register(self,0)
                }
                #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
                #[inline(always)]
                pub fn swsclr(
                    self,
                ) -> crate::common::RegisterFieldBool<30, 1, 0, HssLxErRy_SPEC, crate::common::W>
                {
                    crate::common::RegisterFieldBool::<30,1,0,HssLxErRy_SPEC,crate::common::W>::from_register(self,0)
                }
            }
            impl core::default::Default for HssLxErRy {
                #[inline(always)]
                fn default() -> HssLxErRy {
                    <crate::RegValueT<HssLxErRy_SPEC> as RegisterValue<_>>::new(0)
                }
            }

            #[doc(hidden)]
            #[derive(Copy, Clone, Eq, PartialEq)]
            pub struct HssLxRdIy_SPEC;
            impl crate::sealed::RegSpec for HssLxRdIy_SPEC {
                type DataType = u32;
            }
            #[doc = "HSSL0 Channel 0 Read Data Service Request\n resetvalue={Application Reset:0x0}"]
            pub type HssLxRdIy = crate::RegValueT<HssLxRdIy_SPEC>;

            impl HssLxRdIy {
                #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
                #[inline(always)]
                pub fn srpn(
                    self,
                ) -> crate::common::RegisterField<
                    0,
                    0xff,
                    1,
                    0,
                    u8,
                    HssLxRdIy_SPEC,
                    crate::common::RW,
                > {
                    crate::common::RegisterField::<
                        0,
                        0xff,
                        1,
                        0,
                        u8,
                        HssLxRdIy_SPEC,
                        crate::common::RW,
                    >::from_register(self, 0)
                }
                #[doc = "Service Request Enable"]
                #[inline(always)]
                pub fn sre(
                    self,
                ) -> crate::common::RegisterFieldBool<10, 1, 0, HssLxRdIy_SPEC, crate::common::RW>
                {
                    crate::common::RegisterFieldBool::<10,1,0,HssLxRdIy_SPEC,crate::common::RW>::from_register(self,0)
                }
                #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
                #[inline(always)]
                pub fn tos(
                    self,
                ) -> crate::common::RegisterField<
                    11,
                    0x7,
                    1,
                    0,
                    u8,
                    HssLxRdIy_SPEC,
                    crate::common::RW,
                > {
                    crate::common::RegisterField::<
                        11,
                        0x7,
                        1,
                        0,
                        u8,
                        HssLxRdIy_SPEC,
                        crate::common::RW,
                    >::from_register(self, 0)
                }
                #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
                #[inline(always)]
                pub fn ecc(
                    self,
                ) -> crate::common::RegisterField<
                    16,
                    0x1f,
                    1,
                    0,
                    u8,
                    HssLxRdIy_SPEC,
                    crate::common::RW,
                > {
                    crate::common::RegisterField::<
                        16,
                        0x1f,
                        1,
                        0,
                        u8,
                        HssLxRdIy_SPEC,
                        crate::common::RW,
                    >::from_register(self, 0)
                }
                #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
                #[inline(always)]
                pub fn srr(
                    self,
                ) -> crate::common::RegisterFieldBool<24, 1, 0, HssLxRdIy_SPEC, crate::common::R>
                {
                    crate::common::RegisterFieldBool::<24,1,0,HssLxRdIy_SPEC,crate::common::R>::from_register(self,0)
                }
                #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
                #[inline(always)]
                pub fn clrr(
                    self,
                ) -> crate::common::RegisterFieldBool<25, 1, 0, HssLxRdIy_SPEC, crate::common::W>
                {
                    crate::common::RegisterFieldBool::<25,1,0,HssLxRdIy_SPEC,crate::common::W>::from_register(self,0)
                }
                #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
                #[inline(always)]
                pub fn setr(
                    self,
                ) -> crate::common::RegisterFieldBool<26, 1, 0, HssLxRdIy_SPEC, crate::common::W>
                {
                    crate::common::RegisterFieldBool::<26,1,0,HssLxRdIy_SPEC,crate::common::W>::from_register(self,0)
                }
                #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
                #[inline(always)]
                pub fn iov(
                    self,
                ) -> crate::common::RegisterFieldBool<27, 1, 0, HssLxRdIy_SPEC, crate::common::R>
                {
                    crate::common::RegisterFieldBool::<27,1,0,HssLxRdIy_SPEC,crate::common::R>::from_register(self,0)
                }
                #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
                #[inline(always)]
                pub fn iovclr(
                    self,
                ) -> crate::common::RegisterFieldBool<28, 1, 0, HssLxRdIy_SPEC, crate::common::W>
                {
                    crate::common::RegisterFieldBool::<28,1,0,HssLxRdIy_SPEC,crate::common::W>::from_register(self,0)
                }
                #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
                #[inline(always)]
                pub fn sws(
                    self,
                ) -> crate::common::RegisterFieldBool<29, 1, 0, HssLxRdIy_SPEC, crate::common::R>
                {
                    crate::common::RegisterFieldBool::<29,1,0,HssLxRdIy_SPEC,crate::common::R>::from_register(self,0)
                }
                #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
                #[inline(always)]
                pub fn swsclr(
                    self,
                ) -> crate::common::RegisterFieldBool<30, 1, 0, HssLxRdIy_SPEC, crate::common::W>
                {
                    crate::common::RegisterFieldBool::<30,1,0,HssLxRdIy_SPEC,crate::common::W>::from_register(self,0)
                }
            }
            impl core::default::Default for HssLxRdIy {
                #[inline(always)]
                fn default() -> HssLxRdIy {
                    <crate::RegValueT<HssLxRdIy_SPEC> as RegisterValue<_>>::new(0)
                }
            }

            #[doc(hidden)]
            #[derive(Copy, Clone, Eq, PartialEq)]
            pub struct HssLxTrGy_SPEC;
            impl crate::sealed::RegSpec for HssLxTrGy_SPEC {
                type DataType = u32;
            }
            #[doc = "HSSL0 Channel 0 Trigger Interrupt Service Request\n resetvalue={Application Reset:0x0}"]
            pub type HssLxTrGy = crate::RegValueT<HssLxTrGy_SPEC>;

            impl HssLxTrGy {
                #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
                #[inline(always)]
                pub fn srpn(
                    self,
                ) -> crate::common::RegisterField<
                    0,
                    0xff,
                    1,
                    0,
                    u8,
                    HssLxTrGy_SPEC,
                    crate::common::RW,
                > {
                    crate::common::RegisterField::<
                        0,
                        0xff,
                        1,
                        0,
                        u8,
                        HssLxTrGy_SPEC,
                        crate::common::RW,
                    >::from_register(self, 0)
                }
                #[doc = "Service Request Enable"]
                #[inline(always)]
                pub fn sre(
                    self,
                ) -> crate::common::RegisterFieldBool<10, 1, 0, HssLxTrGy_SPEC, crate::common::RW>
                {
                    crate::common::RegisterFieldBool::<10,1,0,HssLxTrGy_SPEC,crate::common::RW>::from_register(self,0)
                }
                #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
                #[inline(always)]
                pub fn tos(
                    self,
                ) -> crate::common::RegisterField<
                    11,
                    0x7,
                    1,
                    0,
                    u8,
                    HssLxTrGy_SPEC,
                    crate::common::RW,
                > {
                    crate::common::RegisterField::<
                        11,
                        0x7,
                        1,
                        0,
                        u8,
                        HssLxTrGy_SPEC,
                        crate::common::RW,
                    >::from_register(self, 0)
                }
                #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
                #[inline(always)]
                pub fn ecc(
                    self,
                ) -> crate::common::RegisterField<
                    16,
                    0x1f,
                    1,
                    0,
                    u8,
                    HssLxTrGy_SPEC,
                    crate::common::RW,
                > {
                    crate::common::RegisterField::<
                        16,
                        0x1f,
                        1,
                        0,
                        u8,
                        HssLxTrGy_SPEC,
                        crate::common::RW,
                    >::from_register(self, 0)
                }
                #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
                #[inline(always)]
                pub fn srr(
                    self,
                ) -> crate::common::RegisterFieldBool<24, 1, 0, HssLxTrGy_SPEC, crate::common::R>
                {
                    crate::common::RegisterFieldBool::<24,1,0,HssLxTrGy_SPEC,crate::common::R>::from_register(self,0)
                }
                #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
                #[inline(always)]
                pub fn clrr(
                    self,
                ) -> crate::common::RegisterFieldBool<25, 1, 0, HssLxTrGy_SPEC, crate::common::W>
                {
                    crate::common::RegisterFieldBool::<25,1,0,HssLxTrGy_SPEC,crate::common::W>::from_register(self,0)
                }
                #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
                #[inline(always)]
                pub fn setr(
                    self,
                ) -> crate::common::RegisterFieldBool<26, 1, 0, HssLxTrGy_SPEC, crate::common::W>
                {
                    crate::common::RegisterFieldBool::<26,1,0,HssLxTrGy_SPEC,crate::common::W>::from_register(self,0)
                }
                #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
                #[inline(always)]
                pub fn iov(
                    self,
                ) -> crate::common::RegisterFieldBool<27, 1, 0, HssLxTrGy_SPEC, crate::common::R>
                {
                    crate::common::RegisterFieldBool::<27,1,0,HssLxTrGy_SPEC,crate::common::R>::from_register(self,0)
                }
                #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
                #[inline(always)]
                pub fn iovclr(
                    self,
                ) -> crate::common::RegisterFieldBool<28, 1, 0, HssLxTrGy_SPEC, crate::common::W>
                {
                    crate::common::RegisterFieldBool::<28,1,0,HssLxTrGy_SPEC,crate::common::W>::from_register(self,0)
                }
                #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
                #[inline(always)]
                pub fn sws(
                    self,
                ) -> crate::common::RegisterFieldBool<29, 1, 0, HssLxTrGy_SPEC, crate::common::R>
                {
                    crate::common::RegisterFieldBool::<29,1,0,HssLxTrGy_SPEC,crate::common::R>::from_register(self,0)
                }
                #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
                #[inline(always)]
                pub fn swsclr(
                    self,
                ) -> crate::common::RegisterFieldBool<30, 1, 0, HssLxTrGy_SPEC, crate::common::W>
                {
                    crate::common::RegisterFieldBool::<30,1,0,HssLxTrGy_SPEC,crate::common::W>::from_register(self,0)
                }
            }
            impl core::default::Default for HssLxTrGy {
                #[inline(always)]
                fn default() -> HssLxTrGy {
                    <crate::RegValueT<HssLxTrGy_SPEC> as RegisterValue<_>>::new(0)
                }
            }
        }
    }
}
#[doc = "I2C"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C(pub(super) *mut u8);
unsafe impl core::marker::Send for I2C {}
unsafe impl core::marker::Sync for I2C {}
impl I2C {
    #[doc = "I2C"]
    #[inline(always)]
    pub fn i2c(self) -> i2c::I2CI2C {
        unsafe { i2c::I2CI2C(self.0.add(0usize)) }
    }
}
pub mod i2c {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "I2C"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I2CI2C(pub(super) *mut u8);
    unsafe impl core::marker::Send for I2CI2C {}
    unsafe impl core::marker::Sync for I2CI2C {}
    impl I2CI2C {
        #[doc = "I2C0 Data Transfer Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn i2cxdtr(
            &self,
        ) -> crate::common::Reg<i2c_i2c::I2CxDtr_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "I2C0 Error Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn i2cxerr(
            &self,
        ) -> crate::common::Reg<i2c_i2c::I2CxErr_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "I2C0 Protocol Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn i2cxp(&self) -> crate::common::Reg<i2c_i2c::I2CxP_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
        }
    }
    pub mod i2c_i2c {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct I2CxDtr_SPEC;
        impl crate::sealed::RegSpec for I2CxDtr_SPEC {
            type DataType = u32;
        }
        #[doc = "I2C0 Data Transfer Request\n resetvalue={Application Reset:0x0}"]
        pub type I2CxDtr = crate::RegValueT<I2CxDtr_SPEC>;

        impl I2CxDtr {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, I2CxDtr_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, I2CxDtr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, I2CxDtr_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,I2CxDtr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, I2CxDtr_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, I2CxDtr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, I2CxDtr_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, I2CxDtr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, I2CxDtr_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,I2CxDtr_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, I2CxDtr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,I2CxDtr_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, I2CxDtr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,I2CxDtr_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, I2CxDtr_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,I2CxDtr_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, I2CxDtr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,I2CxDtr_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, I2CxDtr_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,I2CxDtr_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, I2CxDtr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,I2CxDtr_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for I2CxDtr {
            #[inline(always)]
            fn default() -> I2CxDtr {
                <crate::RegValueT<I2CxDtr_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct I2CxErr_SPEC;
        impl crate::sealed::RegSpec for I2CxErr_SPEC {
            type DataType = u32;
        }
        #[doc = "I2C0 Error Service Request\n resetvalue={Application Reset:0x0}"]
        pub type I2CxErr = crate::RegValueT<I2CxErr_SPEC>;

        impl I2CxErr {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, I2CxErr_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, I2CxErr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, I2CxErr_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,I2CxErr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, I2CxErr_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, I2CxErr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, I2CxErr_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, I2CxErr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, I2CxErr_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,I2CxErr_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, I2CxErr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,I2CxErr_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, I2CxErr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,I2CxErr_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, I2CxErr_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,I2CxErr_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, I2CxErr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,I2CxErr_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, I2CxErr_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,I2CxErr_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, I2CxErr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,I2CxErr_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for I2CxErr {
            #[inline(always)]
            fn default() -> I2CxErr {
                <crate::RegValueT<I2CxErr_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct I2CxP_SPEC;
        impl crate::sealed::RegSpec for I2CxP_SPEC {
            type DataType = u32;
        }
        #[doc = "I2C0 Protocol Service Request\n resetvalue={Application Reset:0x0}"]
        pub type I2CxP = crate::RegValueT<I2CxP_SPEC>;

        impl I2CxP {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, I2CxP_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, I2CxP_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, I2CxP_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,I2CxP_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, I2CxP_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, I2CxP_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, I2CxP_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, I2CxP_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, I2CxP_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,I2CxP_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, I2CxP_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,I2CxP_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, I2CxP_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,I2CxP_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, I2CxP_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,I2CxP_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, I2CxP_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,I2CxP_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, I2CxP_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,I2CxP_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, I2CxP_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,I2CxP_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for I2CxP {
            #[inline(always)]
            fn default() -> I2CxP {
                <crate::RegValueT<I2CxP_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "MSC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msc(pub(super) *mut u8);
unsafe impl core::marker::Send for Msc {}
unsafe impl core::marker::Sync for Msc {}
impl Msc {
    #[doc = "MSC"]
    #[inline(always)]
    pub fn msc(self) -> [msc::MscMsc; 2] {
        unsafe {
            [
                msc::MscMsc(self.0.add(0x0usize + 0x0usize)),
                msc::MscMsc(self.0.add(0x0usize + 0x4usize)),
            ]
        }
    }
}
pub mod msc {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "MSC"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MscMsc(pub(super) *mut u8);
    unsafe impl core::marker::Send for MscMsc {}
    unsafe impl core::marker::Sync for MscMsc {}
    impl MscMsc {
        #[doc = "MSC0 Service Request 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn mscxsry(
            &self,
        ) -> [crate::common::Reg<msc_msc::MsCxSRy_SPEC, crate::common::RW>; 5] {
            unsafe {
                [
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x8usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0xcusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x10usize)),
                ]
            }
        }
    }
    pub mod msc_msc {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct MsCxSRy_SPEC;
        impl crate::sealed::RegSpec for MsCxSRy_SPEC {
            type DataType = u32;
        }
        #[doc = "MSC0 Service Request 0\n resetvalue={Application Reset:0x0}"]
        pub type MsCxSRy = crate::RegValueT<MsCxSRy_SPEC>;

        impl MsCxSRy {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, MsCxSRy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, MsCxSRy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, MsCxSRy_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,MsCxSRy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, MsCxSRy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, MsCxSRy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, MsCxSRy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, MsCxSRy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, MsCxSRy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,MsCxSRy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, MsCxSRy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,MsCxSRy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, MsCxSRy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,MsCxSRy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, MsCxSRy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,MsCxSRy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, MsCxSRy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,MsCxSRy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, MsCxSRy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,MsCxSRy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, MsCxSRy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,MsCxSRy_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for MsCxSRy {
            #[inline(always)]
            fn default() -> MsCxSRy {
                <crate::RegValueT<MsCxSRy_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "PMS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pms(pub(super) *mut u8);
unsafe impl core::marker::Send for Pms {}
unsafe impl core::marker::Sync for Pms {}
impl Pms {
    #[doc = "PMS"]
    #[inline(always)]
    pub fn pms(self) -> [pms::PmsPms; 4] {
        unsafe {
            [
                pms::PmsPms(self.0.add(0x0usize + 0x0usize)),
                pms::PmsPms(self.0.add(0x0usize + 0x4usize)),
                pms::PmsPms(self.0.add(0x0usize + 0x8usize)),
                pms::PmsPms(self.0.add(0x0usize + 0xcusize)),
            ]
        }
    }
}
pub mod pms {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "PMS"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PmsPms(pub(super) *mut u8);
    unsafe impl core::marker::Send for PmsPms {}
    unsafe impl core::marker::Sync for PmsPms {}
    impl PmsPms {
        #[doc = "Power Management System Service Request 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn pmsx(&self) -> crate::common::Reg<pms_pms::PmSx_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
    }
    pub mod pms_pms {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct PmSx_SPEC;
        impl crate::sealed::RegSpec for PmSx_SPEC {
            type DataType = u32;
        }
        #[doc = "Power Management System Service Request 0\n resetvalue={Application Reset:0x0}"]
        pub type PmSx = crate::RegValueT<PmSx_SPEC>;

        impl PmSx {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, PmSx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, PmSx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, PmSx_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,PmSx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, PmSx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, PmSx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, PmSx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, PmSx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, PmSx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,PmSx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, PmSx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,PmSx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, PmSx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,PmSx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, PmSx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,PmSx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, PmSx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,PmSx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, PmSx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,PmSx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, PmSx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,PmSx_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for PmSx {
            #[inline(always)]
            fn default() -> PmSx {
                <crate::RegValueT<PmSx_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "PSI5"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psi5(pub(super) *mut u8);
unsafe impl core::marker::Send for Psi5 {}
unsafe impl core::marker::Sync for Psi5 {}
impl Psi5 {
    #[doc = "PSI5"]
    #[inline(always)]
    pub fn psi5(self) -> psi5::Psi5Psi5 {
        unsafe { psi5::Psi5Psi5(self.0.add(0usize)) }
    }
}
pub mod psi5 {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "PSI5"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psi5Psi5(pub(super) *mut u8);
    unsafe impl core::marker::Send for Psi5Psi5 {}
    unsafe impl core::marker::Sync for Psi5Psi5 {}
    impl Psi5Psi5 {
        #[doc = "PSI5 Service Request 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn psi5y(
            &self,
        ) -> [crate::common::Reg<psi5_psi5::Psi5Y_SPEC, crate::common::RW>; 8] {
            unsafe {
                [
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x8usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0xcusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x10usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x14usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x18usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x1cusize)),
                ]
            }
        }
    }
    pub mod psi5_psi5 {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Psi5Y_SPEC;
        impl crate::sealed::RegSpec for Psi5Y_SPEC {
            type DataType = u32;
        }
        #[doc = "PSI5 Service Request 0\n resetvalue={Application Reset:0x0}"]
        pub type Psi5Y = crate::RegValueT<Psi5Y_SPEC>;

        impl Psi5Y {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Psi5Y_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, Psi5Y_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, Psi5Y_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,Psi5Y_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Psi5Y_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, Psi5Y_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Psi5Y_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, Psi5Y_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, Psi5Y_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,Psi5Y_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, Psi5Y_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,Psi5Y_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, Psi5Y_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,Psi5Y_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, Psi5Y_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,Psi5Y_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, Psi5Y_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,Psi5Y_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, Psi5Y_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,Psi5Y_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, Psi5Y_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,Psi5Y_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for Psi5Y {
            #[inline(always)]
            fn default() -> Psi5Y {
                <crate::RegValueT<Psi5Y_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "PSI5S"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psi5S(pub(super) *mut u8);
unsafe impl core::marker::Send for Psi5S {}
unsafe impl core::marker::Sync for Psi5S {}
impl Psi5S {
    #[doc = "PSI5S"]
    #[inline(always)]
    pub fn psi5s(self) -> psi5s::Psi5SPsi5S {
        unsafe { psi5s::Psi5SPsi5S(self.0.add(0usize)) }
    }
}
pub mod psi5s {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "PSI5S"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psi5SPsi5S(pub(super) *mut u8);
    unsafe impl core::marker::Send for Psi5SPsi5S {}
    unsafe impl core::marker::Sync for Psi5SPsi5S {}
    impl Psi5SPsi5S {
        #[doc = "PSI5 S Service Request 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn psi5sy(
            &self,
        ) -> [crate::common::Reg<psi5s_psi5s::Psi5Sy_SPEC, crate::common::RW>; 8] {
            unsafe {
                [
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x8usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0xcusize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x10usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x14usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x18usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x1cusize)),
                ]
            }
        }
    }
    pub mod psi5s_psi5s {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Psi5Sy_SPEC;
        impl crate::sealed::RegSpec for Psi5Sy_SPEC {
            type DataType = u32;
        }
        #[doc = "PSI5 S Service Request 0\n resetvalue={Application Reset:0x0}"]
        pub type Psi5Sy = crate::RegValueT<Psi5Sy_SPEC>;

        impl Psi5Sy {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Psi5Sy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, Psi5Sy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, Psi5Sy_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,Psi5Sy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Psi5Sy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, Psi5Sy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Psi5Sy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, Psi5Sy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, Psi5Sy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,Psi5Sy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, Psi5Sy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,Psi5Sy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, Psi5Sy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,Psi5Sy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, Psi5Sy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,Psi5Sy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, Psi5Sy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,Psi5Sy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, Psi5Sy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,Psi5Sy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, Psi5Sy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,Psi5Sy_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for Psi5Sy {
            #[inline(always)]
            fn default() -> Psi5Sy {
                <crate::RegValueT<Psi5Sy_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "QSPI"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspi(pub(super) *mut u8);
unsafe impl core::marker::Send for Qspi {}
unsafe impl core::marker::Sync for Qspi {}
impl Qspi {
    #[doc = "QSPI"]
    #[inline(always)]
    pub fn qspi(self) -> [qspi::QspiQspi; 5] {
        unsafe {
            [
                qspi::QspiQspi(self.0.add(0x0usize + 0x0usize)),
                qspi::QspiQspi(self.0.add(0x0usize + 0x14usize)),
                qspi::QspiQspi(self.0.add(0x0usize + 0x28usize)),
                qspi::QspiQspi(self.0.add(0x0usize + 0x3cusize)),
                qspi::QspiQspi(self.0.add(0x0usize + 0x50usize)),
            ]
        }
    }
}
pub mod qspi {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "QSPI"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct QspiQspi(pub(super) *mut u8);
    unsafe impl core::marker::Send for QspiQspi {}
    unsafe impl core::marker::Sync for QspiQspi {}
    impl QspiQspi {
        #[doc = "QSPI0 Error Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn qspixerr(
            &self,
        ) -> crate::common::Reg<qspi_qspi::QspIxErr_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "QSPI0 Phase Transition Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn qspixpt(
            &self,
        ) -> crate::common::Reg<qspi_qspi::QspIxPt_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "QSPI0 Receive Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn qspixrx(
            &self,
        ) -> crate::common::Reg<qspi_qspi::QspIxRx_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "QSPI0 Transmit Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn qspixtx(
            &self,
        ) -> crate::common::Reg<qspi_qspi::QspIxTx_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "QSPI0 User Defined Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn qspixu(
            &self,
        ) -> crate::common::Reg<qspi_qspi::QspIxU_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
        }
    }
    pub mod qspi_qspi {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct QspIxErr_SPEC;
        impl crate::sealed::RegSpec for QspIxErr_SPEC {
            type DataType = u32;
        }
        #[doc = "QSPI0 Error Service Request\n resetvalue={Application Reset:0x0}"]
        pub type QspIxErr = crate::RegValueT<QspIxErr_SPEC>;

        impl QspIxErr {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, QspIxErr_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, QspIxErr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, QspIxErr_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,QspIxErr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, QspIxErr_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, QspIxErr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, QspIxErr_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, QspIxErr_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, QspIxErr_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,QspIxErr_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, QspIxErr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,QspIxErr_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, QspIxErr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,QspIxErr_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, QspIxErr_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,QspIxErr_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, QspIxErr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,QspIxErr_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, QspIxErr_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,QspIxErr_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, QspIxErr_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,QspIxErr_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for QspIxErr {
            #[inline(always)]
            fn default() -> QspIxErr {
                <crate::RegValueT<QspIxErr_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct QspIxPt_SPEC;
        impl crate::sealed::RegSpec for QspIxPt_SPEC {
            type DataType = u32;
        }
        #[doc = "QSPI0 Phase Transition Service Request\n resetvalue={Application Reset:0x0}"]
        pub type QspIxPt = crate::RegValueT<QspIxPt_SPEC>;

        impl QspIxPt {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, QspIxPt_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, QspIxPt_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, QspIxPt_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,QspIxPt_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, QspIxPt_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, QspIxPt_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, QspIxPt_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, QspIxPt_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, QspIxPt_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,QspIxPt_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, QspIxPt_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,QspIxPt_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, QspIxPt_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,QspIxPt_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, QspIxPt_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,QspIxPt_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, QspIxPt_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,QspIxPt_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, QspIxPt_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,QspIxPt_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, QspIxPt_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,QspIxPt_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for QspIxPt {
            #[inline(always)]
            fn default() -> QspIxPt {
                <crate::RegValueT<QspIxPt_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct QspIxRx_SPEC;
        impl crate::sealed::RegSpec for QspIxRx_SPEC {
            type DataType = u32;
        }
        #[doc = "QSPI0 Receive Service Request\n resetvalue={Application Reset:0x0}"]
        pub type QspIxRx = crate::RegValueT<QspIxRx_SPEC>;

        impl QspIxRx {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, QspIxRx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, QspIxRx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, QspIxRx_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,QspIxRx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, QspIxRx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, QspIxRx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, QspIxRx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, QspIxRx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, QspIxRx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,QspIxRx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, QspIxRx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,QspIxRx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, QspIxRx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,QspIxRx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, QspIxRx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,QspIxRx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, QspIxRx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,QspIxRx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, QspIxRx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,QspIxRx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, QspIxRx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,QspIxRx_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for QspIxRx {
            #[inline(always)]
            fn default() -> QspIxRx {
                <crate::RegValueT<QspIxRx_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct QspIxTx_SPEC;
        impl crate::sealed::RegSpec for QspIxTx_SPEC {
            type DataType = u32;
        }
        #[doc = "QSPI0 Transmit Service Request\n resetvalue={Application Reset:0x0}"]
        pub type QspIxTx = crate::RegValueT<QspIxTx_SPEC>;

        impl QspIxTx {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, QspIxTx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, QspIxTx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, QspIxTx_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,QspIxTx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, QspIxTx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, QspIxTx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, QspIxTx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, QspIxTx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, QspIxTx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,QspIxTx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, QspIxTx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,QspIxTx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, QspIxTx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,QspIxTx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, QspIxTx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,QspIxTx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, QspIxTx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,QspIxTx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, QspIxTx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,QspIxTx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, QspIxTx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,QspIxTx_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for QspIxTx {
            #[inline(always)]
            fn default() -> QspIxTx {
                <crate::RegValueT<QspIxTx_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct QspIxU_SPEC;
        impl crate::sealed::RegSpec for QspIxU_SPEC {
            type DataType = u32;
        }
        #[doc = "QSPI0 User Defined Service Request\n resetvalue={Application Reset:0x0}"]
        pub type QspIxU = crate::RegValueT<QspIxU_SPEC>;

        impl QspIxU {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, QspIxU_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, QspIxU_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, QspIxU_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,QspIxU_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, QspIxU_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, QspIxU_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, QspIxU_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, QspIxU_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, QspIxU_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,QspIxU_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, QspIxU_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,QspIxU_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, QspIxU_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,QspIxU_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, QspIxU_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,QspIxU_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, QspIxU_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,QspIxU_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, QspIxU_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,QspIxU_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, QspIxU_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,QspIxU_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for QspIxU {
            #[inline(always)]
            fn default() -> QspIxU {
                <crate::RegValueT<QspIxU_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "SCU"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scu(pub(super) *mut u8);
unsafe impl core::marker::Send for Scu {}
unsafe impl core::marker::Sync for Scu {}
impl Scu {
    #[doc = "SCU ERU Service Request 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn scuerux(&self) -> [crate::common::Reg<scu::ScuerUx_SPEC, crate::common::RW>; 4] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0xcusize)),
            ]
        }
    }
}
pub mod scu {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ScuerUx_SPEC;
    impl crate::sealed::RegSpec for ScuerUx_SPEC {
        type DataType = u32;
    }
    #[doc = "SCU ERU Service Request 0\n resetvalue={Application Reset:0x0}"]
    pub type ScuerUx = crate::RegValueT<ScuerUx_SPEC>;

    impl ScuerUx {
        #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
        #[inline(always)]
        pub fn srpn(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, ScuerUx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xff,1,0,u8, ScuerUx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Enable"]
        #[inline(always)]
        pub fn sre(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, ScuerUx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,ScuerUx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
        #[inline(always)]
        pub fn tos(
            self,
        ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, ScuerUx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<11,0x7,1,0,u8, ScuerUx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
        #[inline(always)]
        pub fn ecc(
            self,
        ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, ScuerUx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1f,1,0,u8, ScuerUx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
        #[inline(always)]
        pub fn srr(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, ScuerUx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<24,1,0,ScuerUx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
        #[inline(always)]
        pub fn clrr(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, ScuerUx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<25,1,0,ScuerUx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
        #[inline(always)]
        pub fn setr(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, ScuerUx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<26,1,0,ScuerUx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
        #[inline(always)]
        pub fn iov(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, ScuerUx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<27,1,0,ScuerUx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
        #[inline(always)]
        pub fn iovclr(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, ScuerUx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<28,1,0,ScuerUx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
        #[inline(always)]
        pub fn sws(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, ScuerUx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<29,1,0,ScuerUx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
        #[inline(always)]
        pub fn swsclr(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, ScuerUx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<30,1,0,ScuerUx_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for ScuerUx {
        #[inline(always)]
        fn default() -> ScuerUx {
            <crate::RegValueT<ScuerUx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "SENT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sent(pub(super) *mut u8);
unsafe impl core::marker::Send for Sent {}
unsafe impl core::marker::Sync for Sent {}
impl Sent {
    #[doc = "SENT"]
    #[inline(always)]
    pub fn sent(self) -> [sent::SentSent; 10] {
        unsafe {
            [
                sent::SentSent(self.0.add(0x0usize + 0x0usize)),
                sent::SentSent(self.0.add(0x0usize + 0x4usize)),
                sent::SentSent(self.0.add(0x0usize + 0x8usize)),
                sent::SentSent(self.0.add(0x0usize + 0xcusize)),
                sent::SentSent(self.0.add(0x0usize + 0x10usize)),
                sent::SentSent(self.0.add(0x0usize + 0x14usize)),
                sent::SentSent(self.0.add(0x0usize + 0x18usize)),
                sent::SentSent(self.0.add(0x0usize + 0x1cusize)),
                sent::SentSent(self.0.add(0x0usize + 0x20usize)),
                sent::SentSent(self.0.add(0x0usize + 0x24usize)),
            ]
        }
    }
}
pub mod sent {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "SENT"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SentSent(pub(super) *mut u8);
    unsafe impl core::marker::Send for SentSent {}
    unsafe impl core::marker::Sync for SentSent {}
    impl SentSent {
        #[doc = "SENT TRIG0 Service Request\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn sentx(&self) -> crate::common::Reg<sent_sent::SenTx_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
    }
    pub mod sent_sent {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SenTx_SPEC;
        impl crate::sealed::RegSpec for SenTx_SPEC {
            type DataType = u32;
        }
        #[doc = "SENT TRIG0 Service Request\n resetvalue={Application Reset:0x0}"]
        pub type SenTx = crate::RegValueT<SenTx_SPEC>;

        impl SenTx {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, SenTx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, SenTx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, SenTx_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,SenTx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, SenTx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, SenTx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, SenTx_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, SenTx_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, SenTx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,SenTx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, SenTx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,SenTx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, SenTx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,SenTx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, SenTx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,SenTx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, SenTx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,SenTx_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, SenTx_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,SenTx_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, SenTx_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,SenTx_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for SenTx {
            #[inline(always)]
            fn default() -> SenTx {
                <crate::RegValueT<SenTx_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "SMU"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smu(pub(super) *mut u8);
unsafe impl core::marker::Send for Smu {}
unsafe impl core::marker::Sync for Smu {}
impl Smu {
    #[doc = "SMU"]
    #[inline(always)]
    pub fn smu(self) -> smu::SmuSmu {
        unsafe { smu::SmuSmu(self.0.add(0usize)) }
    }
}
pub mod smu {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "SMU"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SmuSmu(pub(super) *mut u8);
    unsafe impl core::marker::Send for SmuSmu {}
    unsafe impl core::marker::Sync for SmuSmu {}
    impl SmuSmu {
        #[doc = "SMU Service Request 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn smuy(&self) -> [crate::common::Reg<smu_smu::SmUy_SPEC, crate::common::RW>; 3] {
            unsafe {
                [
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x8usize)),
                ]
            }
        }
    }
    pub mod smu_smu {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct SmUy_SPEC;
        impl crate::sealed::RegSpec for SmUy_SPEC {
            type DataType = u32;
        }
        #[doc = "SMU Service Request 0\n resetvalue={Application Reset:0x0}"]
        pub type SmUy = crate::RegValueT<SmUy_SPEC>;

        impl SmUy {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, SmUy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, SmUy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, SmUy_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,SmUy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, SmUy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, SmUy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, SmUy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, SmUy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, SmUy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,SmUy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, SmUy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,SmUy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, SmUy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,SmUy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, SmUy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,SmUy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, SmUy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,SmUy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, SmUy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,SmUy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, SmUy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,SmUy_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for SmUy {
            #[inline(always)]
            fn default() -> SmUy {
                <crate::RegValueT<SmUy_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "STM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stm(pub(super) *mut u8);
unsafe impl core::marker::Send for Stm {}
unsafe impl core::marker::Sync for Stm {}
impl Stm {
    #[doc = "STM"]
    #[inline(always)]
    pub fn stm(self) -> [stm::StmStm; 3] {
        unsafe {
            [
                stm::StmStm(self.0.add(0x0usize + 0x0usize)),
                stm::StmStm(self.0.add(0x0usize + 0x4usize)),
                stm::StmStm(self.0.add(0x0usize + 0x8usize)),
            ]
        }
    }
}
pub mod stm {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc = "STM"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct StmStm(pub(super) *mut u8);
    unsafe impl core::marker::Send for StmStm {}
    unsafe impl core::marker::Sync for StmStm {}
    impl StmStm {
        #[doc = "System Timer 0 Service Request 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn stmxsry(
            &self,
        ) -> [crate::common::Reg<stm_stm::StMxSRy_SPEC, crate::common::RW>; 2] {
            unsafe {
                [
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
                ]
            }
        }
    }
    pub mod stm_stm {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct StMxSRy_SPEC;
        impl crate::sealed::RegSpec for StMxSRy_SPEC {
            type DataType = u32;
        }
        #[doc = "System Timer 0 Service Request 0\n resetvalue={Application Reset:0x0}"]
        pub type StMxSRy = crate::RegValueT<StMxSRy_SPEC>;

        impl StMxSRy {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, StMxSRy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, StMxSRy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, StMxSRy_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,StMxSRy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, StMxSRy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, StMxSRy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, StMxSRy_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, StMxSRy_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, StMxSRy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,StMxSRy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, StMxSRy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,StMxSRy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, StMxSRy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,StMxSRy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, StMxSRy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,StMxSRy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, StMxSRy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,StMxSRy_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, StMxSRy_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,StMxSRy_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, StMxSRy_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,StMxSRy_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for StMxSRy {
            #[inline(always)]
            fn default() -> StMxSRy {
                <crate::RegValueT<StMxSRy_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "VADC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vadc(pub(super) *mut u8);
unsafe impl core::marker::Send for Vadc {}
unsafe impl core::marker::Sync for Vadc {}
impl Vadc {
    #[doc = "CG"]
    #[inline(always)]
    pub fn cg(self) -> [crate::src::vadc::VadcG; 2] {
        unsafe {
            [
                crate::src::vadc::VadcG(self.0.add(0xe0usize + 0x0usize)),
                crate::src::vadc::VadcG(self.0.add(0xe0usize + 0x4usize)),
            ]
        }
    }
    #[doc = "FC"]
    #[inline(always)]
    pub fn fc(self) -> [vadc::VadcFc; 4] {
        unsafe {
            [
                vadc::VadcFc(self.0.add(0xc0usize + 0x0usize)),
                vadc::VadcFc(self.0.add(0xc0usize + 0x4usize)),
                vadc::VadcFc(self.0.add(0xc0usize + 0x8usize)),
                vadc::VadcFc(self.0.add(0xc0usize + 0xcusize)),
            ]
        }
    }
    #[doc = "G10"]
    #[inline(always)]
    pub fn g10(self) -> crate::src::vadc::VadcG {
        unsafe { crate::src::vadc::VadcG(self.0.add(160usize)) }
    }
    #[doc = "G11"]
    #[inline(always)]
    pub fn g11(self) -> crate::src::vadc::VadcG {
        unsafe { crate::src::vadc::VadcG(self.0.add(176usize)) }
    }
    #[doc = "G8"]
    #[inline(always)]
    pub fn g8(self) -> crate::src::vadc::VadcG {
        unsafe { crate::src::vadc::VadcG(self.0.add(128usize)) }
    }
    #[doc = "G9"]
    #[inline(always)]
    pub fn g9(self) -> crate::src::vadc::VadcG {
        unsafe { crate::src::vadc::VadcG(self.0.add(144usize)) }
    }
    #[doc = "G"]
    #[inline(always)]
    pub fn g(self) -> [vadc::VadcG; 4] {
        unsafe {
            [
                vadc::VadcG(self.0.add(0x0usize + 0x0usize)),
                vadc::VadcG(self.0.add(0x0usize + 0x4usize)),
                vadc::VadcG(self.0.add(0x0usize + 0x8usize)),
                vadc::VadcG(self.0.add(0x0usize + 0xcusize)),
            ]
        }
    }
}
pub mod vadc {
    #[allow(unused_imports)]
    use crate::common::*;

    #[doc = "FC"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VadcFc(pub(super) *mut u8);
    unsafe impl core::marker::Send for VadcFc {}
    unsafe impl core::marker::Sync for VadcFc {}
    impl VadcFc {
        #[doc = "EVADC Fast Compare 0 Service Request SR0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn vadcfcxsr0(
            &self,
        ) -> crate::common::Reg<vadc_fc::VadcfCxSr0_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
    }
    pub mod vadc_fc {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct VadcfCxSr0_SPEC;
        impl crate::sealed::RegSpec for VadcfCxSr0_SPEC {
            type DataType = u32;
        }
        #[doc = "EVADC Fast Compare 0 Service Request SR0\n resetvalue={Application Reset:0x0}"]
        pub type VadcfCxSr0 = crate::RegValueT<VadcfCxSr0_SPEC>;

        impl VadcfCxSr0 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, VadcfCxSr0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, VadcfCxSr0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, VadcfCxSr0_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,VadcfCxSr0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, VadcfCxSr0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, VadcfCxSr0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, VadcfCxSr0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, VadcfCxSr0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, VadcfCxSr0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,VadcfCxSr0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, VadcfCxSr0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,VadcfCxSr0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, VadcfCxSr0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,VadcfCxSr0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, VadcfCxSr0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,VadcfCxSr0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, VadcfCxSr0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,VadcfCxSr0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, VadcfCxSr0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,VadcfCxSr0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, VadcfCxSr0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,VadcfCxSr0_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for VadcfCxSr0 {
            #[inline(always)]
            fn default() -> VadcfCxSr0 {
                <crate::RegValueT<VadcfCxSr0_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }

    #[doc = "G"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VadcG(pub(super) *mut u8);
    unsafe impl core::marker::Send for VadcG {}
    unsafe impl core::marker::Sync for VadcG {}
    impl VadcG {
        #[doc = "EVADC Group 0 Service Request 3\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn vadcg0sr0(
            &self,
        ) -> crate::common::Reg<vadc_g::Vadcg0Sr0_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "EVADC Group 0 Service Request 3\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn vadcg0sr1(
            &self,
        ) -> crate::common::Reg<vadc_g::Vadcg0Sr1_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "EVADC Group 0 Service Request 3\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn vadcg0sr2(
            &self,
        ) -> crate::common::Reg<vadc_g::Vadcg0Sr2_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "EVADC Group 0 Service Request 3\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn vadcg0sr3(
            &self,
        ) -> crate::common::Reg<vadc_g::Vadcg0Sr3_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
        }
    }
    pub mod vadc_g {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Vadcg0Sr0_SPEC;
        impl crate::sealed::RegSpec for Vadcg0Sr0_SPEC {
            type DataType = u32;
        }
        #[doc = "EVADC Group 0 Service Request 3\n resetvalue={Application Reset:0x0}"]
        pub type Vadcg0Sr0 = crate::RegValueT<Vadcg0Sr0_SPEC>;

        impl Vadcg0Sr0 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Vadcg0Sr0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, Vadcg0Sr0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, Vadcg0Sr0_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,Vadcg0Sr0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Vadcg0Sr0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, Vadcg0Sr0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Vadcg0Sr0_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, Vadcg0Sr0_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, Vadcg0Sr0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,Vadcg0Sr0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, Vadcg0Sr0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,Vadcg0Sr0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, Vadcg0Sr0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,Vadcg0Sr0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, Vadcg0Sr0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,Vadcg0Sr0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, Vadcg0Sr0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,Vadcg0Sr0_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, Vadcg0Sr0_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,Vadcg0Sr0_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, Vadcg0Sr0_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,Vadcg0Sr0_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for Vadcg0Sr0 {
            #[inline(always)]
            fn default() -> Vadcg0Sr0 {
                <crate::RegValueT<Vadcg0Sr0_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Vadcg0Sr1_SPEC;
        impl crate::sealed::RegSpec for Vadcg0Sr1_SPEC {
            type DataType = u32;
        }
        #[doc = "EVADC Group 0 Service Request 3\n resetvalue={Application Reset:0x0}"]
        pub type Vadcg0Sr1 = crate::RegValueT<Vadcg0Sr1_SPEC>;

        impl Vadcg0Sr1 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Vadcg0Sr1_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, Vadcg0Sr1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, Vadcg0Sr1_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,Vadcg0Sr1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Vadcg0Sr1_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, Vadcg0Sr1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Vadcg0Sr1_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, Vadcg0Sr1_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, Vadcg0Sr1_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,Vadcg0Sr1_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, Vadcg0Sr1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,Vadcg0Sr1_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, Vadcg0Sr1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,Vadcg0Sr1_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, Vadcg0Sr1_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,Vadcg0Sr1_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, Vadcg0Sr1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,Vadcg0Sr1_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, Vadcg0Sr1_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,Vadcg0Sr1_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, Vadcg0Sr1_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,Vadcg0Sr1_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for Vadcg0Sr1 {
            #[inline(always)]
            fn default() -> Vadcg0Sr1 {
                <crate::RegValueT<Vadcg0Sr1_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Vadcg0Sr2_SPEC;
        impl crate::sealed::RegSpec for Vadcg0Sr2_SPEC {
            type DataType = u32;
        }
        #[doc = "EVADC Group 0 Service Request 3\n resetvalue={Application Reset:0x0}"]
        pub type Vadcg0Sr2 = crate::RegValueT<Vadcg0Sr2_SPEC>;

        impl Vadcg0Sr2 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Vadcg0Sr2_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, Vadcg0Sr2_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, Vadcg0Sr2_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,Vadcg0Sr2_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Vadcg0Sr2_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, Vadcg0Sr2_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Vadcg0Sr2_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, Vadcg0Sr2_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, Vadcg0Sr2_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,Vadcg0Sr2_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, Vadcg0Sr2_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,Vadcg0Sr2_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, Vadcg0Sr2_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,Vadcg0Sr2_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, Vadcg0Sr2_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,Vadcg0Sr2_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, Vadcg0Sr2_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,Vadcg0Sr2_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, Vadcg0Sr2_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,Vadcg0Sr2_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, Vadcg0Sr2_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,Vadcg0Sr2_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for Vadcg0Sr2 {
            #[inline(always)]
            fn default() -> Vadcg0Sr2 {
                <crate::RegValueT<Vadcg0Sr2_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Vadcg0Sr3_SPEC;
        impl crate::sealed::RegSpec for Vadcg0Sr3_SPEC {
            type DataType = u32;
        }
        #[doc = "EVADC Group 0 Service Request 3\n resetvalue={Application Reset:0x0}"]
        pub type Vadcg0Sr3 = crate::RegValueT<Vadcg0Sr3_SPEC>;

        impl Vadcg0Sr3 {
            #[doc = "Service Request Priority Number. The SRPN bit field defines the priority of a service request with        respect to service requests with to the same service provider  same        SRC.TOS configuration   00   gt  Service request is on lowest priority ... FF   gt  Service request is on highest priority For a CPU 01 is the lowest priority as 00 is never serviced. For a DMA 00 triggers channel 0. For DMA  SRPN must not be greater than the highest implemented DMA          channel number."]
            #[inline(always)]
            pub fn srpn(
                self,
            ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Vadcg0Sr3_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xff,1,0,u8, Vadcg0Sr3_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Enable"]
            #[inline(always)]
            pub fn sre(
                self,
            ) -> crate::common::RegisterFieldBool<10, 1, 0, Vadcg0Sr3_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<10,1,0,Vadcg0Sr3_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Type of Service Control. The TOS bit field configuration maps a Service Request to an Interrupt Service Provider  In products where DMA or CPUx is not implemented  the related TOS encoding will not be used and treated as RESERVED. In products with less than 4 ISPs the effective size of the TOS bit field might be reduced to 1 or 2 bit."]
            #[inline(always)]
            pub fn tos(
                self,
            ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Vadcg0Sr3_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<11,0x7,1,0,u8, Vadcg0Sr3_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Error Correction Code. The ECC bit field will be updated by the SRN under the following        conditions  Write or Read Modify Write to SRC 31 0  Write to SRC 15 0   16 bit write  Write to SRC 15 8  or write to SRC 7 0   byte write  For more details pls. see CROSSREFERENCE . In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Vadcg0Sr3_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, Vadcg0Sr3_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Service Request Flag. The SRR bit shows the status of the Service Request."]
            #[inline(always)]
            pub fn srr(
                self,
            ) -> crate::common::RegisterFieldBool<24, 1, 0, Vadcg0Sr3_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<24,1,0,Vadcg0Sr3_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Clear Bit. The CLRR bit is required to reset SRR SRR ."]
            #[inline(always)]
            pub fn clrr(
                self,
            ) -> crate::common::RegisterFieldBool<25, 1, 0, Vadcg0Sr3_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<25,1,0,Vadcg0Sr3_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Request Set Bit. The SETR bit is required to set SRR SRR ."]
            #[inline(always)]
            pub fn setr(
                self,
            ) -> crate::common::RegisterFieldBool<26, 1, 0, Vadcg0Sr3_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<26,1,0,Vadcg0Sr3_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Bit. The IOV bit is set by HW if a new service request was triggered via        interrupt trigger or SETR SETR bit while the SRN has still an pending service request."]
            #[inline(always)]
            pub fn iov(
                self,
            ) -> crate::common::RegisterFieldBool<27, 1, 0, Vadcg0Sr3_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<27,1,0,Vadcg0Sr3_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Interrupt Trigger Overflow Clear Bit. IOVCLR is required to reset IOV IOV ."]
            #[inline(always)]
            pub fn iovclr(
                self,
            ) -> crate::common::RegisterFieldBool<28, 1, 0, Vadcg0Sr3_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<28,1,0,Vadcg0Sr3_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "SW Sticky Bit. The Software Sticky Bit is set when the SRR SRR bit has been set via the SETR SETR bit. This bit can be cleared by writing with 1 to SWSCLR SWSCLR . Writing to SWS has no effect."]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterFieldBool<29, 1, 0, Vadcg0Sr3_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<29,1,0,Vadcg0Sr3_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "SW Sticky Clear Bit. SWSCLR is required to reset SWS SWS ."]
            #[inline(always)]
            pub fn swsclr(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, Vadcg0Sr3_SPEC, crate::common::W>
            {
                crate::common::RegisterFieldBool::<30,1,0,Vadcg0Sr3_SPEC,crate::common::W>::from_register(self,0)
            }
        }
        impl core::default::Default for Vadcg0Sr3 {
            #[inline(always)]
            fn default() -> Vadcg0Sr3 {
                <crate::RegValueT<Vadcg0Sr3_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
