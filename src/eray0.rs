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
#[doc = r"ERAY"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eray0(pub(super) *mut u8);
unsafe impl core::marker::Send for Eray0 {}
unsafe impl core::marker::Sync for Eray0 {}
impl Eray0 {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2300usize)) }
    }

    #[doc = "Aggregated Channel Status\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn acs(&self) -> crate::common::Reg<self::Acs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(296usize)) }
    }

    #[doc = "Communication Controller Error Vector\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ccev(&self) -> crate::common::Reg<self::Ccev_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }

    #[doc = "Communication Controller Status Vector\n resetvalue={Application Reset:0x104000}"]
    #[inline(always)]
    pub const fn ccsv(&self) -> crate::common::Reg<self::Ccsv_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Core Release Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn crel(&self) -> crate::common::Reg<self::Crel_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1008usize)) }
    }

    #[doc = "Busy and Input Buffer Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cust1(&self) -> crate::common::Reg<self::Cust1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }

    #[doc = "Customer Interface Timeout Counter Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cust3(&self) -> crate::common::Reg<self::Cust3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }

    #[doc = "Error Service Request Enable Reset\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn eier(&self) -> crate::common::Reg<self::Eier_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "Error Service Request Enable Set\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn eies(&self) -> crate::common::Reg<self::Eies_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }

    #[doc = "Error Service Request Line Select\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn eils(&self) -> crate::common::Reg<self::Eils_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }

    #[doc = "Error Service Request Select Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn eir(&self) -> crate::common::Reg<self::Eir_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Endian Register\n resetvalue={Application Reset:0x087654321}"]
    #[inline(always)]
    pub const fn endn(&self) -> crate::common::Reg<self::Endn_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1012usize)) }
    }

    #[doc = "Even Sync ID Symbol Window 01\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn esidn(&self) -> [crate::common::Reg<self::EsiDn_SPEC, crate::common::R>; 15] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x38usize)),
            ]
        }
    }

    #[doc = "FIFO Critical Level\n resetvalue={Application Reset:0x080}"]
    #[inline(always)]
    pub const fn fcl(&self) -> crate::common::Reg<self::Fcl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(780usize)) }
    }

    #[doc = "FIFO Rejection Filter\n resetvalue={Application Reset:0x1800000}"]
    #[inline(always)]
    pub const fn frf(&self) -> crate::common::Reg<self::Frf_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(772usize)) }
    }

    #[doc = "FIFO Rejection Filter Mask\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn frfm(&self) -> crate::common::Reg<self::Frfm_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(776usize)) }
    }

    #[doc = "FIFO Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fsr(&self) -> crate::common::Reg<self::Fsr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(792usize)) }
    }

    #[doc = "GTU Configuration Register 1\n resetvalue={Application Reset:0x280}"]
    #[inline(always)]
    pub const fn gtuc01(&self) -> crate::common::Reg<self::Gtuc01_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }

    #[doc = "GTU Configuration Register 2\n resetvalue={Application Reset:0x2000A}"]
    #[inline(always)]
    pub const fn gtuc02(&self) -> crate::common::Reg<self::Gtuc02_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(164usize)) }
    }

    #[doc = "GTU Configuration Register 3\n resetvalue={Application Reset:0x2020000}"]
    #[inline(always)]
    pub const fn gtuc03(&self) -> crate::common::Reg<self::Gtuc03_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(168usize)) }
    }

    #[doc = "GTU Configuration Register 4\n resetvalue={Application Reset:0x080007}"]
    #[inline(always)]
    pub const fn gtuc04(&self) -> crate::common::Reg<self::Gtuc04_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(172usize)) }
    }

    #[doc = "GTU Configuration Register 5\n resetvalue={Application Reset:0x0E000000}"]
    #[inline(always)]
    pub const fn gtuc05(&self) -> crate::common::Reg<self::Gtuc05_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(176usize)) }
    }

    #[doc = "GTU Configuration Register 6\n resetvalue={Application Reset:0x20000}"]
    #[inline(always)]
    pub const fn gtuc06(&self) -> crate::common::Reg<self::Gtuc06_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(180usize)) }
    }

    #[doc = "GTU Configuration Register 7\n resetvalue={Application Reset:0x20004}"]
    #[inline(always)]
    pub const fn gtuc07(&self) -> crate::common::Reg<self::Gtuc07_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(184usize)) }
    }

    #[doc = "GTU Configuration Register 8\n resetvalue={Application Reset:0x2}"]
    #[inline(always)]
    pub const fn gtuc08(&self) -> crate::common::Reg<self::Gtuc08_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(188usize)) }
    }

    #[doc = "GTU Configuration Register 9\n resetvalue={Application Reset:0x101}"]
    #[inline(always)]
    pub const fn gtuc09(&self) -> crate::common::Reg<self::Gtuc09_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(192usize)) }
    }

    #[doc = "GTU Configuration Register 10\n resetvalue={Application Reset:0x20005}"]
    #[inline(always)]
    pub const fn gtuc10(&self) -> crate::common::Reg<self::Gtuc10_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(196usize)) }
    }

    #[doc = "GTU Configuration Register 11\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gtuc11(&self) -> crate::common::Reg<self::Gtuc11_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(200usize)) }
    }

    #[doc = "Input Buffer Command Mask\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ibcm(&self) -> crate::common::Reg<self::Ibcm_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1296usize)) }
    }

    #[doc = "Input Buffer Command Request\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ibcr(&self) -> crate::common::Reg<self::Ibcr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1300usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x44C000}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Service Request Line Enable\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ile(&self) -> crate::common::Reg<self::Ile_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2292usize)) }
    }

    #[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2288usize)) }
    }

    #[doc = "Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2284usize)) }
    }

    #[doc = "Lock Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn lck(&self) -> crate::common::Reg<self::Lck_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Last Dynamic Transmit Slot\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ldts(&self) -> crate::common::Reg<self::Ldts_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(788usize)) }
    }

    #[doc = "Message Buffer Status\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mbs(&self) -> crate::common::Reg<self::Mbs_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1804usize)) }
    }

    #[doc = "Message Buffer Status Changed 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mbsc1(&self) -> crate::common::Reg<self::Mbsc1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(832usize)) }
    }

    #[doc = "Message Buffer Status Changed 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mbsc2(&self) -> crate::common::Reg<self::Mbsc2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(836usize)) }
    }

    #[doc = "Message Buffer Status Changed 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mbsc3(&self) -> crate::common::Reg<self::Mbsc3_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(840usize)) }
    }

    #[doc = "Message Buffer Status Changed 4\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mbsc4(&self) -> crate::common::Reg<self::Mbsc4_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(844usize)) }
    }

    #[doc = "MHD Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mhdc(&self) -> crate::common::Reg<self::Mhdc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(152usize)) }
    }

    #[doc = "Message Handler Constraints Flags\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mhdf(&self) -> crate::common::Reg<self::Mhdf_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(796usize)) }
    }

    #[doc = "Message Handler Status\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mhds(&self) -> crate::common::Reg<self::Mhds_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(784usize)) }
    }

    #[doc = "Message RAM Configuration\n resetvalue={Application Reset:0x1800000}"]
    #[inline(always)]
    pub const fn mrc(&self) -> crate::common::Reg<self::Mrc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(768usize)) }
    }

    #[doc = "Message Buffer Status Changed Interrupt Control 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn msic1(&self) -> crate::common::Reg<self::Msic1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(952usize)) }
    }

    #[doc = "Message Buffer Status Changed Interrupt Control 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn msic2(&self) -> crate::common::Reg<self::Msic2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(956usize)) }
    }

    #[doc = "Message Buffer Status Changed Interrupt Control 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn msic3(&self) -> crate::common::Reg<self::Msic3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(960usize)) }
    }

    #[doc = "Message Buffer Status Changed Interrupt Control 4\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn msic4(&self) -> crate::common::Reg<self::Msic4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(964usize)) }
    }

    #[doc = "Macrotick and Cycle Counter Value\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mtccv(&self) -> crate::common::Reg<self::Mtccv_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(276usize)) }
    }

    #[doc = "New Data Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ndat1(&self) -> crate::common::Reg<self::Ndat1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(816usize)) }
    }

    #[doc = "New Data Register 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ndat2(&self) -> crate::common::Reg<self::Ndat2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(820usize)) }
    }

    #[doc = "New Data Register 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ndat3(&self) -> crate::common::Reg<self::Ndat3_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(824usize)) }
    }

    #[doc = "New Data Register 4\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ndat4(&self) -> crate::common::Reg<self::Ndat4_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(828usize)) }
    }

    #[doc = "New Data Interrupt Control 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ndic1(&self) -> crate::common::Reg<self::Ndic1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(936usize)) }
    }

    #[doc = "New Data Interrupt Control 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ndic2(&self) -> crate::common::Reg<self::Ndic2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(940usize)) }
    }

    #[doc = "New Data Interrupt Control 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ndic3(&self) -> crate::common::Reg<self::Ndic3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(944usize)) }
    }

    #[doc = "New Data Interrupt Control 4\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ndic4(&self) -> crate::common::Reg<self::Ndic4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(948usize)) }
    }

    #[doc = "NEM Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn nemc(&self) -> crate::common::Reg<self::Nemc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(140usize)) }
    }

    #[doc = "Network Management Vector 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn nmvx(&self) -> [crate::common::Reg<self::NmVx_SPEC, crate::common::R>; 3] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x1b0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1b0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1b0usize + 0x8usize)),
            ]
        }
    }

    #[doc = "Output Buffer Command Mask\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn obcm(&self) -> crate::common::Reg<self::Obcm_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1808usize)) }
    }

    #[doc = "Output Buffer Command Request\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn obcr(&self) -> crate::common::Reg<self::Obcr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1812usize)) }
    }

    #[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2280usize)) }
    }

    #[doc = "Offset Correction Value\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ocv(&self) -> crate::common::Reg<self::Ocv_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(284usize)) }
    }

    #[doc = "Odd Sync ID Symbol Window 01\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn osidn(&self) -> [crate::common::Reg<self::OsiDn_SPEC, crate::common::R>; 15] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x38usize)),
            ]
        }
    }

    #[doc = "OCDS Trigger Set Select\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn otss(&self) -> crate::common::Reg<self::Otss_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2160usize)) }
    }

    #[doc = "PRT Configuration Register 1\n resetvalue={Application Reset:0x084C0633}"]
    #[inline(always)]
    pub const fn prtc1(&self) -> crate::common::Reg<self::Prtc1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }

    #[doc = "PRT Configuration Register 2\n resetvalue={Application Reset:0x0F2D0A0E}"]
    #[inline(always)]
    pub const fn prtc2(&self) -> crate::common::Reg<self::Prtc2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }

    #[doc = "Rate Correction Value\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rcv(&self) -> crate::common::Reg<self::Rcv_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(280usize)) }
    }

    #[doc = "Read Data Section 01\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rddsn(&self) -> [crate::common::Reg<self::RddSn_SPEC, crate::common::R>; 64] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x5cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x60usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x64usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x68usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x6cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x70usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x74usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x78usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x7cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x80usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x84usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x88usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x8cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x90usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x94usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x98usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x9cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xa0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xa4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xa8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xacusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xb0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xb4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xb8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xbcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xc0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xc4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xc8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xccusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xd0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xd4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xd8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xdcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xe0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xe4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xe8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xecusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xf0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xf4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xf8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xfcusize)),
            ]
        }
    }

    #[doc = "Read Header Section 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rdhs1(&self) -> crate::common::Reg<self::Rdhs1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1792usize)) }
    }

    #[doc = "Read Header Section 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rdhs2(&self) -> crate::common::Reg<self::Rdhs2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1796usize)) }
    }

    #[doc = "Read Header Section 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rdhs3(&self) -> crate::common::Reg<self::Rdhs3_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1800usize)) }
    }

    #[doc = "Slot Counter Value\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn scv(&self) -> crate::common::Reg<self::Scv_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(272usize)) }
    }

    #[doc = "SYNC Frame Status\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sfs(&self) -> crate::common::Reg<self::Sfs_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(288usize)) }
    }

    #[doc = "Status Service Request Enable Reset\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sier(&self) -> crate::common::Reg<self::Sier_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "Status Service Request Enable Set\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sies(&self) -> crate::common::Reg<self::Sies_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }

    #[doc = "Status Service Request Line Select\n resetvalue={Application Reset:0x303FFFF}"]
    #[inline(always)]
    pub const fn sils(&self) -> crate::common::Reg<self::Sils_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }

    #[doc = "Status Service Request Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sir(&self) -> crate::common::Reg<self::Sir_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "Stop Watch Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn stpw1(&self) -> crate::common::Reg<self::Stpw1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(76usize)) }
    }

    #[doc = "Stop Watch Register 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn stpw2(&self) -> crate::common::Reg<self::Stpw2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }

    #[doc = "SUC Configuration Register 1\n resetvalue={Application Reset:0x0C401000}"]
    #[inline(always)]
    pub const fn succ1(&self) -> crate::common::Reg<self::Succ1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "SUC Configuration Register 2\n resetvalue={Application Reset:0x1000504}"]
    #[inline(always)]
    pub const fn succ2(&self) -> crate::common::Reg<self::Succ2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }

    #[doc = "SUC Configuration Register 3\n resetvalue={Application Reset:0x11}"]
    #[inline(always)]
    pub const fn succ3(&self) -> crate::common::Reg<self::Succ3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }

    #[doc = "Symbol Window and Network Idle Time Status\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn swnit(&self) -> crate::common::Reg<self::Swnit_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(292usize)) }
    }

    #[doc = "Timer 0 Configuration\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t0c(&self) -> crate::common::Reg<self::T0C_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }

    #[doc = "Timer 1 Configuration\n resetvalue={Application Reset:0x20000}"]
    #[inline(always)]
    pub const fn t1c(&self) -> crate::common::Reg<self::T1C_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }

    #[doc = "Test Register 1\n resetvalue={Application Reset:0x300}"]
    #[inline(always)]
    pub const fn test1(&self) -> crate::common::Reg<self::Test1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "Test Register 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn test2(&self) -> crate::common::Reg<self::Test2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Transmission Request Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn txrq1(&self) -> crate::common::Reg<self::Txrq1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(800usize)) }
    }

    #[doc = "Transmission Request Register 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn txrq2(&self) -> crate::common::Reg<self::Txrq2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(804usize)) }
    }

    #[doc = "Transmission Request Register 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn txrq3(&self) -> crate::common::Reg<self::Txrq3_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(808usize)) }
    }

    #[doc = "Transmission Request Register 4\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn txrq4(&self) -> crate::common::Reg<self::Txrq4_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(812usize)) }
    }

    #[doc = "Write Data Section 01\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn wrdsn(&self) -> [crate::common::Reg<self::WrdSn_SPEC, crate::common::RW>; 64] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x5cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x60usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x64usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x68usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x6cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x70usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x74usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x78usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x7cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x80usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x84usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x88usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x8cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x90usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x94usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x98usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x9cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xa0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xa4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xa8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xacusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xb0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xb4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xb8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xbcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xc0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xc4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xc8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xccusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xd0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xd4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xd8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xdcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xe0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xe4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xe8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xecusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xf0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xf4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xf8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xfcusize)),
            ]
        }
    }

    #[doc = "Write Header Section 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn wrhs1(&self) -> crate::common::Reg<self::Wrhs1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1280usize)) }
    }

    #[doc = "Write Header Section 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn wrhs2(&self) -> crate::common::Reg<self::Wrhs2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1284usize)) }
    }

    #[doc = "Write Header Section 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn wrhs3(&self) -> crate::common::Reg<self::Wrhs3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1288usize)) }
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
pub struct Acs_SPEC;
impl crate::sealed::RegSpec for Acs_SPEC {
    type DataType = u32;
}
#[doc = "Aggregated Channel Status\n resetvalue={Application Reset:0x0}"]
pub type Acs = crate::RegValueT<Acs_SPEC>;

impl Acs {
    #[doc = "Valid Frame Received on Channel A vSS ValidFrameA    VFRA. One or more valid Frames were received on channel A in any static or dynamic slot during the observation period."]
    #[inline(always)]
    pub fn vfra(self) -> crate::common::RegisterFieldBool<0, 1, 0, Acs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Acs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Syntax Error Detected on Channel A vSS SyntaxErrorA    SEDA. One or more syntax errors in static or dynamic slots  symbol window  and network idle time  NIT  were observed on channel A."]
    #[inline(always)]
    pub fn seda(self) -> crate::common::RegisterFieldBool<1, 1, 0, Acs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Acs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Content Error Detected on Channel A vSS ContentErrorA    CEDA. One or more Frames with a content error were received on channel A in any static or dynamic slot during the observation period."]
    #[inline(always)]
    pub fn ceda(self) -> crate::common::RegisterFieldBool<2, 1, 0, Acs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Acs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Communication Indicator Channel A   CIA. One or more valid Frames were received on channel A in slots that also contained any additional communication during the observation period  i.e. one or more slots received a valid Frame AND had any combination of either syntax error OR content error OR slot boundary violation."]
    #[inline(always)]
    pub fn cia(self) -> crate::common::RegisterFieldBool<3, 1, 0, Acs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Acs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Slot Boundary Violation on Channel A vSS BViolationA    SBVA. One or more slot boundary violations were observed on channel A at any time during the observation period  static or dynamic slots  symbol window  and network idle time NIT ."]
    #[inline(always)]
    pub fn sbva(self) -> crate::common::RegisterFieldBool<4, 1, 0, Acs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Acs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Valid Frame Received on Channel B vSS ValidFrameB    VFRB. One or more valid Frames were received on channel B in any static or dynamic slot during the observation period."]
    #[inline(always)]
    pub fn vfrb(self) -> crate::common::RegisterFieldBool<8, 1, 0, Acs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Acs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Syntax Error Detected on Channel B vSS SyntaxErrorB    SEDB. One or more syntax errors in static or dynamic slots  symbol window  and network idle time  NIT  were observed on channel B."]
    #[inline(always)]
    pub fn sedb(self) -> crate::common::RegisterFieldBool<9, 1, 0, Acs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Acs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Content Error Detected on Channel B vSS ContentErrorB    CEDB. One or more Frames with a content error were received on channel B in any static or dynamic slot during the observation period."]
    #[inline(always)]
    pub fn cedb(self) -> crate::common::RegisterFieldBool<10, 1, 0, Acs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Acs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Communication Indicator Channel B   CIB. One or more valid Frames were received on channel B in slots that also contained any additional communication during the observation period  i.e. one or more slots received a valid Frame AND had any combination of either syntax error OR content error OR slot boundary violation."]
    #[inline(always)]
    pub fn cib(self) -> crate::common::RegisterFieldBool<11, 1, 0, Acs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Acs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Slot Boundary Violation on Channel B vSS BViolationB    SBVB. One or more slot boundary violations were observed on channel B at any time during the observation period  static or dynamic slots  symbol window  and network idle time NIT ."]
    #[inline(always)]
    pub fn sbvb(self) -> crate::common::RegisterFieldBool<12, 1, 0, Acs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Acs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Acs {
    #[inline(always)]
    fn default() -> Acs {
        <crate::RegValueT<Acs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccev_SPEC;
impl crate::sealed::RegSpec for Ccev_SPEC {
    type DataType = u32;
}
#[doc = "Communication Controller Error Vector\n resetvalue={Application Reset:0x0}"]
pub type Ccev = crate::RegValueT<Ccev_SPEC>;

impl Ccev {
    #[doc = "Clock Correction Failed Counter vClockCorrectionFailed    CCFC. The Clock Correction Failed Counter is incremented by one at the end of        any odd communication cycle where either the missing offset correction        error or missing rate correction error are active. The Clock Correction        Failed Counter is reset to 0 at the end of an odd communication cycle if        neither the offset correction failed nor the rate correction failed        errors are active. The Clock Correction Failed Counter stops at 15."]
    #[inline(always)]
    pub fn ccfc(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Ccev_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xf, 1, 0, u8, Ccev_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Mode vPOC ErrorMode    ERRM. Indicates the actual error mode of the POC."]
    #[inline(always)]
    pub fn errm(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Ccev_SPEC, crate::common::R> {
        crate::common::RegisterField::<6, 0x3, 1, 0, u8, Ccev_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Passive to Active Count vAllowPassiveToActive    PTAC. Indicates the number of consecutive even   odd cycle pairs that have passed with valid rate and offset correction terms  while the node is waiting to transit from  NORMAL PASSIVE  state to  NORMAL ACTIVE  state. The transition takes place when PTAC equals SUCC1.PTA 1."]
    #[inline(always)]
    pub fn ptac(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, Ccev_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, Ccev_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ccev {
    #[inline(always)]
    fn default() -> Ccev {
        <crate::RegValueT<Ccev_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccsv_SPEC;
impl crate::sealed::RegSpec for Ccsv_SPEC {
    type DataType = u32;
}
#[doc = "Communication Controller Status Vector\n resetvalue={Application Reset:0x104000}"]
pub type Ccsv = crate::RegValueT<Ccsv_SPEC>;

impl Ccsv {
    #[doc = "Protocol Operation Control Status   POCS. 00 H ... 05 H   0F H Indicates the actual        state of operation of the Communication Controller Protocol Operation        Control 06 H   160    8230   160 14 H are reserved. 10 H ... 13 H Indicates the actual state of        operation of the POC in the wakeup path 14 H   160    8230   160 1F H are reserved. 20 H ... 2B H Indicates the actual state of        operation of the POC in the startup path 2C H   160    8230   160 3F H are reserved."]
    #[inline(always)]
    pub fn pocs(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Ccsv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Ccsv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Freeze Status Indicator vPOC Freeze    FSI. Indicates that the POC has entered the   8220 HALT  8221  state due to CHI command          8220 FREEZE  8221  or due to an error condition requiring an immediate POC halt.        Reset by transition from   8220 HALT  8221  to   8220 DEFAULT CONFIG  8221  state."]
    #[inline(always)]
    pub fn fsi(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ccsv_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ccsv_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Halt Request vPOC CHIHaltRequest    HRQ. Indicates that a request from the Host has been received to halt the POC        at the end of the communication cycle. Reset by transition from   8220 HALT  8221         to   8220 DEFAULT CONFIG  8221  state or when entering   8220 READY  8221  state."]
    #[inline(always)]
    pub fn hrq(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ccsv_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ccsv_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Slot Mode vPOC SlotMode    SLM. Indicates the actual slot mode of the POC in states READY  WAKEUP         STARTUP  NORMAL ACTIVE  and NORMAL PASSIVE. Default is   8220 SINGLE  8221 . Changes        to   8220 ALL  8221   depending on configuration bit SUCC1.TSM. In   8220 NORMAL ACTIVE  8221         or   8220 NORMAL PASSIVE  8221  state the CHI command   8220 ALL SLOTS  8221  will change the        slot mode from   8220 SINGLE  8221  over   8220 ALL PENDING  8221  to   8220 ALL  8221 . Set to SINGLE in        all other states."]
    #[inline(always)]
    pub fn slm(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Ccsv_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0x3, 1, 0, u8, Ccsv_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Coldstart Noise Indicator vPOC ColdstartNoise    CSNI. Indicates that the cold start procedure occurred under noisy conditions.        Reset by CHI command   8220 RESET STATUS INDICATORS  8221  or by transition from          8220 HALT  8221  to   8220 DEFAULT CONFIG  8221  state or from   8220 READY  8221  to   8220 STARTUP  8221  state."]
    #[inline(always)]
    pub fn csni(self) -> crate::common::RegisterFieldBool<12, 1, 0, Ccsv_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ccsv_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Coldstart Abort Indicator   CSAI. Coldstart aborted. Reset by CHI command   8220 RESET STATUS INDICATORS  8221  or by        transition from   8220 HALT  8221  to   8220 DEFAULT CONFIG  8221  state or from   8220 READY  8221  to          8220 STARTUP  8221  state."]
    #[inline(always)]
    pub fn csai(self) -> crate::common::RegisterFieldBool<13, 1, 0, Ccsv_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ccsv_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Cold Start Inhibit vColdStartInhibit    CSI. Indicates that the node is disabled from cold starting. The flag is set        whenever the POC enters   8220 READY  8221  state due to CHI command   8220 READY  8221 . The        flag has to be reset under control of the Host by CHI command          8220 ALLOW COLDSTART  8221   SUCC1.CMD  160    160 1001 B  ."]
    #[inline(always)]
    pub fn csi(self) -> crate::common::RegisterFieldBool<14, 1, 0, Ccsv_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ccsv_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wakeup Status vPOC WakeupStatus    WSV. Indicates the status of the current wakeup attempt. Reset by CHI command          8220 RESET STATUS INDICATORS  8221  or by transition from   8220 HALT  8221  to          8220 DEFAULT CONFIG  8221  state."]
    #[inline(always)]
    pub fn wsv(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Ccsv_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Ccsv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Remaining Coldstart Attempts vRemainingColdstartAttempts    RCA. Indicates the number of remaining coldstart attempts. The RUN command        resets this counter to the maximum number of coldstart attempts as        configured by SUCC1.CSA."]
    #[inline(always)]
    pub fn rca(
        self,
    ) -> crate::common::RegisterField<19, 0x1f, 1, 0, u8, Ccsv_SPEC, crate::common::R> {
        crate::common::RegisterField::<19,0x1f,1,0,u8, Ccsv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "POC Status Log   PSL. Status of CCSV.POCS immediately before entering   8220 HALT  8221  state. Set when        entering   8220 HALT  8221  state. Set to   8220 HALT  8221  when FREEZE command is applied        during   8220 HALT  8221  state. Reset to 000000 B when leaving   8220 HALT  8221  state."]
    #[inline(always)]
    pub fn psl(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, Ccsv_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x3f,1,0,u8, Ccsv_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ccsv {
    #[inline(always)]
    fn default() -> Ccsv {
        <crate::RegValueT<Ccsv_SPEC> as RegisterValue<_>>::new(1064960)
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
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control of the module. This bit disables the kernel clocks f CLC ERAY and the sampling clock f SCLK ."]
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
    #[doc = "External Sleep Mode Request Disable Bit   EDIS. Used to control module  8217 s sleep mode. If this bit is cleared the kernel clock f CLC ERAY and the sampling clock f SCLK are disabled during System Sleep Mode."]
    #[inline(always)]
    pub fn edis(self) -> crate::common::RegisterFieldBool<3, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clock Divider in Run Mode   RMC. This bit field is not affected by an application reset. This bit field only controls the kernel clock f CLC ERAY and not the sampling clock f SCLK ."]
    #[inline(always)]
    pub fn rmc(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8, 0x7, 1, 0, u8, Clc_SPEC, crate::common::RW>::from_register(
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
pub struct Crel_SPEC;
impl crate::sealed::RegSpec for Crel_SPEC {
    type DataType = u32;
}
#[doc = "Core Release Register\n resetvalue={Application Reset:0x0}"]
pub type Crel = crate::RegValueT<Crel_SPEC>;

impl Crel {
    #[doc = "Design Time Stamp  Day   DAY. Two digits  BCD coded."]
    #[inline(always)]
    pub fn day(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Crel_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Crel_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Design Time Stamp  Month   MON. Two digits  BCD coded."]
    #[inline(always)]
    pub fn mon(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Crel_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Crel_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Design Time Stamp  Year   YEAR. One digit  BCD coded."]
    #[inline(always)]
    pub fn year(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Crel_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Crel_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Sub Step of Core Release   SUBSTEP. One digits  BCD coded."]
    #[inline(always)]
    pub fn substep(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Crel_SPEC, crate::common::R> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Crel_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Step of Core Release   STEP. One digits  BCD coded."]
    #[inline(always)]
    pub fn step(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Crel_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Crel_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Core Release   REL. One digit  BCD coded."]
    #[inline(always)]
    pub fn rel(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Crel_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Crel_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Crel {
    #[inline(always)]
    fn default() -> Crel {
        <crate::RegValueT<Crel_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cust1_SPEC;
impl crate::sealed::RegSpec for Cust1_SPEC {
    type DataType = u32;
}
#[doc = "Busy and Input Buffer Control Register\n resetvalue={Application Reset:0x0}"]
pub type Cust1 = crate::RegValueT<Cust1_SPEC>;

impl Cust1 {
    #[doc = "CIF Timeout Service Request Status   INT0. INT0 will be set if a timeout has occurred during the auto delay scheme        and must be reset by writing zero to INT0. Software can also set this        bit field. In case hardware sets INT0 and at the same point of time software          clears INT0  INT0 is cleared."]
    #[inline(always)]
    pub fn int0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cust1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cust1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable auto delay scheme for Output Buffer Control Register  OBCR    OEN. This control bit controls the delay scheme for Output Buffer Control        Register  OBCR  read accesses."]
    #[inline(always)]
    pub fn oen(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cust1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cust1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable auto delay scheme for Input Buffer Control Register  IBCR    IEN. This control bit controls the auto delay scheme for Input Buffer Control Register  IBCR  read accesses."]
    #[inline(always)]
    pub fn ien(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cust1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cust1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Buffer Status Register   IBFS. This status bit  eray mhd signal  ifb ifb1 2n  indicates which of the two Input Buffer RAMs  IBF  is accessible by the host  via CIF  as Input Buffer. The other non accessible buffer RAM is currently used as shadow buffer RAM by the ERAY message handler and therefore not accessible by the host. After reset  it is set by hardware."]
    #[inline(always)]
    pub fn ibfs(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cust1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cust1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Buffer 1 Page Select Register   IBF1PAG. This control bit selects if the upper page or lower page of Input Buffer 1  IBF1  currently active. Write is only possible  if Input Buffer RAM 1 is currently accessible by the host  via CIF  and therefore IBFS set."]
    #[inline(always)]
    pub fn ibf1pag(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Cust1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cust1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Buffer 2 Page Select Register   IBF2PAG. This control bit selects if the upper page or lower page of Input Buffer 2  IBF2  currently active. Write is only possible  if Input Buffer RAM 2 is currently accessible by the host  via CIF  and therefore IBFS cleared."]
    #[inline(always)]
    pub fn ibf2pag(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Cust1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Cust1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive Input Select Channel A   RISA"]
    #[inline(always)]
    pub fn risa(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Cust1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Cust1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Input Select Channel B   RISB"]
    #[inline(always)]
    pub fn risb(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Cust1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Cust1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Stop Watch Trigger Input Select   STPWTS"]
    #[inline(always)]
    pub fn stpwts(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Cust1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Cust1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Cust1 {
    #[inline(always)]
    fn default() -> Cust1 {
        <crate::RegValueT<Cust1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cust3_SPEC;
impl crate::sealed::RegSpec for Cust3_SPEC {
    type DataType = u32;
}
#[doc = "Customer Interface Timeout Counter Register\n resetvalue={Application Reset:0x0}"]
pub type Cust3 = crate::RegValueT<Cust3_SPEC>;

impl Cust3 {
    #[doc = "CIF Timeout Reload Value   TO. The 32 bit down counter reload  start up  value must be setup for the automatic delay scheme."]
    #[inline(always)]
    pub fn to(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Cust3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Cust3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Cust3 {
    #[inline(always)]
    fn default() -> Cust3 {
        <crate::RegValueT<Cust3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eier_SPEC;
impl crate::sealed::RegSpec for Eier_SPEC {
    type DataType = u32;
}
#[doc = "Error Service Request Enable Reset\n resetvalue={Application Reset:0x0}"]
pub type Eier = crate::RegValueT<Eier_SPEC>;

impl Eier {
    #[doc = "POC Error Mode Changed Service Request Enable   PEMCE"]
    #[inline(always)]
    pub fn pemce(self) -> crate::common::RegisterFieldBool<0, 1, 0, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Eier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Command Not Accepted Service Request Enable   CNAE"]
    #[inline(always)]
    pub fn cnae(self) -> crate::common::RegisterFieldBool<1, 1, 0, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Eier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SYNC Frames Below Minimum Service Request Enable   SFBME"]
    #[inline(always)]
    pub fn sfbme(self) -> crate::common::RegisterFieldBool<2, 1, 0, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Eier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SYNC Frame Overflow Service Request Enable   SFOE"]
    #[inline(always)]
    pub fn sfoe(self) -> crate::common::RegisterFieldBool<3, 1, 0, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Eier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clock Correction Failure Service Request Enable   CCFE"]
    #[inline(always)]
    pub fn ccfe(self) -> crate::common::RegisterFieldBool<4, 1, 0, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Eier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CHI Command Locked Service Request Enable   CCLE"]
    #[inline(always)]
    pub fn ccle(self) -> crate::common::RegisterFieldBool<5, 1, 0, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Eier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ECC Error Service Request Enable   EERRE"]
    #[inline(always)]
    pub fn eerre(self) -> crate::common::RegisterFieldBool<6, 1, 0, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Eier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive FIFO Overrun Service Request Enable   RFOE"]
    #[inline(always)]
    pub fn rfoe(self) -> crate::common::RegisterFieldBool<7, 1, 0, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Eier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Empty FIFO Access Service Request Enable   EFAE"]
    #[inline(always)]
    pub fn efae(self) -> crate::common::RegisterFieldBool<8, 1, 0, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Eier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Illegal Input Buffer Access Service Request Enable   IIBAE"]
    #[inline(always)]
    pub fn iibae(self) -> crate::common::RegisterFieldBool<9, 1, 0, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Eier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Illegal Output Buffer Access Service Request Enable   IOBAE"]
    #[inline(always)]
    pub fn iobae(self) -> crate::common::RegisterFieldBool<10, 1, 0, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Eier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Handler Constraints Flag Service Request Enable   MHFE"]
    #[inline(always)]
    pub fn mhfe(self) -> crate::common::RegisterFieldBool<11, 1, 0, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Eier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Detected on Channel A Service Request Enable   EDAE"]
    #[inline(always)]
    pub fn edae(self) -> crate::common::RegisterFieldBool<16, 1, 0, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Eier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Latest Transmit Violation Channel A Service Request Enable   LTVAE"]
    #[inline(always)]
    pub fn ltvae(self) -> crate::common::RegisterFieldBool<17, 1, 0, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Eier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Across Boundary Channel A Service Request Enable   TABAE"]
    #[inline(always)]
    pub fn tabae(self) -> crate::common::RegisterFieldBool<18, 1, 0, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Eier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Detected on Channel B Service Request Enable   EDBE"]
    #[inline(always)]
    pub fn edbe(self) -> crate::common::RegisterFieldBool<24, 1, 0, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Eier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Latest Transmit Violation Channel B Service Request Enable   LTVBE"]
    #[inline(always)]
    pub fn ltvbe(self) -> crate::common::RegisterFieldBool<25, 1, 0, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Eier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Across Boundary Channel B Service Request Enable   TABBE"]
    #[inline(always)]
    pub fn tabbe(self) -> crate::common::RegisterFieldBool<26, 1, 0, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Eier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Eier {
    #[inline(always)]
    fn default() -> Eier {
        <crate::RegValueT<Eier_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eies_SPEC;
impl crate::sealed::RegSpec for Eies_SPEC {
    type DataType = u32;
}
#[doc = "Error Service Request Enable Set\n resetvalue={Application Reset:0x0}"]
pub type Eies = crate::RegValueT<Eies_SPEC>;

impl Eies {
    #[doc = "POC Error Mode Changed Service Request Enable   PEMCE"]
    #[inline(always)]
    pub fn pemce(self) -> crate::common::RegisterFieldBool<0, 1, 0, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Eies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Command Not Accepted Service Request Enable   CNAE"]
    #[inline(always)]
    pub fn cnae(self) -> crate::common::RegisterFieldBool<1, 1, 0, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Eies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SYNC Frames Below Minimum Service Request Enable   SFBME"]
    #[inline(always)]
    pub fn sfbme(self) -> crate::common::RegisterFieldBool<2, 1, 0, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Eies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SYNC Frame Overflow Service Request Enable   SFOE"]
    #[inline(always)]
    pub fn sfoe(self) -> crate::common::RegisterFieldBool<3, 1, 0, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Eies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clock Correction Failure Service Request Enable   CCFE"]
    #[inline(always)]
    pub fn ccfe(self) -> crate::common::RegisterFieldBool<4, 1, 0, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Eies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CHI Command Locked Service Request Enable   CCLE"]
    #[inline(always)]
    pub fn ccle(self) -> crate::common::RegisterFieldBool<5, 1, 0, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Eies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ECC Error Service Request Enable   EERRE"]
    #[inline(always)]
    pub fn eerre(self) -> crate::common::RegisterFieldBool<6, 1, 0, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Eies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive FIFO Overrun Service Request Enable   RFOE"]
    #[inline(always)]
    pub fn rfoe(self) -> crate::common::RegisterFieldBool<7, 1, 0, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Eies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Empty FIFO Access Service Request Enable   EFAE"]
    #[inline(always)]
    pub fn efae(self) -> crate::common::RegisterFieldBool<8, 1, 0, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Eies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Illegal Input Buffer Access Service Request Enable   IIBAE"]
    #[inline(always)]
    pub fn iibae(self) -> crate::common::RegisterFieldBool<9, 1, 0, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Eies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Illegal Output Buffer Access Service Request Enable   IOBAE"]
    #[inline(always)]
    pub fn iobae(self) -> crate::common::RegisterFieldBool<10, 1, 0, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Eies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Handler Constraints Flag Service Request Enable   MHFE"]
    #[inline(always)]
    pub fn mhfe(self) -> crate::common::RegisterFieldBool<11, 1, 0, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Eies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Detected on Channel A Service Request Enable   EDAE"]
    #[inline(always)]
    pub fn edae(self) -> crate::common::RegisterFieldBool<16, 1, 0, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Eies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Latest Transmit Violation Channel A Service Request Enable   LTVAE"]
    #[inline(always)]
    pub fn ltvae(self) -> crate::common::RegisterFieldBool<17, 1, 0, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Eies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Across Boundary Channel A Service Request Enable   TABAE"]
    #[inline(always)]
    pub fn tabae(self) -> crate::common::RegisterFieldBool<18, 1, 0, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Eies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Detected on Channel B Service Request Enable   EDBE"]
    #[inline(always)]
    pub fn edbe(self) -> crate::common::RegisterFieldBool<24, 1, 0, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Eies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Latest Transmit Violation Channel B Service Request Enable   LTVBE"]
    #[inline(always)]
    pub fn ltvbe(self) -> crate::common::RegisterFieldBool<25, 1, 0, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Eies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Across Boundary Channel B Service Request Enable   TABBE"]
    #[inline(always)]
    pub fn tabbe(self) -> crate::common::RegisterFieldBool<26, 1, 0, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Eies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Eies {
    #[inline(always)]
    fn default() -> Eies {
        <crate::RegValueT<Eies_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eils_SPEC;
impl crate::sealed::RegSpec for Eils_SPEC {
    type DataType = u32;
}
#[doc = "Error Service Request Line Select\n resetvalue={Application Reset:0x0}"]
pub type Eils = crate::RegValueT<Eils_SPEC>;

impl Eils {
    #[doc = "POC Error Mode Changed Service Request Line   PEMCL"]
    #[inline(always)]
    pub fn pemcl(self) -> crate::common::RegisterFieldBool<0, 1, 0, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Eils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Command Not Accepted Service Request Line   CNAL"]
    #[inline(always)]
    pub fn cnal(self) -> crate::common::RegisterFieldBool<1, 1, 0, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Eils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SYNC Frames Below Minimum Service Request Line   SFBML"]
    #[inline(always)]
    pub fn sfbml(self) -> crate::common::RegisterFieldBool<2, 1, 0, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Eils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SYNC Frame Overflow Service Request Line   SFOL"]
    #[inline(always)]
    pub fn sfol(self) -> crate::common::RegisterFieldBool<3, 1, 0, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Eils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clock Correction Failure Service Request Line   CCFL"]
    #[inline(always)]
    pub fn ccfl(self) -> crate::common::RegisterFieldBool<4, 1, 0, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Eils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CHI Command Locked Service Request Line   CCLL"]
    #[inline(always)]
    pub fn ccll(self) -> crate::common::RegisterFieldBool<5, 1, 0, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Eils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ECC Error Service Request Line   EERRL"]
    #[inline(always)]
    pub fn eerrl(self) -> crate::common::RegisterFieldBool<6, 1, 0, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Eils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive FIFO Overrun Service Request Line   RFOL"]
    #[inline(always)]
    pub fn rfol(self) -> crate::common::RegisterFieldBool<7, 1, 0, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Eils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Empty FIFO Access Service Request Line   EFAL"]
    #[inline(always)]
    pub fn efal(self) -> crate::common::RegisterFieldBool<8, 1, 0, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Eils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Illegal Input Buffer Access Service Request Line   IIBAL"]
    #[inline(always)]
    pub fn iibal(self) -> crate::common::RegisterFieldBool<9, 1, 0, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Eils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Illegal Output Buffer Access Service Request Line   IOBAL"]
    #[inline(always)]
    pub fn iobal(self) -> crate::common::RegisterFieldBool<10, 1, 0, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Eils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Handler Constrains Flag Service Request Line   MHFL"]
    #[inline(always)]
    pub fn mhfl(self) -> crate::common::RegisterFieldBool<11, 1, 0, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Eils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Detected on Channel A Service Request Line   EDAL"]
    #[inline(always)]
    pub fn edal(self) -> crate::common::RegisterFieldBool<16, 1, 0, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Eils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Latest Transmit Violation Channel A Service Request Line   LTVAL"]
    #[inline(always)]
    pub fn ltval(self) -> crate::common::RegisterFieldBool<17, 1, 0, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Eils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Across Boundary Channel A Service Request Line   TABAL"]
    #[inline(always)]
    pub fn tabal(self) -> crate::common::RegisterFieldBool<18, 1, 0, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Eils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Detected on Channel B Service Request Line   EDBL"]
    #[inline(always)]
    pub fn edbl(self) -> crate::common::RegisterFieldBool<24, 1, 0, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Eils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Latest Transmit Violation Channel B Service Request Line   LTVBL"]
    #[inline(always)]
    pub fn ltvbl(self) -> crate::common::RegisterFieldBool<25, 1, 0, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Eils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Across Boundary Channel B Service Request Line   TABBL"]
    #[inline(always)]
    pub fn tabbl(self) -> crate::common::RegisterFieldBool<26, 1, 0, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Eils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Eils {
    #[inline(always)]
    fn default() -> Eils {
        <crate::RegValueT<Eils_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eir_SPEC;
impl crate::sealed::RegSpec for Eir_SPEC {
    type DataType = u32;
}
#[doc = "Error Service Request Select Register\n resetvalue={Application Reset:0x0}"]
pub type Eir = crate::RegValueT<Eir_SPEC>;

impl Eir {
    #[doc = "POC Error Mode Changed   PEMC. This flag is set whenever the error mode signalled by CCEV.ERRM in the        Communication Controller Error Vector register has changed. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn pemc(self) -> crate::common::RegisterFieldBool<0, 1, 0, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Eir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Command Not Accepted   CNA. The flag signals that the write access to the CHI command vector        SUCC1.CMD in the SUC Configuration Register 1 was not successful because        the requested command was not valid in the actual POC state  or because        the CHI command was locked  CCL   1 . This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn cna(self) -> crate::common::RegisterFieldBool<1, 1, 0, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Eir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SYNC Frames Below Minimum   SFBM. This flag signals that the number of SYNC Frames received during the        last communication cycle was below the limit required by the FlexRay  8482         protocol. May be set during startup and therefore should be cleared by        the Host after the Communication Controller entered   8220 NORMAL ACTIVE  8221         state. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn sfbm(self) -> crate::common::RegisterFieldBool<2, 1, 0, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Eir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SYNC Frame Overflow   SFO. Set when either the number of SYNC Frames received during the last        communication cycle or the total number of SYNC Frames received during        the last double cycle exceeds the maximum number of SYNC Frames as        defined by GTUC02.SNM in the GTU Configuration Register  160 2. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn sfo(self) -> crate::common::RegisterFieldBool<3, 1, 0, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Eir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clock Correction Failure   CCF. This flag is set at the end of the cycle whenever one of the following        errors occurred  Missing offset and   or rate correction Clock Correction limit reached The clock correction status is monitored in registers CCEV and SFS. A        failure may occur during startup  therefore bit CCF should be cleared by        the Host after the Communication Controller entered   8220 NORMAL ACTIVE  8221         state. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn ccf(self) -> crate::common::RegisterFieldBool<4, 1, 0, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Eir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CHI Command Locked   CCL. The flag signals that the write access to the CHI command vector        SUCC1.CMD was not successful because the execution of the previous CHI        command has not yet completed. In this case bit EIR.CNA is also set to 1. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn ccl(self) -> crate::common::RegisterFieldBool<5, 1, 0, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Eir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ECC Error   EERR. The flag signals an ECC error to the Host. It is set whenever one of the        flags MHDS.EIBF  MHDS.EOBF  MHDS.EMR  MHDS.ETBF1  MHDS.ETBF2 changes        from 0 to 1. See also   8220 Message Handler Status  8221 . This bit must be cleared at        initialization of the module"]
    #[inline(always)]
    pub fn eerr(self) -> crate::common::RegisterFieldBool<6, 1, 0, Eir_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Eir_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive FIFO Overrun   RFO. The flag is set by the Communication Controller when a receive FIFO        overrun is detected. When a receive FIFO overrun occurs  the oldest        message is overwritten with the actual received message. The actual        state of the FIFO is monitored in register FSR."]
    #[inline(always)]
    pub fn rfo(self) -> crate::common::RegisterFieldBool<7, 1, 0, Eir_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Eir_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Empty FIFO Access   EFA. This flag is set by the Communication Controller when the Host requests        the transfer of a message from the receive FIFO via Output Buffer while        the receive FIFO is empty."]
    #[inline(always)]
    pub fn efa(self) -> crate::common::RegisterFieldBool<8, 1, 0, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Eir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Illegal Input Buffer Access   IIBA. This flag is set by the Communication Controller when the Host wants to        modify a Message Buffer via Input Buffer while the Communication        Controller is not in   8220 CONFIG  8221  or   8220 DEFAULT CONFIG  8221  state and one of the        following conditions applies  The Host writes to the Input Buffer Command Request register to modify          the  Header Section of Message Buffer 0  1 if configured for transmission            in key slot Header Section of static Message Buffers with buffer number  lt             MRC.FDB while MRC.SEC   01 B Header Section of any static or dynamic Message Buffer while MRC.SEC              1x B Header and   or Data Section of any message buffer belonging to the            receive FIFO The Host writes to any register of the Input Buffer while IBCR.IBSYS          is set."]
    #[inline(always)]
    pub fn iiba(self) -> crate::common::RegisterFieldBool<9, 1, 0, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Eir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Illegal Output Buffer Access   IOBA. This flag is set by the Communication Controller when the Host requests        the transfer of a Message Buffer from the Message RAM to the Output        Buffer while OBCR.OBSYS is set to 1."]
    #[inline(always)]
    pub fn ioba(self) -> crate::common::RegisterFieldBool<10, 1, 0, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Eir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Handler Constraints Flag   MHF. The flag signals a Message Handler constraints violation condition. It        is set whenever one of the flags MHDF.SNUA  MHDF.SNUB  MHDF.FNFA         MHDF.FNFB  MHDF.TBFA  MHDF.TBFB  MHDF.TNSA  MHDF.TNSB  MHDF.WAHP changes        from 0 to 1."]
    #[inline(always)]
    pub fn mhf(self) -> crate::common::RegisterFieldBool<11, 1, 0, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Eir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Detected on Channel A   EDA. This bit is set whenever one of the flags ACS.SEDA  ACS.CEDA  ACS.CIA         ACS.SBVA changes from 0 to 1. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn eda(self) -> crate::common::RegisterFieldBool<16, 1, 0, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Eir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Latest Transmit Violation Channel A   LTVA. The flag signals a latest transmit violation on channel A to the Host. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn ltva(self) -> crate::common::RegisterFieldBool<17, 1, 0, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Eir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Across Boundary Channel A   TABA. The flag signals to the Host that a transmission across a slot boundary        occurred for channel A. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn taba(self) -> crate::common::RegisterFieldBool<18, 1, 0, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Eir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Detected on Channel B   EDB. This bit is set whenever one of the flags ACS.SEDB  ACS.CEDB  ACS.CIB         ACS.SBVB changes from 0 to 1. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn edb(self) -> crate::common::RegisterFieldBool<24, 1, 0, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Eir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Latest Transmit Violation Channel B   LTVB. The flag signals a latest transmit violation on channel B to the Host. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn ltvb(self) -> crate::common::RegisterFieldBool<25, 1, 0, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Eir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Across Boundary Channel B   TABB. The flag signals to the Host that a transmission across a slot boundary        occurred for channel B. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn tabb(self) -> crate::common::RegisterFieldBool<26, 1, 0, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Eir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Eir {
    #[inline(always)]
    fn default() -> Eir {
        <crate::RegValueT<Eir_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endn_SPEC;
impl crate::sealed::RegSpec for Endn_SPEC {
    type DataType = u32;
}
#[doc = "Endian Register\n resetvalue={Application Reset:0x087654321}"]
pub type Endn = crate::RegValueT<Endn_SPEC>;

impl Endn {
    #[doc = "Endianness Test Value   ETV. The endianness test value."]
    #[inline(always)]
    pub fn etv(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Endn_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Endn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Endn {
    #[inline(always)]
    fn default() -> Endn {
        <crate::RegValueT<Endn_SPEC> as RegisterValue<_>>::new(2271560481)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EsiDn_SPEC;
impl crate::sealed::RegSpec for EsiDn_SPEC {
    type DataType = u32;
}
#[doc = "Even Sync ID Symbol Window 01\n resetvalue={Application Reset:0x0}"]
pub type EsiDn = crate::RegValueT<EsiDn_SPEC>;

impl EsiDn {
    #[doc = "Even Sync ID vsSyncIDListA B even    EID. SYNC Frame ID even communication cycle."]
    #[inline(always)]
    pub fn eid(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, EsiDn_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, EsiDn_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Received Configured Even Sync ID on Channel A   RXEA. Signals that a SYNC Frame corresponding to the stored even sync ID was received on channel A or that the node is configured to be a sync node with key slot   EID  ESID1 only ."]
    #[inline(always)]
    pub fn rxea(self) -> crate::common::RegisterFieldBool<14, 1, 0, EsiDn_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, EsiDn_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Received Configured Even Sync ID on Channel B   RXEB. Signals that a SYNC Frame corresponding to the stored even sync ID was received on channel B or that the node is configured to be a sync node with key slot   EID  ESID1 only ."]
    #[inline(always)]
    pub fn rxeb(self) -> crate::common::RegisterFieldBool<15, 1, 0, EsiDn_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, EsiDn_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for EsiDn {
    #[inline(always)]
    fn default() -> EsiDn {
        <crate::RegValueT<EsiDn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcl_SPEC;
impl crate::sealed::RegSpec for Fcl_SPEC {
    type DataType = u32;
}
#[doc = "FIFO Critical Level\n resetvalue={Application Reset:0x080}"]
pub type Fcl = crate::RegValueT<Fcl_SPEC>;

impl Fcl {
    #[doc = "Critical Level   CL. When the receive FIFO fill level FSR.RFFL is equal or greater than the critical level configured by CL  the receive FIFO critical level flag FSR.RFCL is set. If CL is programmed to values  gt  128  bit FSR.RFCL is never set. When FSR.RFCL changes from 0 to 1 bit SIR.RFCL is set to 1  and if enabled  a service request is generated."]
    #[inline(always)]
    pub fn cl(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Fcl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Fcl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Fcl {
    #[inline(always)]
    fn default() -> Fcl {
        <crate::RegValueT<Fcl_SPEC> as RegisterValue<_>>::new(128)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frf_SPEC;
impl crate::sealed::RegSpec for Frf_SPEC {
    type DataType = u32;
}
#[doc = "FIFO Rejection Filter\n resetvalue={Application Reset:0x1800000}"]
pub type Frf = crate::RegValueT<Frf_SPEC>;

impl Frf {
    #[doc = "Channel Filter   CH. May be modified in  DEFAULT CONFIG  or  CONFIG  state only."]
    #[inline(always)]
    pub fn ch(self) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Frf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0x3, 1, 0, u8, Frf_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Frame ID Filter   FID. Determines the Frame ID to be rejected by the FIFO. With the additional configuration of register FRFM  the corresponding Frame ID filter bits are ignored  which results in further rejected Frame IDs. When FRFM.MFID is zero  a Frame ID filter value of zero means that no Frame ID is rejected."]
    #[inline(always)]
    pub fn fid(
        self,
    ) -> crate::common::RegisterField<2, 0x7ff, 1, 0, u16, Frf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x7ff,1,0,u16, Frf_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Cycle Counter Filter   CYF. The 7 bit cycle counter filter determines the cycle set to which Frame        ID and channel rejection filter are applied. In cycles not belonging to the cycle set specified by CYF  all Frames are        rejected. For details about the configuration of the cycle counter        filter see   8220 Cycle Counter Filtering  8221 . May be modified in          8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221  state only."]
    #[inline(always)]
    pub fn cyf(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Frf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Frf_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reject in Static Segment   RSS. If this bit is set  the FIFO is used only be used in dynamic segment. May be modified in  DEFAULT CONFIG  or  CONFIG  state only."]
    #[inline(always)]
    pub fn rss(self) -> crate::common::RegisterFieldBool<23, 1, 0, Frf_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Frf_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Reject NULL Frames   RNF. If this bit is set  received NULL Frames are not stored in the FIFO. May be modified in  DEFAULT CONFIG  or  CONFIG  state only."]
    #[inline(always)]
    pub fn rnf(self) -> crate::common::RegisterFieldBool<24, 1, 0, Frf_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Frf_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Frf {
    #[inline(always)]
    fn default() -> Frf {
        <crate::RegValueT<Frf_SPEC> as RegisterValue<_>>::new(25165824)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frfm_SPEC;
impl crate::sealed::RegSpec for Frfm_SPEC {
    type DataType = u32;
}
#[doc = "FIFO Rejection Filter Mask\n resetvalue={Application Reset:0x0}"]
pub type Frfm = crate::RegValueT<Frfm_SPEC>;

impl Frfm {
    #[doc = "Mask Frame ID Filter   MFID. May be modified in   8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221  state only. When  0         written in a bit position  the corresponding Frame ID filter bit is used        for rejection filtering. When  1  written in a bit position  the        corresponding Frame ID filter bit is ignored. Valid values are from        0x000   0x3FF."]
    #[inline(always)]
    pub fn mfid(
        self,
    ) -> crate::common::RegisterField<2, 0x7ff, 1, 0, u16, Frfm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x7ff,1,0,u16, Frfm_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Frfm {
    #[inline(always)]
    fn default() -> Frfm {
        <crate::RegValueT<Frfm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsr_SPEC;
impl crate::sealed::RegSpec for Fsr_SPEC {
    type DataType = u32;
}
#[doc = "FIFO Status Register\n resetvalue={Application Reset:0x0}"]
pub type Fsr = crate::RegValueT<Fsr_SPEC>;

impl Fsr {
    #[doc = "Receive FIFO Not Empty   RFNE. This flag is set by the Communication Controller when a received valid Frame  data or NULL Frame depending on rejection mask  was stored in the FIFO. In addition  service request flag SIR.RFNE is set. The bit is reset after the Host has read all message from the FIFO."]
    #[inline(always)]
    pub fn rfne(self) -> crate::common::RegisterFieldBool<0, 1, 0, Fsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Fsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive FIFO Critical Level   RFCL. This flag is set when the receive FIFO fill level RFFL is equal or greater than the critical level as configured by FCL.CL. The flag is cleared by the Communication Controller as soon as RFFL drops below FCL.CL. When RFCL changes from 0 to 1 bit SIR.RFCL is set to 1  and if enabled  an service request is generated."]
    #[inline(always)]
    pub fn rfcl(self) -> crate::common::RegisterFieldBool<1, 1, 0, Fsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Fsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive FIFO Overrun   RFO. The flag is set by the Communication Controller when a receive FIFO overrun is detected. When a receive FIFO overrun occurs  the oldest message is overwritten with the actual received message. In addition  service request flag EIR.RFO is set.The flag is cleared by the next FIFO read access issued by the Host."]
    #[inline(always)]
    pub fn rfo(self) -> crate::common::RegisterFieldBool<2, 1, 0, Fsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Fsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive FIFO Fill Level   RFFL. Number of FIFO buffers filled up with new data not yet read by the Host. Maximum value is 128."]
    #[inline(always)]
    pub fn rffl(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Fsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Fsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Fsr {
    #[inline(always)]
    fn default() -> Fsr {
        <crate::RegValueT<Fsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuc01_SPEC;
impl crate::sealed::RegSpec for Gtuc01_SPEC {
    type DataType = u32;
}
#[doc = "GTU Configuration Register 1\n resetvalue={Application Reset:0x280}"]
pub type Gtuc01 = crate::RegValueT<Gtuc01_SPEC>;

impl Gtuc01 {
    #[doc = "Microtick per Cycle pMicroPerCycle    UT. Configures the duration of the communication cycle in Microticks. Valid        values are 640 to 640000  280 H to 9C400 H          Microticks."]
    #[inline(always)]
    pub fn ut(
        self,
    ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, Gtuc01_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfffff,1,0,u32, Gtuc01_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtuc01 {
    #[inline(always)]
    fn default() -> Gtuc01 {
        <crate::RegValueT<Gtuc01_SPEC> as RegisterValue<_>>::new(640)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuc02_SPEC;
impl crate::sealed::RegSpec for Gtuc02_SPEC {
    type DataType = u32;
}
#[doc = "GTU Configuration Register 2\n resetvalue={Application Reset:0x2000A}"]
pub type Gtuc02 = crate::RegValueT<Gtuc02_SPEC>;

impl Gtuc02 {
    #[doc = "Macrotick Per Cycle gMacroPerCycle    MPC. Configures the duration of one communication cycle in Macroticks. The        cycle length must be identical in all nodes of a cluster. Valid values        are 10 to 16000  A H to 3E80 H          Macroticks."]
    #[inline(always)]
    pub fn mpc(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Gtuc02_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Gtuc02_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sync Node Max gSyncNodeMax    SNM. Maximum number of Frames within a cluster with SYNC Frame indicator bit        SYN set to 1. Must be identical in all nodes of a cluster. Valid values        are 2 to 15  2 H to F H  ."]
    #[inline(always)]
    pub fn snm(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Gtuc02_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Gtuc02_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtuc02 {
    #[inline(always)]
    fn default() -> Gtuc02 {
        <crate::RegValueT<Gtuc02_SPEC> as RegisterValue<_>>::new(131082)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuc03_SPEC;
impl crate::sealed::RegSpec for Gtuc03_SPEC {
    type DataType = u32;
}
#[doc = "GTU Configuration Register 3\n resetvalue={Application Reset:0x2020000}"]
pub type Gtuc03 = crate::RegValueT<Gtuc03_SPEC>;

impl Gtuc03 {
    #[doc = "Microtick Initial Offset Channel A pMicroInitialOffset A     UIOA. Configures the number of Microticks between the actual time reference        point on channel A and the subsequent Macrotick boundary of the        secondary time reference point. The parameter depends on        pDelayCompensation A  and therefore has to be set for each channel        independently. Valid values are 0 to 240  0 H to F0 H   Microticks."]
    #[inline(always)]
    pub fn uioa(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Gtuc03_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Gtuc03_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Microtick Initial Offset Channel B  pMicroInitialOffset B     UIOB. Configures the number of Microticks between the actual time reference        point on channel B and the subsequent Macrotick boundary of the        secondary time reference point. The parameter depends on        pDelayCompensation B  and therefore has to be set for each channel        independently. Valid values are 0 to 240  0 H to F0 H   Microticks."]
    #[inline(always)]
    pub fn uiob(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Gtuc03_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Gtuc03_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Macrotick Initial Offset Channel A gMacroInitialOffset A     MIOA. Configures the number of Macroticks between the static slot boundary and        the subsequent Macrotick boundary of the secondary time reference point        based on the nominal Macrotick duration. Must be identical in all nodes        of a cluster. Valid values are 2 to 72  2 H to 48 H   Macroticks."]
    #[inline(always)]
    pub fn mioa(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Gtuc03_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Gtuc03_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Macrotick Initial Offset Channel B gMacroInitialOffset B     MIOB. Configures the number of Macroticks between the static slot boundary and        the subsequent Macrotick boundary of the secondary time reference point        based on the nominal Macrotick duration. Must be identical in all nodes        of a cluster. Valid values are 2 to 72  2 H to 48 H   Macroticks."]
    #[inline(always)]
    pub fn miob(
        self,
    ) -> crate::common::RegisterField<24, 0x7f, 1, 0, u8, Gtuc03_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7f,1,0,u8, Gtuc03_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtuc03 {
    #[inline(always)]
    fn default() -> Gtuc03 {
        <crate::RegValueT<Gtuc03_SPEC> as RegisterValue<_>>::new(33685504)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuc04_SPEC;
impl crate::sealed::RegSpec for Gtuc04_SPEC {
    type DataType = u32;
}
#[doc = "GTU Configuration Register 4\n resetvalue={Application Reset:0x080007}"]
pub type Gtuc04 = crate::RegValueT<Gtuc04_SPEC>;

impl Gtuc04 {
    #[doc = "Network Idle Time Start gMacroPerCycle   gdNIT   1    NIT. Configures the starting point of the Network Idle Time  NIT  at the end        of the communication cycle expressed in terms of Macroticks from the        beginning of the cycle. The start of network idle time  NIT  is        recognized if Macrotick   gMacroPerCycle   gdNIT  1 and the increment        pulse of Macrotick is set. Must be identical in all nodes of a cluster.        Valid values are 7 to 15997  7 H to 3E7D H          Macroticks."]
    #[inline(always)]
    pub fn nit(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Gtuc04_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Gtuc04_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Offset Correction Start  gOffsetCorrectionStart   1    OCS. Determines the start of the offset correction within the network idle        time  NIT  phase  calculated from start of cycle. Must be identical in        all nodes of a cluster. For cluster consisting of E Ray implementations only  it is sufficient to program OCS   NIT   1. Valid        values are 8 to 15998  8 H to 3E7E H          Macroticks."]
    #[inline(always)]
    pub fn ocs(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, Gtuc04_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3fff,1,0,u16, Gtuc04_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtuc04 {
    #[inline(always)]
    fn default() -> Gtuc04 {
        <crate::RegValueT<Gtuc04_SPEC> as RegisterValue<_>>::new(524295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuc05_SPEC;
impl crate::sealed::RegSpec for Gtuc05_SPEC {
    type DataType = u32;
}
#[doc = "GTU Configuration Register 5\n resetvalue={Application Reset:0x0E000000}"]
pub type Gtuc05 = crate::RegValueT<Gtuc05_SPEC>;

impl Gtuc05 {
    #[doc = "Delay Compensation Channel A pDelayCompensation A     DCA. Used to compensate for reception delays on channel A. This covers        assumed propagation delay up to cPropagationDelayMax for Microticks in        the range of 0.0125  160   181 s to 0.05  160   181 s. In practice  the minimum of the        propagation delays of all sync nodes should be applied. Valid values are 0 to 200  0 H to C8 H          Microticks."]
    #[inline(always)]
    pub fn dca(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Gtuc05_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Gtuc05_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Delay Compensation Channel B  pDelayCompensation B     DCB. Used to compensate for reception delays on channel B. This covers        assumed propagation delay up to cPropagationDelayMax for Microticks in        the range of 0.0125 to 0.05  160   181 s. In practice  the minimum of the        propagation delays of all sync nodes should be applied. Valid values are 0 to 200  0 H to C8 H          Microticks."]
    #[inline(always)]
    pub fn dcb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Gtuc05_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Gtuc05_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Cluster Drift Damping pClusterDriftDamping    CDD. Configures the cluster drift damping value used in clock synchronization        to minimize accumulation of rounding errors. Valid values are 0 to 20  0 H to 14 H   Microticks."]
    #[inline(always)]
    pub fn cdd(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Gtuc05_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Gtuc05_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Decoding Correction pDecodingCorrection    DEC. Configures the decoding correction value used to determine the primary        time reference point. Valid values are 14 to 143  E H to 8F H   Microticks."]
    #[inline(always)]
    pub fn dec(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Gtuc05_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Gtuc05_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtuc05 {
    #[inline(always)]
    fn default() -> Gtuc05 {
        <crate::RegValueT<Gtuc05_SPEC> as RegisterValue<_>>::new(234881024)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuc06_SPEC;
impl crate::sealed::RegSpec for Gtuc06_SPEC {
    type DataType = u32;
}
#[doc = "GTU Configuration Register 6\n resetvalue={Application Reset:0x20000}"]
pub type Gtuc06 = crate::RegValueT<Gtuc06_SPEC>;

impl Gtuc06 {
    #[doc = "Accepted Startup Range pdAcceptedStartupRange    ASR. Number of Microticks constituting the expanded range of measured deviation for startup Frames during integration. Valid values are 0 to 1875  0 H to 753 H   Microticks."]
    #[inline(always)]
    pub fn asr(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Gtuc06_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Gtuc06_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Maximum Oscillator Drift pdMaxDrift 1    MOD. Maximum drift offset between two nodes that operate with unsynchronized clocks over one communication cycle in Microticks. Valid values are 2 to 1923  2 H to 783 H   Microticks."]
    #[inline(always)]
    pub fn r#mod(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, Gtuc06_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7ff,1,0,u16, Gtuc06_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtuc06 {
    #[inline(always)]
    fn default() -> Gtuc06 {
        <crate::RegValueT<Gtuc06_SPEC> as RegisterValue<_>>::new(131072)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuc07_SPEC;
impl crate::sealed::RegSpec for Gtuc07_SPEC {
    type DataType = u32;
}
#[doc = "GTU Configuration Register 7\n resetvalue={Application Reset:0x20004}"]
pub type Gtuc07 = crate::RegValueT<Gtuc07_SPEC>;

impl Gtuc07 {
    #[doc = "Static Slot Length gdStaticSlot    SSL. Configures the duration of a static slot in Macroticks. The static slot        length must be identical in all nodes of a cluster. Valid values are 4        to 659  4 H to 293 H          Macroticks."]
    #[inline(always)]
    pub fn ssl(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Gtuc07_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Gtuc07_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Number of Static Slots gNumberOfStaticSlots    NSS. Configures the number of static slots in a cycle. At least 2 coldstart        nodes must be configured to startup a FlexRay  8482  network. The number of        static slots must be identical in all nodes of a cluster. Valid values        are 2 to 1023  2 H to 3FF H  ."]
    #[inline(always)]
    pub fn nss(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Gtuc07_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Gtuc07_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtuc07 {
    #[inline(always)]
    fn default() -> Gtuc07 {
        <crate::RegValueT<Gtuc07_SPEC> as RegisterValue<_>>::new(131076)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuc08_SPEC;
impl crate::sealed::RegSpec for Gtuc08_SPEC {
    type DataType = u32;
}
#[doc = "GTU Configuration Register 8\n resetvalue={Application Reset:0x2}"]
pub type Gtuc08 = crate::RegValueT<Gtuc08_SPEC>;

impl Gtuc08 {
    #[doc = "Minislot Length gdMinislot    MSL. Configures the duration of a minislot in Macroticks. The minislot length        must be identical in all nodes of a cluster. Valid values are 2 to 63  2 H to 3F H   Macroticks."]
    #[inline(always)]
    pub fn msl(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Gtuc08_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Gtuc08_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Number of Minislots gNumberOfMinislots    NMS. Configures the number of minislots within the dynamic segment of a        cycle. The number of minislots must be identical in all nodes of a        cluster. Valid values are 0 to 7986  0 H to 1F32 H  ."]
    #[inline(always)]
    pub fn nms(
        self,
    ) -> crate::common::RegisterField<16, 0x1fff, 1, 0, u16, Gtuc08_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1fff,1,0,u16, Gtuc08_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtuc08 {
    #[inline(always)]
    fn default() -> Gtuc08 {
        <crate::RegValueT<Gtuc08_SPEC> as RegisterValue<_>>::new(2)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuc09_SPEC;
impl crate::sealed::RegSpec for Gtuc09_SPEC {
    type DataType = u32;
}
#[doc = "GTU Configuration Register 9\n resetvalue={Application Reset:0x101}"]
pub type Gtuc09 = crate::RegValueT<Gtuc09_SPEC>;

impl Gtuc09 {
    #[doc = "Action Point Offset gdActionPointOffset    APO. Configures the action point offset in Macroticks within static slots and        symbol window. Must be identical in all nodes of a cluster. Valid values        are 1 to 63  1 H to 3F H          Macroticks."]
    #[inline(always)]
    pub fn apo(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Gtuc09_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Gtuc09_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Minislot Action Point Offset  gd Minislot Action Point Offset    MAPO. Configures the action point offset in Macroticks within the minislots of        the dynamic segment. Must be identical in all nodes of a cluster. Valid        values are 1 to 31  1 H to 1F H          Macroticks."]
    #[inline(always)]
    pub fn mapo(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, Gtuc09_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, Gtuc09_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Dynamic Slot Idle Phase  gdDynamicSlotIdlePhase    DSI. The duration of the dynamic slot idle phase has to be greater or equal        than the idle detection time. Must be identical in all nodes of a        cluster. Valid values are 0 to 2 Minislot."]
    #[inline(always)]
    pub fn dsi(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Gtuc09_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Gtuc09_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtuc09 {
    #[inline(always)]
    fn default() -> Gtuc09 {
        <crate::RegValueT<Gtuc09_SPEC> as RegisterValue<_>>::new(257)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuc10_SPEC;
impl crate::sealed::RegSpec for Gtuc10_SPEC {
    type DataType = u32;
}
#[doc = "GTU Configuration Register 10\n resetvalue={Application Reset:0x20005}"]
pub type Gtuc10 = crate::RegValueT<Gtuc10_SPEC>;

impl Gtuc10 {
    #[doc = "Maximum Offset Correction pOffsetCorrectionOut    MOC. Holds the maximum permitted offset correction value to be applied by the        internal clock synchronization algorithm  absolute value . The        Communication Controller checks only the internal offset correction        value against the maximum offset correction value. Valid values are 5 to        15266  5 H to 3BA2 H          Microticks."]
    #[inline(always)]
    pub fn moc(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Gtuc10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Gtuc10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Maximum Rate Correction  pRateCorrectionOut    MRC. Holds the maximum permitted rate correction value to be applied by the        internal clock synchronization algorithm. The communication controller        checks only the internal rate correction value against the maximum rate        correction value  absolute value . Valid values are 2 to 1923  2 H to 783 H   Microticks."]
    #[inline(always)]
    pub fn mrc(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, Gtuc10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7ff,1,0,u16, Gtuc10_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtuc10 {
    #[inline(always)]
    fn default() -> Gtuc10 {
        <crate::RegValueT<Gtuc10_SPEC> as RegisterValue<_>>::new(131077)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuc11_SPEC;
impl crate::sealed::RegSpec for Gtuc11_SPEC {
    type DataType = u32;
}
#[doc = "GTU Configuration Register 11\n resetvalue={Application Reset:0x0}"]
pub type Gtuc11 = crate::RegValueT<Gtuc11_SPEC>;

impl Gtuc11 {
    #[doc = "External Offset Correction Control pExternOffsetControl    EOCC. By writing to EOCC the external offset correction is enabled as        specified below. Should be modified only outside network idle time  NIT ."]
    #[inline(always)]
    pub fn eocc(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Gtuc11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Gtuc11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Rate Correction Control pExternRateControl    ERCC. By writing to ERCC the external rate correction is enabled as specified        below. Should be modified only outside network idle time  NIT ."]
    #[inline(always)]
    pub fn ercc(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Gtuc11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Gtuc11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Offset Correction pExternOffsetCorrection    EOC. Holds the external clock offset correction value in Microticks to be        applied by the internal synchronization algorithm. The value is        subtracted   added from   to the calculated offset correction value. The        value is applied during network idle time  NIT . May be modified in          8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221  state only. Valid values are 0 to 7        Microticks."]
    #[inline(always)]
    pub fn eoc(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Gtuc11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Gtuc11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Rate Correction  pExternRateCorrection    ERC. Holds the external rate correction value in Microticks to be applied by        the internal clock synchronization algorithm. The value is subtracted          added from   to the calculated rate correction value. The value is        applied during network idle time  NIT . Can be modified in          8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221  state only. Valid values are 0 to 7        Microticks."]
    #[inline(always)]
    pub fn erc(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Gtuc11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Gtuc11_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtuc11 {
    #[inline(always)]
    fn default() -> Gtuc11 {
        <crate::RegValueT<Gtuc11_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ibcm_SPEC;
impl crate::sealed::RegSpec for Ibcm_SPEC {
    type DataType = u32;
}
#[doc = "Input Buffer Command Mask\n resetvalue={Application Reset:0x0}"]
pub type Ibcm = crate::RegValueT<Ibcm_SPEC>;

impl Ibcm {
    #[doc = "Load Header Section Host   LHSH"]
    #[inline(always)]
    pub fn lhsh(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ibcm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ibcm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Load Data Section Host   LDSH"]
    #[inline(always)]
    pub fn ldsh(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ibcm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ibcm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Transmission Request Host   STXRH. If this bit is set to 1  the Transmission Request flag TXRQ1.TXRn  n   0 31  to TXRQ4.TXRn  n   96 127  for the selected Message Buffer is set in the Transmission Request Registers to release the Message Buffer for transmission. In single shot mode the flag is cleared by the Communication Controller after transmission has completed. TXRQ1.TXRn  n   0 31  to TXRQ4.TXRn  n   96 127  are evaluated for transmit buffer only."]
    #[inline(always)]
    pub fn stxrh(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ibcm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ibcm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Load Header Section Shadow   LHSS"]
    #[inline(always)]
    pub fn lhss(self) -> crate::common::RegisterFieldBool<16, 1, 0, Ibcm_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ibcm_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Load Data Section Shadow   LDSS"]
    #[inline(always)]
    pub fn ldss(self) -> crate::common::RegisterFieldBool<17, 1, 0, Ibcm_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ibcm_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request Shadow   STXRS. If this bit is set to 1  the Transmission Request flag TXRQ1.TXRn  n   0 31  to TXRQ4.TXRn  n   96 127  for the selected Message Buffer is set in the Transmission Request Registers to release the Message Buffer for transmission. In single shot mode the flag is cleared by the Communication Controller after transmission has completed. TXRQ1.TXRn  n   0 31  to TXRQ4.TXRn  n   96 127  are evaluated for transmit buffer only."]
    #[inline(always)]
    pub fn stxrs(self) -> crate::common::RegisterFieldBool<18, 1, 0, Ibcm_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ibcm_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ibcm {
    #[inline(always)]
    fn default() -> Ibcm {
        <crate::RegValueT<Ibcm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ibcr_SPEC;
impl crate::sealed::RegSpec for Ibcr_SPEC {
    type DataType = u32;
}
#[doc = "Input Buffer Command Request\n resetvalue={Application Reset:0x0}"]
pub type Ibcr = crate::RegValueT<Ibcr_SPEC>;

impl Ibcr {
    #[doc = "Input Buffer Request Host   IBRH. Selects the target Message Buffer in the Message RAM for data transfer from Input Buffer. Valid values are 00 H to 7F H  0 127 ."]
    #[inline(always)]
    pub fn ibrh(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Ibcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8, Ibcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Buffer Busy Host   IBSYH. Set to 1 by writing IBRH while IBSYS is still 1. After the ongoing transfer between IBF Shadow and the Message RAM has completed  the IBSYH is set back to 0."]
    #[inline(always)]
    pub fn ibsyh(self) -> crate::common::RegisterFieldBool<15, 1, 0, Ibcr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ibcr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Buffer Request Shadow   IBRS. Number of the target Message Buffer actually updated lately updated. Valid values are 00 H to 7F H  0 127 ."]
    #[inline(always)]
    pub fn ibrs(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Ibcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Ibcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Input Buffer Busy Shadow   IBSYS. Set to 1 after writing IBRH. When the transfer between IBF Shadow and the Message RAM has completed  IBSYS is set back to 0."]
    #[inline(always)]
    pub fn ibsys(self) -> crate::common::RegisterFieldBool<31, 1, 0, Ibcr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ibcr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ibcr {
    #[inline(always)]
    fn default() -> Ibcr {
        <crate::RegValueT<Ibcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x44C000}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MOD REV. MOD REV defines the module revision number. The value of a module revision starts with 01 H  first revision ."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MOD TYPE. The value of this bit field is C0 H . It defines the module as a 32 bit module."]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MOD NUMBER. This bit field defines a module identification number. For the E Ray module the module identification number is 44 H ."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(4505600)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ile_SPEC;
impl crate::sealed::RegSpec for Ile_SPEC {
    type DataType = u32;
}
#[doc = "Service Request Line Enable\n resetvalue={Application Reset:0x0}"]
pub type Ile = crate::RegValueT<Ile_SPEC>;

impl Ile {
    #[doc = "Enable Service Request Line 0 INT0SRC    EINT0"]
    #[inline(always)]
    pub fn eint0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ile_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ile_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Service Request Line 1 INT1SRC    EINT1"]
    #[inline(always)]
    pub fn eint1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ile_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ile_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ile {
    #[inline(always)]
    fn default() -> Ile {
        <crate::RegValueT<Ile_SPEC> as RegisterValue<_>>::new(0)
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
pub struct Lck_SPEC;
impl crate::sealed::RegSpec for Lck_SPEC {
    type DataType = u32;
}
#[doc = "Lock Register\n resetvalue={Application Reset:0x0}"]
pub type Lck = crate::RegValueT<Lck_SPEC>;

impl Lck {
    #[doc = "Configuration Lock Key   CLK. To leave  CONFIG  state by writing to SUCC1.CMD commands READY  MONITOR MODE  ATM  LOOP BACK  in the SUC Configuration Register 1  the write operation has to be directly preceded by two consecutive write accesses to the Configuration Lock Key  unlock sequence . If the write sequence below is interrupted by other write accesses between the second write to the Configuration Lock Key and the write access to the SUCC1 register  the Communication Controller remains in  CONFIG  state and the sequence has to be repeated. First write  LCK.CLK   CE H   1100 1110 B Second write  LCK.CLK   31 H   0011 0001 B Third write  SUCC1.CMD Returns 0 if read"]
    #[inline(always)]
    pub fn clk(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Lck_SPEC, crate::common::W> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Lck_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Test Mode Key   TMK. To set bit TEST1.WRTEN the write operation has to be directly preceded by two consecutive write accesses to the Test Mode Key. If the write sequence is interrupted by other write accesses between the second write to the Test Mode Key and the write access to the Test1 register  bit TEST1.WRTEN is not set to 1 and the sequence has to be repeated. First write  LCK.TMK   75 H   0111 0101 B Second write  LCK.TMK   8A H   1000 1010 B Second write  TEST1.WRTEN   1 Returns 0 if read"]
    #[inline(always)]
    pub fn tmk(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Lck_SPEC, crate::common::W> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Lck_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Lck {
    #[inline(always)]
    fn default() -> Lck {
        <crate::RegValueT<Lck_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldts_SPEC;
impl crate::sealed::RegSpec for Ldts_SPEC {
    type DataType = u32;
}
#[doc = "Last Dynamic Transmit Slot\n resetvalue={Application Reset:0x0}"]
pub type Ldts = crate::RegValueT<Ldts_SPEC>;

impl Ldts {
    #[doc = "Last Dynamic Transmission Channel A   LDTA. Value of  vSlotCounter A   at the time of the last Frame transmission on channel A in the dynamic segment of this node. It is updated at the end of the dynamic segment and is reset to zero if no Frame was transmitted during the dynamic segment."]
    #[inline(always)]
    pub fn ldta(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Ldts_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Ldts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Last Dynamic Transmission Channel B   LDTB. Value of  vSlotCounter B   at the time of the last Frame transmission on channel B in the dynamic segment of this node. It is updated at the end of the dynamic segment and is reset to zero if no Frame was transmitted during the dynamic segment."]
    #[inline(always)]
    pub fn ldtb(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, Ldts_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7ff,1,0,u16, Ldts_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ldts {
    #[inline(always)]
    fn default() -> Ldts {
        <crate::RegValueT<Ldts_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbs_SPEC;
impl crate::sealed::RegSpec for Mbs_SPEC {
    type DataType = u32;
}
#[doc = "Message Buffer Status\n resetvalue={Application Reset:0x0}"]
pub type Mbs = crate::RegValueT<Mbs_SPEC>;

impl Mbs {
    #[doc = "Valid Frame Received on Channel A vSS ValidFrameA    VFRA. A valid Frame indication is set if a valid Frame was received on channel A."]
    #[inline(always)]
    pub fn vfra(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mbs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Valid Frame Received on Channel B vSS ValidFrameB    VFRB. A valid Frame indication is set if a valid Frame was received on channel B."]
    #[inline(always)]
    pub fn vfrb(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mbs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Syntax Error Observed on Channel A vSS SyntaxErrorA    SEOA. A syntax error was observed in the assigned slot on channel A."]
    #[inline(always)]
    pub fn seoa(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mbs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Syntax Error Observed on Channel B vSS SyntaxErrorB    SEOB. A syntax error was observed in the assigned slot on channel B."]
    #[inline(always)]
    pub fn seob(self) -> crate::common::RegisterFieldBool<3, 1, 0, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Mbs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Content Error Observed on Channel A vSS ContentErrorA    CEOA. A content error was observed in the assigned slot on channel A."]
    #[inline(always)]
    pub fn ceoa(self) -> crate::common::RegisterFieldBool<4, 1, 0, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Mbs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Content Error Observed on Channel B vSS ContentErrorB    CEOB. A content error was observed in the assigned slot on channel B."]
    #[inline(always)]
    pub fn ceob(self) -> crate::common::RegisterFieldBool<5, 1, 0, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Mbs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Slot Boundary Violation Observed on Channel A vSS BViolationA    SVOA. A slot boundary violation  channel active at the start or at the end of the assigned slot  was observed on channel A."]
    #[inline(always)]
    pub fn svoa(self) -> crate::common::RegisterFieldBool<6, 1, 0, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Mbs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Slot Boundary Violation Observed on Channel B vSS BViolationB    SVOB. A slot boundary violation  channel active at the start or at the end of the assigned slot  was observed on channel B."]
    #[inline(always)]
    pub fn svob(self) -> crate::common::RegisterFieldBool<7, 1, 0, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Mbs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Conflict Indication Channel A vSS TxConflictA    TCIA. A transmission conflict indication is set if a transmission conflict has occurred on channel A."]
    #[inline(always)]
    pub fn tcia(self) -> crate::common::RegisterFieldBool<8, 1, 0, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Mbs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Conflict Indication Channel B vSS TxConflictB    TCIB. A transmission conflict indication is set if a transmission conflict has occurred on channel B."]
    #[inline(always)]
    pub fn tcib(self) -> crate::common::RegisterFieldBool<9, 1, 0, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Mbs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Empty Slot Channel A   ESA. In an empty slot there is no activity detected on the bus. The condition is checked in static and dynamic slots."]
    #[inline(always)]
    pub fn esa(self) -> crate::common::RegisterFieldBool<10, 1, 0, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Mbs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Empty Slot Channel B   ESB. In an empty slot there is no activity detected on the bus. The condition is checked in static and dynamic slots."]
    #[inline(always)]
    pub fn esb(self) -> crate::common::RegisterFieldBool<11, 1, 0, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Mbs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Lost   MLST. The flag is set in case the Host did not read the message before the Message Buffer was updated from a received Data Frame. Not affected by reception of NULL Frames except for Message Buffers belonging to the receive FIFO. The flag is reset by a Host write to the Message Buffer via IBF or when a new message is stored into the Message Buffer after the Message Buffers ND flag was reset by reading out the Message Buffer via OBF."]
    #[inline(always)]
    pub fn mlst(self) -> crate::common::RegisterFieldBool<12, 1, 0, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Mbs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Frame Transmitted on Channel A   FTA. Indicates that this node has transmitted a Data Frame in the assigned slot on channel A. The FlexRay  protocol specification requires that FTA can only be reset by the Host. Therefore the Cycle Count Status CCS for these bits is only valid for the cycle where the bits are set to 1"]
    #[inline(always)]
    pub fn fta(self) -> crate::common::RegisterFieldBool<14, 1, 0, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Mbs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Frame Transmitted on Channel B   FTB. Indicates that this node has transmitted a Data Frame in the assigned slot on channel B. The FlexRay  protocol specification requires that FTB can only be reset by the Host. Therefore the Cycle Count Status CCS for these bits is only valid for the cycle where the bits are set to 1"]
    #[inline(always)]
    pub fn ftb(self) -> crate::common::RegisterFieldBool<15, 1, 0, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Mbs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Cycle Count Status   CCS. Cycle Count when status  MBS register  has been updated."]
    #[inline(always)]
    pub fn ccs(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Mbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Received on Channel Indicator Status vSS Channel    RCIS. Indicates the channel on which the Frame was received. For receive buffers  CFG   0  the RCIS is updated from both valid data and NULL Frames. If no valid Frame was received  the previous value is maintained. For transmit buffers the flags have no meaning and should be ignored."]
    #[inline(always)]
    pub fn rcis(self) -> crate::common::RegisterFieldBool<24, 1, 0, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Mbs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Startup Frame Indicator Status vRF Header SuFIndicator    SFIS. A Startup Frame is marked by the Startup Frame indicator. For receive buffers  CFG   0  the SFIS is updated from both valid data and NULL Frames. If no valid Frame was received  the previous value is maintained. For transmit buffers the flags have no meaning and should be ignored."]
    #[inline(always)]
    pub fn sfis(self) -> crate::common::RegisterFieldBool<25, 1, 0, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Mbs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SYNC Frame Indicator Status vRF Header SyFIndicator    SYNS. A SYNC Frame is marked by the SYNC Frame indicator. For receive buffers  CFG   0  the SYNS is updated from both valid data and NULL Frames. If no valid Frame was received  the previous value is maintained. For transmit buffers the flags have no meaning and should be ignored."]
    #[inline(always)]
    pub fn syns(self) -> crate::common::RegisterFieldBool<26, 1, 0, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Mbs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NULL Frame Indicator Status vRF Header NFIndicator    NFIS. If reset to 0 the Payload Segment of the received Frame contains no usable data. For receive buffers  CFG   0  the NFIS is updated from both valid data and NULL Frames. If no valid Frame was received  the previous value is maintained. For transmit buffers the flags have no meaning and should be ignored."]
    #[inline(always)]
    pub fn nfis(self) -> crate::common::RegisterFieldBool<27, 1, 0, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Mbs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Payload Preamble Indictor Status vRF Header PPIndicator    PPIS. The payload preamble indicator defines whether a Network Management vector or message ID is contained within the Payload Segment of the received Frame. For receive buffers  CFG   0  the PPIS is updated from both valid data and NULL Frames. If no valid Frame was received  the previous value is maintained. For transmit buffers the flags have no meaning and should be ignored."]
    #[inline(always)]
    pub fn ppis(self) -> crate::common::RegisterFieldBool<28, 1, 0, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Mbs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mbs {
    #[inline(always)]
    fn default() -> Mbs {
        <crate::RegValueT<Mbs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbsc1_SPEC;
impl crate::sealed::RegSpec for Mbsc1_SPEC {
    type DataType = u32;
}
#[doc = "Message Buffer Status Changed 1\n resetvalue={Application Reset:0x0}"]
pub type Mbsc1 = crate::RegValueT<Mbsc1_SPEC>;

impl Mbsc1 {
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc25(self) -> crate::common::RegisterFieldBool<25, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc26(self) -> crate::common::RegisterFieldBool<26, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc27(self) -> crate::common::RegisterFieldBool<27, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc28(self) -> crate::common::RegisterFieldBool<28, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc29(self) -> crate::common::RegisterFieldBool<29, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc30(self) -> crate::common::RegisterFieldBool<30, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc31(self) -> crate::common::RegisterFieldBool<31, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mbsc1 {
    #[inline(always)]
    fn default() -> Mbsc1 {
        <crate::RegValueT<Mbsc1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbsc2_SPEC;
impl crate::sealed::RegSpec for Mbsc2_SPEC {
    type DataType = u32;
}
#[doc = "Message Buffer Status Changed 2\n resetvalue={Application Reset:0x0}"]
pub type Mbsc2 = crate::RegValueT<Mbsc2_SPEC>;

impl Mbsc2 {
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc42(self) -> crate::common::RegisterFieldBool<10, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc43(self) -> crate::common::RegisterFieldBool<11, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc44(self) -> crate::common::RegisterFieldBool<12, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc45(self) -> crate::common::RegisterFieldBool<13, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc46(self) -> crate::common::RegisterFieldBool<14, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc47(self) -> crate::common::RegisterFieldBool<15, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc48(self) -> crate::common::RegisterFieldBool<16, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc49(self) -> crate::common::RegisterFieldBool<17, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc50(self) -> crate::common::RegisterFieldBool<18, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc51(self) -> crate::common::RegisterFieldBool<19, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc52(self) -> crate::common::RegisterFieldBool<20, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc53(self) -> crate::common::RegisterFieldBool<21, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc54(self) -> crate::common::RegisterFieldBool<22, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc55(self) -> crate::common::RegisterFieldBool<23, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc56(self) -> crate::common::RegisterFieldBool<24, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc57(self) -> crate::common::RegisterFieldBool<25, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc58(self) -> crate::common::RegisterFieldBool<26, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc59(self) -> crate::common::RegisterFieldBool<27, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc60(self) -> crate::common::RegisterFieldBool<28, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc61(self) -> crate::common::RegisterFieldBool<29, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc62(self) -> crate::common::RegisterFieldBool<30, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc63(self) -> crate::common::RegisterFieldBool<31, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mbsc2 {
    #[inline(always)]
    fn default() -> Mbsc2 {
        <crate::RegValueT<Mbsc2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbsc3_SPEC;
impl crate::sealed::RegSpec for Mbsc3_SPEC {
    type DataType = u32;
}
#[doc = "Message Buffer Status Changed 3\n resetvalue={Application Reset:0x0}"]
pub type Mbsc3 = crate::RegValueT<Mbsc3_SPEC>;

impl Mbsc3 {
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc64(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc65(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc66(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc67(self) -> crate::common::RegisterFieldBool<3, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc68(self) -> crate::common::RegisterFieldBool<4, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc69(self) -> crate::common::RegisterFieldBool<5, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc70(self) -> crate::common::RegisterFieldBool<6, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc71(self) -> crate::common::RegisterFieldBool<7, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc72(self) -> crate::common::RegisterFieldBool<8, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc73(self) -> crate::common::RegisterFieldBool<9, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc74(self) -> crate::common::RegisterFieldBool<10, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc75(self) -> crate::common::RegisterFieldBool<11, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc76(self) -> crate::common::RegisterFieldBool<12, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc77(self) -> crate::common::RegisterFieldBool<13, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc78(self) -> crate::common::RegisterFieldBool<14, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc79(self) -> crate::common::RegisterFieldBool<15, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc80(self) -> crate::common::RegisterFieldBool<16, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc81(self) -> crate::common::RegisterFieldBool<17, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc82(self) -> crate::common::RegisterFieldBool<18, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc83(self) -> crate::common::RegisterFieldBool<19, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc84(self) -> crate::common::RegisterFieldBool<20, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc85(self) -> crate::common::RegisterFieldBool<21, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc86(self) -> crate::common::RegisterFieldBool<22, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc87(self) -> crate::common::RegisterFieldBool<23, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc88(self) -> crate::common::RegisterFieldBool<24, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc89(self) -> crate::common::RegisterFieldBool<25, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc90(self) -> crate::common::RegisterFieldBool<26, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc91(self) -> crate::common::RegisterFieldBool<27, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc92(self) -> crate::common::RegisterFieldBool<28, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc93(self) -> crate::common::RegisterFieldBool<29, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc94(self) -> crate::common::RegisterFieldBool<30, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc95(self) -> crate::common::RegisterFieldBool<31, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mbsc3 {
    #[inline(always)]
    fn default() -> Mbsc3 {
        <crate::RegValueT<Mbsc3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbsc4_SPEC;
impl crate::sealed::RegSpec for Mbsc4_SPEC {
    type DataType = u32;
}
#[doc = "Message Buffer Status Changed 4\n resetvalue={Application Reset:0x0}"]
pub type Mbsc4 = crate::RegValueT<Mbsc4_SPEC>;

impl Mbsc4 {
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc96(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc97(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc98(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc99(self) -> crate::common::RegisterFieldBool<3, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc100(self) -> crate::common::RegisterFieldBool<4, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc101(self) -> crate::common::RegisterFieldBool<5, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc102(self) -> crate::common::RegisterFieldBool<6, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc103(self) -> crate::common::RegisterFieldBool<7, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc104(self) -> crate::common::RegisterFieldBool<8, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc105(self) -> crate::common::RegisterFieldBool<9, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc106(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc107(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc108(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc109(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc110(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc111(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc112(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc113(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc114(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc115(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc116(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc117(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc118(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc119(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc120(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc121(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc122(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc123(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc124(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc125(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc126(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc127(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mbsc4 {
    #[inline(always)]
    fn default() -> Mbsc4 {
        <crate::RegValueT<Mbsc4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mhdc_SPEC;
impl crate::sealed::RegSpec for Mhdc_SPEC {
    type DataType = u32;
}
#[doc = "MHD Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Mhdc = crate::RegValueT<Mhdc_SPEC>;

impl Mhdc {
    #[doc = "Static Frame Data Length gPayloadLengthStatic    SFDL. Configures the cluster wide payload length for all Frames sent in the        static segment in double byte. The payload length must be identical in        all nodes of a cluster. Valid values are 0 to 127  0 to 7F H  ."]
    #[inline(always)]
    pub fn sfdl(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Mhdc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8, Mhdc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Start of Latest Transmit pLatestTx     SLT. Configures the maximum minislot value allowed before inhibiting Frame        transmission in the dynamic segment of the cycle. There is no        transmission dynamic segment if SLT is reset to zero. Valid values are 0        to 7981  0 to 1F2D H   minislots."]
    #[inline(always)]
    pub fn slt(
        self,
    ) -> crate::common::RegisterField<16, 0x1fff, 1, 0, u16, Mhdc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1fff,1,0,u16, Mhdc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Mhdc {
    #[inline(always)]
    fn default() -> Mhdc {
        <crate::RegValueT<Mhdc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mhdf_SPEC;
impl crate::sealed::RegSpec for Mhdf_SPEC {
    type DataType = u32;
}
#[doc = "Message Handler Constraints Flags\n resetvalue={Application Reset:0x0}"]
pub type Mhdf = crate::RegValueT<Mhdf_SPEC>;

impl Mhdf {
    #[doc = "Status Not Updated Channel A   SNUA. This flag is set by the Communication Controller when the Message Handler  due to overload condition  was not able to update a Message Buffer s status MBS with respect to channel A."]
    #[inline(always)]
    pub fn snua(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mhdf_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mhdf_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Status Not Updated Channel B   SNUB. This flag is set by the Communication Controller when the Message Handler  due to overload condition  was not able to update a Message Buffer s status MBS with respect to channel B."]
    #[inline(always)]
    pub fn snub(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mhdf_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mhdf_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Find Sequence Not Finished Channel A   FNFA. This flag is set by the Communication Controller when the Message Handler  due to overload condition  was not able to finish a find sequence  scan of Message RAM for matching Message Buffer  with respect to channel A."]
    #[inline(always)]
    pub fn fnfa(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mhdf_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mhdf_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Find Sequence Not Finished Channel B   FNFB. This flag is set by the Communication Controller when the Message Handler  due to overload condition  was not able to finish a find sequence  scan of Message RAM for matching Message Buffer  with respect to channel B."]
    #[inline(always)]
    pub fn fnfb(self) -> crate::common::RegisterFieldBool<3, 1, 0, Mhdf_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Mhdf_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transient Buffer Access Failure A   TBFA. This flag is set by the Communication Controller when a read or write access to Transient Buffer A requested by PRT A could not complete within the available time."]
    #[inline(always)]
    pub fn tbfa(self) -> crate::common::RegisterFieldBool<4, 1, 0, Mhdf_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Mhdf_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transient Buffer Access Failure B   TBFB. This flag is set by the Communication Controller when a read or write access to Transient Buffer B requested by PRT B could not complete within the available time."]
    #[inline(always)]
    pub fn tbfb(self) -> crate::common::RegisterFieldBool<5, 1, 0, Mhdf_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Mhdf_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Not Started Channel A   TNSA. This flag is set by the CC when the Message Handler was not ready to start a scheduled transmission on channel A at the action point of the configured slot."]
    #[inline(always)]
    pub fn tnsa(self) -> crate::common::RegisterFieldBool<6, 1, 0, Mhdf_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Mhdf_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Not Started Channel B   TNSB. This flag is set by the CC when the Message Handler was not ready to start a scheduled transmission on channel B at the action point of the configured slot."]
    #[inline(always)]
    pub fn tnsb(self) -> crate::common::RegisterFieldBool<7, 1, 0, Mhdf_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Mhdf_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Write Attempt to Header Partition   WAHP. Outside  DEFAULT CONFIG  and  CONFIG  state this flag is set by the Communication Controller when the message handler tries to write message data into the Header Partition of the Message RAM due to faulty configuration of a Message Buffer. The write attempt is not executed  to protect the Header Partition from unintended write accesses."]
    #[inline(always)]
    pub fn wahp(self) -> crate::common::RegisterFieldBool<8, 1, 0, Mhdf_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Mhdf_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mhdf {
    #[inline(always)]
    fn default() -> Mhdf {
        <crate::RegValueT<Mhdf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mhds_SPEC;
impl crate::sealed::RegSpec for Mhds_SPEC {
    type DataType = u32;
}
#[doc = "Message Handler Status\n resetvalue={Application Reset:0x0}"]
pub type Mhds = crate::RegValueT<Mhds_SPEC>;

impl Mhds {
    #[doc = "ECC Error Input Buffer RAM 1 2   EIBF"]
    #[inline(always)]
    pub fn eibf(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mhds_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mhds_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ECC Error Output Buffer RAM 1 2   EOBF"]
    #[inline(always)]
    pub fn eobf(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mhds_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mhds_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ECC Error Message RAM   EMR"]
    #[inline(always)]
    pub fn emr(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mhds_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mhds_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ECC Error Transient Buffer RAM A   ETBF1"]
    #[inline(always)]
    pub fn etbf1(self) -> crate::common::RegisterFieldBool<3, 1, 0, Mhds_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Mhds_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ECC Error Transient Buffer RAM B   ETBF2"]
    #[inline(always)]
    pub fn etbf2(self) -> crate::common::RegisterFieldBool<4, 1, 0, Mhds_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Mhds_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Faulty Message Buffer Detected   FMBD"]
    #[inline(always)]
    pub fn fmbd(self) -> crate::common::RegisterFieldBool<5, 1, 0, Mhds_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Mhds_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Multiple Faulty Message Buffers detected   MFMB"]
    #[inline(always)]
    pub fn mfmb(self) -> crate::common::RegisterFieldBool<6, 1, 0, Mhds_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Mhds_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear all internal RAM s   CRAM. Signals that execution of the CHI command CLEAR RAMS is ongoing  all bits of all internal RAM blocks are written to 0 . The bit is set by CHI command CLEAR RAMS."]
    #[inline(always)]
    pub fn cram(self) -> crate::common::RegisterFieldBool<7, 1, 0, Mhds_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Mhds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Faulty Message Buffer   FMB. ECC error occurred when reading from the Message Buffer or when transferring data from Input Buffer or Transient Buffer A or Transient Buffer B to the Message Buffer referenced by MHDS.FMB. Value only valid when one of the flags MHDS.EIBF  MHDS.EMR  MHDS.ETBF1  MHDS.ETBF2  and flag MHDS.FMBD is set. Updated only after the Host has reset flag MHDS.FMBD."]
    #[inline(always)]
    pub fn fmb(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, Mhds_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x7f,1,0,u8, Mhds_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Message Buffer Transmitted   MBT. Number of last successfully transmitted Message Buffer. If the Message Buffer is configured for single shot mode  the respective TXR flag in the Transmission Request Registers TXRQ1 to TXRQ4 was reset. MBT is reset when the Communication Controller leaves  CONFIG  state or enters  STARTUP  state."]
    #[inline(always)]
    pub fn mbt(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Mhds_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Mhds_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Message Buffer Updated   MBU. Number of Message Buffer that was updated last. For this Message Buffer the respective NDn  n   0 31  to NDn  n   96 127 and   or MBCn  n   0 31  to MBCn  n   96 127  flag in the New Data Registers NDAT1 to NDAT4 and the Message Buffer Status Changed MBSC1 to MBSC4 registers are also set. MBU is reset when the Communication Controller leaves  CONFIG  state or enters  STARTUP  state."]
    #[inline(always)]
    pub fn mbu(
        self,
    ) -> crate::common::RegisterField<24, 0x7f, 1, 0, u8, Mhds_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x7f,1,0,u8, Mhds_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Mhds {
    #[inline(always)]
    fn default() -> Mhds {
        <crate::RegValueT<Mhds_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrc_SPEC;
impl crate::sealed::RegSpec for Mrc_SPEC {
    type DataType = u32;
}
#[doc = "Message RAM Configuration\n resetvalue={Application Reset:0x1800000}"]
pub type Mrc = crate::RegValueT<Mrc_SPEC>;

impl Mrc {
    #[doc = "First Dynamic Buffer   FDB. May be modified in  DEFAULT CONFIG  or  CONFIG  state only."]
    #[inline(always)]
    pub fn fdb(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Mrc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Mrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "First Buffer of FIFO   FFB. May be modified in  DEFAULT CONFIG  or  CONFIG  state only."]
    #[inline(always)]
    pub fn ffb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Mrc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Mrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Last Configured Buffer   LCB. May be only modified in  DEFAULT CONFIG  or  CONFIG  state."]
    #[inline(always)]
    pub fn lcb(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Mrc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Mrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Secure Buffers   SEC. Not evaluated when the Communication Controller is in   8220 DEFAULT CONFIG  8221         or   8220 CONFIG  8221  state. For temporary unlocking see   8220 Host Handling of Errors  8221 . In nodes configured for SYNC Frame transmission or for single slot          mode operation Message Buffer 0  and if SPLM   1  also Message Buffer          1  Reconfiguration of all Message Buffers is always locked"]
    #[inline(always)]
    pub fn sec(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, Mrc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, Mrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SYNC Frame Payload Multiplex   SPLM. This bit is only evaluated if the node is configured as sync node  SUCC1.TXSY   1  or for single slot mode operation  SUCC1.TSM   1 . When this bit is set to 1 Message Buffers 0 and 1 are dedicated for SYNC Frame transmission with different payload data on channel A and B. When this bit is reset to 0  SYNC Frames are transmitted from Message Buffer 0 with the same payload data on both channels. Note that the channel filter configuration for Message Buffer 0 resp. Message Buffer 1 has to be chosen accordingly."]
    #[inline(always)]
    pub fn splm(self) -> crate::common::RegisterFieldBool<26, 1, 0, Mrc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Mrc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mrc {
    #[inline(always)]
    fn default() -> Mrc {
        <crate::RegValueT<Mrc_SPEC> as RegisterValue<_>>::new(25165824)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msic1_SPEC;
impl crate::sealed::RegSpec for Msic1_SPEC {
    type DataType = u32;
}
#[doc = "Message Buffer Status Changed Interrupt Control 1\n resetvalue={Application Reset:0x0}"]
pub type Msic1 = crate::RegValueT<Msic1_SPEC>;

impl Msic1 {
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Msic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Msic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Msic1 {
    #[inline(always)]
    fn default() -> Msic1 {
        <crate::RegValueT<Msic1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msic2_SPEC;
impl crate::sealed::RegSpec for Msic2_SPEC {
    type DataType = u32;
}
#[doc = "Message Buffer Status Changed Interrupt Control 2\n resetvalue={Application Reset:0x0}"]
pub type Msic2 = crate::RegValueT<Msic2_SPEC>;

impl Msic2 {
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip32(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip33(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip34(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip35(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip36(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip37(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip38(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip39(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip40(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip41(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip42(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip43(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip44(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip45(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip46(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip47(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip48(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip49(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip50(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip51(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip52(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip53(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip54(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip55(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip56(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip57(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip58(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip59(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip60(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip61(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip62(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip63(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Msic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Msic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Msic2 {
    #[inline(always)]
    fn default() -> Msic2 {
        <crate::RegValueT<Msic2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msic3_SPEC;
impl crate::sealed::RegSpec for Msic3_SPEC {
    type DataType = u32;
}
#[doc = "Message Buffer Status Changed Interrupt Control 3\n resetvalue={Application Reset:0x0}"]
pub type Msic3 = crate::RegValueT<Msic3_SPEC>;

impl Msic3 {
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip64(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip65(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip66(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip67(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip68(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip69(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip70(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip71(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip72(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip73(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip74(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip75(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip76(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip77(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip78(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip79(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip80(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip81(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip82(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip83(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip84(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip85(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip86(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip87(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip88(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip89(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip90(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip91(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip92(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip93(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip94(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip95(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Msic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Msic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Msic3 {
    #[inline(always)]
    fn default() -> Msic3 {
        <crate::RegValueT<Msic3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msic4_SPEC;
impl crate::sealed::RegSpec for Msic4_SPEC {
    type DataType = u32;
}
#[doc = "Message Buffer Status Changed Interrupt Control 4\n resetvalue={Application Reset:0x0}"]
pub type Msic4 = crate::RegValueT<Msic4_SPEC>;

impl Msic4 {
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip96(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip97(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip98(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip99(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip100(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip101(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip102(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip103(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip104(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip105(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip106(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip107(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip108(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip109(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip110(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip111(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip112(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip113(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip114(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip115(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip116(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip117(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip118(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip119(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip120(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip121(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip122(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip123(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip124(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip125(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip126(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip127(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Msic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Msic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Msic4 {
    #[inline(always)]
    fn default() -> Msic4 {
        <crate::RegValueT<Msic4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtccv_SPEC;
impl crate::sealed::RegSpec for Mtccv_SPEC {
    type DataType = u32;
}
#[doc = "Macrotick and Cycle Counter Value\n resetvalue={Application Reset:0x0}"]
pub type Mtccv = crate::RegValueT<Mtccv_SPEC>;

impl Mtccv {
    #[doc = "Macrotick Value vMacrotick    MTV. Current Macrotick value. The value is incremented by the Communication Controller and reset at the start of a communication cycle. Valid values are 0 to 16000  0 H to 3E80 H  ."]
    #[inline(always)]
    pub fn mtv(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Mtccv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Mtccv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Cycle Counter Value vCycleCounter    CCV. Current cycle counter value. The value is incremented by the Communication Controller at the start of a communication cycle. Valid values are 0 to 63  0 H to 3F H  ."]
    #[inline(always)]
    pub fn ccv(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Mtccv_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Mtccv_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Mtccv {
    #[inline(always)]
    fn default() -> Mtccv {
        <crate::RegValueT<Mtccv_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ndat1_SPEC;
impl crate::sealed::RegSpec for Ndat1_SPEC {
    type DataType = u32;
}
#[doc = "New Data Register 1\n resetvalue={Application Reset:0x0}"]
pub type Ndat1 = crate::RegValueT<Ndat1_SPEC>;

impl Ndat1 {
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd25(self) -> crate::common::RegisterFieldBool<25, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd26(self) -> crate::common::RegisterFieldBool<26, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd27(self) -> crate::common::RegisterFieldBool<27, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd28(self) -> crate::common::RegisterFieldBool<28, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd29(self) -> crate::common::RegisterFieldBool<29, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd30(self) -> crate::common::RegisterFieldBool<30, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd31(self) -> crate::common::RegisterFieldBool<31, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ndat1 {
    #[inline(always)]
    fn default() -> Ndat1 {
        <crate::RegValueT<Ndat1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ndat2_SPEC;
impl crate::sealed::RegSpec for Ndat2_SPEC {
    type DataType = u32;
}
#[doc = "New Data Register 2\n resetvalue={Application Reset:0x0}"]
pub type Ndat2 = crate::RegValueT<Ndat2_SPEC>;

impl Ndat2 {
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd42(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd43(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd44(self) -> crate::common::RegisterFieldBool<12, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd45(self) -> crate::common::RegisterFieldBool<13, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd46(self) -> crate::common::RegisterFieldBool<14, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd47(self) -> crate::common::RegisterFieldBool<15, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd48(self) -> crate::common::RegisterFieldBool<16, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd49(self) -> crate::common::RegisterFieldBool<17, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd50(self) -> crate::common::RegisterFieldBool<18, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd51(self) -> crate::common::RegisterFieldBool<19, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd52(self) -> crate::common::RegisterFieldBool<20, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd53(self) -> crate::common::RegisterFieldBool<21, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd54(self) -> crate::common::RegisterFieldBool<22, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd55(self) -> crate::common::RegisterFieldBool<23, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd56(self) -> crate::common::RegisterFieldBool<24, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd57(self) -> crate::common::RegisterFieldBool<25, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd58(self) -> crate::common::RegisterFieldBool<26, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd59(self) -> crate::common::RegisterFieldBool<27, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd60(self) -> crate::common::RegisterFieldBool<28, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd61(self) -> crate::common::RegisterFieldBool<29, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd62(self) -> crate::common::RegisterFieldBool<30, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd63(self) -> crate::common::RegisterFieldBool<31, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ndat2 {
    #[inline(always)]
    fn default() -> Ndat2 {
        <crate::RegValueT<Ndat2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ndat3_SPEC;
impl crate::sealed::RegSpec for Ndat3_SPEC {
    type DataType = u32;
}
#[doc = "New Data Register 3\n resetvalue={Application Reset:0x0}"]
pub type Ndat3 = crate::RegValueT<Ndat3_SPEC>;

impl Ndat3 {
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd64(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd65(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd66(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd67(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd68(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd69(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd70(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd71(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd72(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd73(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd74(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd75(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd76(self) -> crate::common::RegisterFieldBool<12, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd77(self) -> crate::common::RegisterFieldBool<13, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd78(self) -> crate::common::RegisterFieldBool<14, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd79(self) -> crate::common::RegisterFieldBool<15, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd80(self) -> crate::common::RegisterFieldBool<16, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd81(self) -> crate::common::RegisterFieldBool<17, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd82(self) -> crate::common::RegisterFieldBool<18, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd83(self) -> crate::common::RegisterFieldBool<19, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd84(self) -> crate::common::RegisterFieldBool<20, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd85(self) -> crate::common::RegisterFieldBool<21, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd86(self) -> crate::common::RegisterFieldBool<22, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd87(self) -> crate::common::RegisterFieldBool<23, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd88(self) -> crate::common::RegisterFieldBool<24, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd89(self) -> crate::common::RegisterFieldBool<25, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd90(self) -> crate::common::RegisterFieldBool<26, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd91(self) -> crate::common::RegisterFieldBool<27, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd92(self) -> crate::common::RegisterFieldBool<28, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd93(self) -> crate::common::RegisterFieldBool<29, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd94(self) -> crate::common::RegisterFieldBool<30, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd95(self) -> crate::common::RegisterFieldBool<31, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ndat3 {
    #[inline(always)]
    fn default() -> Ndat3 {
        <crate::RegValueT<Ndat3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ndat4_SPEC;
impl crate::sealed::RegSpec for Ndat4_SPEC {
    type DataType = u32;
}
#[doc = "New Data Register 4\n resetvalue={Application Reset:0x0}"]
pub type Ndat4 = crate::RegValueT<Ndat4_SPEC>;

impl Ndat4 {
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd96(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd97(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd98(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd99(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd100(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd101(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd102(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd103(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd104(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd105(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd106(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd107(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd108(self) -> crate::common::RegisterFieldBool<12, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd109(self) -> crate::common::RegisterFieldBool<13, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd110(self) -> crate::common::RegisterFieldBool<14, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd111(self) -> crate::common::RegisterFieldBool<15, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd112(self) -> crate::common::RegisterFieldBool<16, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd113(self) -> crate::common::RegisterFieldBool<17, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd114(self) -> crate::common::RegisterFieldBool<18, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd115(self) -> crate::common::RegisterFieldBool<19, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd116(self) -> crate::common::RegisterFieldBool<20, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd117(self) -> crate::common::RegisterFieldBool<21, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd118(self) -> crate::common::RegisterFieldBool<22, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd119(self) -> crate::common::RegisterFieldBool<23, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd120(self) -> crate::common::RegisterFieldBool<24, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd121(self) -> crate::common::RegisterFieldBool<25, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd122(self) -> crate::common::RegisterFieldBool<26, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd123(self) -> crate::common::RegisterFieldBool<27, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd124(self) -> crate::common::RegisterFieldBool<28, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd125(self) -> crate::common::RegisterFieldBool<29, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd126(self) -> crate::common::RegisterFieldBool<30, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd127(self) -> crate::common::RegisterFieldBool<31, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ndat4 {
    #[inline(always)]
    fn default() -> Ndat4 {
        <crate::RegValueT<Ndat4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ndic1_SPEC;
impl crate::sealed::RegSpec for Ndic1_SPEC {
    type DataType = u32;
}
#[doc = "New Data Interrupt Control 1\n resetvalue={Application Reset:0x0}"]
pub type Ndic1 = crate::RegValueT<Ndic1_SPEC>;

impl Ndic1 {
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Ndic1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ndic1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ndic1 {
    #[inline(always)]
    fn default() -> Ndic1 {
        <crate::RegValueT<Ndic1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ndic2_SPEC;
impl crate::sealed::RegSpec for Ndic2_SPEC {
    type DataType = u32;
}
#[doc = "New Data Interrupt Control 2\n resetvalue={Application Reset:0x0}"]
pub type Ndic2 = crate::RegValueT<Ndic2_SPEC>;

impl Ndic2 {
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip32(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip33(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip34(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip35(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip36(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip37(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip38(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip39(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip40(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip41(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip42(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip43(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip44(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip45(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip46(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip47(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip48(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip49(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip50(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip51(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip52(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip53(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip54(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip55(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip56(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip57(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip58(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip59(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip60(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip61(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip62(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip63(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Ndic2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ndic2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ndic2 {
    #[inline(always)]
    fn default() -> Ndic2 {
        <crate::RegValueT<Ndic2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ndic3_SPEC;
impl crate::sealed::RegSpec for Ndic3_SPEC {
    type DataType = u32;
}
#[doc = "New Data Interrupt Control 3\n resetvalue={Application Reset:0x0}"]
pub type Ndic3 = crate::RegValueT<Ndic3_SPEC>;

impl Ndic3 {
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip64(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip65(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip66(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip67(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip68(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip69(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip70(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip71(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip72(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip73(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip74(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip75(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip76(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip77(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip78(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip79(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip80(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip81(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip82(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip83(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip84(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip85(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip86(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip87(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip88(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip89(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip90(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip91(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip92(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip93(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip94(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip95(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Ndic3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ndic3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ndic3 {
    #[inline(always)]
    fn default() -> Ndic3 {
        <crate::RegValueT<Ndic3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ndic4_SPEC;
impl crate::sealed::RegSpec for Ndic4_SPEC {
    type DataType = u32;
}
#[doc = "New Data Interrupt Control 4\n resetvalue={Application Reset:0x0}"]
pub type Ndic4 = crate::RegValueT<Ndic4_SPEC>;

impl Ndic4 {
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip96(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip97(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip98(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip99(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip100(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip101(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip102(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip103(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip104(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip105(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip106(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip107(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip108(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip109(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip110(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip111(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip112(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip113(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip114(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip115(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip116(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip117(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip118(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip119(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip120(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip121(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip122(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip123(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip124(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip125(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip126(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip127(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Ndic4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ndic4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ndic4 {
    #[inline(always)]
    fn default() -> Ndic4 {
        <crate::RegValueT<Ndic4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nemc_SPEC;
impl crate::sealed::RegSpec for Nemc_SPEC {
    type DataType = u32;
}
#[doc = "NEM Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Nemc = crate::RegValueT<Nemc_SPEC>;

impl Nemc {
    #[doc = "Network Management Vector Length gNetworkManagementVectorLength    NML. These bits configure the length of the NM Vector. The configured length must be identical in all nodes of a cluster. Valid values are 0 to 12  0 H to C H   bytes."]
    #[inline(always)]
    pub fn nml(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Nemc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Nemc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Nemc {
    #[inline(always)]
    fn default() -> Nemc {
        <crate::RegValueT<Nemc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NmVx_SPEC;
impl crate::sealed::RegSpec for NmVx_SPEC {
    type DataType = u32;
}
#[doc = "Network Management Vector 1\n resetvalue={Application Reset:0x0}"]
pub type NmVx = crate::RegValueT<NmVx_SPEC>;

impl NmVx {
    #[doc = "Network Management Vector   NM"]
    #[inline(always)]
    pub fn nm(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, NmVx_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, NmVx_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for NmVx {
    #[inline(always)]
    fn default() -> NmVx {
        <crate::RegValueT<NmVx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Obcm_SPEC;
impl crate::sealed::RegSpec for Obcm_SPEC {
    type DataType = u32;
}
#[doc = "Output Buffer Command Mask\n resetvalue={Application Reset:0x0}"]
pub type Obcm = crate::RegValueT<Obcm_SPEC>;

impl Obcm {
    #[doc = "Read Header Section Shadow   RHSS"]
    #[inline(always)]
    pub fn rhss(self) -> crate::common::RegisterFieldBool<0, 1, 0, Obcm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Obcm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Read Data Section Shadow   RDSS"]
    #[inline(always)]
    pub fn rdss(self) -> crate::common::RegisterFieldBool<1, 1, 0, Obcm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Obcm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Read Header Section Host   RHSH"]
    #[inline(always)]
    pub fn rhsh(self) -> crate::common::RegisterFieldBool<16, 1, 0, Obcm_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Obcm_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Read Data Section Host   RDSH"]
    #[inline(always)]
    pub fn rdsh(self) -> crate::common::RegisterFieldBool<17, 1, 0, Obcm_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Obcm_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Obcm {
    #[inline(always)]
    fn default() -> Obcm {
        <crate::RegValueT<Obcm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Obcr_SPEC;
impl crate::sealed::RegSpec for Obcr_SPEC {
    type DataType = u32;
}
#[doc = "Output Buffer Command Request\n resetvalue={Application Reset:0x0}"]
pub type Obcr = crate::RegValueT<Obcr_SPEC>;

impl Obcr {
    #[doc = "Output Buffer Request Shadow   OBRS. Number of source Message Buffer to be transferred from the Message RAM        to OBF Shadow. Valid values are 00 H to 7F H  0 to 127 . If the number of the first Message Buffer of the receive FIFO is written        to this register the Message Handler transfers the Message Buffer        addressed by the GET Index Register  GIDX    8220 FIFO Function  8221   to OBF        Shadow."]
    #[inline(always)]
    pub fn obrs(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Obcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8, Obcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "View Shadow Buffer   VIEW. Toggles between OBF Shadow and OBF Host. Only writeable while OBCR.OBSYS   0."]
    #[inline(always)]
    pub fn view(self) -> crate::common::RegisterFieldBool<8, 1, 0, Obcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Obcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Request Message RAM Transfer   REQ. Requests transfer of Message Buffer addressed by OBCR.OBRS from Message RAM to OBF Shadow. Only writeable while OBCR.OBSYS   0."]
    #[inline(always)]
    pub fn req(self) -> crate::common::RegisterFieldBool<9, 1, 0, Obcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Obcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Output Buffer Busy Shadow   OBSYS. Set to 1 after setting bit OBCR.REQ. When the transfer between the        Message RAM and OBF Shadow has completed  OBCR.OBSYS is cleared again."]
    #[inline(always)]
    pub fn obsys(self) -> crate::common::RegisterFieldBool<15, 1, 0, Obcr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Obcr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Output Buffer Request Host   OBRH. Number of Message Buffer currently accessible by the Host via RDHS1 to        RDHS3  MBS  and RDDSnn  nn   01 64 . By setting OBCR.VIEW OBF Shadow and        OBF Host are swapped and the transferred Message Buffer is accessible by        the Host. Valid values are 00 H to 7F H  01 to 127 ."]
    #[inline(always)]
    pub fn obrh(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Obcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Obcr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Obcr {
    #[inline(always)]
    fn default() -> Obcr {
        <crate::RegValueT<Obcr_SPEC> as RegisterValue<_>>::new(0)
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
pub struct Ocv_SPEC;
impl crate::sealed::RegSpec for Ocv_SPEC {
    type DataType = u32;
}
#[doc = "Offset Correction Value\n resetvalue={Application Reset:0x0}"]
pub type Ocv = crate::RegValueT<Ocv_SPEC>;

impl Ocv {
    #[doc = "Offset Correction Value vOffsetCorrection    OCV. Offset correction value  two s complement . Calculated internal offset correction value before limitation. If the OCV value exceeds the limits defined by GTUC10.MOC flag SFS.OCLR is set to 1."]
    #[inline(always)]
    pub fn ocv(
        self,
    ) -> crate::common::RegisterField<0, 0x7ffff, 1, 0, u32, Ocv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ffff,1,0,u32, Ocv_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ocv {
    #[inline(always)]
    fn default() -> Ocv {
        <crate::RegValueT<Ocv_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OsiDn_SPEC;
impl crate::sealed::RegSpec for OsiDn_SPEC {
    type DataType = u32;
}
#[doc = "Odd Sync ID Symbol Window 01\n resetvalue={Application Reset:0x0}"]
pub type OsiDn = crate::RegValueT<OsiDn_SPEC>;

impl OsiDn {
    #[doc = "Odd Sync ID vsSyncIDListA B odd    OID. SYNC Frame ID even communication cycle."]
    #[inline(always)]
    pub fn oid(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, OsiDn_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, OsiDn_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Received Odd Sync ID on Channel A   RXOA. Signals that a SYNC Frame corresponding to the stored odd sync ID was received on channel A or that the node is configured to be a sync node with key slot   OID  OSID1 only ."]
    #[inline(always)]
    pub fn rxoa(self) -> crate::common::RegisterFieldBool<14, 1, 0, OsiDn_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, OsiDn_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Received Odd Sync ID on Channel B   RXOB. Signals that a SYNC Frame corresponding to the stored odd sync ID was received on channel B or that the node is configured to be a sync node with key slot   OID  OSID1 only"]
    #[inline(always)]
    pub fn rxob(self) -> crate::common::RegisterFieldBool<15, 1, 0, OsiDn_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, OsiDn_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for OsiDn {
    #[inline(always)]
    fn default() -> OsiDn {
        <crate::RegValueT<OsiDn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otss_SPEC;
impl crate::sealed::RegSpec for Otss_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Trigger Set Select\n resetvalue={Application Reset:0x0}"]
pub type Otss = crate::RegValueT<Otss_SPEC>;

impl Otss {
    #[doc = "Trigger Set for OTGB0   OTGB0"]
    #[inline(always)]
    pub fn otgb0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Otss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Otss_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Set for OTGB1   OTGB1"]
    #[inline(always)]
    pub fn otgb1(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Otss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Otss_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Set for OTGB2   OTGB2"]
    #[inline(always)]
    pub fn otgb2(self) -> crate::common::RegisterFieldBool<16, 1, 0, Otss_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Otss_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Otss {
    #[inline(always)]
    fn default() -> Otss {
        <crate::RegValueT<Otss_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prtc1_SPEC;
impl crate::sealed::RegSpec for Prtc1_SPEC {
    type DataType = u32;
}
#[doc = "PRT Configuration Register 1\n resetvalue={Application Reset:0x084C0633}"]
pub type Prtc1 = crate::RegValueT<Prtc1_SPEC>;

impl Prtc1 {
    #[doc = "Transmission Start Sequence Transmitter gdTSSTransmitter    TSST. Configures the duration of the Transmission Start Sequence  TSS  in        terms of Bit Times  1 bit time  160   4  160 Microticks  160   100ns at 10Mbps . Must        be identical in all nodes of a cluster. Valid values are 3 to 15  3 H to F H   Bit Times."]
    #[inline(always)]
    pub fn tsst(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Prtc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Prtc1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Collision Avoidance Symbol Maximum  gdCASRxLowMax    CASM. Configures the upper limit of the acceptance window for a collision        avoidance symbol  CAS . Valid values are 67 to 99  43 H to 63 H  . Most significant bit of CASM        is hard wired to 1 and can not be modified."]
    #[inline(always)]
    pub fn casm(
        self,
    ) -> crate::common::RegisterField<4, 0x7f, 1, 0, u8, Prtc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7f,1,0,u8, Prtc1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Strobe Point Position   SPP. Defines the sample count value for strobing. The strobed bit value is        set to the voted value when the sample count is incremented to the value        configured by SPP. The current revision 2.1 of the FlexRay  8482  protocol requires that SPP            00 B . The alternate strobe point          positions could be used to compensate for asymmetries in the physical          layer."]
    #[inline(always)]
    pub fn spp(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Prtc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Prtc1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Baud Rate Prescaler  gdSampleClockPeriod  pSamplePerMicrotick    BRP. The baud rate prescaler configures the baud rate on the FlexRay  8482  bus.        The baud rates listed below are valid with a sample clock f SCLK          80  160 MHz. One bit time always consists of 8 samples independent of the        configured baud rate."]
    #[inline(always)]
    pub fn brp(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Prtc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Prtc1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wakeup Symbol Receive Window Length  gdWakeupSymbolRxWindow    RXW. Configures the number of Bit Times used by the node to test the duration        of the received wakeup pattern. Must be identical in all nodes of a        cluster. Valid values are 76 to 301  4C H to 12D H   Bit Times."]
    #[inline(always)]
    pub fn rxw(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, Prtc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1ff,1,0,u16, Prtc1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Repetitions of Tx Wakeup Pattern  pWakeupPattern    RWP. Configures the number of repetitions  sequences  of the Tx wakeup        symbol. Valid values are 2 to 63  2 H to 3F H  ."]
    #[inline(always)]
    pub fn rwp(
        self,
    ) -> crate::common::RegisterField<26, 0x3f, 1, 0, u8, Prtc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3f,1,0,u8, Prtc1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Prtc1 {
    #[inline(always)]
    fn default() -> Prtc1 {
        <crate::RegValueT<Prtc1_SPEC> as RegisterValue<_>>::new(139200051)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prtc2_SPEC;
impl crate::sealed::RegSpec for Prtc2_SPEC {
    type DataType = u32;
}
#[doc = "PRT Configuration Register 2\n resetvalue={Application Reset:0x0F2D0A0E}"]
pub type Prtc2 = crate::RegValueT<Prtc2_SPEC>;

impl Prtc2 {
    #[doc = "Wakeup Symbol Receive Idle gdWakeupSymbolRxIdle    RXI. Configures the number of Bit Times used by the node to test the duration        of the idle phase of the received wakeup symbol. Must be identical in        all nodes of a cluster. Valid values are 14 to 59  E H to 3B H   Bit Times."]
    #[inline(always)]
    pub fn rxi(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Prtc2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Prtc2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wakeup Symbol Receive Low  gdWakeupSymbolRxLow    RXL. Configures the number of Bit Times used by the node to test the duration        of the low phase of the received wakeup symbol. Must be identical in all        nodes of a cluster. Valid values are 10 to 55  A H to 37 H   Bit Times."]
    #[inline(always)]
    pub fn rxl(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, Prtc2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3f,1,0,u8, Prtc2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wakeup Symbol Transmit Idle  gdWakeupSymbolTxIdle    TXI. Configures the number of Bit Times used by the node to transmit the idle        phase of the wakeup symbol. Must be identical in all nodes of a cluster.        Valid values are 45 to 180  2D H to B4 H  Bit        Times."]
    #[inline(always)]
    pub fn txi(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Prtc2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Prtc2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wakeup Symbol Transmit Low  gdWakeupSymbolTxLow    TXL. Configures the number of Bit Times used by the node to transmit the low        phase of the wakeup symbol. Must be identical in all nodes of a cluster.        Valid values are 15 to 60  F H to 3C H          Bit Times."]
    #[inline(always)]
    pub fn txl(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, Prtc2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3f,1,0,u8, Prtc2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Prtc2 {
    #[inline(always)]
    fn default() -> Prtc2 {
        <crate::RegValueT<Prtc2_SPEC> as RegisterValue<_>>::new(254609934)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcv_SPEC;
impl crate::sealed::RegSpec for Rcv_SPEC {
    type DataType = u32;
}
#[doc = "Rate Correction Value\n resetvalue={Application Reset:0x0}"]
pub type Rcv = crate::RegValueT<Rcv_SPEC>;

impl Rcv {
    #[doc = "Rate Correction Value vRateCorrection    RCV. Rate correction value  two s complement . Calculated internal rate correction value before limitation. If the RCV value exceeds the limits defined by GTUC10.MRC  flag SFS.RCLR is set to 1."]
    #[inline(always)]
    pub fn rcv(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, Rcv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xfff,1,0,u16, Rcv_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Rcv {
    #[inline(always)]
    fn default() -> Rcv {
        <crate::RegValueT<Rcv_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RddSn_SPEC;
impl crate::sealed::RegSpec for RddSn_SPEC {
    type DataType = u32;
}
#[doc = "Read Data Section 01\n resetvalue={Application Reset:0x0}"]
pub type RddSn = crate::RegValueT<RddSn_SPEC>;

impl RddSn {
    #[doc = "32 Bit Word nn  Byte 0   MDRB0"]
    #[inline(always)]
    pub fn mdrb0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, RddSn_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, RddSn_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "32 Bit Word nn  Byte 1   MDRB1"]
    #[inline(always)]
    pub fn mdrb1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, RddSn_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, RddSn_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "32 Bit Word nn  Byte 2   MDRB2"]
    #[inline(always)]
    pub fn mdrb2(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, RddSn_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, RddSn_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "32 Bit Word nn  Byte 3   MDRB3"]
    #[inline(always)]
    pub fn mdrb3(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, RddSn_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, RddSn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for RddSn {
    #[inline(always)]
    fn default() -> RddSn {
        <crate::RegValueT<RddSn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdhs1_SPEC;
impl crate::sealed::RegSpec for Rdhs1_SPEC {
    type DataType = u32;
}
#[doc = "Read Header Section 1\n resetvalue={Application Reset:0x0}"]
pub type Rdhs1 = crate::RegValueT<Rdhs1_SPEC>;

impl Rdhs1 {
    #[doc = "Frame ID   FID"]
    #[inline(always)]
    pub fn fid(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Rdhs1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Rdhs1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Cycle Code   CYC"]
    #[inline(always)]
    pub fn cyc(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Rdhs1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Rdhs1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Channel Filter Control A   CHA"]
    #[inline(always)]
    pub fn cha(self) -> crate::common::RegisterFieldBool<24, 1, 0, Rdhs1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Rdhs1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel Filter Control B   CHB"]
    #[inline(always)]
    pub fn chb(self) -> crate::common::RegisterFieldBool<25, 1, 0, Rdhs1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Rdhs1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Direction Configuration Bit   CFG"]
    #[inline(always)]
    pub fn cfg(self) -> crate::common::RegisterFieldBool<26, 1, 0, Rdhs1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Rdhs1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Payload Preamble Indicator Transmit   PPIT"]
    #[inline(always)]
    pub fn ppit(self) -> crate::common::RegisterFieldBool<27, 1, 0, Rdhs1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Rdhs1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Mode   TXM"]
    #[inline(always)]
    pub fn txm(self) -> crate::common::RegisterFieldBool<28, 1, 0, Rdhs1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Rdhs1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Service Request   MBI"]
    #[inline(always)]
    pub fn mbi(self) -> crate::common::RegisterFieldBool<29, 1, 0, Rdhs1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Rdhs1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Rdhs1 {
    #[inline(always)]
    fn default() -> Rdhs1 {
        <crate::RegValueT<Rdhs1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdhs2_SPEC;
impl crate::sealed::RegSpec for Rdhs2_SPEC {
    type DataType = u32;
}
#[doc = "Read Header Section 2\n resetvalue={Application Reset:0x0}"]
pub type Rdhs2 = crate::RegValueT<Rdhs2_SPEC>;

impl Rdhs2 {
    #[doc = "Header CRC vRF Header HeaderCRC    CRC. Receive Buffer  Configuration not required. Header CRC updated from receive Data Frames. Transmit Buffer  Header CRC calculated and configured by the Host"]
    #[inline(always)]
    pub fn crc(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Rdhs2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Rdhs2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Payload Length Configured   PLC. Length of Data Section  number of 2 byte words  as configured by the Host."]
    #[inline(always)]
    pub fn plc(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Rdhs2_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Rdhs2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Payload Length Received vRF Header Length    PLR. Payload length value updated from received Data Frame  exception  if Message Buffer belongs to the receive FIFO PLR is also updated from received NULL Frames . When a message is stored into a Message Buffer the following behavior with respect to payload length received and payload length configured is implemented  PLR  gt  PLC  The payload data stored in the Message Buffer is truncated to the payload length configured for even PLC or else truncated to PLC   1. PLR   PLC  The received payload data is stored into the Message Buffers Data Section. The remaining data bytes of the Data Section as configured by PLC are filled with undefined data. PLR   0  The Message Buffer s Data Section is filled with undefined data. PLC   0  Message Buffer has no Data Section configured. No data is stored into the  Message Buffer s Data Section."]
    #[inline(always)]
    pub fn plr(
        self,
    ) -> crate::common::RegisterField<24, 0x7f, 1, 0, u8, Rdhs2_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x7f,1,0,u8, Rdhs2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Rdhs2 {
    #[inline(always)]
    fn default() -> Rdhs2 {
        <crate::RegValueT<Rdhs2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdhs3_SPEC;
impl crate::sealed::RegSpec for Rdhs3_SPEC {
    type DataType = u32;
}
#[doc = "Read Header Section 3\n resetvalue={Application Reset:0x0}"]
pub type Rdhs3 = crate::RegValueT<Rdhs3_SPEC>;

impl Rdhs3 {
    #[doc = "Data Pointer   DP. Pointer to the first 32 bit word of the Data Section of the addressed Message Buffer in the Message RAM."]
    #[inline(always)]
    pub fn dp(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Rdhs3_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Rdhs3_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive Cycle Count vRF Header CycleCount    RCC. Cycle counter value updated from received Data Frame."]
    #[inline(always)]
    pub fn rcc(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Rdhs3_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Rdhs3_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Received on Channel Indicator vSS Channel    RCI. Indicates the channel from which the received Data Frame was taken to update the respective receive buffer."]
    #[inline(always)]
    pub fn rci(self) -> crate::common::RegisterFieldBool<24, 1, 0, Rdhs3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Rdhs3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Startup Frame Indicator vRF Header SuFIndicator    SFI. A Startup Frame is marked by the Startup Frame indicator."]
    #[inline(always)]
    pub fn sfi(self) -> crate::common::RegisterFieldBool<25, 1, 0, Rdhs3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Rdhs3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SYNC Frame Indicator vRF Header SyFIndicator    SYN. A SYNC Frame is marked by the SYNC Frame indicator."]
    #[inline(always)]
    pub fn syn(self) -> crate::common::RegisterFieldBool<26, 1, 0, Rdhs3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Rdhs3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "NULL Frame Indicator vRF Header NFIndicator    NFI. Is set to 1 after storage of the first received Data Frame."]
    #[inline(always)]
    pub fn nfi(self) -> crate::common::RegisterFieldBool<27, 1, 0, Rdhs3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Rdhs3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Payload Preamble Indicator vRF Header PPIndicator    PPI. The payload preamble indicator defines whether a Network Management vector or message ID is contained within the Payload Segment of the received Frame."]
    #[inline(always)]
    pub fn ppi(self) -> crate::common::RegisterFieldBool<28, 1, 0, Rdhs3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Rdhs3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Rdhs3 {
    #[inline(always)]
    fn default() -> Rdhs3 {
        <crate::RegValueT<Rdhs3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scv_SPEC;
impl crate::sealed::RegSpec for Scv_SPEC {
    type DataType = u32;
}
#[doc = "Slot Counter Value\n resetvalue={Application Reset:0x0}"]
pub type Scv = crate::RegValueT<Scv_SPEC>;

impl Scv {
    #[doc = "Slot Counter Channel A vSlotCounter A     SCCA. Current slot counter value on channel A. The value is incremented by the        Communication Controller and reset at the start of a communication        cycle. Valid values are 0 to 2047  0 H to 7FF H  ."]
    #[inline(always)]
    pub fn scca(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Scv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Scv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Slot Counter Channel B vSlotCounter B     SCCB. Current slot counter value on channel B. The value is incremented by the        Communication Controller and reset at the start of a communication        cycle. Valid values are 0 to 2047  0 H to 7FF H  ."]
    #[inline(always)]
    pub fn sccb(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, Scv_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7ff,1,0,u16, Scv_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Scv {
    #[inline(always)]
    fn default() -> Scv {
        <crate::RegValueT<Scv_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfs_SPEC;
impl crate::sealed::RegSpec for Sfs_SPEC {
    type DataType = u32;
}
#[doc = "SYNC Frame Status\n resetvalue={Application Reset:0x0}"]
pub type Sfs = crate::RegValueT<Sfs_SPEC>;

impl Sfs {
    #[doc = "Valid SYNC Frames Channel A  even communication cycle   VSAE. Holds the number of valid SYNC Frames received on channel A in the even communication cycle. If transmission of SYNC Frames is enabled by SUCC1.TXSY the value is incremented by one. The value is updated during the network idle time  NIT  of each even communication cycle. This bit field is only valid if the channel A is assigned to the Communication Controller by SUCC1.CCHA."]
    #[inline(always)]
    pub fn vsae(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Sfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xf, 1, 0, u8, Sfs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Valid SYNC Frames Channel A  odd communication cycle   VSAO. Holds the number of valid SYNC Frames received on channel A in the odd communication cycle. If transmission of SYNC Frames is enabled by SUCC1.TXSY the value is incremented by one. The value is updated during the network idle time  NIT  of each odd communication cycle. This bit field is only valid if the channel A is assigned to the Communication Controller by SUCC1.CCHA."]
    #[inline(always)]
    pub fn vsao(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Sfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, Sfs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Valid SYNC Frames Channel B  even communication cycle   VSBE. Holds the number of valid SYNC Frames received on channel B in the even communication cycle. If transmission of SYNC Frames is enabled by SUCC1.TXSY the value is incremented by one. The value is updated during the network idle time  NIT  of each even communication cycle. This bit field is only valid if the channel B is assigned to the Communication Controller by SUCC1.CCHB."]
    #[inline(always)]
    pub fn vsbe(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Sfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xf, 1, 0, u8, Sfs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Valid SYNC Frames Channel B  odd communication cycle   VSBO. Holds the number of valid SYNC Frames received on channel B in the odd communication cycle. If transmission of SYNC Frames is enabled by SUCC1.TXSY the value is incremented by one. The value is updated during the network idle time  NIT  of each odd communication cycle. This bit field is only valid if the channel B is assigned to the Communication Controller by SUCC1.CCHB."]
    #[inline(always)]
    pub fn vsbo(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Sfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<12, 0xf, 1, 0, u8, Sfs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Missing Offset Correction Signal   MOCS. The Missing Offset Correction flag signals to the Host  that no offset correction calculation can be performed because no SYNC Frames were received. The flag is updated by the Communication Controller at start of offset correction phase."]
    #[inline(always)]
    pub fn mocs(self) -> crate::common::RegisterFieldBool<16, 1, 0, Sfs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Sfs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Offset Correction Limit Reached   OCLR. The Offset Correction Limit Reached flag signals to the Host  that the offset correction value has exceeded its limit as defined by GTUC10.MOC. The flag is updated by the Communication Controller at start of offset correction phase."]
    #[inline(always)]
    pub fn oclr(self) -> crate::common::RegisterFieldBool<17, 1, 0, Sfs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Sfs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Missing Rate Correction Signal   MRCS. The Missing Rate Correction Flag signals to the Host  that no rate correction calculation can be performed because no pairs of even   odd SYNC Frames were received. The flag is updated by the Communication Controller at start of offset correction phase."]
    #[inline(always)]
    pub fn mrcs(self) -> crate::common::RegisterFieldBool<18, 1, 0, Sfs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Sfs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Rate Correction Limit Reached   RCLR. The Rate Correction Limit Reached flag signals to the Host  that the rate correction value has exceeded its limit.as defined by GTUC10.MRC. The flag is updated by the Communication Controller at start of offset correction phase."]
    #[inline(always)]
    pub fn rclr(self) -> crate::common::RegisterFieldBool<19, 1, 0, Sfs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Sfs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Sfs {
    #[inline(always)]
    fn default() -> Sfs {
        <crate::RegValueT<Sfs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sier_SPEC;
impl crate::sealed::RegSpec for Sier_SPEC {
    type DataType = u32;
}
#[doc = "Status Service Request Enable Reset\n resetvalue={Application Reset:0x0}"]
pub type Sier = crate::RegValueT<Sier_SPEC>;

impl Sier {
    #[doc = "Wakeup Status Service Request Enable   WSTE"]
    #[inline(always)]
    pub fn wste(self) -> crate::common::RegisterFieldBool<0, 1, 0, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Sier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Collision Avoidance Symbol Service Request Enable   CASE"]
    #[inline(always)]
    pub fn case(self) -> crate::common::RegisterFieldBool<1, 1, 0, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Sier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Cycle Start Service Request Enable   CYCSE"]
    #[inline(always)]
    pub fn cycse(self) -> crate::common::RegisterFieldBool<2, 1, 0, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Sier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmit Service Request Enable   TXIE"]
    #[inline(always)]
    pub fn txie(self) -> crate::common::RegisterFieldBool<3, 1, 0, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Sier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive Service Request Enable   RXIE"]
    #[inline(always)]
    pub fn rxie(self) -> crate::common::RegisterFieldBool<4, 1, 0, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Sier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive FIFO Not Empty Service Request Enable   RFNEE"]
    #[inline(always)]
    pub fn rfnee(self) -> crate::common::RegisterFieldBool<5, 1, 0, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Sier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive FIFO Critical Level Service Request Enable   RFCLE"]
    #[inline(always)]
    pub fn rfcle(self) -> crate::common::RegisterFieldBool<6, 1, 0, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Sier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Network Management Vector Changed Service Request Enable   NMVCE"]
    #[inline(always)]
    pub fn nmvce(self) -> crate::common::RegisterFieldBool<7, 1, 0, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Sier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer Service Request 0 Enable   TI0E"]
    #[inline(always)]
    pub fn ti0e(self) -> crate::common::RegisterFieldBool<8, 1, 0, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Sier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer Service Request 1 Enable   TI1E"]
    #[inline(always)]
    pub fn ti1e(self) -> crate::common::RegisterFieldBool<9, 1, 0, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Sier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transfer Input Buffer Completed Service Request Enable   TIBCE"]
    #[inline(always)]
    pub fn tibce(self) -> crate::common::RegisterFieldBool<10, 1, 0, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Sier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transfer Output Buffer Completed Service Request Enable   TOBCE"]
    #[inline(always)]
    pub fn tobce(self) -> crate::common::RegisterFieldBool<11, 1, 0, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Sier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Stop Watch Event Service Request Enable   SWEE"]
    #[inline(always)]
    pub fn swee(self) -> crate::common::RegisterFieldBool<12, 1, 0, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Sier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Startup Completed Successfully Service Request Enable   SUCSE"]
    #[inline(always)]
    pub fn sucse(self) -> crate::common::RegisterFieldBool<13, 1, 0, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Sier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Service Request Enable   MBSIE"]
    #[inline(always)]
    pub fn mbsie(self) -> crate::common::RegisterFieldBool<14, 1, 0, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Sier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Start of Dynamic Segment Service Request Enable   SDSE"]
    #[inline(always)]
    pub fn sdse(self) -> crate::common::RegisterFieldBool<15, 1, 0, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Sier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Wakeup Pattern Channel A Service Request Enable   WUPAE"]
    #[inline(always)]
    pub fn wupae(self) -> crate::common::RegisterFieldBool<16, 1, 0, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Sier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Media Access Test Symbol Channel A Service Request Enable   MTSAE"]
    #[inline(always)]
    pub fn mtsae(self) -> crate::common::RegisterFieldBool<17, 1, 0, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Sier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Wakeup Pattern Channel B Service Request Enable   WUPBE"]
    #[inline(always)]
    pub fn wupbe(self) -> crate::common::RegisterFieldBool<24, 1, 0, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Sier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Media Access Test Symbol Channel B Service Request Enable   MTSBE"]
    #[inline(always)]
    pub fn mtsbe(self) -> crate::common::RegisterFieldBool<25, 1, 0, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Sier_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Sier {
    #[inline(always)]
    fn default() -> Sier {
        <crate::RegValueT<Sier_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sies_SPEC;
impl crate::sealed::RegSpec for Sies_SPEC {
    type DataType = u32;
}
#[doc = "Status Service Request Enable Set\n resetvalue={Application Reset:0x0}"]
pub type Sies = crate::RegValueT<Sies_SPEC>;

impl Sies {
    #[doc = "Wakeup Status Service Request Enable   WSTE"]
    #[inline(always)]
    pub fn wste(self) -> crate::common::RegisterFieldBool<0, 1, 0, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Sies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Collision Avoidance Symbol Service Request Enable   CASE"]
    #[inline(always)]
    pub fn case(self) -> crate::common::RegisterFieldBool<1, 1, 0, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Sies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Cycle Start Service Request Enable   CYCSE"]
    #[inline(always)]
    pub fn cycse(self) -> crate::common::RegisterFieldBool<2, 1, 0, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Sies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmit Service Request Enable   TXIE"]
    #[inline(always)]
    pub fn txie(self) -> crate::common::RegisterFieldBool<3, 1, 0, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Sies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive Service Request Enable   RXIE"]
    #[inline(always)]
    pub fn rxie(self) -> crate::common::RegisterFieldBool<4, 1, 0, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Sies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive FIFO Not Empty Service Request Enable   RFNEE"]
    #[inline(always)]
    pub fn rfnee(self) -> crate::common::RegisterFieldBool<5, 1, 0, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Sies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive FIFO Critical Level Service Request Enable   RFCLE"]
    #[inline(always)]
    pub fn rfcle(self) -> crate::common::RegisterFieldBool<6, 1, 0, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Sies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Network Management Vector Changed Service Request Enable   NMVCE"]
    #[inline(always)]
    pub fn nmvce(self) -> crate::common::RegisterFieldBool<7, 1, 0, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Sies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer Service Request 0 Enable   TI0E"]
    #[inline(always)]
    pub fn ti0e(self) -> crate::common::RegisterFieldBool<8, 1, 0, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Sies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer Service Request 1 Enable   TI1E"]
    #[inline(always)]
    pub fn ti1e(self) -> crate::common::RegisterFieldBool<9, 1, 0, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Sies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transfer Input Buffer Completed Service Request Enable   TIBCE"]
    #[inline(always)]
    pub fn tibce(self) -> crate::common::RegisterFieldBool<10, 1, 0, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Sies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transfer Output Buffer Completed Service Request Enable   TOBCE"]
    #[inline(always)]
    pub fn tobce(self) -> crate::common::RegisterFieldBool<11, 1, 0, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Sies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Stop Watch Event Service Request Enable   SWEE"]
    #[inline(always)]
    pub fn swee(self) -> crate::common::RegisterFieldBool<12, 1, 0, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Sies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Startup Completed Successfully Service Request Enable   SUCSE"]
    #[inline(always)]
    pub fn sucse(self) -> crate::common::RegisterFieldBool<13, 1, 0, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Sies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Service Request Enable   MBSIE"]
    #[inline(always)]
    pub fn mbsie(self) -> crate::common::RegisterFieldBool<14, 1, 0, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Sies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Start of Dynamic Segment Service Request Enable   SDSE"]
    #[inline(always)]
    pub fn sdse(self) -> crate::common::RegisterFieldBool<15, 1, 0, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Sies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Wakeup Pattern Channel A Service Request Enable   WUPAE"]
    #[inline(always)]
    pub fn wupae(self) -> crate::common::RegisterFieldBool<16, 1, 0, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Sies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Media Access Test Symbol Channel A Service Request Enable   MTSAE"]
    #[inline(always)]
    pub fn mtsae(self) -> crate::common::RegisterFieldBool<17, 1, 0, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Sies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Wakeup Pattern Channel B Service Request Enable   WUPBE"]
    #[inline(always)]
    pub fn wupbe(self) -> crate::common::RegisterFieldBool<24, 1, 0, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Sies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Media Access Test Symbol Channel B Service Request Enable   MTSBE"]
    #[inline(always)]
    pub fn mtsbe(self) -> crate::common::RegisterFieldBool<25, 1, 0, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Sies_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Sies {
    #[inline(always)]
    fn default() -> Sies {
        <crate::RegValueT<Sies_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sils_SPEC;
impl crate::sealed::RegSpec for Sils_SPEC {
    type DataType = u32;
}
#[doc = "Status Service Request Line Select\n resetvalue={Application Reset:0x303FFFF}"]
pub type Sils = crate::RegValueT<Sils_SPEC>;

impl Sils {
    #[doc = "Wakeup Status Service Request Line   WSTL"]
    #[inline(always)]
    pub fn wstl(self) -> crate::common::RegisterFieldBool<0, 1, 0, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Sils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Collision Avoidance Symbol Service Request Line   CASL"]
    #[inline(always)]
    pub fn casl(self) -> crate::common::RegisterFieldBool<1, 1, 0, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Sils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Cycle Start Service Request Line   CYCSL"]
    #[inline(always)]
    pub fn cycsl(self) -> crate::common::RegisterFieldBool<2, 1, 0, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Sils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmit Service Request Line   TXIL"]
    #[inline(always)]
    pub fn txil(self) -> crate::common::RegisterFieldBool<3, 1, 0, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Sils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive Service Request Line   RXIL"]
    #[inline(always)]
    pub fn rxil(self) -> crate::common::RegisterFieldBool<4, 1, 0, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Sils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive FIFO Not Empty Service Request Line   RFNEL"]
    #[inline(always)]
    pub fn rfnel(self) -> crate::common::RegisterFieldBool<5, 1, 0, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Sils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive FIFO Critical Level Service Request Line   RFCLL"]
    #[inline(always)]
    pub fn rfcll(self) -> crate::common::RegisterFieldBool<6, 1, 0, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Sils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Network Management Vector Changed Service Request Line   NMVCL"]
    #[inline(always)]
    pub fn nmvcl(self) -> crate::common::RegisterFieldBool<7, 1, 0, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Sils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer Service Request 0 Line   TI0L"]
    #[inline(always)]
    pub fn ti0l(self) -> crate::common::RegisterFieldBool<8, 1, 0, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Sils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer Service Request 1 Line   TI1L"]
    #[inline(always)]
    pub fn ti1l(self) -> crate::common::RegisterFieldBool<9, 1, 0, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Sils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transfer Input Buffer Completed Service Request Line   TIBCL"]
    #[inline(always)]
    pub fn tibcl(self) -> crate::common::RegisterFieldBool<10, 1, 0, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Sils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transfer Output Buffer Completed Service Request Line   TOBCL"]
    #[inline(always)]
    pub fn tobcl(self) -> crate::common::RegisterFieldBool<11, 1, 0, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Sils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Stop Watch Event Service Request Line   SWEL"]
    #[inline(always)]
    pub fn swel(self) -> crate::common::RegisterFieldBool<12, 1, 0, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Sils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Startup Completed Successfully Service Request Line   SUCSL"]
    #[inline(always)]
    pub fn sucsl(self) -> crate::common::RegisterFieldBool<13, 1, 0, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Sils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Service Request Line   MBSIL"]
    #[inline(always)]
    pub fn mbsil(self) -> crate::common::RegisterFieldBool<14, 1, 0, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Sils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Start of Dynamic Segment Service Request Line   SDSL"]
    #[inline(always)]
    pub fn sdsl(self) -> crate::common::RegisterFieldBool<15, 1, 0, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Sils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Wakeup Pattern Channel A Service Request Line   WUPAL"]
    #[inline(always)]
    pub fn wupal(self) -> crate::common::RegisterFieldBool<16, 1, 0, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Sils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Media Access Test Symbol Channel A Service Request Line   MTSAL"]
    #[inline(always)]
    pub fn mtsal(self) -> crate::common::RegisterFieldBool<17, 1, 0, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Sils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Wakeup Pattern Channel B Service Request Line   WUPBL"]
    #[inline(always)]
    pub fn wupbl(self) -> crate::common::RegisterFieldBool<24, 1, 0, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Sils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Media Access Test Symbol Channel B Service Request Line   MTSBL"]
    #[inline(always)]
    pub fn mtsbl(self) -> crate::common::RegisterFieldBool<25, 1, 0, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Sils_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Sils {
    #[inline(always)]
    fn default() -> Sils {
        <crate::RegValueT<Sils_SPEC> as RegisterValue<_>>::new(50593791)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sir_SPEC;
impl crate::sealed::RegSpec for Sir_SPEC {
    type DataType = u32;
}
#[doc = "Status Service Request Register\n resetvalue={Application Reset:0x0}"]
pub type Sir = crate::RegValueT<Sir_SPEC>;

impl Sir {
    #[doc = "Wakeup Status   WST. This flag is set when the wakeup status vector CCSV.WSV in the Communication Controller Status Vector register changes to a value other than UNDEFINED. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn wst(self) -> crate::common::RegisterFieldBool<0, 1, 0, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Sir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Collision Avoidance Symbol   CAS. This flag is set by the Communication Controller during STARTUP state when a CAS or potential CAS was received. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn cas(self) -> crate::common::RegisterFieldBool<1, 1, 0, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Sir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Cycle Start Service Request   CYCS. This flag is set by the Communication Controller when a communication        cycle starts This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn cycs(self) -> crate::common::RegisterFieldBool<2, 1, 0, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Sir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmit Service Request   TXI. This flag is set by the Communication Controller at the end of Frame        transmission if bit WRHS1.MBI in the respective Message Buffer is set. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn txi(self) -> crate::common::RegisterFieldBool<3, 1, 0, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Sir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive Service Request   RXI. This flag is set by the Communication Controller whenever the set        condition of a Message Buffer ND flag is fulfilled and if bit WRHS1.MBI        of that Message Buffer is set to 1. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn rxi(self) -> crate::common::RegisterFieldBool<4, 1, 0, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Sir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive FIFO Not Empty   RFNE. This flag is set by the Communication Controller when a received valid        Frame was stored into the empty receive FIFO.m The actual state of the        receive FIFO is monitored in register FSR"]
    #[inline(always)]
    pub fn rfne(self) -> crate::common::RegisterFieldBool<5, 1, 0, Sir_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Sir_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive FIFO Critical Level   RFCL. This flag is set when a valid receive FIFO fill level FSR.RFFL is equal or greater than the critical level as configured by FCL.CL."]
    #[inline(always)]
    pub fn rfcl(self) -> crate::common::RegisterFieldBool<6, 1, 0, Sir_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Sir_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Network Management Vector Changed   NMVC. This service request flag signals a change in the Network Management Vector visible to the Host. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn nmvc(self) -> crate::common::RegisterFieldBool<7, 1, 0, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Sir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer Service Request 0   TI0. This flag is set whenever timer 0 matches the conditions configured in the Timer Service Request 0 Configuration Register T0C. A Timer Service Request 0 is also signalled by TINT0SRC. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn ti0(self) -> crate::common::RegisterFieldBool<8, 1, 0, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Sir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer Service Request 1   TI1. This flag is set whenever the conditions programmed in the Timer Service Request 1 Configuration Register T1C are met. A Timer Service Request 1 is also signalled by TINT1SRC. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn ti1(self) -> crate::common::RegisterFieldBool<9, 1, 0, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Sir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transfer Input Buffer Completed   TIBC. This flag is set whenever a transfer from Input Buffer to the Message RAM has completed and bit IBCR.IBSYS in the Input Buffer Command Request register has been reset by the Message Handler. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn tibc(self) -> crate::common::RegisterFieldBool<10, 1, 0, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Sir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transfer Output Buffer Completed   TOBC. This flag is set whenever a transfer from Message RAM to the Output Buffer has completed and bit OBCR.OBSYS in the Output Buffer Command Request register has been reset by the Message Handler. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn tobc(self) -> crate::common::RegisterFieldBool<11, 1, 0, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Sir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Stop Watch Event   SWE. This flag is set after a stop watch activation when the current cycle counter and Macrotick value are stored in the Stop Watch Register 1  STPW1 . This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn swe(self) -> crate::common::RegisterFieldBool<12, 1, 0, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Sir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Startup Completed Successfully   SUCS. This flag is set whenever a startup completed successfully and the Communication Controller entered  NORMAL ACTIVE  state. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn sucs(self) -> crate::common::RegisterFieldBool<13, 1, 0, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Sir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Service Request   MBSI. This flag is set by the Communication Controller when the Message Buffer        status MBS has changed and if bit RDHS1.MBI of that Message Buffer is        set. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn mbsi(self) -> crate::common::RegisterFieldBool<14, 1, 0, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Sir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Start of Dynamic Segment   SDS. This flag is set by the Communication Controller when the dynamic        segment starts."]
    #[inline(always)]
    pub fn sds(self) -> crate::common::RegisterFieldBool<15, 1, 0, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Sir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Wakeup Pattern Channel A   WUPA. This flag is set by the Communication Controller when a wakeup pattern was received on channel A. Only set when the Communication Controller is in  WAKEUP    READY   or  STARTUP  state  or when in Monitor mode. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn wupa(self) -> crate::common::RegisterFieldBool<16, 1, 0, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Sir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "MTS Received on Channel A vSS ValidMTSA    MTSA. Media Access Test symbol received on channel A during the proceeding symbol window. Updated by the Communication Controller for each channel at the end of the symbol window. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn mtsa(self) -> crate::common::RegisterFieldBool<17, 1, 0, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Sir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Wakeup Pattern Channel B   WUPB. This flag is set by the Communication Controller when a wakeup pattern        was received on channel B. Only set when the Communication Controller is        in   8220 WAKEUP  8221     8220 READY  8221   or   8220 STARTUP  8221  state  or when in Monitor mode. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn wupb(self) -> crate::common::RegisterFieldBool<24, 1, 0, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Sir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "MTS Received on Channel B   MTSB. Media Access Test symbol received on channel B during the proceeding        symbol window. Updated by the Communication Controller for each channel        at the end of the symbol window. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn mtsb(self) -> crate::common::RegisterFieldBool<25, 1, 0, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Sir_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Sir {
    #[inline(always)]
    fn default() -> Sir {
        <crate::RegValueT<Sir_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stpw1_SPEC;
impl crate::sealed::RegSpec for Stpw1_SPEC {
    type DataType = u32;
}
#[doc = "Stop Watch Register 1\n resetvalue={Application Reset:0x0}"]
pub type Stpw1 = crate::RegValueT<Stpw1_SPEC>;

impl Stpw1 {
    #[doc = "Enable Stop Watch Trigger   ESWT. If enabled an edge on input STPW  pin eray stpwt   if embedded  eray stpwt0  eray stpwt1  eray stpwt2  or eray stpwt3  or a service request 0 or 1 event  rising edge on signal INT0SR or INT1SR   eray int0 or eray int 1  activates the stop watch. In single shot mode this bit is reset to 0 after the actual cycle counter and Macrotick value are stored in the Stop Watch register."]
    #[inline(always)]
    pub fn eswt(self) -> crate::common::RegisterFieldBool<0, 1, 0, Stpw1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Stpw1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Stop Watch Mode Select   SWMS. It is not possible to change the Stop Watch Mode during enabled stop watch trigger  STPW1.ESWT"]
    #[inline(always)]
    pub fn swms(self) -> crate::common::RegisterFieldBool<1, 1, 0, Stpw1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Stpw1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Stop Watch Trigger Edge Select   EDGE"]
    #[inline(always)]
    pub fn edge(self) -> crate::common::RegisterFieldBool<2, 1, 0, Stpw1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Stpw1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Software Stop Watch Trigger   SSWT. When the Host writes this bit to 1 the stop watch is activated. After the actual cycle counter and Macrotick value are stored in the Stop Watch register this bit is reset to 0. The bit is only writeable while ESWT   0."]
    #[inline(always)]
    pub fn sswt(self) -> crate::common::RegisterFieldBool<3, 1, 0, Stpw1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Stpw1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable External Trigger Pin   EETP. Enables stop watch trigger event via signal STPW  eray stpwt  if ESWT   1."]
    #[inline(always)]
    pub fn eetp(self) -> crate::common::RegisterFieldBool<4, 1, 0, Stpw1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Stpw1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Service Request 0 Trigger   EINT0. Enables stop watch trigger by service request 0 event if ESWT   1."]
    #[inline(always)]
    pub fn eint0(self) -> crate::common::RegisterFieldBool<5, 1, 0, Stpw1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Stpw1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Service Request 1 Trigger   EINT1. Enables stop watch trigger by service request 1 event if ESWT   1."]
    #[inline(always)]
    pub fn eint1(self) -> crate::common::RegisterFieldBool<6, 1, 0, Stpw1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Stpw1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Stopped Cycle Counter Value   SCCV. State of the cycle counter when the stop watch event occurred. Valid values are  0 3F H Valid Values"]
    #[inline(always)]
    pub fn sccv(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, Stpw1_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3f,1,0,u8, Stpw1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Stopped Macrotick Value   SMTV. State of the Macrotick counter when the stop watch event occurred. Valid values are  0 3F H Valid Values"]
    #[inline(always)]
    pub fn smtv(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, Stpw1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3fff,1,0,u16, Stpw1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Stpw1 {
    #[inline(always)]
    fn default() -> Stpw1 {
        <crate::RegValueT<Stpw1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stpw2_SPEC;
impl crate::sealed::RegSpec for Stpw2_SPEC {
    type DataType = u32;
}
#[doc = "Stop Watch Register 2\n resetvalue={Application Reset:0x0}"]
pub type Stpw2 = crate::RegValueT<Stpw2_SPEC>;

impl Stpw2 {
    #[doc = "Stop Watch Captured Slot Counter Value Channel A   SSCVA. State of the slot counter for channel A when the stop watch event occurred. Valid values are 0 to 2047  0 H to 7FF H  ."]
    #[inline(always)]
    pub fn sscva(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Stpw2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Stpw2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Stop Watch Captured Slot Counter Value Channel B   SSCVB. State of the slot counter for channel B when the stop watch event occurred. Valid values are 0 to 2047  0 H to 7FF H  ."]
    #[inline(always)]
    pub fn sscvb(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, Stpw2_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7ff,1,0,u16, Stpw2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Stpw2 {
    #[inline(always)]
    fn default() -> Stpw2 {
        <crate::RegValueT<Stpw2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Succ1_SPEC;
impl crate::sealed::RegSpec for Succ1_SPEC {
    type DataType = u32;
}
#[doc = "SUC Configuration Register 1\n resetvalue={Application Reset:0x0C401000}"]
pub type Succ1 = crate::RegValueT<Succ1_SPEC>;

impl Succ1 {
    #[doc = "CHI Command Vector   CMD. The host may write any CHI command at any time  but certain commands are        only enabled in specific POC states. A disabled command will not be        executed  the CHI command vector CMD will be reset to 0000 B     8220 COMMAND NOT ACCEPTED  8221   and flag EIR.CNA in the Error Service Request        register will be set to 1. In case the previous CHI command has not yet        completed  EIR.CCL is set to 1 together with EIR.CNA  the CHI command        needs to be repeated. Except for HALT state  POC state change command        applied while the Communication Controller is already in the requested        POC state will be ignored. Reading SUCC1.CMD shows whether the last CHI command was accepted.        CCSV.POCS monitors the actual POC state. The reserved CHI commands code        hardware test functions."]
    #[inline(always)]
    pub fn cmd(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Succ1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Succ1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "POC Busy   PBSY. Signals that the POC is busy and cannot accept a command from the Host.        SUCC1.CMD is locked against write accesses. Set to 1 after hard reset        during initialization of internal RAM blocks."]
    #[inline(always)]
    pub fn pbsy(self) -> crate::common::RegisterFieldBool<7, 1, 0, Succ1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Succ1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmit Startup Frame in Key Slot   pKeySlotUsedForStartup    TXST. Defines whether the key slot is used to transmit startup Frames. The bit        can be modified in   8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221  state only."]
    #[inline(always)]
    pub fn txst(self) -> crate::common::RegisterFieldBool<8, 1, 0, Succ1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Succ1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmit SYNC Frame in Key Slot  pKeySlotUsedForSync    TXSY. Defines whether the key slot is used to transmit SYNC Frames. The bit        can be modified in   8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221  state only. 1         2"]
    #[inline(always)]
    pub fn txsy(self) -> crate::common::RegisterFieldBool<9, 1, 0, Succ1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Succ1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Cold Start Attempts  gColdStartAttempts    CSA. Configures the maximum number of attempts that a cold starting node is        permitted to try to start up the network without receiving any valid        response from another node. It can be modified in   8220 DEFAULT CONFIG  8221  or          8220 CONFIG  8221  state only. Must be identical in all nodes of a cluster. Valid        values are 2 to 31."]
    #[inline(always)]
    pub fn csa(
        self,
    ) -> crate::common::RegisterField<11, 0x1f, 1, 0, u8, Succ1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1f,1,0,u8, Succ1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Passive to Active  pAllowPassiveToActive    PTA. Defines the number of consecutive even   odd cycle pairs that must have        valid clock correction terms before the Communication Controller is        allowed to transit from   8220 NORMAL PASSIVE  8221  to   8220 NORMAL ACTIVE  8221  state. If        set to 00000 B the Communication        Controller is not allowed to transit from   8220 NORMAL PASSIVE  8221  to          8220 NORMAL ACTIVE  8221  state. It can be modified in   8220 DEFAULT CONFIG  8221  or          8220 CONFIG  8221  state only. Valid values are 0 to 31 even   odd cycle pairs."]
    #[inline(always)]
    pub fn pta(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Succ1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Succ1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wakeup Channel Select  pWakeupChannel    WUCS. With this bit the Host selects the channel on which the Communication        Controller sends the Wakeup pattern. The Communication Controller        ignores any attempt to change the status of this bit when not in          8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221  state."]
    #[inline(always)]
    pub fn wucs(self) -> crate::common::RegisterFieldBool<21, 1, 0, Succ1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Succ1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Slot Mode  pSingleSlotEnabled    TSM. Selects the initial transmission slot mode. In SINGLE slot mode the        Communication Controller may only transmit in the preconfigured key        slot. The key slot ID is configured in the Header Section of Message        Buffer 0 respectively Message Buffers 0 and 1 depending on bit MRC.SPLM.        In case SUCC1.TSM   1  Message Buffer 0 respectively Message Buffers 0 1        can be  re configured in   8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221  state only. In ALL        slot mode the Communication Controller may transmit in all slots. The        bit can be written in   8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221  state only. The        communication controller changes to ALL slot mode when the Host        successfully applied the ALL SLOTS command by writing SUCC1.CMD  160    160 0101 B in POC states   8220 NORMAL ACTIVE  8221  or   8220 NORMAL PASSIVE  8221 . The actual slot mode        is monitored by CCSV.SLM."]
    #[inline(always)]
    pub fn tsm(self) -> crate::common::RegisterFieldBool<22, 1, 0, Succ1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Succ1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Halt due to Clock Sync Error  pAllowHaltDueToClock    HCSE. Controls the transition to   8220 HALT  8221  state due to a clock synchronization        error. The bit can be modified in   8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221  state        only."]
    #[inline(always)]
    pub fn hcse(self) -> crate::common::RegisterFieldBool<23, 1, 0, Succ1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Succ1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Select Channel A for MTS Transmission   MTSA. The bit selects channel A for MTS symbol transmission. The flag is reset        by default and may be modified only in   8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221         state."]
    #[inline(always)]
    pub fn mtsa(self) -> crate::common::RegisterFieldBool<24, 1, 0, Succ1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Succ1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Select Channel B for MTS Transmission   MTSB. The bit selects channel B for MTS symbol transmission. The flag is reset        by default and may be modified only in   8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221         state."]
    #[inline(always)]
    pub fn mtsb(self) -> crate::common::RegisterFieldBool<25, 1, 0, Succ1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Succ1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Connected to Channel A  pChannels    CCHA. Configures whether the node is connected to channel A."]
    #[inline(always)]
    pub fn ccha(self) -> crate::common::RegisterFieldBool<26, 1, 0, Succ1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Succ1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Connected to Channel B  pChannels    CCHB. Configures whether the node is connected to channel B."]
    #[inline(always)]
    pub fn cchb(self) -> crate::common::RegisterFieldBool<27, 1, 0, Succ1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Succ1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Succ1 {
    #[inline(always)]
    fn default() -> Succ1 {
        <crate::RegValueT<Succ1_SPEC> as RegisterValue<_>>::new(205524992)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Succ2_SPEC;
impl crate::sealed::RegSpec for Succ2_SPEC {
    type DataType = u32;
}
#[doc = "SUC Configuration Register 2\n resetvalue={Application Reset:0x1000504}"]
pub type Succ2 = crate::RegValueT<Succ2_SPEC>;

impl Succ2 {
    #[doc = "Listen Timeout pdListenTimeout    LT. Configures wakeup   startup listen timeout in Microticks. The range for wakeup   startup listen timeout  pdListenTimeout  is 1284 to 1283846  504 H to 139706 H   Microticks"]
    #[inline(always)]
    pub fn lt(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffff, 1, 0, u32, Succ2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1fffff,1,0,u32, Succ2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Listen Time out Noise  gListenNoise   1     LTN. Configures the upper limit for startup and wakeup listen timeout in the        presence of noise expressed as a multiple of the cluster constant        pdListenTimeout. The range of pdListenTimeout 2 to 16. LTN must be        configured identical in all nodes of a cluster. This bit can be updated        in  quot DEFAULT CONFIG quot  or  quot CONFIG quot  state only."]
    #[inline(always)]
    pub fn ltn(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Succ2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Succ2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Succ2 {
    #[inline(always)]
    fn default() -> Succ2 {
        <crate::RegValueT<Succ2_SPEC> as RegisterValue<_>>::new(16778500)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Succ3_SPEC;
impl crate::sealed::RegSpec for Succ3_SPEC {
    type DataType = u32;
}
#[doc = "SUC Configuration Register 3\n resetvalue={Application Reset:0x11}"]
pub type Succ3 = crate::RegValueT<Succ3_SPEC>;

impl Succ3 {
    #[doc = "Maximum Without Clock Correction Passive gMaxWithoutClockCorrectionPassive    WCP. Defines the number of consecutive even   odd cycle pairs with missing        clock correction terms that will cause a transition from   8220 NORMAL ACTIVE  8221         to   8220 NORMAL PASSIVE  8221  state. Must be identical in all nodes of a cluster.        Valid values are 1 to 15  1 H to F H          cycle pairs."]
    #[inline(always)]
    pub fn wcp(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Succ3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Succ3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Maximum Without Clock Correction Fatal  gMaxWithoutClockCorrecti on Fatal    WCF. Defines the number of consecutive even   odd cycle pairs with missing        clock correction terms that will cause a transition from   8220 NORMAL ACTIVE  8221         or   8220 NORMAL PASSIVE  8221  to   8220 HALT  8221  state. Must be identical in all nodes of a        cluster. Valid values are 1 to 15  1 H to F H  cycle pairs."]
    #[inline(always)]
    pub fn wcf(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Succ3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Succ3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Succ3 {
    #[inline(always)]
    fn default() -> Succ3 {
        <crate::RegValueT<Succ3_SPEC> as RegisterValue<_>>::new(17)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swnit_SPEC;
impl crate::sealed::RegSpec for Swnit_SPEC {
    type DataType = u32;
}
#[doc = "Symbol Window and Network Idle Time Status\n resetvalue={Application Reset:0x0}"]
pub type Swnit = crate::RegValueT<Swnit_SPEC>;

impl Swnit {
    #[doc = "Syntax Error in Symbol Window Channel A vSS SyntaxErrorA    SESA"]
    #[inline(always)]
    pub fn sesa(self) -> crate::common::RegisterFieldBool<0, 1, 0, Swnit_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Swnit_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Slot Boundary Violation in Symbol Window Channel A vSS BViolationA    SBSA"]
    #[inline(always)]
    pub fn sbsa(self) -> crate::common::RegisterFieldBool<1, 1, 0, Swnit_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Swnit_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Conflict in Symbol Window Channel A vSS TxConflictA    TCSA"]
    #[inline(always)]
    pub fn tcsa(self) -> crate::common::RegisterFieldBool<2, 1, 0, Swnit_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Swnit_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Syntax Error in Symbol Window Channel B vSS SyntaxErrorB    SESB"]
    #[inline(always)]
    pub fn sesb(self) -> crate::common::RegisterFieldBool<3, 1, 0, Swnit_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Swnit_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Slot Boundary Violation in Symbol Window Channel B vSS BViolationB    SBSB"]
    #[inline(always)]
    pub fn sbsb(self) -> crate::common::RegisterFieldBool<4, 1, 0, Swnit_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Swnit_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Conflict in Symbol Window Channel B vSS TxConflictB    TCSB"]
    #[inline(always)]
    pub fn tcsb(self) -> crate::common::RegisterFieldBool<5, 1, 0, Swnit_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Swnit_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MTS Received on Channel A vSS ValidMTSA    MTSA. Media Access Test symbol received on channel A during the proceeding        symbol window. Updated by the Communication Controller for each channel        at the end of the symbol window. When this bit is set to 1  also        interrupt flag SIR.MTSA is set to 1."]
    #[inline(always)]
    pub fn mtsa(self) -> crate::common::RegisterFieldBool<6, 1, 0, Swnit_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Swnit_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MTS Received on Channel B vSS ValidMTSB    MTSB. Media Access Test symbol received on channel B during the proceeding        symbol window. Updated by the Communication Controller for each channel        at the end of the symbol window. When this bit is set to 1  also        interrupt flag SIR.MTSB is set to 1."]
    #[inline(always)]
    pub fn mtsb(self) -> crate::common::RegisterFieldBool<7, 1, 0, Swnit_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Swnit_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Syntax Error during network idle time  NIT  Channel A vSS SyntaxErrorA    SENA. Updated by the Communication Controller channel A at the end of the NIT."]
    #[inline(always)]
    pub fn sena(self) -> crate::common::RegisterFieldBool<8, 1, 0, Swnit_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Swnit_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Slot Boundary Violation during network idle time  NIT  Channel A vSS BViolationA    SBNA. Updated by the Communication Controller channel A at the end of the NIT."]
    #[inline(always)]
    pub fn sbna(self) -> crate::common::RegisterFieldBool<9, 1, 0, Swnit_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Swnit_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Syntax Error during network idle time  NIT  Channel B vSS SyntaxErrorB    SENB. Updated by the Communication Controller channel B at the end of the NIT."]
    #[inline(always)]
    pub fn senb(self) -> crate::common::RegisterFieldBool<10, 1, 0, Swnit_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Swnit_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Slot Boundary Violation during network idle time  NIT  Channel B vSS BViolationB    SBNB. Updated by the Communication Controller channel B at the end of the NIT."]
    #[inline(always)]
    pub fn sbnb(self) -> crate::common::RegisterFieldBool<11, 1, 0, Swnit_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Swnit_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Swnit {
    #[inline(always)]
    fn default() -> Swnit {
        <crate::RegValueT<Swnit_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T0C_SPEC;
impl crate::sealed::RegSpec for T0C_SPEC {
    type DataType = u32;
}
#[doc = "Timer 0 Configuration\n resetvalue={Application Reset:0x0}"]
pub type T0C = crate::RegValueT<T0C_SPEC>;

impl T0C {
    #[doc = "Timer 0 Run Control   T0RC"]
    #[inline(always)]
    pub fn t0rc(self) -> crate::common::RegisterFieldBool<0, 1, 0, T0C_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, T0C_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer 0 Mode Select   T0MS"]
    #[inline(always)]
    pub fn t0ms(self) -> crate::common::RegisterFieldBool<1, 1, 0, T0C_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, T0C_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer 0 Cycle Code   T0CC. The 7 bit timer 0 cycle code determines the cycle set used for        generation of the timer 0 service request. For details about the        configuration of the cycle code see   8220 Cycle Counter Filtering  8221 ."]
    #[inline(always)]
    pub fn t0cc(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, T0C_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7f,1,0,u8, T0C_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer 0 Macrotick Offset   T0MO. Configures the Macrotick offset from the beginning of the cycle where the service request is to occur. The Timer 0 Service Request occurs at this offset for each cycle of the cycle set."]
    #[inline(always)]
    pub fn t0mo(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, T0C_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3fff,1,0,u16, T0C_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T0C {
    #[inline(always)]
    fn default() -> T0C {
        <crate::RegValueT<T0C_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T1C_SPEC;
impl crate::sealed::RegSpec for T1C_SPEC {
    type DataType = u32;
}
#[doc = "Timer 1 Configuration\n resetvalue={Application Reset:0x20000}"]
pub type T1C = crate::RegValueT<T1C_SPEC>;

impl T1C {
    #[doc = "Timer 1 Run Control   T1RC"]
    #[inline(always)]
    pub fn t1rc(self) -> crate::common::RegisterFieldBool<0, 1, 0, T1C_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, T1C_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer 1 Mode Select   T1MS"]
    #[inline(always)]
    pub fn t1ms(self) -> crate::common::RegisterFieldBool<1, 1, 0, T1C_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, T1C_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer 1 Macrotick Count   T1MC. When the configured Macrotick count is reached the timer 1 service request is generated. Valid values are  2 H  3FFF H Macroticks in continuous mode 1 H  3FFF H Macroticks in single shot mode"]
    #[inline(always)]
    pub fn t1mc(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, T1C_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3fff,1,0,u16, T1C_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T1C {
    #[inline(always)]
    fn default() -> T1C {
        <crate::RegValueT<T1C_SPEC> as RegisterValue<_>>::new(131072)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Test1_SPEC;
impl crate::sealed::RegSpec for Test1_SPEC {
    type DataType = u32;
}
#[doc = "Test Register 1\n resetvalue={Application Reset:0x300}"]
pub type Test1 = crate::RegValueT<Test1_SPEC>;

impl Test1 {
    #[doc = "Write Test Register Enable   WRTEN. Enables write access to the test registers. To set the bit from 0 to 1        the test mode key has to be written as defined on   8220 Lock Register  8221 . The        unlock sequence is not required when TEST1.WRTEN is kept at 1 while        other bits of the register are changed. The bit can be reset to 0 at any        time."]
    #[inline(always)]
    pub fn wrten(self) -> crate::common::RegisterFieldBool<0, 1, 0, Test1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Test1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "External Loop Back Enable   ELBE. There are two possibilities to perform a loop back test. External loop back via physical layer or internal loop back for in system self test  default . In case of an internal loop back pins TXENA and TXENB are in their inactive state  pins TXDA and TXDB are set to HIGH  pins RXDA and RXDB are not evaluated. Bit ELBE is evaluated only when POC is in loop back mode and test multiplexer control is in non multiplexed mode TMC   00."]
    #[inline(always)]
    pub fn elbe(self) -> crate::common::RegisterFieldBool<1, 1, 0, Test1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Test1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Test Multiplexer Control   TMC"]
    #[inline(always)]
    pub fn tmc(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Test1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Test1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Activity on A   AOA. The channel idle condition is specified in the FlexRay  protocol spec v2.1  chapter 3  BITSTRB process  zChannelIdle ."]
    #[inline(always)]
    pub fn aoa(self) -> crate::common::RegisterFieldBool<8, 1, 0, Test1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Test1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Activity on B   AOB. The channel idle condition is specified in the FlexRay  8482  protocol spec        v2.1  chapter 3  BITSTRB process  zChannelIdle ."]
    #[inline(always)]
    pub fn aob(self) -> crate::common::RegisterFieldBool<9, 1, 0, Test1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Test1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Read Channel A Receive Pin   RXA. This bit field shows the current logic state of RXDA."]
    #[inline(always)]
    pub fn rxa(self) -> crate::common::RegisterFieldBool<16, 1, 0, Test1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Test1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Read Channel B Receive Pin   RXB. This bit field shows the current logic state of RXDB."]
    #[inline(always)]
    pub fn rxb(self) -> crate::common::RegisterFieldBool<17, 1, 0, Test1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Test1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Read or Write to Channel A Transmit Pin   TXA. A write to this bit field sets the TXDA to corresponding logic state. A        read from this bit field shows the current logic state of TXDA."]
    #[inline(always)]
    pub fn txa(self) -> crate::common::RegisterFieldBool<18, 1, 0, Test1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Test1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Read or Write to Channel B Transmit Pin   TXB. A write to this bit field sets the TXDB to corresponding logic state. A        read from this bit field shows the current logic state of TXDB."]
    #[inline(always)]
    pub fn txb(self) -> crate::common::RegisterFieldBool<19, 1, 0, Test1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Test1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Read or Write to Channel A Transmit Enable Pin   TXENA. A write to this bit field sets the TXENA to corresponding logic state. A        read from this bit field shows the current logic state of TXENA."]
    #[inline(always)]
    pub fn txena(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Test1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Test1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Read or Write to Channel B Transmit Enable Pin   TXENB. A write to this bit field sets the TXENB to corresponding logic state. A        read from this bit field shows the current logic state of TXENB."]
    #[inline(always)]
    pub fn txenb(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Test1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Test1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Coding Error Report Channel A   CERA. Set when a coding error is detected on channel A. Reset to zero when        register TEST1 is read or written. Once the CERA is set it will remain        unchanged until the Host accesses the TEST1 register. Other combinations are reserved."]
    #[inline(always)]
    pub fn cera(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Test1_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Test1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Coding Error Report Channel B    CERB. Set when a coding error is detected on channel B. Reset to zero when        register TEST1 is read or written. Once the CERB is set it will remain        unchanged until the Host accesses the TEST1 register. Other combinations are reserved."]
    #[inline(always)]
    pub fn cerb(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Test1_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Test1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Test1 {
    #[inline(always)]
    fn default() -> Test1 {
        <crate::RegValueT<Test1_SPEC> as RegisterValue<_>>::new(768)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Test2_SPEC;
impl crate::sealed::RegSpec for Test2_SPEC {
    type DataType = u32;
}
#[doc = "Test Register 2\n resetvalue={Application Reset:0x0}"]
pub type Test2 = crate::RegValueT<Test2_SPEC>;

impl Test2 {
    #[doc = "RAM Select   RS. In RAM Test mode the RAM blocks selected by RS are mapped to module        address 0000  160 0400 H to 0000  160 07FF H  1024 byte addresses ."]
    #[inline(always)]
    pub fn rs(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Test2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Test2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Segment Select   SSEL. To enable access to the complete Message RAM  8192 byte addresses  the        Message RAM is segmented."]
    #[inline(always)]
    pub fn ssel(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, Test2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8, Test2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Write ECC Data Enable   WRECC. Content of ECCW is transferred to the RAM  Test mode must be entered. See   8220 Test Register 1  8221"]
    #[inline(always)]
    pub fn wrecc(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Test2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Test2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Test2 {
    #[inline(always)]
    fn default() -> Test2 {
        <crate::RegValueT<Test2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txrq1_SPEC;
impl crate::sealed::RegSpec for Txrq1_SPEC {
    type DataType = u32;
}
#[doc = "Transmission Request Register 1\n resetvalue={Application Reset:0x0}"]
pub type Txrq1 = crate::RegValueT<Txrq1_SPEC>;

impl Txrq1 {
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr25(self) -> crate::common::RegisterFieldBool<25, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr26(self) -> crate::common::RegisterFieldBool<26, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr27(self) -> crate::common::RegisterFieldBool<27, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr28(self) -> crate::common::RegisterFieldBool<28, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr29(self) -> crate::common::RegisterFieldBool<29, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr30(self) -> crate::common::RegisterFieldBool<30, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr31(self) -> crate::common::RegisterFieldBool<31, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Txrq1 {
    #[inline(always)]
    fn default() -> Txrq1 {
        <crate::RegValueT<Txrq1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txrq2_SPEC;
impl crate::sealed::RegSpec for Txrq2_SPEC {
    type DataType = u32;
}
#[doc = "Transmission Request Register 2\n resetvalue={Application Reset:0x0}"]
pub type Txrq2 = crate::RegValueT<Txrq2_SPEC>;

impl Txrq2 {
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr42(self) -> crate::common::RegisterFieldBool<10, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr43(self) -> crate::common::RegisterFieldBool<11, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr44(self) -> crate::common::RegisterFieldBool<12, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr45(self) -> crate::common::RegisterFieldBool<13, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr46(self) -> crate::common::RegisterFieldBool<14, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr47(self) -> crate::common::RegisterFieldBool<15, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr48(self) -> crate::common::RegisterFieldBool<16, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr49(self) -> crate::common::RegisterFieldBool<17, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr50(self) -> crate::common::RegisterFieldBool<18, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr51(self) -> crate::common::RegisterFieldBool<19, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr52(self) -> crate::common::RegisterFieldBool<20, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr53(self) -> crate::common::RegisterFieldBool<21, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr54(self) -> crate::common::RegisterFieldBool<22, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr55(self) -> crate::common::RegisterFieldBool<23, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr56(self) -> crate::common::RegisterFieldBool<24, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr57(self) -> crate::common::RegisterFieldBool<25, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr58(self) -> crate::common::RegisterFieldBool<26, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr59(self) -> crate::common::RegisterFieldBool<27, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr60(self) -> crate::common::RegisterFieldBool<28, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr61(self) -> crate::common::RegisterFieldBool<29, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr62(self) -> crate::common::RegisterFieldBool<30, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr63(self) -> crate::common::RegisterFieldBool<31, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Txrq2 {
    #[inline(always)]
    fn default() -> Txrq2 {
        <crate::RegValueT<Txrq2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txrq3_SPEC;
impl crate::sealed::RegSpec for Txrq3_SPEC {
    type DataType = u32;
}
#[doc = "Transmission Request Register 3\n resetvalue={Application Reset:0x0}"]
pub type Txrq3 = crate::RegValueT<Txrq3_SPEC>;

impl Txrq3 {
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr64(self) -> crate::common::RegisterFieldBool<0, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr65(self) -> crate::common::RegisterFieldBool<1, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr66(self) -> crate::common::RegisterFieldBool<2, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr67(self) -> crate::common::RegisterFieldBool<3, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr68(self) -> crate::common::RegisterFieldBool<4, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr69(self) -> crate::common::RegisterFieldBool<5, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr70(self) -> crate::common::RegisterFieldBool<6, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr71(self) -> crate::common::RegisterFieldBool<7, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr72(self) -> crate::common::RegisterFieldBool<8, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr73(self) -> crate::common::RegisterFieldBool<9, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr74(self) -> crate::common::RegisterFieldBool<10, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr75(self) -> crate::common::RegisterFieldBool<11, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr76(self) -> crate::common::RegisterFieldBool<12, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr77(self) -> crate::common::RegisterFieldBool<13, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr78(self) -> crate::common::RegisterFieldBool<14, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr79(self) -> crate::common::RegisterFieldBool<15, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr80(self) -> crate::common::RegisterFieldBool<16, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr81(self) -> crate::common::RegisterFieldBool<17, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr82(self) -> crate::common::RegisterFieldBool<18, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr83(self) -> crate::common::RegisterFieldBool<19, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr84(self) -> crate::common::RegisterFieldBool<20, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr85(self) -> crate::common::RegisterFieldBool<21, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr86(self) -> crate::common::RegisterFieldBool<22, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr87(self) -> crate::common::RegisterFieldBool<23, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr88(self) -> crate::common::RegisterFieldBool<24, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr89(self) -> crate::common::RegisterFieldBool<25, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr90(self) -> crate::common::RegisterFieldBool<26, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr91(self) -> crate::common::RegisterFieldBool<27, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr92(self) -> crate::common::RegisterFieldBool<28, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr93(self) -> crate::common::RegisterFieldBool<29, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr94(self) -> crate::common::RegisterFieldBool<30, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr95(self) -> crate::common::RegisterFieldBool<31, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Txrq3 {
    #[inline(always)]
    fn default() -> Txrq3 {
        <crate::RegValueT<Txrq3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txrq4_SPEC;
impl crate::sealed::RegSpec for Txrq4_SPEC {
    type DataType = u32;
}
#[doc = "Transmission Request Register 4\n resetvalue={Application Reset:0x0}"]
pub type Txrq4 = crate::RegValueT<Txrq4_SPEC>;

impl Txrq4 {
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr96(self) -> crate::common::RegisterFieldBool<0, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr97(self) -> crate::common::RegisterFieldBool<1, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr98(self) -> crate::common::RegisterFieldBool<2, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr99(self) -> crate::common::RegisterFieldBool<3, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr100(self) -> crate::common::RegisterFieldBool<4, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr101(self) -> crate::common::RegisterFieldBool<5, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr102(self) -> crate::common::RegisterFieldBool<6, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr103(self) -> crate::common::RegisterFieldBool<7, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr104(self) -> crate::common::RegisterFieldBool<8, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr105(self) -> crate::common::RegisterFieldBool<9, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr106(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr107(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr108(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr109(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr110(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr111(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr112(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr113(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr114(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr115(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr116(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr117(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr118(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr119(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr120(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr121(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr122(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr123(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr124(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr125(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr126(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr127(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Txrq4 {
    #[inline(always)]
    fn default() -> Txrq4 {
        <crate::RegValueT<Txrq4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WrdSn_SPEC;
impl crate::sealed::RegSpec for WrdSn_SPEC {
    type DataType = u32;
}
#[doc = "Write Data Section 01\n resetvalue={Application Reset:0x0}"]
pub type WrdSn = crate::RegValueT<WrdSn_SPEC>;

impl WrdSn {
    #[doc = "32 Bit Word nn  Byte 0   MDWB0"]
    #[inline(always)]
    pub fn mdwb0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, WrdSn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, WrdSn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "32 Bit Word nn  Byte 1   MDWB1"]
    #[inline(always)]
    pub fn mdwb1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, WrdSn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, WrdSn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "32 Bit Word nn  Byte 2   MDWB2"]
    #[inline(always)]
    pub fn mdwb2(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, WrdSn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, WrdSn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "32 Bit Word nn  Byte 3   MDWB3"]
    #[inline(always)]
    pub fn mdwb3(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, WrdSn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, WrdSn_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for WrdSn {
    #[inline(always)]
    fn default() -> WrdSn {
        <crate::RegValueT<WrdSn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wrhs1_SPEC;
impl crate::sealed::RegSpec for Wrhs1_SPEC {
    type DataType = u32;
}
#[doc = "Write Header Section 1\n resetvalue={Application Reset:0x0}"]
pub type Wrhs1 = crate::RegValueT<Wrhs1_SPEC>;

impl Wrhs1 {
    #[doc = "Frame ID   FID. Frame ID of the selected Message Buffer. The Frame ID defines the slot        number for transmission   reception of the respective message. Message        Buffers with Frame ID   0 are considered as not valid."]
    #[inline(always)]
    pub fn fid(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Wrhs1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Wrhs1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Cycle Code   CYC. The 7 bit cycle code determines the cycle set used for cycle counter        filtering. For details about the configuration of the cycle code see        Section  160  quot Cycle Counter Filtering quot ."]
    #[inline(always)]
    pub fn cyc(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Wrhs1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Wrhs1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel Filter Control A   CHA. The channel filtering field A associated with the buffer serves of        channel A as a filter for receive buffers  and as a control field for        transmit buffers"]
    #[inline(always)]
    pub fn cha(self) -> crate::common::RegisterFieldBool<24, 1, 0, Wrhs1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Wrhs1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel Filter Control B   CHB. The channel filtering field B associated with the buffer serves of        channel B as a filter for receive buffers  and as a control field for        transmit buffers"]
    #[inline(always)]
    pub fn chb(self) -> crate::common::RegisterFieldBool<25, 1, 0, Wrhs1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Wrhs1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Direction Configuration Bit   CFG. This bit is used to configure the corresponding buffer as a transmit        buffer or as a receive buffer. For Message Buffers belonging to the        receive FIFO the bit is not evaluated."]
    #[inline(always)]
    pub fn cfg(self) -> crate::common::RegisterFieldBool<26, 1, 0, Wrhs1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Wrhs1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Payload Preamble Indicator Transmit   PPIT. This bit is used to control the state of the Payload Preamble Indicator        in transmit Frames. If the bit is set in a static Message Buffer  the        respective Message Buffer holds Network Management information. If the        bit is set in a dynamic Message Buffer the first two byte of the Payload        Segment may be used for message ID filtering by the receiver. Message ID        filtering of received FlexRay  8482  Frames is not supported by the E Ray module  but can be done by the Host."]
    #[inline(always)]
    pub fn ppit(self) -> crate::common::RegisterFieldBool<27, 1, 0, Wrhs1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Wrhs1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Mode   TXM. This bit is used to select the transmission mode  see   8220 Transmit        Buffers  8221  ."]
    #[inline(always)]
    pub fn txm(self) -> crate::common::RegisterFieldBool<28, 1, 0, Wrhs1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Wrhs1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Service Request   MBI. This bit enables the receive   transmit service request for the        corresponding Message Buffer. After a dedicated receive buffer has been        updated by the Message Handler  flag SIR.RXI and  or SIR.MBSI in the        Status Service Request register are set. After a transmission has        completed flag SIR.TXI is set."]
    #[inline(always)]
    pub fn mbi(self) -> crate::common::RegisterFieldBool<29, 1, 0, Wrhs1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Wrhs1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Wrhs1 {
    #[inline(always)]
    fn default() -> Wrhs1 {
        <crate::RegValueT<Wrhs1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wrhs2_SPEC;
impl crate::sealed::RegSpec for Wrhs2_SPEC {
    type DataType = u32;
}
#[doc = "Write Header Section 2\n resetvalue={Application Reset:0x0}"]
pub type Wrhs2 = crate::RegValueT<Wrhs2_SPEC>;

impl Wrhs2 {
    #[doc = "Header CRC vRF Header HeaderCRC    CRC. Receive Buffer  Configuration not required Transmit Buffer  Header CRC calculated and configured by the Host. For calculation of the Header CRC the payload length of the Frame send on the bus has to be considered. In static segment the payload length of all Frames is configured by MHDC.SFDL."]
    #[inline(always)]
    pub fn crc(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Wrhs2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Wrhs2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Payload Length Configured   PLC. Length of Data Section  number of 2 byte words  as configured by the Host. During static segment the static Frame payload length as configured by MHDC.SFDL in the MHD Configuration Register defines the payload length for all static Frames. If the payload length configured by PLC is shorter than this value padding byte are inserted to ensure that Frames have proper physical length. The padding pattern is logical zero."]
    #[inline(always)]
    pub fn plc(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Wrhs2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Wrhs2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Wrhs2 {
    #[inline(always)]
    fn default() -> Wrhs2 {
        <crate::RegValueT<Wrhs2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wrhs3_SPEC;
impl crate::sealed::RegSpec for Wrhs3_SPEC {
    type DataType = u32;
}
#[doc = "Write Header Section 3\n resetvalue={Application Reset:0x0}"]
pub type Wrhs3 = crate::RegValueT<Wrhs3_SPEC>;

impl Wrhs3 {
    #[doc = "Data Pointer   DP. Pointer to the first 32 bit word of the Data Section of the addressed Message Buffer in the Message RAM."]
    #[inline(always)]
    pub fn dp(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Wrhs3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Wrhs3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Wrhs3 {
    #[inline(always)]
    fn default() -> Wrhs3 {
        <crate::RegValueT<Wrhs3_SPEC> as RegisterValue<_>>::new(0)
    }
}
