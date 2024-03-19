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
#[doc = r"System Control Unit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scu(pub(super) *mut u8);
unsafe impl core::marker::Send for Scu {}
unsafe impl core::marker::Sync for Scu {}
impl Scu {
    #[doc = "Access Enable Register 00\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen00(&self) -> crate::common::Reg<self::Accen00_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1020usize)) }
    }

    #[doc = "Access Enable Register 10\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen10(&self) -> crate::common::Reg<self::Accen10_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1012usize)) }
    }

    #[doc = "Application Reset Disable Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn arstdis(&self) -> crate::common::Reg<self::Arstdis_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(92usize)) }
    }

    #[doc = "CCU Clock Control Register 0\n resetvalue={After SSW execution:0x5120112,System Reset:0x3130113}"]
    #[inline(always)]
    pub const fn ccucon0(&self) -> crate::common::Reg<self::Ccucon0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }

    #[doc = "CCU Clock Control Register 1\n resetvalue={System Reset:0x10100302}"]
    #[inline(always)]
    pub const fn ccucon1(&self) -> crate::common::Reg<self::Ccucon1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "CCU Clock Control Register 2\n resetvalue={System Reset:0x7000101}"]
    #[inline(always)]
    pub const fn ccucon2(&self) -> crate::common::Reg<self::Ccucon2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "CCU Clock Control Register 3\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn ccucon3(&self) -> crate::common::Reg<self::Ccucon3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }

    #[doc = "CCU Clock Control Register 4\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn ccucon4(&self) -> crate::common::Reg<self::Ccucon4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }

    #[doc = "CCU Clock Control Register 5\n resetvalue={System Reset:0x30}"]
    #[inline(always)]
    pub const fn ccucon5(&self) -> crate::common::Reg<self::Ccucon5_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(76usize)) }
    }

    #[doc = "CCU Clock Control Register 6\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn ccucon6(&self) -> crate::common::Reg<self::Ccucon6_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "CCU Clock Control Register 7\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn ccucon7(&self) -> crate::common::Reg<self::Ccucon7_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }

    #[doc = "CCU Clock Control Register 8\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn ccucon8(&self) -> crate::common::Reg<self::Ccucon8_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }

    #[doc = "Chip Identification Register\n resetvalue={System Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn chipid(&self) -> crate::common::Reg<self::Chipid_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(320usize)) }
    }

    #[doc = "Core Die Temperature Sensor Limit Register\n resetvalue={Application Reset:0x0CD806D6}"]
    #[inline(always)]
    pub const fn dtsclim(&self) -> crate::common::Reg<self::Dtsclim_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(264usize)) }
    }

    #[doc = "Core Die Temperature Sensor Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dtscstat(&self) -> crate::common::Reg<self::Dtscstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }

    #[doc = "ENDINIT Global Control Register 0\n resetvalue={Application Reset:0x0FFFC000E}"]
    #[inline(always)]
    pub const fn eicon0(&self) -> crate::common::Reg<self::Eicon0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(668usize)) }
    }

    #[doc = "ENDINIT Global Control Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn eicon1(&self) -> crate::common::Reg<self::Eicon1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(672usize)) }
    }

    #[doc = "External Input Channel Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn eicri(&self) -> [crate::common::Reg<self::EicRi_SPEC, crate::common::RW>; 4] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x210usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x210usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x210usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x210usize + 0xcusize)),
            ]
        }
    }

    #[doc = "External Input Filter Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn eifilt(&self) -> crate::common::Reg<self::Eifilt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(524usize)) }
    }

    #[doc = "External Input Flag Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn eifr(&self) -> crate::common::Reg<self::Eifr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(544usize)) }
    }

    #[doc = "ENDINIT Timeout Counter Status Register\n resetvalue={Application Reset:0x0FFFC0000}"]
    #[inline(always)]
    pub const fn eisr(&self) -> crate::common::Reg<self::Eisr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(676usize)) }
    }

    #[doc = "Emergency Stop Register\n resetvalue={Application Reset:0x1}"]
    #[inline(always)]
    pub const fn emsr(&self) -> crate::common::Reg<self::Emsr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "Emergency Stop Software set and clear register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn emssw(&self) -> crate::common::Reg<self::Emssw_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
    }

    #[doc = "ESR Output Configuration Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn esrocfg(&self) -> crate::common::Reg<self::Esrocfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(120usize)) }
    }

    #[doc = "External Clock Control Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn extcon(&self) -> crate::common::Reg<self::Extcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "Fractional Divider Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn fdr(&self) -> crate::common::Reg<self::Fdr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }

    #[doc = "Flag Modification Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fmr(&self) -> crate::common::Reg<self::Fmr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(548usize)) }
    }

    #[doc = "Identification Register\n resetvalue={System Reset:0x0C4C0C1}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Flag Gating Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn igcrj(&self) -> [crate::common::Reg<self::IgcRj_SPEC, crate::common::RW>; 4] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x22cusize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x22cusize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x22cusize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x22cusize + 0xcusize)),
            ]
        }
    }

    #[doc = "ESR Input Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn r#in(&self) -> crate::common::Reg<self::In_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(172usize)) }
    }

    #[doc = "Input Output Control Register\n resetvalue={System Reset:0x0E0}"]
    #[inline(always)]
    pub const fn iocr(&self) -> crate::common::Reg<self::Iocr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }

    #[doc = "Logic BIST Control 0 Register\n resetvalue={System Reset:0x0,CFS Value:0x400}"]
    #[inline(always)]
    pub const fn lbistctrl0(&self) -> crate::common::Reg<self::Lbistctrl0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(356usize)) }
    }

    #[doc = "Logic BIST Control 1 Register\n resetvalue={System Reset:0x0,CFS Value:0x54000007}"]
    #[inline(always)]
    pub const fn lbistctrl1(&self) -> crate::common::Reg<self::Lbistctrl1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(360usize)) }
    }

    #[doc = "Logic BIST Control 2 Register\n resetvalue={System Reset:0x0,CFS Value:0x3D}"]
    #[inline(always)]
    pub const fn lbistctrl2(&self) -> crate::common::Reg<self::Lbistctrl2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(364usize)) }
    }

    #[doc = "Logic BIST Control 3 Register\n resetvalue={System Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn lbistctrl3(&self) -> crate::common::Reg<self::Lbistctrl3_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(368usize)) }
    }

    #[doc = "LCL CPU0 and CPU2 Control Register\n resetvalue={Cold PowerOn Reset:0x080018001,Cold PowerOn Reset:0x080010000,Cold PowerOn Reset:0x080018001}"]
    #[inline(always)]
    pub const fn lclcon0(&self) -> crate::common::Reg<self::Lclcon0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(308usize)) }
    }

    #[doc = "LCL CPU1 and CPU3 Control Register\n resetvalue={Cold PowerOn Reset:0x080018001,Cold PowerOn Reset:0x080010000,Cold PowerOn Reset:0x0,Cold PowerOn Reset:0x080018001}"]
    #[inline(always)]
    pub const fn lclcon1(&self) -> crate::common::Reg<self::Lclcon1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(312usize)) }
    }

    #[doc = "LCL Test Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn lcltest(&self) -> crate::common::Reg<self::Lcltest_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(316usize)) }
    }

    #[doc = "Manufacturer Identification Register\n resetvalue={System Reset:0x1820}"]
    #[inline(always)]
    pub const fn manid(&self) -> crate::common::Reg<self::Manid_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(324usize)) }
    }

    #[doc = "ESR Output Modification Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn omr(&self) -> crate::common::Reg<self::Omr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(168usize)) }
    }

    #[doc = "OSC Control Register\n resetvalue={System Reset:0x10,System Reset:0x258}"]
    #[inline(always)]
    pub const fn osccon(&self) -> crate::common::Reg<self::Osccon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "ESR Output Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn out(&self) -> crate::common::Reg<self::Out_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(164usize)) }
    }

    #[doc = "Overlay Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ovccon(&self) -> crate::common::Reg<self::Ovccon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(484usize)) }
    }

    #[doc = "Overlay Enable Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ovcenable(&self) -> crate::common::Reg<self::Ovcenable_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(480usize)) }
    }

    #[doc = "Pad Disable Control Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn pdisc(&self) -> crate::common::Reg<self::Pdisc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(396usize)) }
    }

    #[doc = "ESR Pad Driver Mode Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn pdr(&self) -> crate::common::Reg<self::Pdr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(156usize)) }
    }

    #[doc = "Pattern Detection Result Register\n resetvalue={Application Reset:0x0FF}"]
    #[inline(always)]
    pub const fn pdrr(&self) -> crate::common::Reg<self::Pdrr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(552usize)) }
    }

    #[doc = "Peripheral PLL Configuration 0 Register\n resetvalue={System Reset:0x3E00}"]
    #[inline(always)]
    pub const fn perpllcon0(&self) -> crate::common::Reg<self::Perpllcon0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }

    #[doc = "Peripheral PLL Configuration 1 Register\n resetvalue={System Reset:0x1}"]
    #[inline(always)]
    pub const fn perpllcon1(&self) -> crate::common::Reg<self::Perpllcon1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }

    #[doc = "Peripheral PLL Status Register\n resetvalue={System Reset:0x2}"]
    #[inline(always)]
    pub const fn perpllstat(&self) -> crate::common::Reg<self::Perpllstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
    #[inline(always)]
    pub const fn pmcsr0(&self) -> crate::common::Reg<self::Pmcsr0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(200usize)) }
    }

    #[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
    #[inline(always)]
    pub const fn pmcsr1(&self) -> crate::common::Reg<self::Pmcsr1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(204usize)) }
    }

    #[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
    #[inline(always)]
    pub const fn pmcsr2(&self) -> crate::common::Reg<self::Pmcsr2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(208usize)) }
    }

    #[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
    #[inline(always)]
    pub const fn pmcsr3(&self) -> crate::common::Reg<self::Pmcsr3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(212usize)) }
    }

    #[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
    #[inline(always)]
    pub const fn pmcsr4(&self) -> crate::common::Reg<self::Pmcsr4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(216usize)) }
    }

    #[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
    #[inline(always)]
    pub const fn pmcsr5(&self) -> crate::common::Reg<self::Pmcsr5_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(220usize)) }
    }

    #[doc = "Power Management Status Register 0\n resetvalue={Application Reset:0x1}"]
    #[inline(always)]
    pub const fn pmstat0(&self) -> crate::common::Reg<self::Pmstat0_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(228usize)) }
    }

    #[doc = "Standby and Wake up Control Register 1\n resetvalue={Cold PowerOn Reset:0x1000000}"]
    #[inline(always)]
    pub const fn pmswcr1(&self) -> crate::common::Reg<self::Pmswcr1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(232usize)) }
    }

    #[doc = "Power Management Transition Control and Status Register 0\n resetvalue={Cold PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn pmtrcsr0(&self) -> crate::common::Reg<self::Pmtrcsr0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(408usize)) }
    }

    #[doc = "Power Management Transition Control and Status Register 1\n resetvalue={Cold PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn pmtrcsr1(&self) -> crate::common::Reg<self::Pmtrcsr1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(412usize)) }
    }

    #[doc = "Power Management Transition Control and Status Register 2\n resetvalue={Cold PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn pmtrcsr2(&self) -> crate::common::Reg<self::Pmtrcsr2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(416usize)) }
    }

    #[doc = "Power Management Transition Control and Status Register 3\n resetvalue={Cold PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn pmtrcsr3(&self) -> crate::common::Reg<self::Pmtrcsr3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(420usize)) }
    }

    #[doc = "Reset Configuration Register\n resetvalue={PowerOn Reset:0x282,PowerOn Reset:0x282}"]
    #[inline(always)]
    pub const fn rstcon(&self) -> crate::common::Reg<self::Rstcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(88usize)) }
    }

    #[doc = "Additional Reset Control Register\n resetvalue={Cold PowerOn Reset:0x0,Cold PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn rstcon2(&self) -> crate::common::Reg<self::Rstcon2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(100usize)) }
    }

    #[doc = "Reset Status Register\n resetvalue={Cold PowerOn Reset:0x3810000,Cold PowerOn Reset:0x13810000,Cold PowerOn Reset:0x10010000}"]
    #[inline(always)]
    pub const fn rststat(&self) -> crate::common::Reg<self::Rststat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }

    #[doc = "Safety ENDINIT Control Register 0\n resetvalue={Application Reset:0x0FFFC000E}"]
    #[inline(always)]
    pub const fn seicon0(&self) -> crate::common::Reg<self::Seicon0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(692usize)) }
    }

    #[doc = "Safety ENDINIT Control Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn seicon1(&self) -> crate::common::Reg<self::Seicon1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(696usize)) }
    }

    #[doc = "Safety ENDINIT Timeout Status Register\n resetvalue={Application Reset:0x0FFFC0000}"]
    #[inline(always)]
    pub const fn seisr(&self) -> crate::common::Reg<self::Seisr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(700usize)) }
    }

    #[doc = "Start up Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn stcon(&self) -> crate::common::Reg<self::Stcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(196usize)) }
    }

    #[doc = "Start up Memory Register 1\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn stmem1(&self) -> crate::common::Reg<self::Stmem1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(388usize)) }
    }

    #[doc = "Start up Memory Register 2\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn stmem2(&self) -> crate::common::Reg<self::Stmem2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(392usize)) }
    }

    #[doc = "Start up Memory Register 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn stmem3(&self) -> crate::common::Reg<self::Stmem3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(448usize)) }
    }

    #[doc = "Start up Memory Register 4\n resetvalue={Cold PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn stmem4(&self) -> crate::common::Reg<self::Stmem4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(452usize)) }
    }

    #[doc = "Start up Memory Register 5\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn stmem5(&self) -> crate::common::Reg<self::Stmem5_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(456usize)) }
    }

    #[doc = "Start up Memory Register 6\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn stmem6(&self) -> crate::common::Reg<self::Stmem6_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(460usize)) }
    }

    #[doc = "Start up Status Register\n resetvalue={PowerOn Reset:0x08000}"]
    #[inline(always)]
    pub const fn ststat(&self) -> crate::common::Reg<self::Ststat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(192usize)) }
    }

    #[doc = "Address Map Control Register\n resetvalue={System Reset:0x1}"]
    #[inline(always)]
    pub const fn swapctrl(&self) -> crate::common::Reg<self::Swapctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(332usize)) }
    }

    #[doc = "Software Reset Configuration Register\n resetvalue={PowerOn Reset:0x0,PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn swrstcon(&self) -> crate::common::Reg<self::Swrstcon_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }

    #[doc = "System Control Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn syscon(&self) -> crate::common::Reg<self::Syscon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(124usize)) }
    }

    #[doc = "System PLL Configuration 0 Register\n resetvalue={System Reset:0x40003A00}"]
    #[inline(always)]
    pub const fn syspllcon0(&self) -> crate::common::Reg<self::Syspllcon0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "System PLL Configuration 1 Register\n resetvalue={System Reset:0x5}"]
    #[inline(always)]
    pub const fn syspllcon1(&self) -> crate::common::Reg<self::Syspllcon1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "System PLL Configuration 2 Register\n resetvalue={System Reset:0x6000}"]
    #[inline(always)]
    pub const fn syspllcon2(&self) -> crate::common::Reg<self::Syspllcon2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "System PLL Status Register\n resetvalue={System Reset:0x2,System Reset:0x2}"]
    #[inline(always)]
    pub const fn syspllstat(&self) -> crate::common::Reg<self::Syspllstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Trap Clear Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn trapclr(&self) -> crate::common::Reg<self::Trapclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(300usize)) }
    }

    #[doc = "Trap Disable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn trapdis0(&self) -> crate::common::Reg<self::Trapdis0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(304usize)) }
    }

    #[doc = "Trap Set Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn trapset(&self) -> crate::common::Reg<self::Trapset_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(296usize)) }
    }

    #[doc = "Trap Status Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn trapstat(&self) -> crate::common::Reg<self::Trapstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(292usize)) }
    }
    #[doc = "ESRCFGx"]
    #[inline(always)]
    pub fn esrcfgx(self) -> [self::EsrcfGx; 2] {
        unsafe {
            [
                self::EsrcfGx(self.0.add(0x70usize + 0x0usize)),
                self::EsrcfGx(self.0.add(0x70usize + 0x4usize)),
            ]
        }
    }
    #[doc = "CPU watchdogs"]
    #[inline(always)]
    pub fn wdtcpu(self) -> [self::Wdtcpu; 3] {
        unsafe {
            [
                self::Wdtcpu(self.0.add(0x24cusize + 0x0usize)),
                self::Wdtcpu(self.0.add(0x24cusize + 0xcusize)),
                self::Wdtcpu(self.0.add(0x24cusize + 0x18usize)),
            ]
        }
    }
    #[doc = "WDTS"]
    #[inline(always)]
    pub fn wdts(self) -> self::Wdts {
        unsafe { self::Wdts(self.0.add(680usize)) }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accen00_SPEC;
impl crate::sealed::RegSpec for Accen00_SPEC {
    type DataType = u32;
}
#[doc = "Access Enable Register 00\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accen00 = crate::RegValueT<Accen00_SPEC>;

impl Accen00 {
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Accen00_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Accen00_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Accen00 {
    #[inline(always)]
    fn default() -> Accen00 {
        <crate::RegValueT<Accen00_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accen10_SPEC;
impl crate::sealed::RegSpec for Accen10_SPEC {
    type DataType = u32;
}
#[doc = "Access Enable Register 10\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accen10 = crate::RegValueT<Accen10_SPEC>;

impl Accen10 {
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Accen10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Accen10_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Accen10 {
    #[inline(always)]
    fn default() -> Accen10 {
        <crate::RegValueT<Accen10_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Arstdis_SPEC;
impl crate::sealed::RegSpec for Arstdis_SPEC {
    type DataType = u32;
}
#[doc = "Application Reset Disable Register\n resetvalue={PowerOn Reset:0x0}"]
pub type Arstdis = crate::RegValueT<Arstdis_SPEC>;

impl Arstdis {
    #[doc = "STM0 Disable Reset   STM0DIS. This bit field defines if an Application Reset leads to an reset for the        STM0."]
    #[inline(always)]
    pub fn stm0dis(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Arstdis_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Arstdis_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "STM1 Disable Reset   STM1DIS. This bit field defines if an Application Reset leads to an reset for the        STM1."]
    #[inline(always)]
    pub fn stm1dis(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Arstdis_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Arstdis_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "STM2 Disable Reset   STM2DIS. This bit field defines if an Application Reset leads to an reset for the        STM2."]
    #[inline(always)]
    pub fn stm2dis(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Arstdis_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Arstdis_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Arstdis {
    #[inline(always)]
    fn default() -> Arstdis {
        <crate::RegValueT<Arstdis_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccucon0_SPEC;
impl crate::sealed::RegSpec for Ccucon0_SPEC {
    type DataType = u32;
}
#[doc = "CCU Clock Control Register 0\n resetvalue={After SSW execution:0x5120112,System Reset:0x3130113}"]
pub type Ccucon0 = crate::RegValueT<Ccucon0_SPEC>;

impl Ccucon0 {
    #[doc = "STM Divider Reload Value   STMDIV. The resulting STM frequency is configured to f STM   xa0    xa0  f source0   xa0    xa0 STMDIV for the allowed configurations. For STMDIV  xa0    xa0 0000 B the clock is shut off. f source0 can be configured either to f PLL0  CLKSEL  xa0    xa0 01 B   or f BACK  CLKSEL  xa0    xa0 00 B"]
    #[inline(always)]
    pub fn stmdiv(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Ccucon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Ccucon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTM Divider Reload Value   GTMDIV. The resulting GTM frequency is configured to f GTM   f SOURCEGTM   GTMDIV for the allowed configurations. For GTMDIV   0000 B the clock is shut off."]
    #[inline(always)]
    pub fn gtmdiv(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Ccucon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Ccucon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SRI Divider Reload Value   SRIDIV. The resulting SRI frequency is configured to f SRI   f source0   SRIDIV for the allowed configurations. f source0 could be configured either to f PLL0  CLKSEL   01 B   or f BACK  CLKSEL   00 B"]
    #[inline(always)]
    pub fn sridiv(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Ccucon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Ccucon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Low Power Divider Reload Value   LPDIV. The selected divider is valid for all clocks derived from f XXX with XXX   SPB  SRI  BBB  FSI  GETH  GTM  ADAS."]
    #[inline(always)]
    pub fn lpdiv(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, Ccucon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8, Ccucon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SPB Divider Reload Value   SPBDIV. f source0 could be configured either to f PLL  CLKSEL   01 B   or f BACK  CLKSEL   00 B"]
    #[inline(always)]
    pub fn spbdiv(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Ccucon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Ccucon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "BBB Divider Reload Value   BBBDIV. The resulting BBB frequency is configured to f BBB   160    160  f source0   160    160 BBBDIV        for all allowed configurations. For BBBDIV  160    160 0000 B the clock is shut off. f source0 could be        configured either to f PLL0  CLKSEL  160    160 01 B   or f BACK  CLKSEL  160    160 00 B"]
    #[inline(always)]
    pub fn bbbdiv(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Ccucon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Ccucon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FSI Divider Reload Value   FSIDIV"]
    #[inline(always)]
    pub fn fsidiv(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, Ccucon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, Ccucon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FSI2 Divider Reload Value   FSI2DIV"]
    #[inline(always)]
    pub fn fsi2div(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, Ccucon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3,1,0,u8, Ccucon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Selection for Source   CLKSEL. This bit field defines the clock source that is used for the clock        generation of f sourcex ."]
    #[inline(always)]
    pub fn clksel(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, Ccucon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, Ccucon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Update Request   UP. Setting this bit will request an update for CCUCON0 and CCUCON5. Only one UP bit must be set for either CCUCON0 or CCUCON5. This bit always reads as zero."]
    #[inline(always)]
    pub fn up(self) -> crate::common::RegisterFieldBool<30, 1, 0, Ccucon0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Ccucon0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if the register is locked and a write action from the bus side has no effect. The lock bit is set when an update of CCUCON0 5 has been requested  and released when the update is complete."]
    #[inline(always)]
    pub fn lck(self) -> crate::common::RegisterFieldBool<31, 1, 0, Ccucon0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ccucon0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ccucon0 {
    #[inline(always)]
    fn default() -> Ccucon0 {
        <crate::RegValueT<Ccucon0_SPEC> as RegisterValue<_>>::new(17957138)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccucon1_SPEC;
impl crate::sealed::RegSpec for Ccucon1_SPEC {
    type DataType = u32;
}
#[doc = "CCU Clock Control Register 1\n resetvalue={System Reset:0x10100302}"]
pub type Ccucon1 = crate::RegValueT<Ccucon1_SPEC>;

impl Ccucon1 {
    #[doc = "MCAN Divider Reload Value   MCANDIV. The resulting MCAN frequency is configured to f MCANI   160    160  f source1   160    160 MCANDIV        for the allowed configurations. For MCANDIV  160    160 0000 B the clock is shut off."]
    #[inline(always)]
    pub fn mcandiv(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Ccucon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Ccucon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Selection for MCAN   CLKSELMCAN. This bit field defines the clock source that is used for the clock generation of f SOURCEMCAN . For switching between two non zero configurations the following sequence has to be applied  First step is to switch to 00 B . Second step is to switch to the new target configuration."]
    #[inline(always)]
    pub fn clkselmcan(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Ccucon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Ccucon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Divider Disable for fPLL1   PLL1DIVDIS. Depending on CCUCON0.CLKSEL  this bit selects whether f source1 is half f pll1 ."]
    #[inline(always)]
    pub fn pll1divdis(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ccucon1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ccucon1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "I2C Divider Reload Value   I2CDIV. The resulting I2C frequency is configured to f I2C   xa0    xa0  f SOURCE2   xa0    xa0 I2CDIV for the allowed configurations. For I2CDIV  xa0    xa0 0000 B the clock is shut off."]
    #[inline(always)]
    pub fn i2cdiv(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Ccucon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Ccucon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MSC Divider Reload Value   MSCDIV. The resulting MSC frequency is configured to f MSC   160    160  f SOURCEMSC   160    160 MSCDIV        for the allowed configurations. For MSCDIV  160    160 0000 B the clock is shut off."]
    #[inline(always)]
    pub fn mscdiv(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Ccucon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Ccucon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Selection for MSC   CLKSELMSC. This bit field defines the clock source that is used for the clock generation of f SOURCEMSC . For switching between two non zero configurations the following sequence has to be applied  First step is to switch to 00 B . Second step is to switch to the new target configuration."]
    #[inline(always)]
    pub fn clkselmsc(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Ccucon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Ccucon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "QSPI Divider Reload Value   QSPIDIV. The resulting QSPI frequency is configured to f QSPI   160    160  f SOURCEQSPI   160    160 QSPIDIV        for the allowed configurations. For QSPIDIV  160    160 0000 B the clock is shut off."]
    #[inline(always)]
    pub fn qspidiv(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Ccucon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Ccucon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Selection for QSPI   CLKSELQSPI. This bit field defines the clock source that is used for the clock generation of f SOURCEQSPI . For switching between two non zero configurations the following sequence has to be applied  First step is to switch to 00 B . Second step is to switch to the new target configuration."]
    #[inline(always)]
    pub fn clkselqspi(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, Ccucon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, Ccucon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if the register is locked and a write action from the bus side has no effect. The lock bit is set when at least one bit field is changed  and released when this change is executed."]
    #[inline(always)]
    pub fn lck(self) -> crate::common::RegisterFieldBool<31, 1, 0, Ccucon1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ccucon1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ccucon1 {
    #[inline(always)]
    fn default() -> Ccucon1 {
        <crate::RegValueT<Ccucon1_SPEC> as RegisterValue<_>>::new(269484802)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccucon2_SPEC;
impl crate::sealed::RegSpec for Ccucon2_SPEC {
    type DataType = u32;
}
#[doc = "CCU Clock Control Register 2\n resetvalue={System Reset:0x7000101}"]
pub type Ccucon2 = crate::RegValueT<Ccucon2_SPEC>;

impl Ccucon2 {
    #[doc = "ASCLIN Fast Divider Reload Value   ASCLINFDIV. The resulting ASCLIN frequency is configured to f ASCLINF   f source2   ASCLINFDIV for the allowed configurations. For ASCLINFDIV   0000 B the clock is shut off. f source2 could be configured either to f PLL2  CLKSEL   01 B   or f BACK  CLKSEL   00 B"]
    #[inline(always)]
    pub fn asclinfdiv(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Ccucon2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Ccucon2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ASCLIN Slow Divider Reload Value   ASCLINSDIV. The resulting ASCLIN frequency is configured to f ASCLINSI   f source1   ASCLINSDIV for the allowed configurations. For ASCLINSDIV   0000 B the clock is shut off. f source1 could be configured either to f PLL1  CLKSEL   01 B   or f BACK  CLKSEL   00 B"]
    #[inline(always)]
    pub fn asclinsdiv(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Ccucon2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Ccucon2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Selection for ASCLINS   CLKSELASCLINS. This bit field defines the clock source that is used for the clock generation of f ASCLINS . For switching between two non zero configurations the following sequence has to be applied  First step is to switch to 00 B . Second step is to switch to the new target configuration."]
    #[inline(always)]
    pub fn clkselasclins(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Ccucon2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Ccucon2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Power Safe SwitchOff for ERAY Clock   ERAYPERON. This bit is used to control the ERAY peripheral clock f ERAY for power saving purposes if the logic is not used by the application."]
    #[inline(always)]
    pub fn erayperon(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Ccucon2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ccucon2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if the register is locked and a write action from the bus side has no effect. The lock bit is set when at least one bit field is changed  and released when this change is executed."]
    #[inline(always)]
    pub fn lck(self) -> crate::common::RegisterFieldBool<31, 1, 0, Ccucon2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ccucon2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ccucon2 {
    #[inline(always)]
    fn default() -> Ccucon2 {
        <crate::RegValueT<Ccucon2_SPEC> as RegisterValue<_>>::new(117440769)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccucon3_SPEC;
impl crate::sealed::RegSpec for Ccucon3_SPEC {
    type DataType = u32;
}
#[doc = "CCU Clock Control Register 3\n resetvalue={System Reset:0x0}"]
pub type Ccucon3 = crate::RegValueT<Ccucon3_SPEC>;

impl Ccucon3 {
    #[doc = "PLL0 Clock Monitor Enable   PLL0MONEN"]
    #[inline(always)]
    pub fn pll0monen(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Ccucon3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ccucon3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PLL1 Clock Monitor Enable   PLL1MONEN"]
    #[inline(always)]
    pub fn pll1monen(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Ccucon3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ccucon3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PLL2 Clock Monitor Enable   PLL2MONEN"]
    #[inline(always)]
    pub fn pll2monen(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Ccucon3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ccucon3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SPB Clock Monitor Enable   SPBMONEN"]
    #[inline(always)]
    pub fn spbmonen(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ccucon3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ccucon3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Backup Clock Monitor Enable   BACKMONEN"]
    #[inline(always)]
    pub fn backmonen(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ccucon3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ccucon3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PLL0 Clock Monitor Test   PLL0MONTST. The test enable bit is not a direct trigger for the alarm  but an inhibit for the clock to be monitored. This is to test the monitoring logic itself as well."]
    #[inline(always)]
    pub fn pll0montst(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Ccucon3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ccucon3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PLL1 Clock Monitor Test   PLL1MONTST. The test enable bit is not a direct trigger for the alarm  but an inhibit for the clock to be monitored. This is to test the monitoring logic itself as well."]
    #[inline(always)]
    pub fn pll1montst(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ccucon3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ccucon3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PLL2 Clock Monitor Test   PLL2MONTST. The test enable bit is not a direct trigger for the alarm  but an inhibit for the clock to be monitored. This is to test the monitoring logic itself as well."]
    #[inline(always)]
    pub fn pll2montst(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ccucon3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ccucon3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SPB Clock Monitor Test   SPBMONTST. The test enable bit is not a direct trigger for the alarm  but an inhibit for the clock to be monitored. This is to test the monitoring logic itself as well."]
    #[inline(always)]
    pub fn spbmontst(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ccucon3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ccucon3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Backup Clock Monitor Test   BACKMONTST. The test enable bit is not a direct trigger for the alarm  but an inhibit for the clock to be monitored. This is to test the monitoring logic itself as well."]
    #[inline(always)]
    pub fn backmontst(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Ccucon3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ccucon3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Update Request   UP. Setting this bit will request an update for CCUCON3 and CCUCON4. Only one UP bit must be set for either CCUCON3 or CCUCON4. This bit always reads as zero."]
    #[inline(always)]
    pub fn up(self) -> crate::common::RegisterFieldBool<30, 1, 0, Ccucon3_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Ccucon3_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if the register is locked and a write action from the bus side has no effect. The lock bit is set when an update of CCUCON3 4 has been requested  and released when the update is complete."]
    #[inline(always)]
    pub fn lck(self) -> crate::common::RegisterFieldBool<31, 1, 0, Ccucon3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ccucon3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ccucon3 {
    #[inline(always)]
    fn default() -> Ccucon3 {
        <crate::RegValueT<Ccucon3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccucon4_SPEC;
impl crate::sealed::RegSpec for Ccucon4_SPEC {
    type DataType = u32;
}
#[doc = "CCU Clock Control Register 4\n resetvalue={System Reset:0x0}"]
pub type Ccucon4 = crate::RegValueT<Ccucon4_SPEC>;

impl Ccucon4 {
    #[doc = "Backup Clock Monitor Lower Threshold   LOTHR. lower threshold   512  f PLL0   0.9   100 MHz For proper operation and to avoid false alarms  the monitor needs to be disabled via MONEN 0 before changing setting the threshold values."]
    #[inline(always)]
    pub fn lothr(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, Ccucon4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16, Ccucon4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Backup Clock Monitor Upper Threshold   UPTHR. upper threshold   512  f PLL0   1.1   100 MHz For proper operation and to avoid false alarms  the monitor needs to be disabled via MONEN 0 before changing setting the threshold values."]
    #[inline(always)]
    pub fn upthr(
        self,
    ) -> crate::common::RegisterField<12, 0xfff, 1, 0, u16, Ccucon4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xfff,1,0,u16, Ccucon4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Backup Clock Monitor Enable   MONEN"]
    #[inline(always)]
    pub fn monen(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Ccucon4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ccucon4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Backup Clock Monitor Test   MONTST. Set this bit to 1 to test alarm generation. The test enable bit is a direct trigger for the alarm."]
    #[inline(always)]
    pub fn montst(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Ccucon4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ccucon4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Update Request   UP. Setting this bit will request an update for CCUCON3 and CCUCON4. Only one UP bit must be set for either CCUCON3 or CCUCON4. This bit always reads as zero."]
    #[inline(always)]
    pub fn up(self) -> crate::common::RegisterFieldBool<30, 1, 0, Ccucon4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Ccucon4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if the register is locked and a write action from the bus side has no effect. The lock bit is set when an update of CCUCON3 4 has been requested  and released when the update is complete."]
    #[inline(always)]
    pub fn lck(self) -> crate::common::RegisterFieldBool<31, 1, 0, Ccucon4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ccucon4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ccucon4 {
    #[inline(always)]
    fn default() -> Ccucon4 {
        <crate::RegValueT<Ccucon4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccucon5_SPEC;
impl crate::sealed::RegSpec for Ccucon5_SPEC {
    type DataType = u32;
}
#[doc = "CCU Clock Control Register 5\n resetvalue={System Reset:0x30}"]
pub type Ccucon5 = crate::RegValueT<Ccucon5_SPEC>;

impl Ccucon5 {
    #[doc = "GETH Divider Reload Value   GETHDIV. The resulting GETH frequency is configured to f GETH   xa0    xa0  f source0   xa0    xa0 GETHDIV for the allowed configurations. For GETHDIV  xa0    xa0 0000 B the clock is shut off. f source0 could be configured either to f PLL0  CLKSEL  xa0    xa0 01 B   or f BACK  CLKSEL  xa0    xa0 00 B   GETHDIV must be enabled    0  during an application reset to allow firmware related installation tasks."]
    #[inline(always)]
    pub fn gethdiv(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Ccucon5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Ccucon5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MCANH Divider Reload Value   MCANHDIV. The resulting MCANH frequency is configured to f MCANH   f SOURCE0   MCANHDIV for the allowed configurations. For MCANHDIV   0000 B the clock is shut off."]
    #[inline(always)]
    pub fn mcanhdiv(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Ccucon5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Ccucon5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Update Request   UP. Setting this bit will request an update for CCUCON0 and CCUCON5. Only one UP bit must be set either CCUCON0 or CCUCON5. This bit always reads as zero."]
    #[inline(always)]
    pub fn up(self) -> crate::common::RegisterFieldBool<30, 1, 0, Ccucon5_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Ccucon5_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if the register is locked and a write action from the bus side has no effect. The lock bit is set when an update of CCUCON0 5 has been requested  and released when the update is complete."]
    #[inline(always)]
    pub fn lck(self) -> crate::common::RegisterFieldBool<31, 1, 0, Ccucon5_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ccucon5_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ccucon5 {
    #[inline(always)]
    fn default() -> Ccucon5 {
        <crate::RegValueT<Ccucon5_SPEC> as RegisterValue<_>>::new(48)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccucon6_SPEC;
impl crate::sealed::RegSpec for Ccucon6_SPEC {
    type DataType = u32;
}
#[doc = "CCU Clock Control Register 6\n resetvalue={System Reset:0x0}"]
pub type Ccucon6 = crate::RegValueT<Ccucon6_SPEC>;

impl Ccucon6 {
    #[doc = "CPU0 Divider Reload Value   CPU0DIV. The resulting CPU0 frequency  performance  is configured to f CPU0   xa0    xa0  f SRI   xa0    xa0  64  xa0    xa0 CPU0DIV   xa0    xa0 64. For CPU0DIV  xa0    xa0 000000 B   f CPU0   xa0    xa0  f SRI ."]
    #[inline(always)]
    pub fn cpu0div(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Ccucon6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Ccucon6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ccucon6 {
    #[inline(always)]
    fn default() -> Ccucon6 {
        <crate::RegValueT<Ccucon6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccucon7_SPEC;
impl crate::sealed::RegSpec for Ccucon7_SPEC {
    type DataType = u32;
}
#[doc = "CCU Clock Control Register 7\n resetvalue={System Reset:0x0}"]
pub type Ccucon7 = crate::RegValueT<Ccucon7_SPEC>;

impl Ccucon7 {
    #[doc = "CPU1 Divider Reload Value   CPU1DIV. The resulting CPU1 frequency  performance  is configured to f CPU1   xa0    xa0  f SRI   xa0    xa0  64  xa0    xa0 CPU1DIV   xa0    xa0 64. For CPU1DIV  xa0    xa0 000000 B   f CPU1   xa0    xa0  f SRI ."]
    #[inline(always)]
    pub fn cpu1div(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Ccucon7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Ccucon7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ccucon7 {
    #[inline(always)]
    fn default() -> Ccucon7 {
        <crate::RegValueT<Ccucon7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccucon8_SPEC;
impl crate::sealed::RegSpec for Ccucon8_SPEC {
    type DataType = u32;
}
#[doc = "CCU Clock Control Register 8\n resetvalue={System Reset:0x0}"]
pub type Ccucon8 = crate::RegValueT<Ccucon8_SPEC>;

impl Ccucon8 {
    #[doc = "CPU2 Divider Reload Value   CPU2DIV. The resulting CPU2 frequency  performance  is configured to f CPU2   xa0    xa0  f SRI   xa0    xa0  64  xa0    xa0 CPU2DIV   xa0    xa0 64. For CPU2DIV  xa0    xa0 000000 B   f CPU2   xa0    xa0  f SRI ."]
    #[inline(always)]
    pub fn cpu2div(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Ccucon8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Ccucon8_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ccucon8 {
    #[inline(always)]
    fn default() -> Ccucon8 {
        <crate::RegValueT<Ccucon8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chipid_SPEC;
impl crate::sealed::RegSpec for Chipid_SPEC {
    type DataType = u32;
}
#[doc = "Chip Identification Register\n resetvalue={System Reset:0x0,CFS Value:0x0}"]
pub type Chipid = crate::RegValueT<Chipid_SPEC>;

impl Chipid {
    #[doc = "Chip Revision Number   CHREV. This bit field indicates the revision number of the AurixPlus        Platform device. The value of this bit field is defined in the        product Data Sheet. Bits  3 0  are connected to the layer prog inputs  3 0 . Bits         3 0  are used to indicate the steps. These updates can be done with any        metal fix or FW ROM change. Bits  5 4  are hard wired on top level. Bits  5 4  define the        major silicon design steps  A  B  C  D  ... . These bits can be changed        only with a major redesign."]
    #[inline(always)]
    pub fn chrev(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Chipid_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Chipid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Chip Family   CHTEC. These bits indicate the product family and are changed only with a redesign. Hard wired on top level."]
    #[inline(always)]
    pub fn chtec(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Chipid_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Chipid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Chip Package   CHPK. These bits indicate the package. For further details refer to the Product Datasheet Updated by SSW from config sector. Use future variant codes for downconfigured silicon"]
    #[inline(always)]
    pub fn chpk(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Chipid_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Chipid_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Chip Product   CHID. These bits indicate the base product. For further details refer to the Product Datasheet Updated by SSW from config sector."]
    #[inline(always)]
    pub fn chid(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Chipid_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Chipid_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emulation or ADAS Extension Available   EEA. Indicates if the emulation or ADAS extension hardware is available or not."]
    #[inline(always)]
    pub fn eea(self) -> crate::common::RegisterFieldBool<16, 1, 0, Chipid_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Chipid_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "µCode Version   UCODE. This bit field displays the Version X.Y of the flash µCode."]
    #[inline(always)]
    pub fn ucode(
        self,
    ) -> crate::common::RegisterField<17, 0x7f, 1, 0, u8, Chipid_SPEC, crate::common::RW> {
        crate::common::RegisterField::<17,0x7f,1,0,u8, Chipid_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Program Flash Size   FSIZE. This bit field indicates available program flash size for this device. For more details see Product Datasheet."]
    #[inline(always)]
    pub fn fsize(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Chipid_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Chipid_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Variant   VART. This bit field is used for variant identification. It is used to identify product variants with non standard temperature profile  max frequency  package pitch or customer feature sets. Note that variants do not include HSM availability  Flash Size or Emulation availability options which are handled by the SEC  FSIZE and EEA fields respectively. For coding details see Product Datasheet"]
    #[inline(always)]
    pub fn vart(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, Chipid_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x7,1,0,u8, Chipid_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Security Device Available   SEC. This bit field is updated by SSW from config sector. This bit field indicates whether the product has a Hardware Security Module"]
    #[inline(always)]
    pub fn sec(self) -> crate::common::RegisterFieldBool<31, 1, 0, Chipid_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Chipid_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Chipid {
    #[inline(always)]
    fn default() -> Chipid {
        <crate::RegValueT<Chipid_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtsclim_SPEC;
impl crate::sealed::RegSpec for Dtsclim_SPEC {
    type DataType = u32;
}
#[doc = "Core Die Temperature Sensor Limit Register\n resetvalue={Application Reset:0x0CD806D6}"]
pub type Dtsclim = crate::RegValueT<Dtsclim_SPEC>;

impl Dtsclim {
    #[doc = "DTSC Lower Limit   LOWER. This bit field defines the lower limit of the DTSC temperature check.        The DTSC measurement result is compared against this value and if the        measurement result is less than or equal to the configured LOWER        bitfield value  flag LLU is set."]
    #[inline(always)]
    pub fn lower(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, Dtsclim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16, Dtsclim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DTSC Bandgap OK. This bitfield indicates that the bandgap reference for the Core Die        Temperature Sensor  DTSC  is available and ok."]
    #[inline(always)]
    pub fn bgpok(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Dtsclim_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Dtsclim_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "DTSC Enable. This bitfield enables the Core Die Temperature Sensor  DTSC . The        bitfield is reset on an application reset."]
    #[inline(always)]
    pub fn en(self) -> crate::common::RegisterFieldBool<14, 1, 0, Dtsclim_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Dtsclim_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "DTSC Lower Limit Underflow   LLU. When this bit is set the related SMU DTSC alarm trigger is generated.        This bit has to be written with zero in order to clear it. Writing a one        has no effect. This bit is set when a DTSC measurement is finished and        the result is below the lower limit  i.e. DTSCLIM.LOWER ."]
    #[inline(always)]
    pub fn llu(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Dtsclim_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Dtsclim_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "DTSC Upper Limit   UPPER. This bit field defines the upper limit of the DTSC temperature check.        The DTSC measurement result is compared against this value and if the        measurement result is greater than or equal to the configured UPPER        bitfield value  flag UOF is set."]
    #[inline(always)]
    pub fn upper(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, Dtsclim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xfff,1,0,u16, Dtsclim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DTSC Interrupt Enable. This bitfield enables the Core Die Temperature Sensor  DTSC  interrupt.        The bitfield is reset on an application reset."]
    #[inline(always)]
    pub fn inten(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Dtsclim_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Dtsclim_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "DTSC Interrupt status flag. This bit is set when SMU DTSC interrupt is generated when a DTSC        measurement is finished. This bit is cleared by writing a zero. Writing        a one has no effect."]
    #[inline(always)]
    pub fn int(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Dtsclim_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Dtsclim_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "DTSC Upper Limit Overflow   UOF. When this bit is set  the related SMU DTSC alarm trigger is generated.        This bit has to be written with zero in order to clear it. Writing a one        has no effect. This bit is set when a DTSC measurement is finished and        the result is exceeding the upper limit  i.e. DTSCLIM.UPPER ."]
    #[inline(always)]
    pub fn uof(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Dtsclim_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Dtsclim_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Dtsclim {
    #[inline(always)]
    fn default() -> Dtsclim {
        <crate::RegValueT<Dtsclim_SPEC> as RegisterValue<_>>::new(215484118)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtscstat_SPEC;
impl crate::sealed::RegSpec for Dtscstat_SPEC {
    type DataType = u32;
}
#[doc = "Core Die Temperature Sensor Status Register\n resetvalue={Application Reset:0x0}"]
pub type Dtscstat = crate::RegValueT<Dtscstat_SPEC>;

impl Dtscstat {
    #[doc = "Result of the DTSC Measurement   RESULT. This bit field shows the result of the DTSC measurement. The value given        is directly related to the die temperature and can be evaluated using        the following formula. T    176 C     RESULT   Gnom    273.15 T   176 K   RESULT  G nom RESULT  G nom    T    176 C    273.15    G nom   T    176 K  G nom   7.505"]
    #[inline(always)]
    pub fn result(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, Dtscstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xfff,1,0,u16, Dtscstat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Dtscstat {
    #[inline(always)]
    fn default() -> Dtscstat {
        <crate::RegValueT<Dtscstat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eicon0_SPEC;
impl crate::sealed::RegSpec for Eicon0_SPEC {
    type DataType = u32;
}
#[doc = "ENDINIT Global Control Register 0\n resetvalue={Application Reset:0x0FFFC000E}"]
pub type Eicon0 = crate::RegValueT<Eicon0_SPEC>;

impl Eicon0 {
    #[doc = "End of Initialization Control Bit   ENDINIT. The current value of ENDINIT is controlled by hardware. It is cleared        after a valid EndInit Password Access to EICON0  and it is automatically        set again after a valid EndInit Modify Access to EICON0. During a write        to EICON0  the value written to this bit is only used for the        password protection mechanism and is not stored. This bit must be cleared during a Password Access to EICON0  and set        during a Modify Access to EICON0."]
    #[inline(always)]
    pub fn endinit(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Eicon0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Eicon0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "User Definable ENDINIT Password Field   EPW. This bit field is written with an ENDINIT password value during a Modify        Access. This password is independent from the CPU WDT passwords. A read from this bitfield returns this password  but bits  7 2  are        inverted  toggled  to ensure that a simple read write is not sufficient        to service the WDT. This bit field must be written with its current contents during a        Password Access. The default ENDINIT password after Application Reset is 00000000111100 B"]
    #[inline(always)]
    pub fn epw(
        self,
    ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, Eicon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3fff,1,0,u16, Eicon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reload Value for the ENDINIT Timeout Counter   REL. The reload value for the ENDINIT Timeout Counter is fixed. This bitfield        always reads as FFFCh and cannot be changed. This bit field must be written with its current contents during a        Password Access. During a Modify Access this bitfield may contain any        value and is ignored."]
    #[inline(always)]
    pub fn rel(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Eicon0_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Eicon0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Eicon0 {
    #[inline(always)]
    fn default() -> Eicon0 {
        <crate::RegValueT<Eicon0_SPEC> as RegisterValue<_>>::new(4294705166)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eicon1_SPEC;
impl crate::sealed::RegSpec for Eicon1_SPEC {
    type DataType = u32;
}
#[doc = "ENDINIT Global Control Register 1\n resetvalue={Application Reset:0x0}"]
pub type Eicon1 = crate::RegValueT<Eicon1_SPEC>;

impl Eicon1 {
    #[doc = "Input Frequency Request Control   IR1 IR0. Bit IR0 and IR1 should be programmed together to determine the ENDINIT        Timeout Counter frequency. These bits can only be modified if system ENDINIT  E  is de asserted.        EISR.IS0 and EISR.IS1 are updated by these bits only when system ENDINIT         E  is re asserted. As long as system ENDINIT  E  is de asserted         EISR.IS0 and EISR.IS1 control the current input frequency of the ENDINIT        Timeout Timer. When System ENDINIT  E  is re asserted  EISR.IS0 and        EISR.IS1 are updated with the new values of IR0 and IR1."]
    #[inline(always)]
    pub fn ir0(self) -> crate::common::RegisterFieldBool<2, 1, 0, Eicon1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Eicon1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable Request Control Bit   DR. This bit can only be modified if the system ENDINIT  E  is de asserted.        EISR.DS is updated when system ENDINIT  E  is re asserted. As long as        system ENDINIT E  is cleared  bit EISR.DS controls the current        enable disable status of the ENDINIT Timeout Counter. When system        ENDINIT  E  is re asserted  EISR.DS is updated with the state of DR."]
    #[inline(always)]
    pub fn dr(self) -> crate::common::RegisterFieldBool<3, 1, 0, Eicon1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Eicon1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Frequency Request Control   IR1 IR0. Bit IR0 and IR1 should be programmed together to determine the ENDINIT        Timeout Counter frequency. These bits can only be modified if system ENDINIT  E  is de asserted.        EISR.IS0 and EISR.IS1 are updated by these bits only when system ENDINIT         E  is re asserted. As long as system ENDINIT  E  is de asserted         EISR.IS0 and EISR.IS1 control the current input frequency of the ENDINIT        Timeout Timer. When System ENDINIT  E  is re asserted  EISR.IS0 and        EISR.IS1 are updated with the new values of IR0 and IR1."]
    #[inline(always)]
    pub fn ir1(self) -> crate::common::RegisterFieldBool<5, 1, 0, Eicon1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Eicon1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Eicon1 {
    #[inline(always)]
    fn default() -> Eicon1 {
        <crate::RegValueT<Eicon1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EicRi_SPEC;
impl crate::sealed::RegSpec for EicRi_SPEC {
    type DataType = u32;
}
#[doc = "External Input Channel Register 0\n resetvalue={Application Reset:0x0}"]
pub type EicRi = crate::RegValueT<EicRi_SPEC>;

impl EicRi {
    #[doc = "External Input Selection 0   EXIS0. This bit field determines which input line is selected for Input Channel  2i ."]
    #[inline(always)]
    pub fn exis0(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, EicRi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8, EicRi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Falling Edge Enable 0   FEN0. This bit determines if the falling edge of Input Channel  2i  is used to set bit INTF 2i ."]
    #[inline(always)]
    pub fn fen0(self) -> crate::common::RegisterFieldBool<8, 1, 0, EicRi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, EicRi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Rising Edge Enable 0   REN0. This bit determines if the rising edge of Input Channel  2 i  is used to set bit INTF 2i ."]
    #[inline(always)]
    pub fn ren0(self) -> crate::common::RegisterFieldBool<9, 1, 0, EicRi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, EicRi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Level Detection Enable 0   LDEN0. This bit determines if bit INTF 2i  is cleared automatically if an edge of the input Input Channel  2i  is detected  which has not been selected  rising edge with REN0   0 or falling edge with FEN0   0 ."]
    #[inline(always)]
    pub fn lden0(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, EicRi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, EicRi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "External Input Enable 0   EIEN0. This bit enables the generation of a trigger event for request channel  2i   e.g. for interrupt generation  when a selected edge is detected."]
    #[inline(always)]
    pub fn eien0(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, EicRi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, EicRi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Node Pointer   INP0. This bit field determines the destination  output channel  for trigger event  2i   if enabled by EIEN 2i  ."]
    #[inline(always)]
    pub fn inp0(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, EicRi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8, EicRi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Input Selection 1   EXIS1. This bit field determines which input line is selected for Input Channel  2i 1 ."]
    #[inline(always)]
    pub fn exis1(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, EicRi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x7,1,0,u8, EicRi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Falling Edge Enable 1   FEN1. This bit determines if the falling edge of Input Channel  2i 1  is used to set bit INTF 2i 1 ."]
    #[inline(always)]
    pub fn fen1(self) -> crate::common::RegisterFieldBool<24, 1, 0, EicRi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, EicRi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Rising Edge Enable 1   REN1. This bit determines if the rising edge of Input Channel  2i 1  is used to set bit INTF 2i 1 ."]
    #[inline(always)]
    pub fn ren1(self) -> crate::common::RegisterFieldBool<25, 1, 0, EicRi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, EicRi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Level Detection Enable 1   LDEN1. This bit determines if bit INTF 2i 1  is cleared automatically if an edge of the input Input Channel  2i 1  is detected  which has not been selected  rising edge with REN1   0 or falling edge with FEN1   0 ."]
    #[inline(always)]
    pub fn lden1(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, EicRi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, EicRi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "External Input Enable 1   EIEN1. This bit enables the generation of a trigger event for request channel  2i 1   e.g. for interrupt generation  when a selected edge is detected."]
    #[inline(always)]
    pub fn eien1(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, EicRi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, EicRi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Node Pointer   INP1. This bit field determines the destination  output channel  for trigger event  2i 1   if enabled by EIEN 2i 1  ."]
    #[inline(always)]
    pub fn inp1(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, EicRi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x7,1,0,u8, EicRi_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for EicRi {
    #[inline(always)]
    fn default() -> EicRi {
        <crate::RegValueT<EicRi_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eifilt_SPEC;
impl crate::sealed::RegSpec for Eifilt_SPEC {
    type DataType = u32;
}
#[doc = "External Input Filter Register\n resetvalue={Application Reset:0x0}"]
pub type Eifilt = crate::RegValueT<Eifilt_SPEC>;

impl Eifilt {
    #[doc = "Filter Enable for REQ0A   FILRQ0A"]
    #[inline(always)]
    pub fn filrq0a(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Eifilt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Eifilt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Filter Enable for REQ5A   FILRQ5A"]
    #[inline(always)]
    pub fn filrq5a(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Eifilt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Eifilt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Filter Enable for REQ2A   FILRQ2A"]
    #[inline(always)]
    pub fn filrq2a(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Eifilt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Eifilt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Filter Enable for REQ3A   FILRQ3A"]
    #[inline(always)]
    pub fn filrq3a(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Eifilt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Eifilt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Filter Enable for REQ0C   FILRQ0C"]
    #[inline(always)]
    pub fn filrq0c(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Eifilt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Eifilt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Filter Enable for REQ1C   FILRQ1C"]
    #[inline(always)]
    pub fn filrq1c(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Eifilt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Eifilt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Filter Enable for REQ3C   FILRQ3C"]
    #[inline(always)]
    pub fn filrq3c(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Eifilt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Eifilt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Filter Enable for REQ2C   FILRQ2C"]
    #[inline(always)]
    pub fn filrq2c(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Eifilt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Eifilt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Filter Enable for REQ4A   FILRQ4A"]
    #[inline(always)]
    pub fn filrq4a(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Eifilt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Eifilt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Filter Enable for REQ6A   FILRQ6A"]
    #[inline(always)]
    pub fn filrq6a(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Eifilt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Eifilt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Filter Enable for REQ1A   FILRQ1A"]
    #[inline(always)]
    pub fn filrq1a(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Eifilt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Eifilt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Filter Enable for REQ7A   FILRQ7A"]
    #[inline(always)]
    pub fn filrq7a(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Eifilt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Eifilt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Filter Enable for REQ6D   FILRQ6D"]
    #[inline(always)]
    pub fn filrq6d(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Eifilt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Eifilt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Filter Enable for REQ4D   FILRQ4D"]
    #[inline(always)]
    pub fn filrq4d(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Eifilt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Eifilt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Filter Enable for REQ2B   FILRQ2B"]
    #[inline(always)]
    pub fn filrq2b(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Eifilt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Eifilt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Filter Enable for REQ3B   FILRQ3B"]
    #[inline(always)]
    pub fn filrq3b(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Eifilt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Eifilt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Filter Enable for REQ7C   FILRQ7C"]
    #[inline(always)]
    pub fn filrq7c(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Eifilt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Eifilt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Digital Glitch Filter Clock Predivider   FILTDIV. This field controls a predivider to generate the digital filter sample clock T filt   T spb   FILTDIV A value of zero in this register disables all glitch filtering."]
    #[inline(always)]
    pub fn filtdiv(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Eifilt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Eifilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Digital Glitch Filter Depth   DEPTH. DEPTH determines the number of port input samples considered in the calculation of the floating average digital filter output for all enabled FLRQ filters. A value of zero in this register disables all glitch filtering."]
    #[inline(always)]
    pub fn depth(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Eifilt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Eifilt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Eifilt {
    #[inline(always)]
    fn default() -> Eifilt {
        <crate::RegValueT<Eifilt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eifr_SPEC;
impl crate::sealed::RegSpec for Eifr_SPEC {
    type DataType = u32;
}
#[doc = "External Input Flag Register\n resetvalue={Application Reset:0x0}"]
pub type Eifr = crate::RegValueT<Eifr_SPEC>;

impl Eifr {
    #[doc = "External Event Flag of Channel 7   INTF7. This bit monitors the status flag of the event trigger condition for the input channel x. This bit is automatically cleared when the selected condition  see RENx  FENx  is no longer met  if LDENx   1  or remains set until it is cleared by software  if LDENx   0 ."]
    #[inline(always)]
    pub fn intf0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Eifr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Eifr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "External Event Flag of Channel 7   INTF7. This bit monitors the status flag of the event trigger condition for the input channel x. This bit is automatically cleared when the selected condition  see RENx  FENx  is no longer met  if LDENx   1  or remains set until it is cleared by software  if LDENx   0 ."]
    #[inline(always)]
    pub fn intf1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Eifr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Eifr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "External Event Flag of Channel 7   INTF7. This bit monitors the status flag of the event trigger condition for the input channel x. This bit is automatically cleared when the selected condition  see RENx  FENx  is no longer met  if LDENx   1  or remains set until it is cleared by software  if LDENx   0 ."]
    #[inline(always)]
    pub fn intf2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Eifr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Eifr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "External Event Flag of Channel 7   INTF7. This bit monitors the status flag of the event trigger condition for the input channel x. This bit is automatically cleared when the selected condition  see RENx  FENx  is no longer met  if LDENx   1  or remains set until it is cleared by software  if LDENx   0 ."]
    #[inline(always)]
    pub fn intf3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Eifr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Eifr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "External Event Flag of Channel 7   INTF7. This bit monitors the status flag of the event trigger condition for the input channel x. This bit is automatically cleared when the selected condition  see RENx  FENx  is no longer met  if LDENx   1  or remains set until it is cleared by software  if LDENx   0 ."]
    #[inline(always)]
    pub fn intf4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Eifr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Eifr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "External Event Flag of Channel 7   INTF7. This bit monitors the status flag of the event trigger condition for the input channel x. This bit is automatically cleared when the selected condition  see RENx  FENx  is no longer met  if LDENx   1  or remains set until it is cleared by software  if LDENx   0 ."]
    #[inline(always)]
    pub fn intf5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Eifr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Eifr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "External Event Flag of Channel 7   INTF7. This bit monitors the status flag of the event trigger condition for the input channel x. This bit is automatically cleared when the selected condition  see RENx  FENx  is no longer met  if LDENx   1  or remains set until it is cleared by software  if LDENx   0 ."]
    #[inline(always)]
    pub fn intf6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Eifr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Eifr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "External Event Flag of Channel 7   INTF7. This bit monitors the status flag of the event trigger condition for the input channel x. This bit is automatically cleared when the selected condition  see RENx  FENx  is no longer met  if LDENx   1  or remains set until it is cleared by software  if LDENx   0 ."]
    #[inline(always)]
    pub fn intf7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Eifr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Eifr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Eifr {
    #[inline(always)]
    fn default() -> Eifr {
        <crate::RegValueT<Eifr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eisr_SPEC;
impl crate::sealed::RegSpec for Eisr_SPEC {
    type DataType = u32;
}
#[doc = "ENDINIT Timeout Counter Status Register\n resetvalue={Application Reset:0x0FFFC0000}"]
pub type Eisr = crate::RegValueT<Eisr_SPEC>;

impl Eisr {
    #[doc = "EICON0 Access Error Status Flag   AE. This bit is set when an illegal Password Access or Modify Access to        register EICON0 was attempted. This bit is only cleared on a valid        EICON0.ENDINIT Modify Access"]
    #[inline(always)]
    pub fn ae(self) -> crate::common::RegisterFieldBool<0, 1, 0, Eisr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Eisr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "EI Timeout Overflow Error Status Flag   OE. This bit is set when EISR.TIM overflows from FFFF H to FFFC H . This bit is only cleared on        a valid EICON0 Modify Access."]
    #[inline(always)]
    pub fn oe(self) -> crate::common::RegisterFieldBool<1, 1, 0, Eisr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Eisr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "EI Timeout Input Clock Status   IS1 IS0. Bit IS0 and IS1 should be programmed together. These bits indicate the        current ENDINIT Timeout Counter frequency."]
    #[inline(always)]
    pub fn is0(self) -> crate::common::RegisterFieldBool<2, 1, 0, Eisr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Eisr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "EI Timeout Enable Disable Status Flag   DS"]
    #[inline(always)]
    pub fn ds(self) -> crate::common::RegisterFieldBool<3, 1, 0, Eisr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Eisr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "EI Time Out Mode Flag   TO"]
    #[inline(always)]
    pub fn to(self) -> crate::common::RegisterFieldBool<4, 1, 0, Eisr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Eisr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "EI Timeout Input Clock Status   IS1 IS0. Bit IS0 and IS1 should be programmed together. These bits indicate the        current ENDINIT Timeout Counter frequency."]
    #[inline(always)]
    pub fn is1(self) -> crate::common::RegisterFieldBool<5, 1, 0, Eisr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Eisr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer Value   TIM. Reflects the current content of the ENDINIT Timeout Counter.  Only        bits 17 and 16 are implemented in EISR. Others return   8216 1  8217"]
    #[inline(always)]
    pub fn tim(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Eisr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Eisr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Eisr {
    #[inline(always)]
    fn default() -> Eisr {
        <crate::RegValueT<Eisr_SPEC> as RegisterValue<_>>::new(4294705152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emsr_SPEC;
impl crate::sealed::RegSpec for Emsr_SPEC {
    type DataType = u32;
}
#[doc = "Emergency Stop Register\n resetvalue={Application Reset:0x1}"]
pub type Emsr = crate::RegValueT<Emsr_SPEC>;

impl Emsr {
    #[doc = "Input Polarity   POL. This bit determines the polarity of the configured Emergency Stop input."]
    #[inline(always)]
    pub fn pol(self) -> crate::common::RegisterFieldBool<0, 1, 0, Emsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Emsr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Mode Selection   MODE. This bit determines the operating mode of the emergency stop signal."]
    #[inline(always)]
    pub fn mode(self) -> crate::common::RegisterFieldBool<1, 1, 0, Emsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Emsr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable ON   ENON. This bit enables the setting of flag EMSF by an inactive to active level transition of input signal."]
    #[inline(always)]
    pub fn enon(self) -> crate::common::RegisterFieldBool<2, 1, 0, Emsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Emsr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PORT Select   PSEL. This bit selects which one of the two Emergency Stop port options is monitored."]
    #[inline(always)]
    pub fn psel(self) -> crate::common::RegisterFieldBool<3, 1, 0, Emsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Emsr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Flag   EMSF. This bit indicates that a synchronous mode port triggered emergency stop        condition has occurred."]
    #[inline(always)]
    pub fn emsf(self) -> crate::common::RegisterFieldBool<16, 1, 0, Emsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Emsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SMU Emergency Stop Flag   SEMSF. This bit indicates that an SMU Safety Alarm triggered emergency stop        condition has occurred."]
    #[inline(always)]
    pub fn semsf(self) -> crate::common::RegisterFieldBool<17, 1, 0, Emsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Emsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Emsr {
    #[inline(always)]
    fn default() -> Emsr {
        <crate::RegValueT<Emsr_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emssw_SPEC;
impl crate::sealed::RegSpec for Emssw_SPEC {
    type DataType = u32;
}
#[doc = "Emergency Stop Software set and clear register\n resetvalue={Application Reset:0x0}"]
pub type Emssw = crate::RegValueT<Emssw_SPEC>;

impl Emssw {
    #[doc = "Emergency Stop Flag Modification   EMSFM. This bit field sets or clears flag EMSF via software. In case of a simultaneous hardware and software modification request         the hardware operation will be executed. EMSFM is always read as 00 B ."]
    #[inline(always)]
    pub fn emsfm(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, Emssw_SPEC, crate::common::W> {
        crate::common::RegisterField::<24,0x3,1,0,u8, Emssw_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "SMU Emergency Stop Flag Modification   SEMSFM. This bit field sets or clears flag SEMSF via software. In case of a simultaneous hardware and software modification request         the hardware operation will be executed. SEMSFM is always read as 00 B ."]
    #[inline(always)]
    pub fn semsfm(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, Emssw_SPEC, crate::common::W> {
        crate::common::RegisterField::<26,0x3,1,0,u8, Emssw_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Emssw {
    #[inline(always)]
    fn default() -> Emssw {
        <crate::RegValueT<Emssw_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Esrocfg_SPEC;
impl crate::sealed::RegSpec for Esrocfg_SPEC {
    type DataType = u32;
}
#[doc = "ESR Output Configuration Register\n resetvalue={System Reset:0x0}"]
pub type Esrocfg = crate::RegValueT<Esrocfg_SPEC>;

impl Esrocfg {
    #[doc = "Application Reset Indicator   ARI. This bit is set when an Application Reset request trigger occurs and        cleared by writing to ARC. When the ARI bit is set and an ESR pin is configured as a reset output         the corresponding ESR input will not re trigger a reset. This prevents        feedback of the reset indication causing a new reset request. Extension of the reset by an external ESR source is handled by SSW. Observed reset value after boot will depend upon ARI mode."]
    #[inline(always)]
    pub fn ari(self) -> crate::common::RegisterFieldBool<0, 1, 0, Esrocfg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Esrocfg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Application Reset Indicator Clear   ARC. Read as 0"]
    #[inline(always)]
    pub fn arc(self) -> crate::common::RegisterFieldBool<1, 1, 0, Esrocfg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Esrocfg_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Esrocfg {
    #[inline(always)]
    fn default() -> Esrocfg {
        <crate::RegValueT<Esrocfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extcon_SPEC;
impl crate::sealed::RegSpec for Extcon_SPEC {
    type DataType = u32;
}
#[doc = "External Clock Control Register\n resetvalue={System Reset:0x0}"]
pub type Extcon = crate::RegValueT<Extcon_SPEC>;

impl Extcon {
    #[doc = "External Clock Enable for EXTCLK0   EN0. If the generation of the external clock signal is disabled  the signal is tied to zero. This bit field can be overruled in Test Mode via the TCU."]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Extcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Extcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "External Clock Select for EXTCLK0   SEL0. This bit field defines the clock source that is selected as output for        pin EXTCLK0. This bit field can be overruled in Test Mode via the TCU."]
    #[inline(always)]
    pub fn sel0(
        self,
    ) -> crate::common::RegisterField<2, 0xf, 1, 0, u8, Extcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0xf,1,0,u8, Extcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Clock Enable for EXTCLK1   EN1. If the generation of the external clock signal is disabled  the signal is tied to zero. This bit can be overruled in Test Mode via the TCU."]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<16, 1, 0, Extcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Extcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Negation Selection   NSEL. This bit can be overruled in Test Mode via the TCU."]
    #[inline(always)]
    pub fn nsel(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Extcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Extcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "External Clock Select for EXTCLK1   SEL1. This bit field defines the clock source that is selected as the output for pin EXTCLK1. This bit field can be overruled in Test Mode via the TCU."]
    #[inline(always)]
    pub fn sel1(
        self,
    ) -> crate::common::RegisterField<18, 0xf, 1, 0, u8, Extcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0xf,1,0,u8, Extcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Clock Divider for EXTCLK1   DIV1. This value defines the reload value of the divider that generates f OUT out of f SPB   f OUT   f SPB   DIV1 1  . The divider itself is cleared each time bit EN1 is cleared."]
    #[inline(always)]
    pub fn div1(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Extcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Extcon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Extcon {
    #[inline(always)]
    fn default() -> Extcon {
        <crate::RegValueT<Extcon_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdr_SPEC;
impl crate::sealed::RegSpec for Fdr_SPEC {
    type DataType = u32;
}
#[doc = "Fractional Divider Register\n resetvalue={System Reset:0x0}"]
pub type Fdr = crate::RegValueT<Fdr_SPEC>;

impl Fdr {
    #[doc = "Step Value   STEP. In Normal Divider Mode  STEP contains the reload value for RESULT. In Fractional Divider Mode  this bit field determines the 10 bit value that is added to RESULT with each input clock cycle."]
    #[inline(always)]
    pub fn step(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Fdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Fdr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Divider Mode   DM. These bit fields determine the functionality of the fractional divider block."]
    #[inline(always)]
    pub fn dm(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Fdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Fdr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Result Value   RESULT. In Normal Divider Mode  RESULT acts as reload counter  addition  1 . In Fractional Divider Mode  this bit field contains the result of the addition RESULT   STEP. If DM is written with 01 B or 10 B   RESULT is loaded with 3FF H ."]
    #[inline(always)]
    pub fn result(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Fdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Fdr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Disable Clock   DISCLK"]
    #[inline(always)]
    pub fn disclk(self) -> crate::common::RegisterFieldBool<31, 1, 0, Fdr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Fdr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
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
pub struct Fmr_SPEC;
impl crate::sealed::RegSpec for Fmr_SPEC {
    type DataType = u32;
}
#[doc = "Flag Modification Register\n resetvalue={Application Reset:0x0}"]
pub type Fmr = crate::RegValueT<Fmr_SPEC>;

impl Fmr {
    #[doc = "Set Flag INTFx for Channel 7   FS7. Setting this bit will set the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fs0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Fmr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Flag INTFx for Channel 7   FS7. Setting this bit will set the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fs1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Fmr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Flag INTFx for Channel 7   FS7. Setting this bit will set the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fs2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Fmr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Flag INTFx for Channel 7   FS7. Setting this bit will set the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fs3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Fmr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Flag INTFx for Channel 7   FS7. Setting this bit will set the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fs4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Fmr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Flag INTFx for Channel 7   FS7. Setting this bit will set the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fs5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Fmr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Flag INTFx for Channel 7   FS7. Setting this bit will set the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fs6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Fmr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Flag INTFx for Channel 7   FS7. Setting this bit will set the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fs7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Fmr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Flag INTFx for Channel 7   FC7. Setting this bit will clear the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fc0(self) -> crate::common::RegisterFieldBool<16, 1, 0, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Fmr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Flag INTFx for Channel 7   FC7. Setting this bit will clear the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fc1(self) -> crate::common::RegisterFieldBool<17, 1, 0, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Fmr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Flag INTFx for Channel 7   FC7. Setting this bit will clear the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fc2(self) -> crate::common::RegisterFieldBool<18, 1, 0, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Fmr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Flag INTFx for Channel 7   FC7. Setting this bit will clear the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fc3(self) -> crate::common::RegisterFieldBool<19, 1, 0, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, Fmr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Flag INTFx for Channel 7   FC7. Setting this bit will clear the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fc4(self) -> crate::common::RegisterFieldBool<20, 1, 0, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, Fmr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Flag INTFx for Channel 7   FC7. Setting this bit will clear the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fc5(self) -> crate::common::RegisterFieldBool<21, 1, 0, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, Fmr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Flag INTFx for Channel 7   FC7. Setting this bit will clear the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fc6(self) -> crate::common::RegisterFieldBool<22, 1, 0, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, Fmr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Flag INTFx for Channel 7   FC7. Setting this bit will clear the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fc7(self) -> crate::common::RegisterFieldBool<23, 1, 0, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, Fmr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Fmr {
    #[inline(always)]
    fn default() -> Fmr {
        <crate::RegValueT<Fmr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Identification Register\n resetvalue={System Reset:0x0C4C0C1}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MODREV. This bit field indicates the revision number of the SCU module  C1 H  . Bits  5 0  defines the SCU module IP revision number. Bits  7 6  defines        the family. This allows a continuous numbering of the revision AND a        deviation of the different SCU generations. The revision number starts        with 000001 and is incremented when a SCU change is detectable by        customer software. MODREV for other variants will be defined by Design Team"]
    #[inline(always)]
    pub fn modrev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MODTYPE. This bit field is C0 H . It defines a 32 bit module"]
    #[inline(always)]
    pub fn modtype(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number   MODNUMBER. This bit field defines the module identification number. The identification number for the SCU is 00C4 H"]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(12894401)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IgcRj_SPEC;
impl crate::sealed::RegSpec for IgcRj_SPEC {
    type DataType = u32;
}
#[doc = "Flag Gating Register 0\n resetvalue={Application Reset:0x0}"]
pub type IgcRj = crate::RegValueT<IgcRj_SPEC>;

impl IgcRj {
    #[doc = "Flag Pattern Enable for Channel 0   IPEN07. Bit IPEN0x determines if the flag INTFx of channel x takes part in the pattern detection for the gating of the requests for the output signals GOUT 2 j  and IOUT 2 j ."]
    #[inline(always)]
    pub fn ipen00(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, IgcRj_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, IgcRj_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Flag Pattern Enable for Channel 0   IPEN07. Bit IPEN0x determines if the flag INTFx of channel x takes part in the pattern detection for the gating of the requests for the output signals GOUT 2 j  and IOUT 2 j ."]
    #[inline(always)]
    pub fn ipen01(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, IgcRj_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, IgcRj_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Flag Pattern Enable for Channel 0   IPEN07. Bit IPEN0x determines if the flag INTFx of channel x takes part in the pattern detection for the gating of the requests for the output signals GOUT 2 j  and IOUT 2 j ."]
    #[inline(always)]
    pub fn ipen02(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, IgcRj_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, IgcRj_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Flag Pattern Enable for Channel 0   IPEN07. Bit IPEN0x determines if the flag INTFx of channel x takes part in the pattern detection for the gating of the requests for the output signals GOUT 2 j  and IOUT 2 j ."]
    #[inline(always)]
    pub fn ipen03(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, IgcRj_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, IgcRj_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Flag Pattern Enable for Channel 0   IPEN07. Bit IPEN0x determines if the flag INTFx of channel x takes part in the pattern detection for the gating of the requests for the output signals GOUT 2 j  and IOUT 2 j ."]
    #[inline(always)]
    pub fn ipen04(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, IgcRj_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, IgcRj_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Flag Pattern Enable for Channel 0   IPEN07. Bit IPEN0x determines if the flag INTFx of channel x takes part in the pattern detection for the gating of the requests for the output signals GOUT 2 j  and IOUT 2 j ."]
    #[inline(always)]
    pub fn ipen05(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, IgcRj_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, IgcRj_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Flag Pattern Enable for Channel 0   IPEN07. Bit IPEN0x determines if the flag INTFx of channel x takes part in the pattern detection for the gating of the requests for the output signals GOUT 2 j  and IOUT 2 j ."]
    #[inline(always)]
    pub fn ipen06(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, IgcRj_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, IgcRj_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Flag Pattern Enable for Channel 0   IPEN07. Bit IPEN0x determines if the flag INTFx of channel x takes part in the pattern detection for the gating of the requests for the output signals GOUT 2 j  and IOUT 2 j ."]
    #[inline(always)]
    pub fn ipen07(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, IgcRj_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, IgcRj_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Generate Event Enable 0   GEEN0. Bit GEEN0 enables the generation of a trigger event for output channel         2 j  when the result of the pattern detection changes. When using this        feature  a trigger  e.g. for an interrupt  is generated during the first        clock cycle when a pattern is detected or when it is no longer detected."]
    #[inline(always)]
    pub fn geen0(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, IgcRj_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, IgcRj_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Gating Pattern 0   IGP0. In each register IGCRj  bit field IGP0 determines how the pattern        detection influences the output lines GOUT 2j  and IOUT 2j ."]
    #[inline(always)]
    pub fn igp0(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, IgcRj_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, IgcRj_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Pattern Enable for Channel 1   IPEN17. Bit IPEN 2j 1 x determines if the flag INTFx of channel  2j 1  takes part in the pattern detection for the gating of the requests for the output signals GOUT 2j 1  and IOUT 2j 1 ."]
    #[inline(always)]
    pub fn ipen10(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, IgcRj_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, IgcRj_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pattern Enable for Channel 1   IPEN17. Bit IPEN 2j 1 x determines if the flag INTFx of channel  2j 1  takes part in the pattern detection for the gating of the requests for the output signals GOUT 2j 1  and IOUT 2j 1 ."]
    #[inline(always)]
    pub fn ipen11(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, IgcRj_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, IgcRj_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pattern Enable for Channel 1   IPEN17. Bit IPEN 2j 1 x determines if the flag INTFx of channel  2j 1  takes part in the pattern detection for the gating of the requests for the output signals GOUT 2j 1  and IOUT 2j 1 ."]
    #[inline(always)]
    pub fn ipen12(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, IgcRj_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, IgcRj_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pattern Enable for Channel 1   IPEN17. Bit IPEN 2j 1 x determines if the flag INTFx of channel  2j 1  takes part in the pattern detection for the gating of the requests for the output signals GOUT 2j 1  and IOUT 2j 1 ."]
    #[inline(always)]
    pub fn ipen13(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, IgcRj_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, IgcRj_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pattern Enable for Channel 1   IPEN17. Bit IPEN 2j 1 x determines if the flag INTFx of channel  2j 1  takes part in the pattern detection for the gating of the requests for the output signals GOUT 2j 1  and IOUT 2j 1 ."]
    #[inline(always)]
    pub fn ipen14(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, IgcRj_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, IgcRj_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pattern Enable for Channel 1   IPEN17. Bit IPEN 2j 1 x determines if the flag INTFx of channel  2j 1  takes part in the pattern detection for the gating of the requests for the output signals GOUT 2j 1  and IOUT 2j 1 ."]
    #[inline(always)]
    pub fn ipen15(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, IgcRj_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, IgcRj_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pattern Enable for Channel 1   IPEN17. Bit IPEN 2j 1 x determines if the flag INTFx of channel  2j 1  takes part in the pattern detection for the gating of the requests for the output signals GOUT 2j 1  and IOUT 2j 1 ."]
    #[inline(always)]
    pub fn ipen16(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, IgcRj_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, IgcRj_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pattern Enable for Channel 1   IPEN17. Bit IPEN 2j 1 x determines if the flag INTFx of channel  2j 1  takes part in the pattern detection for the gating of the requests for the output signals GOUT 2j 1  and IOUT 2j 1 ."]
    #[inline(always)]
    pub fn ipen17(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, IgcRj_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, IgcRj_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Generate Event Enable 1   GEEN1. Bit GEEN1 enables the generation of a trigger event for output channel         2j 1  when the result of the pattern detection changes. When using this        feature  a trigger  e.g. for an interrupt  is generated during the first        clock cycle when a pattern is detected  or when it is no longer detected."]
    #[inline(always)]
    pub fn geen1(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, IgcRj_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, IgcRj_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Gating Pattern 1   IGP1. In each register IGCRj  bit field IGP1 determines how the pattern detection influences the output lines GOUT 2j 1  and IOUT 2j 1 ."]
    #[inline(always)]
    pub fn igp1(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, IgcRj_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, IgcRj_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for IgcRj {
    #[inline(always)]
    fn default() -> IgcRj {
        <crate::RegValueT<IgcRj_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct In_SPEC;
impl crate::sealed::RegSpec for In_SPEC {
    type DataType = u32;
}
#[doc = "ESR Input Register\n resetvalue={System Reset:0x0}"]
pub type In = crate::RegValueT<In_SPEC>;

impl In {
    #[doc = "Input Bit 1   P1. This bit indicates the level at the input pin ESR x."]
    #[inline(always)]
    pub fn p0(self) -> crate::common::RegisterFieldBool<0, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Bit 1   P1. This bit indicates the level at the input pin ESR x."]
    #[inline(always)]
    pub fn p1(self) -> crate::common::RegisterFieldBool<1, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, In_SPEC, crate::common::R>::from_register(
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
pub struct Iocr_SPEC;
impl crate::sealed::RegSpec for Iocr_SPEC {
    type DataType = u32;
}
#[doc = "Input Output Control Register\n resetvalue={System Reset:0x0E0}"]
pub type Iocr = crate::RegValueT<Iocr_SPEC>;

impl Iocr {
    #[doc = "Control for ESR0 Pin   PC0. This bit field defines the ESR0 functionality according to the coding        tables."]
    #[inline(always)]
    pub fn pc0(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Iocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Iocr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control for ESR1 Pin   PC1. This bit field defines the ESR1 functionality according to the coding tables. The reset value of SCU IOCR.PC1 is influenced by HWCFG6 and PMSWCR5.TRISTREQ. When a cold reset is activated and HWCFG6 1 then PC1 is reset to 2H and ESR1 will have input pull up mode. If HWCFG6 0 then PC1 is reset to 0H and ESR1 will have tri state mode. PC1 and the ESR1 reset state can also be configured by software with the PMSWCR5.TRISTREQ bit. PMSWCR5.TRISTREQ is not affected by warm reset or wake up from standby so the IOCR.PC1 reset value is configured as per the state of the TRISTREQ bit prior to the warm reset"]
    #[inline(always)]
    pub fn pc1(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Iocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Iocr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Iocr {
    #[inline(always)]
    fn default() -> Iocr {
        <crate::RegValueT<Iocr_SPEC> as RegisterValue<_>>::new(224)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lbistctrl0_SPEC;
impl crate::sealed::RegSpec for Lbistctrl0_SPEC {
    type DataType = u32;
}
#[doc = "Logic BIST Control 0 Register\n resetvalue={System Reset:0x0,CFS Value:0x400}"]
pub type Lbistctrl0 = crate::RegValueT<Lbistctrl0_SPEC>;

impl Lbistctrl0 {
    #[doc = "LBIST Request   LBISTREQ. If written high this bit requests the execution of an automatic        scan test procedure. The request will only be approved if LBISTCTRL0 .LBISTDONE        bit reflects a   8217 0  8217  value  i.e. no LBIST procedure was triggered since        the last power on reset or LBIST controller has been restarted through        the LBISTCTRL0 .LBISTRES bit .        If read this bit always returns a   8216 0  8217 . This bit shall be implemented in a safety relevant way to avoid        unintended activation of LBIST during application. LBIST execution time depends on the number of scan loads as defined in          the PATTERNS field."]
    #[inline(always)]
    pub fn lbistreq(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Lbistctrl0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,Lbistctrl0_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "LBIST Reset  LBISTRES. If written high this bit synchronously brings back the LBIST controller to its initial Reset Idle state and also clears the stored MISR signature to allow another execution from CPU side. As a consequence the LBISTCTRL0 .LBISTDONE  and SCU LBISTCTRL3.SIGNATURE bits will be set to   x2019 0  x2019 . If read this bit always returns a   x2018 0  x2019 . It is strongly recommended to not change the LBISTFREQU parameter in LBISTCTRL1 after this bit has been set to  1   because there is no guarantee that the new frequency parameter value will be transferred to the LBIST controller in time before the next LBIST run is started from user side  i.e. LBISTCTRL0.LBISTREQ is set to  1  ."]
    #[inline(always)]
    pub fn lbistres(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Lbistctrl0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,Lbistctrl0_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "LBIST Pattern Number   PATTERNS. This field defines the number of scan patterns  i.e. scan loads   which        will be executed during the LBIST procedure. Please note that the value        programmed to this field determines the number scan capture phases not        the number of scan chain load unload phases  i.e. a value of 0x00001        will result in two scan chain loads with on capture in between  a value        of 0x00002 will result in 3 scan chain loads with 2 captures  etc. .        Consequently a value of 0x00000 is not valid  because no capture would        be executed in this case."]
    #[inline(always)]
    pub fn patterns(
        self,
    ) -> crate::common::RegisterField<2, 0x3ffff, 1, 0, u32, Lbistctrl0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3ffff,1,0,u32, Lbistctrl0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LBIST Execution Indicator   LBISTDONE. This bit indicates the actual LBIST controller execution status"]
    #[inline(always)]
    pub fn lbistdone(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Lbistctrl0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28,1,0,Lbistctrl0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "LBIST   Test Mode Alarm Error Injection. If written high this bit trigger both  the LBIST  and the        test mode alarm. This is required to allow self testing of all        LBIST  and test mode  related safety mechanisms in the TCU. The bit will        be reset automatically once the LBIST and test mode alarm indicator        signals from TCU are asserted. From these indicator signals SCU will        also generate corresponding alarm trigger signals for SMU."]
    #[inline(always)]
    pub fn lbisterrinj(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Lbistctrl0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,Lbistctrl0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LBIST Request Redundancy. This bit represents the safety double of LBISTCTRL0.LBISTREQ. In order        to generate a new LBIST request both  LBISTREQRED and LBISTREQ bits must        be set to high due to safety reasons. The request will only be approved        if LBISTCTRL0.LBISTDONE bit reflects a  0  value. If read this bit        always returns a  0 ."]
    #[inline(always)]
    pub fn lbistreqred(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Lbistctrl0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,Lbistctrl0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Lbistctrl0 {
    #[inline(always)]
    fn default() -> Lbistctrl0 {
        <crate::RegValueT<Lbistctrl0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lbistctrl1_SPEC;
impl crate::sealed::RegSpec for Lbistctrl1_SPEC {
    type DataType = u32;
}
#[doc = "Logic BIST Control 1 Register\n resetvalue={System Reset:0x0,CFS Value:0x54000007}"]
pub type Lbistctrl1 = crate::RegValueT<Lbistctrl1_SPEC>;

impl Lbistctrl1 {
    #[doc = "LBIST Seed   SEED. This field determines  which pattern is applied to the EDT channel        inputs 1 19 during LBIST execution."]
    #[inline(always)]
    pub fn seed(
        self,
    ) -> crate::common::RegisterField<0, 0x7ffff, 1, 0, u32, Lbistctrl1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ffff,1,0,u32, Lbistctrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LBIST Split Shift Selection   SPLITSH. The value of this bit will allow to run LBIST with partitioned        scan shift operation in order to reduce the power consumption."]
    #[inline(always)]
    pub fn splitsh(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Lbistctrl1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Lbistctrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Body Application Indicator   BODY. The value of this bit will determine the static reset behavior of all GPIOs during LBIST execution. If set to low GPIOs will show a weak pull up behavior  if set to high GPIOs are constrained to tri state. A high value must be written to this bit in case LBIST shall be executed for body applications."]
    #[inline(always)]
    pub fn body(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Lbistctrl1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,Lbistctrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LBIST Frequency Selection   LBISTFREQU. Through this register field a pre scaler factor between 1..16 is selectable for LBIST operation clock  derived from EVR oscillator . This will allow to determine the LBIST scan shift frequency. Value of these bits will be mirrored inside of LBIST controller and become effective if a new LBIST procedure has been successfully initiated from system side  via LBISTCTRL0 .LBISTREQ . It is strongly recommended not to change the value of this field after LBISTCTRL0.LBISTRES has been set to high  because there is no guarantee that the new frequency parameter value will be transferred to the LBIST controller in time before the next LBIST run is started from user side  i.e. LBISTCTRL0.LBISTREQ is set to  1"]
    #[inline(always)]
    pub fn lbistfrequ(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Lbistctrl1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Lbistctrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Lbistctrl1 {
    #[inline(always)]
    fn default() -> Lbistctrl1 {
        <crate::RegValueT<Lbistctrl1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lbistctrl2_SPEC;
impl crate::sealed::RegSpec for Lbistctrl2_SPEC {
    type DataType = u32;
}
#[doc = "Logic BIST Control 2 Register\n resetvalue={System Reset:0x0,CFS Value:0x3D}"]
pub type Lbistctrl2 = crate::RegValueT<Lbistctrl2_SPEC>;

impl Lbistctrl2 {
    #[doc = "LBIST Maximum Scan Chain Length   LENGTH. This field defines the number of shift cycles for each LBIST scan load. It will be automatically loaded with the product specific value  stored in Flash config sector during startup software execution."]
    #[inline(always)]
    pub fn length(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, Lbistctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16, Lbistctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Lbistctrl2 {
    #[inline(always)]
    fn default() -> Lbistctrl2 {
        <crate::RegValueT<Lbistctrl2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lbistctrl3_SPEC;
impl crate::sealed::RegSpec for Lbistctrl3_SPEC {
    type DataType = u32;
}
#[doc = "Logic BIST Control 3 Register\n resetvalue={System Reset:0x0,CFS Value:0x0}"]
pub type Lbistctrl3 = crate::RegValueT<Lbistctrl3_SPEC>;

impl Lbistctrl3 {
    #[doc = "LBIST Signature   SIGNATURE. This field reflects the MISR signature from the last LBIST execution. It is mirrored from LBIST controller inside TCU and only valid if LBISTCTRL0 .LBISTDONE is read with a high value. In case of a restart of the LBIST controller  via LBISTCTRL0 .LBISTRES function   the signature value will be synchronously reset to all 0. Please address the specific device appendix document for a description on the SIGNATURE value  depending on the LBIST run configuration."]
    #[inline(always)]
    pub fn signature(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Lbistctrl3_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Lbistctrl3_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Lbistctrl3 {
    #[inline(always)]
    fn default() -> Lbistctrl3 {
        <crate::RegValueT<Lbistctrl3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lclcon0_SPEC;
impl crate::sealed::RegSpec for Lclcon0_SPEC {
    type DataType = u32;
}
#[doc = "LCL CPU0 and CPU2 Control Register\n resetvalue={Cold PowerOn Reset:0x080018001,Cold PowerOn Reset:0x080010000,Cold PowerOn Reset:0x080018001}"]
pub type Lclcon0 = crate::RegValueT<Lclcon0_SPEC>;

impl Lclcon0 {
    #[doc = "Lockstep Mode Status   LS0. This bit indicates whether CPU0 is currently running in lockstep monitor        mode"]
    #[inline(always)]
    pub fn ls0(self) -> crate::common::RegisterFieldBool<16, 1, 0, Lclcon0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Lclcon0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Lockstep Enable   LSEN0. This bit may only be written by SSW during boot. Enable lockstep CPU monitoring for the associated processor core  CPU0. After cold reset  lockstep is enabled by default. The LSEN bit may be        cleared during the boot to disable lockstep mode. SMU lockstep fault        reporting should be disabled when lockstep is disabled."]
    #[inline(always)]
    pub fn lsen0(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Lclcon0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Lclcon0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Lclcon0 {
    #[inline(always)]
    fn default() -> Lclcon0 {
        <crate::RegValueT<Lclcon0_SPEC> as RegisterValue<_>>::new(2147549184)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lclcon1_SPEC;
impl crate::sealed::RegSpec for Lclcon1_SPEC {
    type DataType = u32;
}
#[doc = "LCL CPU1 and CPU3 Control Register\n resetvalue={Cold PowerOn Reset:0x080018001,Cold PowerOn Reset:0x080010000,Cold PowerOn Reset:0x0,Cold PowerOn Reset:0x080018001}"]
pub type Lclcon1 = crate::RegValueT<Lclcon1_SPEC>;

impl Lclcon1 {
    #[doc = "Lockstep Mode Status   LS1. This bit indicates whether CPU1 is currently running in lockstep monitor        mode"]
    #[inline(always)]
    pub fn ls1(self) -> crate::common::RegisterFieldBool<16, 1, 0, Lclcon1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Lclcon1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Lockstep Enable   LSEN1. This bit may only be written by SSW during boot. Enable lockstep CPU monitoring for the associated processor core  CPU1.       If the product has no lockstep capability for CPU1  then this enables only the PFLASH access monitoring for CPU1. After cold reset  lockstep is enabled by default. The LSEN bit may be        cleared during the boot to disable lockstep mode. SMU lockstep fault        reporting should be disabled when lockstep is disabled."]
    #[inline(always)]
    pub fn lsen1(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Lclcon1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Lclcon1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Lclcon1 {
    #[inline(always)]
    fn default() -> Lclcon1 {
        <crate::RegValueT<Lclcon1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcltest_SPEC;
impl crate::sealed::RegSpec for Lcltest_SPEC {
    type DataType = u32;
}
#[doc = "LCL Test Register\n resetvalue={System Reset:0x0}"]
pub type Lcltest = crate::RegValueT<Lcltest_SPEC>;

impl Lcltest {
    #[doc = "LCL0 Lockstep Test   LCLT0. Fault injection for LCL0. Reads as zero."]
    #[inline(always)]
    pub fn lclt0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Lcltest_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Lcltest_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "LCL1 Lockstep Test   LCLT1. Fault injection for LCL1. Reads as zero."]
    #[inline(always)]
    pub fn lclt1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Lcltest_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Lcltest_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "LCL2 Lockstep Test   LCLT2. Fault injection for LCL2. Reads as zero."]
    #[inline(always)]
    pub fn lclt2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Lcltest_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Lcltest_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "PFI0 Lockstep Test. Fault injection for PFI0 lockstep. Reads as zero."]
    #[inline(always)]
    pub fn plclt0(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Lcltest_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Lcltest_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "PFI1 Lockstep Test . Fault injection for PFI1 lockstep. Reads as zero."]
    #[inline(always)]
    pub fn plclt1(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Lcltest_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Lcltest_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "PFI2 Lockstep Test . Fault injection for PFI2 lockstep. Reads as zero."]
    #[inline(always)]
    pub fn plclt2(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Lcltest_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Lcltest_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Lcltest {
    #[inline(always)]
    fn default() -> Lcltest {
        <crate::RegValueT<Lcltest_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Manid_SPEC;
impl crate::sealed::RegSpec for Manid_SPEC {
    type DataType = u32;
}
#[doc = "Manufacturer Identification Register\n resetvalue={System Reset:0x1820}"]
pub type Manid = crate::RegValueT<Manid_SPEC>;

impl Manid {
    #[doc = "Department Identification Number   DEPT.   00 H   indicates the Automotive  amp  Industrial microcontroller department within Infineon Technologies."]
    #[inline(always)]
    pub fn dept(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, Manid_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, Manid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Manufacturer Identification Number   MANUF. This is a JEDEC normalized manufacturer code. MANUF   C1 H stands for Infineon Technologies."]
    #[inline(always)]
    pub fn manuf(
        self,
    ) -> crate::common::RegisterField<5, 0x7ff, 1, 0, u16, Manid_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x7ff,1,0,u16, Manid_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Manid {
    #[inline(always)]
    fn default() -> Manid {
        <crate::RegValueT<Manid_SPEC> as RegisterValue<_>>::new(6176)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Omr_SPEC;
impl crate::sealed::RegSpec for Omr_SPEC {
    type DataType = u32;
}
#[doc = "ESR Output Modification Register\n resetvalue={System Reset:0x0}"]
pub type Omr = crate::RegValueT<Omr_SPEC>;

impl Omr {
    #[doc = "ESRx Pin Set Bit 1   PS1. Setting this bit will set or toggle the corresponding bit in the output        register OUT. Reading this bit returns 0."]
    #[inline(always)]
    pub fn ps0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "ESRx Pin Set Bit 1   PS1. Setting this bit will set or toggle the corresponding bit in the output        register OUT. Reading this bit returns 0."]
    #[inline(always)]
    pub fn ps1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "ESRx Pin Clear Bit 1   PCL1. Setting this bit will clear or toggle the corresponding bit in the port        output register OUT. Reading this bit returns 0."]
    #[inline(always)]
    pub fn pcl0(self) -> crate::common::RegisterFieldBool<16, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "ESRx Pin Clear Bit 1   PCL1. Setting this bit will clear or toggle the corresponding bit in the port        output register OUT. Reading this bit returns 0."]
    #[inline(always)]
    pub fn pcl1(self) -> crate::common::RegisterFieldBool<17, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Omr_SPEC, crate::common::W>::from_register(
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
pub struct Osccon_SPEC;
impl crate::sealed::RegSpec for Osccon_SPEC {
    type DataType = u32;
}
#[doc = "OSC Control Register\n resetvalue={System Reset:0x10,System Reset:0x258}"]
pub type Osccon = crate::RegValueT<Osccon_SPEC>;

impl Osccon {
    #[doc = "Oscillator for PLL Valid Low Status Bit   PLLLV. This bit indicates if the frequency output f osc of the oscillator is above the lower threshold frequency f LV   i.e. usable for the DCO part of the PLL. This is checked by the Oscillator Watchdog of the PLL using the backup clock f BACK . By using the crystal s nominal frequency   f oscnom    the lower threshold calculates as follows  f LV   f oscnom   0 96   0 31"]
    #[inline(always)]
    pub fn plllv(self) -> crate::common::RegisterFieldBool<1, 1, 0, Osccon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Osccon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Oscillator Watchdog Reset   OSCRES. Always read as zero."]
    #[inline(always)]
    pub fn oscres(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Osccon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Osccon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Oscillator Gain Selection   GAINSEL. In Normal Mode this value should not be changed from the reset value 11 B . When using Vext 3.3V  the LVDS bias distributor has to be adjusted to 3.3V supply via P21 LPCR2.PS   0 otherwise the oscillator gain can be too low for a reliable oscillator startup at cold temperature. In case of using Vext 5V  the LVDS bias distributor setting stays at the reset value P21 LPCR2.PS   1."]
    #[inline(always)]
    pub fn gainsel(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, Osccon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x3,1,0,u8, Osccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Oscillator Mode   MODE. This bit field defines which mode can be used and if the oscillator entered the Power Saving Mode or not. In Test Mode the shaper can be bypassed even if the oscillator is in the Oscillator Power Saving Mode. This is controlled by the TCU."]
    #[inline(always)]
    pub fn mode(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, Osccon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8, Osccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Shaper Bypass   SHBY"]
    #[inline(always)]
    pub fn shby(self) -> crate::common::RegisterFieldBool<7, 1, 0, Osccon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Osccon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Oscillator for PLL Valid High Status Bit   PLLHV. This bit indicates if the frequency output f osc of the oscillator is below the upper threshold frequency f HV   i.e. usable for the DCO part of the PLL. This is checked by the Oscillator Watchdog of the PLL using the backup clock f BACK . By using the crystal s nominal frequency   f oscnom    the upper threshold calculates as follows  f HV   f oscnom   1 04   0 29"]
    #[inline(always)]
    pub fn pllhv(self) -> crate::common::RegisterFieldBool<8, 1, 0, Osccon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Osccon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Hysteresis Enable"]
    #[inline(always)]
    pub fn hysen(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Osccon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Osccon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Hysteresis Control"]
    #[inline(always)]
    pub fn hysctl(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Osccon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Osccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Amplitude Control"]
    #[inline(always)]
    pub fn ampctl(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Osccon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Osccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OSC Frequency Value   OSCVAL. This bit field defines the divider value that generates the reference        clock that is supervised by the oscillator watchdog. f OSC   160    160 OSCCON.OSCVAL  160    160 1  160    160 16  160 MHz."]
    #[inline(always)]
    pub fn oscval(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Osccon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Osccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Amplitude Regulation Enable   APREN. This bit field enables and disables Amplitude Regulation mode. When enabled  the bit field GAINSEL limits the maximum gain."]
    #[inline(always)]
    pub fn apren(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Osccon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Osccon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Capacitance 0 Enable   CAP0EN. Total capacitance for each XTAL1 and XTAL2 is the sum of the enabled capacitance 0 to 3."]
    #[inline(always)]
    pub fn cap0en(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Osccon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Osccon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Capacitance 1 Enable   CAP1EN. Total capacitance for each XTAL1 and XTAL2 is the sum of the enabled capacitance 0 to 3."]
    #[inline(always)]
    pub fn cap1en(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Osccon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Osccon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Capacitance 2 Enable   CAP2EN. Total capacitance for each XTAL1 and XTAL2 is the sum of the enabled capacitance 0 to 3."]
    #[inline(always)]
    pub fn cap2en(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Osccon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Osccon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Capacitance 3 Enable   CAP3EN. Total capacitance for each XTAL1 and XTAL2 is the sum of the enabled capacitance 0 to 3."]
    #[inline(always)]
    pub fn cap3en(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Osccon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Osccon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Osccon {
    #[inline(always)]
    fn default() -> Osccon {
        <crate::RegValueT<Osccon_SPEC> as RegisterValue<_>>::new(16)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Out_SPEC;
impl crate::sealed::RegSpec for Out_SPEC {
    type DataType = u32;
}
#[doc = "ESR Output Register\n resetvalue={System Reset:0x0}"]
pub type Out = crate::RegValueT<Out_SPEC>;

impl Out {
    #[doc = "Output Bit 1   P1. This bit determines the level at the output pin ESR x if the output is selected as GPIO output. Px can also be set cleared by control bits of the OMR register."]
    #[inline(always)]
    pub fn p0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Output Bit 1   P1. This bit determines the level at the output pin ESR x if the output is selected as GPIO output. Px can also be set cleared by control bits of the OMR register."]
    #[inline(always)]
    pub fn p1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Out_SPEC, crate::common::RW>::from_register(
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
pub struct Ovccon_SPEC;
impl crate::sealed::RegSpec for Ovccon_SPEC {
    type DataType = u32;
}
#[doc = "Overlay Control Register\n resetvalue={Application Reset:0x0}"]
pub type Ovccon = crate::RegValueT<Ovccon_SPEC>;

impl Ovccon {
    #[doc = "CPU Select 0   CSEL0. Return 0 if read."]
    #[inline(always)]
    pub fn csel0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ovccon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ovccon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU Select 1  If product has CPU1    CSEL1. Return 0 if read."]
    #[inline(always)]
    pub fn csel1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ovccon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ovccon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU Select 2  If product has CPU2    CSEL2. Return 0 if read."]
    #[inline(always)]
    pub fn csel2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ovccon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ovccon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Overlay Start   OVSTRT. CPUs which are not selected are not affected. No action is taken if OVSTP is also set. Return 0 if read."]
    #[inline(always)]
    pub fn ovstrt(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Ovccon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ovccon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Overlay Stop   OVSTP. CPUs which are not selected are not affected No action is taken if OVSTRT is also set. Return 0 if read."]
    #[inline(always)]
    pub fn ovstp(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Ovccon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ovccon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Data Cache Invalidate   DCINVAL. No function in devices without data cache in CPU. This bit  when set  generates a pulse on signal ... ovc dcinval. Data Cache is affected only in the CPUs selected with CSEL. Return 0 if read."]
    #[inline(always)]
    pub fn dcinval(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Ovccon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ovccon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Overlay Configured   OVCONF. Overlay configured status bit This bit may be used as handshake bit between a debug device  via JTAG        interface and Cerberus  and CPU s ."]
    #[inline(always)]
    pub fn ovconf(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Ovccon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ovccon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Write Protection for OVCONF   POVCONF. This bit enables OVCONF write during OVCCON write. Return 0 if read."]
    #[inline(always)]
    pub fn povconf(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Ovccon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ovccon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ovccon {
    #[inline(always)]
    fn default() -> Ovccon {
        <crate::RegValueT<Ovccon_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ovcenable_SPEC;
impl crate::sealed::RegSpec for Ovcenable_SPEC {
    type DataType = u32;
}
#[doc = "Overlay Enable Register\n resetvalue={Application Reset:0x0}"]
pub type Ovcenable = crate::RegValueT<Ovcenable_SPEC>;

impl Ovcenable {
    #[doc = "Overlay Enable 0   OVEN0"]
    #[inline(always)]
    pub fn oven0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Ovcenable_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Ovcenable_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Overlay Enable 1  If product has CPU1    OVEN1"]
    #[inline(always)]
    pub fn oven1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Ovcenable_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Ovcenable_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Overlay Enable 2  If product has CPU2    OVEN2"]
    #[inline(always)]
    pub fn oven2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Ovcenable_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Ovcenable_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ovcenable {
    #[inline(always)]
    fn default() -> Ovcenable {
        <crate::RegValueT<Ovcenable_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdisc_SPEC;
impl crate::sealed::RegSpec for Pdisc_SPEC {
    type DataType = u32;
}
#[doc = "Pad Disable Control Register\n resetvalue={System Reset:0x0}"]
pub type Pdisc = crate::RegValueT<Pdisc_SPEC>;

impl Pdisc {
    #[doc = "Pad Disable for ESR Pin 1   PDIS1. This bit disables the pad."]
    #[inline(always)]
    pub fn pdis0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Pdisc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pdisc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pad Disable for ESR Pin 1   PDIS1. This bit disables the pad."]
    #[inline(always)]
    pub fn pdis1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Pdisc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pdisc_SPEC, crate::common::RW>::from_register(
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
pub struct Pdr_SPEC;
impl crate::sealed::RegSpec for Pdr_SPEC {
    type DataType = u32;
}
#[doc = "ESR Pad Driver Mode Register\n resetvalue={System Reset:0x0}"]
pub type Pdr = crate::RegValueT<Pdr_SPEC>;

impl Pdr {
    #[doc = "Pad Driver Mode for ESR Pins 0   PD0.  See PDR register description in PORTs chapter"]
    #[inline(always)]
    pub fn pd0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Pdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0x3, 1, 0, u8, Pdr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pad Level Selection for ESR Pins 0   PL0.  See PDR register description in PORTs chapter"]
    #[inline(always)]
    pub fn pl0(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Pdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2, 0x3, 1, 0, u8, Pdr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pad Driver Mode for ESR Pins 1   PD1.  See PDR register description in PORTs chapter"]
    #[inline(always)]
    pub fn pd1(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Pdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4, 0x3, 1, 0, u8, Pdr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pad Level Selection for ESR Pins 1   PL1.  See PDR register description in PORTs chapter"]
    #[inline(always)]
    pub fn pl1(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Pdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6, 0x3, 1, 0, u8, Pdr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Pdr {
    #[inline(always)]
    fn default() -> Pdr {
        <crate::RegValueT<Pdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdrr_SPEC;
impl crate::sealed::RegSpec for Pdrr_SPEC {
    type DataType = u32;
}
#[doc = "Pattern Detection Result Register\n resetvalue={Application Reset:0x0FF}"]
pub type Pdrr = crate::RegValueT<Pdrr_SPEC>;

impl Pdrr {
    #[doc = "Pattern Detection Result of Channel 7   PDR7. This bit monitors the output status of the pattern detection for the output channel y."]
    #[inline(always)]
    pub fn pdr0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Pdrr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pdrr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pattern Detection Result of Channel 7   PDR7. This bit monitors the output status of the pattern detection for the output channel y."]
    #[inline(always)]
    pub fn pdr1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Pdrr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pdrr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pattern Detection Result of Channel 7   PDR7. This bit monitors the output status of the pattern detection for the output channel y."]
    #[inline(always)]
    pub fn pdr2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Pdrr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Pdrr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pattern Detection Result of Channel 7   PDR7. This bit monitors the output status of the pattern detection for the output channel y."]
    #[inline(always)]
    pub fn pdr3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Pdrr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Pdrr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pattern Detection Result of Channel 7   PDR7. This bit monitors the output status of the pattern detection for the output channel y."]
    #[inline(always)]
    pub fn pdr4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Pdrr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pdrr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pattern Detection Result of Channel 7   PDR7. This bit monitors the output status of the pattern detection for the output channel y."]
    #[inline(always)]
    pub fn pdr5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Pdrr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Pdrr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pattern Detection Result of Channel 7   PDR7. This bit monitors the output status of the pattern detection for the output channel y."]
    #[inline(always)]
    pub fn pdr6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Pdrr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Pdrr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pattern Detection Result of Channel 7   PDR7. This bit monitors the output status of the pattern detection for the output channel y."]
    #[inline(always)]
    pub fn pdr7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Pdrr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Pdrr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Pdrr {
    #[inline(always)]
    fn default() -> Pdrr {
        <crate::RegValueT<Pdrr_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Perpllcon0_SPEC;
impl crate::sealed::RegSpec for Perpllcon0_SPEC {
    type DataType = u32;
}
#[doc = "Peripheral PLL Configuration 0 Register\n resetvalue={System Reset:0x3E00}"]
pub type Perpllcon0 = crate::RegValueT<Perpllcon0_SPEC>;

impl Perpllcon0 {
    #[doc = "Divider Bypass   DIVBY"]
    #[inline(always)]
    pub fn divby(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Perpllcon0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Perpllcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N Divider Value   NDIV. The value the N Divider operates with is NDIV 1."]
    #[inline(always)]
    pub fn ndiv(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, Perpllcon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8, Perpllcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral PLL Power Saving Mode   PLLPWD. If the PLL has been powered down and is getting re enabled via PLLPWD   1  a wait period of 1 ms has to be applied until it is stable without jitter."]
    #[inline(always)]
    pub fn pllpwd(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Perpllcon0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Perpllcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Restart DCO Lock Detection   RESLD. Setting this bit will clear bit SYSPLLSTAT.LOCK and restart the DCO lock detection. Reading this bit returns always a zero."]
    #[inline(always)]
    pub fn resld(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Perpllcon0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18,1,0,Perpllcon0_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "P Divider Value   PDIV. The value the P Divider operates with is PDIV 1."]
    #[inline(always)]
    pub fn pdiv(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Perpllcon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Perpllcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Perpllcon0 {
    #[inline(always)]
    fn default() -> Perpllcon0 {
        <crate::RegValueT<Perpllcon0_SPEC> as RegisterValue<_>>::new(15872)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Perpllcon1_SPEC;
impl crate::sealed::RegSpec for Perpllcon1_SPEC {
    type DataType = u32;
}
#[doc = "Peripheral PLL Configuration 1 Register\n resetvalue={System Reset:0x1}"]
pub type Perpllcon1 = crate::RegValueT<Perpllcon1_SPEC>;

impl Perpllcon1 {
    #[doc = "K2 Divider Value   K2DIV. The value the K2 Divider operates with is K2DIV 1. While PERPLLSTAT.K2RDY   0  K2DIV is locked."]
    #[inline(always)]
    pub fn k2div(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Perpllcon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Perpllcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "K3 Divider Value   K3DIV. The value the K3 Divider operates with is K3DIV 1. While PERPLLSTAT.K3RDY   0  K3DIV is locked."]
    #[inline(always)]
    pub fn k3div(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Perpllcon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Perpllcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Perpllcon1 {
    #[inline(always)]
    fn default() -> Perpllcon1 {
        <crate::RegValueT<Perpllcon1_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Perpllstat_SPEC;
impl crate::sealed::RegSpec for Perpllstat_SPEC {
    type DataType = u32;
}
#[doc = "Peripheral PLL Status Register\n resetvalue={System Reset:0x2}"]
pub type Perpllstat = crate::RegValueT<Perpllstat_SPEC>;

impl Perpllstat {
    #[doc = "Peripheral PLL Power saving Mode Status   PWDSTAT"]
    #[inline(always)]
    pub fn pwdstat(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Perpllstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,Perpllstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Peripheral PLL Lock Status   LOCK. In case of a loss of lock  the f DCO is kept on the previous constant frequency."]
    #[inline(always)]
    pub fn lock(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Perpllstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,Perpllstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "K3 Divider Ready Status   K3RDY. This bit indicates whether the K3 divider operates on the configured value. This is of interest when the PERPLLCON1.K3DIV value is changed. The PLL must be enabled and clocked to set the K3RDY field."]
    #[inline(always)]
    pub fn k3rdy(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Perpllstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,Perpllstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "K2 Divider Ready Status   K2RDY. This bit indicates whether the K2 divider operates on the configured value. This is of interest when the PERPLLCON1.K2DIV value is changed. The PLL must be enabled and clocked to set the K2RDY field."]
    #[inline(always)]
    pub fn k2rdy(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Perpllstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,Perpllstat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Perpllstat {
    #[inline(always)]
    fn default() -> Perpllstat {
        <crate::RegValueT<Perpllstat_SPEC> as RegisterValue<_>>::new(2)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmcsr0_SPEC;
impl crate::sealed::RegSpec for Pmcsr0_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
pub type Pmcsr0 = crate::RegValueT<Pmcsr0_SPEC>;

impl Pmcsr0 {
    #[doc = "Idle Mode and Sleep Mode Request   REQSLP. In Idle Mode or Sleep Mode  these bits are cleared in response to an        interrupt for the CPU  or when bit 15 of the corresponding CPU Watchdog        Timer register  bit WDTCPUxSR.TIM 15   changes from 0 to 1. In Standby        Mode  these bits are cleared on wake up. REQSLP maybe written only when        either CPU or Safety ENDINIT bits are set to 0. CPU ENDINIT bit has to        be set back after REQSLP is written for the mode transition to take        place. In case of Safety ENDINIT  the mode transition will be issued        immediately and does not wait till Safety ENDINIT is set back to 1 again. No explicit acknowledge to SCU on System Standby request. System Standby        entry in a few cycles."]
    #[inline(always)]
    pub fn reqslp(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Pmcsr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Pmcsr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Power management Status   PMST. This bit field reflects the current status of the CPU."]
    #[inline(always)]
    pub fn pmst(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Pmcsr0_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Pmcsr0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pmcsr0 {
    #[inline(always)]
    fn default() -> Pmcsr0 {
        <crate::RegValueT<Pmcsr0_SPEC> as RegisterValue<_>>::new(256)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmcsr1_SPEC;
impl crate::sealed::RegSpec for Pmcsr1_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
pub type Pmcsr1 = crate::RegValueT<Pmcsr1_SPEC>;

impl Pmcsr1 {
    #[doc = "Idle Mode and Sleep Mode Request   REQSLP. In Idle Mode or Sleep Mode  these bits are cleared in response to an interrupt for the CPU  or when bit 15 of the corresponding CPU Watchdog Timer register  bit WDTCPUxSR.TIM 15   changes from 0 to 1. In Standby Mode  these bits are cleared on wake up. REQSLP maybe written only when either CPU or Safety ENDINIT bits are set to 0. CPU ENDINIT bit has to be set back after REQSLP is written for the mode transition to take place. In case of Safety ENDINIT  the mode transition will be issued immediately and does not wait till Safety ENDINIT is set back to 1 again. No explicit acknowledge to SCU on System Standby request. System Standby entry in a few cycles."]
    #[inline(always)]
    pub fn reqslp(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Pmcsr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Pmcsr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Power management Status   PMST. This bit field reflects the current status of the CPU."]
    #[inline(always)]
    pub fn pmst(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Pmcsr1_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Pmcsr1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pmcsr1 {
    #[inline(always)]
    fn default() -> Pmcsr1 {
        <crate::RegValueT<Pmcsr1_SPEC> as RegisterValue<_>>::new(256)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmcsr2_SPEC;
impl crate::sealed::RegSpec for Pmcsr2_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
pub type Pmcsr2 = crate::RegValueT<Pmcsr2_SPEC>;

impl Pmcsr2 {
    #[doc = "Idle Mode and Sleep Mode Request   REQSLP. In Idle Mode or Sleep Mode  these bits are cleared in response to an interrupt for the CPU  or when bit 15 of the corresponding CPU Watchdog Timer register  bit WDTCPUxSR.TIM 15   changes from 0 to 1. In Standby Mode  these bits are cleared on wake up. REQSLP maybe written only when either CPU or Safety ENDINIT bits are set to 0. CPU ENDINIT bit has to be set back after REQSLP is written for the mode transition to take place. In case of Safety ENDINIT  the mode transition will be issued immediately and does not wait till Safety ENDINIT is set back to 1 again. No explicit acknowledge to SCU on System Standby request. System Standby entry in a few cycles."]
    #[inline(always)]
    pub fn reqslp(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Pmcsr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Pmcsr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Power management Status   PMST. This bit field reflects the current status of the CPU."]
    #[inline(always)]
    pub fn pmst(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Pmcsr2_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Pmcsr2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pmcsr2 {
    #[inline(always)]
    fn default() -> Pmcsr2 {
        <crate::RegValueT<Pmcsr2_SPEC> as RegisterValue<_>>::new(256)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmcsr3_SPEC;
impl crate::sealed::RegSpec for Pmcsr3_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
pub type Pmcsr3 = crate::RegValueT<Pmcsr3_SPEC>;

impl Pmcsr3 {
    #[doc = "Idle Mode and Sleep Mode Request   REQSLP. In Idle Mode or Sleep Mode  these bits are cleared in response to an interrupt for the CPU  or when bit 15 of the corresponding CPU Watchdog Timer register  bit WDTCPUxSR.TIM 15   changes from 0 to 1. In Standby Mode  these bits are cleared on wake up. REQSLP maybe written only when either CPU or Safety ENDINIT bits are set to 0. CPU ENDINIT bit has to be set back after REQSLP is written for the mode transition to take place. In case of Safety ENDINIT  the mode transition will be issued immediately and does not wait till Safety ENDINIT is set back to 1 again. No explicit acknowledge to SCU on System Standby request. System Standby entry in a few cycles."]
    #[inline(always)]
    pub fn reqslp(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Pmcsr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Pmcsr3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Power management Status   PMST. This bit field reflects the current status of the CPU."]
    #[inline(always)]
    pub fn pmst(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Pmcsr3_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Pmcsr3_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pmcsr3 {
    #[inline(always)]
    fn default() -> Pmcsr3 {
        <crate::RegValueT<Pmcsr3_SPEC> as RegisterValue<_>>::new(256)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmcsr4_SPEC;
impl crate::sealed::RegSpec for Pmcsr4_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
pub type Pmcsr4 = crate::RegValueT<Pmcsr4_SPEC>;

impl Pmcsr4 {
    #[doc = "Idle Mode and Sleep Mode Request   REQSLP. In Idle Mode or Sleep Mode  these bits are cleared in response to an interrupt for the CPU  or when bit 15 of the corresponding CPU Watchdog Timer register  bit WDTCPUxSR.TIM 15   changes from 0 to 1. In Standby Mode  these bits are cleared on wake up. REQSLP maybe written only when either CPU or Safety ENDINIT bits are set to 0. CPU ENDINIT bit has to be set back after REQSLP is written for the mode transition to take place. In case of Safety ENDINIT  the mode transition will be issued immediately and does not wait till Safety ENDINIT is set back to 1 again. No explicit acknowledge to SCU on System Standby request. System Standby entry in a few cycles."]
    #[inline(always)]
    pub fn reqslp(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Pmcsr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Pmcsr4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Power management Status   PMST. This bit field reflects the current status of the CPU."]
    #[inline(always)]
    pub fn pmst(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Pmcsr4_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Pmcsr4_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pmcsr4 {
    #[inline(always)]
    fn default() -> Pmcsr4 {
        <crate::RegValueT<Pmcsr4_SPEC> as RegisterValue<_>>::new(256)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmcsr5_SPEC;
impl crate::sealed::RegSpec for Pmcsr5_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
pub type Pmcsr5 = crate::RegValueT<Pmcsr5_SPEC>;

impl Pmcsr5 {
    #[doc = "Idle Mode and Sleep Mode Request   REQSLP. In Idle Mode or Sleep Mode  these bits are cleared in response to an interrupt for the CPU  or when bit 15 of the corresponding CPU Watchdog Timer register  bit WDTCPUxSR.TIM 15   changes from 0 to 1. In Standby Mode  these bits are cleared on wake up. REQSLP maybe written only when either CPU or Safety ENDINIT bits are set to 0. CPU ENDINIT bit has to be set back after REQSLP is written for the mode transition to take place. In case of Safety ENDINIT  the mode transition will be issued immediately and does not wait till Safety ENDINIT is set back to 1 again. No explicit acknowledge to SCU on System Standby request. System Standby entry in a few cycles."]
    #[inline(always)]
    pub fn reqslp(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Pmcsr5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Pmcsr5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Power management Status   PMST. This bit field reflects the current status of the CPU."]
    #[inline(always)]
    pub fn pmst(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Pmcsr5_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Pmcsr5_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pmcsr5 {
    #[inline(always)]
    fn default() -> Pmcsr5 {
        <crate::RegValueT<Pmcsr5_SPEC> as RegisterValue<_>>::new(256)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmstat0_SPEC;
impl crate::sealed::RegSpec for Pmstat0_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Status Register 0\n resetvalue={Application Reset:0x1}"]
pub type Pmstat0 = crate::RegValueT<Pmstat0_SPEC>;

impl Pmstat0 {
    #[doc = "CPU0 Status   CPU0. This bit field reflects the current status of CPU0."]
    #[inline(always)]
    pub fn cpu0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Pmstat0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pmstat0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 Status   CPU1. This bit field reflects the current status of CPU1."]
    #[inline(always)]
    pub fn cpu1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Pmstat0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pmstat0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 Status   CPU2. This bit field reflects the current status of CPU2."]
    #[inline(always)]
    pub fn cpu2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Pmstat0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Pmstat0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 Status   CPU3. This bit field reflects the current status of CPU3."]
    #[inline(always)]
    pub fn cpu3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Pmstat0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Pmstat0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 Status   CPU4. This bit field reflects the current status of CPU4."]
    #[inline(always)]
    pub fn cpu4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Pmstat0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pmstat0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 Status   CPU5. This bit field reflects the current status of CPU5."]
    #[inline(always)]
    pub fn cpu5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Pmstat0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Pmstat0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU0LS Status   CPU0LS. This bit field reflects the current status of CPU0 Lockstep Checker        Core.The activation of the Lockstep is configured in UCB BMI        configuration and determines the default reset value. The default reset        value 0 is for the case where CPU0LS is disabled in UCB BMI        configuration."]
    #[inline(always)]
    pub fn cpu0ls(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Pmstat0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Pmstat0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1LS Status   CPU1LS. This bit field reflects the current status of CPU1 Lockstep Checker Core.The activation of the Lockstep is configured in UCB BMI configuration and determines the default status. The default reset value 0 is for the case where CPU0LS is disabled in UCB BMI configuration."]
    #[inline(always)]
    pub fn cpu1ls(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Pmstat0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Pmstat0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2LS Status   CPU2LS. This bit field reflects the current status of CPU2 Lockstep Checker Core.The activation of the Lockstep is configured in UCB BMI configuration and determines the default status. The default reset value 0 is for the case where CPU0LS is disabled in UCB BMI configuration."]
    #[inline(always)]
    pub fn cpu2ls(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Pmstat0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Pmstat0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3LS Status   CPU3LS. This bit field reflects the current status of CPU3 Lockstep Checker Core.The activation of the Lockstep is configured in UCB BMI configuration and determines the default status. The default reset value 0 is for the case where CPU0LS is disabled in UCB BMI configuration."]
    #[inline(always)]
    pub fn cpu3ls(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Pmstat0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Pmstat0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Pmstat0 {
    #[inline(always)]
    fn default() -> Pmstat0 {
        <crate::RegValueT<Pmstat0_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmswcr1_SPEC;
impl crate::sealed::RegSpec for Pmswcr1_SPEC {
    type DataType = u32;
}
#[doc = "Standby and Wake up Control Register 1\n resetvalue={Cold PowerOn Reset:0x1000000}"]
pub type Pmswcr1 = crate::RegValueT<Pmswcr1_SPEC>;

impl Pmswcr1 {
    #[doc = "CPU selection for Idle mode   CPUIDLSEL. This bit field allows a CPUx to issue Idle request to other CPUs in addition to itself. A request for Idle via PMCSRx.REQSLP 01 by CPUx will also trigger Idle requests to all other CPUs. All other CPUIDLSEL bit combinations are reserved."]
    #[inline(always)]
    pub fn cpuidlsel(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Pmswcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Pmswcr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Idle Request Acknowledge Sequence Disable   IRADIS. This bit enables SCU Idle Request Acknowledge sequence to all modules on Standby entry. IRADIS bit has no effect incase of Standby entry triggered via PWRWKEN register bit. This bit shall be set before Standby entry to disable Idle request acknowledge sequence so that standby request is not blocked by a pending reset idle request acknowledge sequence."]
    #[inline(always)]
    pub fn iradis(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Pmswcr1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Pmswcr1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU selection for Sleep and Standby mode   CPUSEL. All other CPUSEL bit combinations are reserved."]
    #[inline(always)]
    pub fn cpusel(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Pmswcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Pmswcr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Standby Entry Event configuration enable   STBYEVEN"]
    #[inline(always)]
    pub fn stbyeven(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Pmswcr1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, Pmswcr1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Standby Entry Event Configuration   STBYEV. All other bit combinations are reserved."]
    #[inline(always)]
    pub fn stbyev(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, Pmswcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x7,1,0,u8, Pmswcr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pmswcr1 {
    #[inline(always)]
    fn default() -> Pmswcr1 {
        <crate::RegValueT<Pmswcr1_SPEC> as RegisterValue<_>>::new(16777216)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmtrcsr0_SPEC;
impl crate::sealed::RegSpec for Pmtrcsr0_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Transition Control and Status Register 0\n resetvalue={Cold PowerOn Reset:0x0}"]
pub type Pmtrcsr0 = crate::RegValueT<Pmtrcsr0_SPEC>;

impl Pmtrcsr0 {
    #[doc = "Load Jump Timer Enable   LJTEN. This bit field enables the usage of load jump timer."]
    #[inline(always)]
    pub fn ljten(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Pmtrcsr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pmtrcsr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Load Jump Timer Overflow Enable   LJTOVEN. This bit field enables the update of LJTOV status bit on timer overflow        or time out."]
    #[inline(always)]
    pub fn ljtoven(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Pmtrcsr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pmtrcsr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Load Jump Timer Overflow Interrupt Enable   LJTOVIEN. This bit field enables the activation of interrupt on timer overflow or        time out."]
    #[inline(always)]
    pub fn ljtovien(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Pmtrcsr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Pmtrcsr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Load Jump Timer Start   LJTSTRT. This bit field starts Load jump timer. This is intended for test        purposes. The LJTSTRT remains set on a write and is cleared when LJTOV        bit is set if LJTOVEN bit is enabled."]
    #[inline(always)]
    pub fn ljtstrt(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Pmtrcsr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Pmtrcsr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Load Jump Timer Stop   LJTSTP. This bit field stops Load jump timer. This is intended for test        purposes. The LJTSTP remains set on a write and is to be explicitly        cleared by software. The LJTSTP stops the counter at the current value        and timer re starts from that value when LJTSTP is cleared and LJTSTRT        is set."]
    #[inline(always)]
    pub fn ljtstp(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Pmtrcsr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pmtrcsr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Load Jump Timer Clear   LJTCLR. This bit field clear Load jump timer count. This is intended for test        purposes. This bit resets LJT and clears LJTRUN if LJTEN bit is set."]
    #[inline(always)]
    pub fn ljtclr(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Pmtrcsr0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Pmtrcsr0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Droop Voltage Step vdroop step i    SDSTEP. This bit field defines the voltage offset for droop compensation on a        load jump to the EVRC setpoint value. The request is made via SCU PMTRCSR3 .VDROOPREQ  sd droop cntr i   on        an anticipated load jump with a voltage offset equal to the SDSTEP x        5  160 mV. The droop step is a positive offset if VDROOPREQ  160    160 01b and is a        negative offset if VDROOPREQ  160    160 10b and no offset is applied if        VDROOPREQ  160    160 00b. Maximum Droop   80  160 mV."]
    #[inline(always)]
    pub fn sdstep(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Pmtrcsr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Pmtrcsr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage Droop Timer Enable   VDTEN. This bit field enables the usage of Voltage Droop timer."]
    #[inline(always)]
    pub fn vdten(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Pmtrcsr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Pmtrcsr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage Droop Timer Overflow Enable   VDTOVEN. This bit field enables the update of VDTOV status bit on timer overflow        or time out."]
    #[inline(always)]
    pub fn vdtoven(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Pmtrcsr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,Pmtrcsr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage Droop Timer Overflow Interrupt Enable   VDTOVIEN. This bit field enables the activation of interrupt on timer overflow or        time out."]
    #[inline(always)]
    pub fn vdtovien(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Pmtrcsr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,Pmtrcsr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage Droop Timer Start   VDTSTRT. This bit field starts Voltage Droop timer. This is intended for test        purposes. The VDTSTRT remains set on a write and is cleared when VDTOV        bit is set if VDTOVEN bit is enabled."]
    #[inline(always)]
    pub fn vdtstrt(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Pmtrcsr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,Pmtrcsr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage Droop Timer Stop   VDTSTP. This bit field stops Voltage Droop timer. SCU cancels the droop request        via signal sd droop cntr i  160    160 00. This is intended for test purposes. The        VDTSTP remains set on a write and is to be explicitly cleared by        software. The VDTSTP stops the counter at the current value and timer        re starts from that value when VDTSTP is cleared and VDTSTRT is set."]
    #[inline(always)]
    pub fn vdtstp(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Pmtrcsr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,Pmtrcsr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage Droop Timer Clear   VDTCLR. This bit field clear Voltage Droop timer count. This is intended for        test purposes. This bit resets VDT and clears VDTRUN if VDTEN bit is set."]
    #[inline(always)]
    pub fn vdtclr(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Pmtrcsr0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, Pmtrcsr0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "EVRC Low Power Mode activation on a Sleep Request   LPSLPEN. PMS  This bit field enables the activation of LPM EVRC mode on a sleep request. If this bit is set   SCU sends lp enable signal on Sleep request to PMS which is OR ed with EVRSDCTRL2.EVRCMOD value and is provided to EVRC regulator. PMSLE  Reserved  no function  no LPM for SC DCDC EVRC ."]
    #[inline(always)]
    pub fn lpslpen(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Pmtrcsr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,Pmtrcsr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pmtrcsr0 {
    #[inline(always)]
    fn default() -> Pmtrcsr0 {
        <crate::RegValueT<Pmtrcsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmtrcsr1_SPEC;
impl crate::sealed::RegSpec for Pmtrcsr1_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Transition Control and Status Register 1\n resetvalue={Cold PowerOn Reset:0x0}"]
pub type Pmtrcsr1 = crate::RegValueT<Pmtrcsr1_SPEC>;

impl Pmtrcsr1 {
    #[doc = "Load Jump Timer Compare Setpoint Value   LJTCV. This bit field defines the compare setpoint value of Load Jump timer.        The compare event would lead to LJTOV bit being set and LJT interrupt        being raised. The LJTRUN status bit  LDJMPREQ bit and LJTCNT value is        reset to 0 on a compare event.   160   160 X  160 us is the compare value. LSB  160  1  160 us. Total range  160    160 65.5  160 ms"]
    #[inline(always)]
    pub fn ljtcv(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Pmtrcsr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Pmtrcsr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage Droop Timer Compare Setpoint Value   VDTCV. This bit field defines the compare setpoint value of Voltage Droop        timer. The compare event would lead to VDTOV bit being set and VDT        interrupt being raised. The VDTRUN status bit  VDROOPREQ bit and VDTCNT        value is reset to 0 on a compare event.   160   160 X  160 us is the compare value. LSB  160  1  160 us. Total range  160    160 1023  160 us"]
    #[inline(always)]
    pub fn vdtcv(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Pmtrcsr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Pmtrcsr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pmtrcsr1 {
    #[inline(always)]
    fn default() -> Pmtrcsr1 {
        <crate::RegValueT<Pmtrcsr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmtrcsr2_SPEC;
impl crate::sealed::RegSpec for Pmtrcsr2_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Transition Control and Status Register 2\n resetvalue={Cold PowerOn Reset:0x0}"]
pub type Pmtrcsr2 = crate::RegValueT<Pmtrcsr2_SPEC>;

impl Pmtrcsr2 {
    #[doc = "Load Jump Request   LDJMPREQ. This bit requests a Load Jump consequently leading to Load Jump Timer        start and LJTRUN bit being set if        LJTEN 1. The request is not taken if LJTRUN bit is already in set state        and LJT is currently running. The request is not taken if VDTRUN bit is        already in set state and VDT is currently running. The request is also        not taken if  LJTOV bit is set AND LJTOVEN bit is enabled . The request        is also not taken if  VDTOV bit is set AND VDTOVEN bit is enabled . The        LDJMPREQ bit is cleared on a compare overflow. All other bit combinations are reserved."]
    #[inline(always)]
    pub fn ldjmpreq(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Pmtrcsr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Pmtrcsr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Load Jump Timer Run Status   LJTRUN. This status bit indicates that the Load Jump timer is currently running        and a Load Jump is currently taking place. The LJTRUN bit is cleared on        a compare overflow. All other bit combinations are reserved."]
    #[inline(always)]
    pub fn ljtrun(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Pmtrcsr2_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Pmtrcsr2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Load Jump Timer Overflow Status   LJTOV. This status bit indicates that the Load Jump timer compare match has        happened. if LJTOVEN bit is enabled  then LJTOV can only be cleared        explicitly via LJTOVCLR bit. if LJTOVEN bit is disabled  LJTOV is cleared on a taken Load Jump        Request  A new Load Jump request is        taken only if both LJT  amp  VDT are not currently running and no active        Load Jump request is being processed  . LJTOV being set will lead        to an interrupt if LJTOVIEN is enabled."]
    #[inline(always)]
    pub fn ljtov(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Pmtrcsr2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Pmtrcsr2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Load Jump Timer Overflow Status Clear   LJTOVCLR. This bit clears LJTOV status bit and sets VDROOPREQ and LDJMPREQ to 0 if        LJTOVEN bit is enabled. This bit always reads as 0."]
    #[inline(always)]
    pub fn ljtovclr(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Pmtrcsr2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, Pmtrcsr2_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Load Jump Timer Value   LJTCNT. This bit field reflects the current Load Jump timer value. LJTCNT value is cleared on timer overflow and on a taken Load Jump Request X us is the compare value. LSB  1 us. Total range   65.5 ms"]
    #[inline(always)]
    pub fn ljtcnt(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Pmtrcsr2_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Pmtrcsr2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pmtrcsr2 {
    #[inline(always)]
    fn default() -> Pmtrcsr2 {
        <crate::RegValueT<Pmtrcsr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmtrcsr3_SPEC;
impl crate::sealed::RegSpec for Pmtrcsr3_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Transition Control and Status Register 3\n resetvalue={Cold PowerOn Reset:0x0}"]
pub type Pmtrcsr3 = crate::RegValueT<Pmtrcsr3_SPEC>;

impl Pmtrcsr3 {
    #[doc = "Voltage Droop Request   VDROOPREQ. This bit requests a Voltage Droop consequently leading to Voltage Droop        Timer start and VDTRUN bit being set if        VDTEN 1 . The request is not taken if VDTRUN bit is already in        set state and VDT is currently running. The request is also not taken if         VDTOV bit is set AND VDTOVEN bit is enabled . The droop step is a        positive offset if sd droop cntr i  160    160 01 and is a negative offset if        sd droop cntr i  160    160 10 and no offset is applied if sd droop cntr i  160    160 00        and is applied immediately."]
    #[inline(always)]
    pub fn vdroopreq(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Pmtrcsr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Pmtrcsr3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage Droop Timer Run Status   VDTRUN. This status bit indicates that the Voltage Droop timer is currently running and a Voltage Droop is currently taking place. The VDTRUN bit is cleared on a compare overflow. All other bit combinations are reserved."]
    #[inline(always)]
    pub fn vdtrun(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Pmtrcsr3_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Pmtrcsr3_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Voltage Droop Timer Overflow Status   VDTOV. This status bit indicates that the Voltage Droop timer compare match has happened. if VDTOVEN bit is enabled  then VDTOV can only be cleared by explicitly via VDTOVCLR bit. if VDTOVEN bit is disabled  VDTOV is cleared on a taken Voltage Droop Request  A new Voltage Droop request is taken only if both LJT  amp  VDT are not currently running and no active Voltage Droop request is being processed  . VDTOV being set will lead to an interrupt if VDTOVIEN is enabled. Incase SDVOK is set by EVRC before VDT compare match  VDTOV bit is not set."]
    #[inline(always)]
    pub fn vdtov(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Pmtrcsr3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Pmtrcsr3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Voltage Droop Timer Overflow Status Clear   VDTOVCLR. This bit clears VDTOV status bit if VDTOVEN bit is enabled. If VDTOVEN bit is disabled  this bit has no effect. This bit always reads as 0."]
    #[inline(always)]
    pub fn vdtovclr(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Pmtrcsr3_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, Pmtrcsr3_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Voltage Droop Timer Value   VDTCNT. This bit field reflects the current Voltage Droop timer value. VDTCNT value is cleared on timer overflow and on a taken Voltage Droop Request.   X us is the compare value. LSB  1 us. Total range   65.5 ms"]
    #[inline(always)]
    pub fn vdtcnt(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Pmtrcsr3_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Pmtrcsr3_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pmtrcsr3 {
    #[inline(always)]
    fn default() -> Pmtrcsr3 {
        <crate::RegValueT<Pmtrcsr3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstcon_SPEC;
impl crate::sealed::RegSpec for Rstcon_SPEC {
    type DataType = u32;
}
#[doc = "Reset Configuration Register\n resetvalue={PowerOn Reset:0x282,PowerOn Reset:0x282}"]
pub type Rstcon = crate::RegValueT<Rstcon_SPEC>;

impl Rstcon {
    #[doc = "ESR0 Reset Request Trigger Reset Configuration   ESR0. This bit field defines which reset is generated by a reset request        trigger from ESR0 reset."]
    #[inline(always)]
    pub fn esr0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Rstcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Rstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ESR1 Reset Request Trigger Reset Configuration   ESR1. This bit field defines which reset is generated by a reset request        trigger from ESR1 reset."]
    #[inline(always)]
    pub fn esr1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Rstcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Rstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SMU Reset Request Trigger Reset Configuration   SMU. This bit field defines which reset is generated by a reset request        trigger from SMU reset."]
    #[inline(always)]
    pub fn smu(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Rstcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Rstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SW Reset Request Trigger Reset Configuration   SW. This bit field defines which reset is generated by a reset request        trigger from software reset."]
    #[inline(always)]
    pub fn sw(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Rstcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Rstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "STM0 Reset Request Trigger Reset Configuration   STM0. This bit field defines which reset is generated by a reset request        trigger from STM0 compare match reset."]
    #[inline(always)]
    pub fn stm0(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Rstcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Rstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "STM1 Reset Request Trigger Reset Configuration  If Product has STM1    STM1. This bit field defines which reset is generated by a reset request        trigger from STM1 compare match reset."]
    #[inline(always)]
    pub fn stm1(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Rstcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Rstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "STM2 Reset Request Trigger Reset Configuration  If Product has STM2    STM2. This bit field defines which reset is generated by a reset request        trigger from STM2 compare match reset."]
    #[inline(always)]
    pub fn stm2(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Rstcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Rstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Rstcon {
    #[inline(always)]
    fn default() -> Rstcon {
        <crate::RegValueT<Rstcon_SPEC> as RegisterValue<_>>::new(642)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstcon2_SPEC;
impl crate::sealed::RegSpec for Rstcon2_SPEC {
    type DataType = u32;
}
#[doc = "Additional Reset Control Register\n resetvalue={Cold PowerOn Reset:0x0,Cold PowerOn Reset:0x0}"]
pub type Rstcon2 = crate::RegValueT<Rstcon2_SPEC>;

impl Rstcon2 {
    #[doc = "Force Reset Timeout   FRTO"]
    #[inline(always)]
    pub fn frto(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Rstcon2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Rstcon2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Cold Reset Status   CLRC. This bit simultaneously clears the sticky status bits which may indicate any previous cold reset  i.e. RSTSTAT.STBYR  RSTSTAT.SWD  RSTSTAT.EVR33  RSTSTAT.EVRC  RSTSTAT.PORST  RSTSTAT.LBPORST and RSTSTAT.LBTERM ."]
    #[inline(always)]
    pub fn clrc(self) -> crate::common::RegisterFieldBool<1, 1, 0, Rstcon2_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Rstcon2_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU x Shutdown State Reached   CSSx. The state of CPU x before the last warm reset. If any bit is zero after an Application Reset  or higher  then it is possible that SRAM content could have been corrupted by the reset. For products with fewer CPUs  only the LSBs are active and unused upper bits will always read   x2018 1  x2019 . For products with no MCDS miniMCDS MCDSlight  the bit will always read  1 ."]
    #[inline(always)]
    pub fn cssx(
        self,
    ) -> crate::common::RegisterField<7, 0x3f, 1, 0, u8, Rstcon2_SPEC, crate::common::R> {
        crate::common::RegisterField::<7,0x3f,1,0,u8, Rstcon2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "User Information   USRINFO. User data register  Cleared only on Cold Power on reset . This may be used by an application to store information which must        survive all warm resets"]
    #[inline(always)]
    pub fn usrinfo(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Rstcon2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Rstcon2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Rstcon2 {
    #[inline(always)]
    fn default() -> Rstcon2 {
        <crate::RegValueT<Rstcon2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rststat_SPEC;
impl crate::sealed::RegSpec for Rststat_SPEC {
    type DataType = u32;
}
#[doc = "Reset Status Register\n resetvalue={Cold PowerOn Reset:0x3810000,Cold PowerOn Reset:0x13810000,Cold PowerOn Reset:0x10010000}"]
pub type Rststat = crate::RegValueT<Rststat_SPEC>;

impl Rststat {
    #[doc = "Reset Request Trigger Reset Status for ESR0   ESR0"]
    #[inline(always)]
    pub fn esr0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Rststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Rststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Request Trigger Reset Status for ESR1   ESR1"]
    #[inline(always)]
    pub fn esr1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Rststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Rststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Request Trigger Reset Status for SMU   SMU.  See SMU section for SMU trigger sources  including Watchdog Timers"]
    #[inline(always)]
    pub fn smu(self) -> crate::common::RegisterFieldBool<3, 1, 0, Rststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Rststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Request Trigger Reset Status for SW   SW"]
    #[inline(always)]
    pub fn sw(self) -> crate::common::RegisterFieldBool<4, 1, 0, Rststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Rststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Request Trigger Reset Status for STM0 Compare Match   STM0"]
    #[inline(always)]
    pub fn stm0(self) -> crate::common::RegisterFieldBool<5, 1, 0, Rststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Rststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Request Trigger Reset Status for STM1 Compare Match  If Product has STM1    STM1"]
    #[inline(always)]
    pub fn stm1(self) -> crate::common::RegisterFieldBool<6, 1, 0, Rststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Rststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Request Trigger Reset Status for STM2 Compare Match  If Product has STM2    STM2"]
    #[inline(always)]
    pub fn stm2(self) -> crate::common::RegisterFieldBool<7, 1, 0, Rststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Rststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Request Trigger Reset Status for PORST   PORST. This bit is also set if the bits CB0  CB1  and CB3 are set in parallel."]
    #[inline(always)]
    pub fn porst(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Rststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Rststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Request Trigger Reset Status for Cerberus System Reset   CB0"]
    #[inline(always)]
    pub fn cb0(self) -> crate::common::RegisterFieldBool<18, 1, 0, Rststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Rststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Request Trigger Reset Status for Cerberus Debug Reset   CB1"]
    #[inline(always)]
    pub fn cb1(self) -> crate::common::RegisterFieldBool<19, 1, 0, Rststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Rststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Request Trigger Reset Status for Cerberus Application Reset   CB3"]
    #[inline(always)]
    pub fn cb3(self) -> crate::common::RegisterFieldBool<20, 1, 0, Rststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Rststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Request Trigger Reset Status for EVRC   EVRC"]
    #[inline(always)]
    pub fn evrc(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Rststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Rststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Request Trigger Reset Status for EVR33   EVR33"]
    #[inline(always)]
    pub fn evr33(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Rststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Rststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Request Trigger Reset Status for Supply Watchdog  SWD    SWD. The Supply Watchdog trigger is described in Power Management Controller          8220 Supply Monitoring  8221  chapter"]
    #[inline(always)]
    pub fn swd(self) -> crate::common::RegisterFieldBool<25, 1, 0, Rststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Rststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Request Trigger Reset Status for HSM System Reset  HSM S    HSMS"]
    #[inline(always)]
    pub fn hsms(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Rststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Rststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Request Trigger Reset Status for HSM Application Reset  HSM A    HSMA"]
    #[inline(always)]
    pub fn hsma(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Rststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Rststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Reset Request Trigger Reset Status for Standby Regulator Watchdog  STBYR    STBYR"]
    #[inline(always)]
    pub fn stbyr(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Rststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Rststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "LBIST termination due to PORST. This bitfield indicates if the LBIST was early terminated due to the occurrence of a Power On Reset. If the status of this bitfield is 0  the application must still check the LBTERM to check if the LBIST was terminated properly. This bitfield is cleared when the RSTCON2.CLRC is set."]
    #[inline(always)]
    pub fn lbporst(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Rststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Rststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "LBIST was properly terminated. This bitfield indicates if the LBIST was terminated properly. This bitfield is cleared when the RSTCON2.CLRC is set."]
    #[inline(always)]
    pub fn lbterm(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Rststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Rststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Rststat {
    #[inline(always)]
    fn default() -> Rststat {
        <crate::RegValueT<Rststat_SPEC> as RegisterValue<_>>::new(65536)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seicon0_SPEC;
impl crate::sealed::RegSpec for Seicon0_SPEC {
    type DataType = u32;
}
#[doc = "Safety ENDINIT Control Register 0\n resetvalue={Application Reset:0x0FFFC000E}"]
pub type Seicon0 = crate::RegValueT<Seicon0_SPEC>;

impl Seicon0 {
    #[doc = "End of Initialization Control Bit   ENDINIT. The current value of ENDINIT is controlled by hardware. It is cleared        after a valid EndInit Password Access to SEICON0  and it is        automatically set again after a valid EndInit Modify Access to SEICON0.        During a write to SEICON0  the value written to this bit is only used        for the password protection mechanism and is not stored. This bit must be cleared during a Password Access to SEICON0  and set        during a Modify Access to SEICON0."]
    #[inline(always)]
    pub fn endinit(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Seicon0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Seicon0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "User Definable Safety ENDINIT Password Field   EPW. This bit field is written with an ENDINIT password value during a Modify        Access. This password is independent from the CPU WDT or WDTS passwords. A read from this bitfield returns this password  but bits  7 2  are        inverted  toggled  to ensure that a simple read write is not sufficient        to service the Safety ENDINIT Timeout Counter. This bit field must be written with its current contents during a        Password Access. The default ENDINIT password after Application Reset is 00000000111100 B"]
    #[inline(always)]
    pub fn epw(
        self,
    ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, Seicon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3fff,1,0,u16, Seicon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reload Value for the Safety  ENDINIT Timeout Counter   REL. The reload value for the Safety ENDINIT Timeout Counter is fixed. This        bitfield always reads as FFFCh and cannot be changed. This bit field must be written with its current contents during a        Password Access. During a Modify Access this bitfield may contain any        value and is ignored."]
    #[inline(always)]
    pub fn rel(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Seicon0_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Seicon0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Seicon0 {
    #[inline(always)]
    fn default() -> Seicon0 {
        <crate::RegValueT<Seicon0_SPEC> as RegisterValue<_>>::new(4294705166)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seicon1_SPEC;
impl crate::sealed::RegSpec for Seicon1_SPEC {
    type DataType = u32;
}
#[doc = "Safety ENDINIT Control Register 1\n resetvalue={Application Reset:0x0}"]
pub type Seicon1 = crate::RegValueT<Seicon1_SPEC>;

impl Seicon1 {
    #[doc = "Input Frequency Request Control   IR1 IR0. Bit IR0 and IR1 should be programmed together to determine the Safety        ENDINIT Timeout Counter frequency. These bits can only be modified when Safety ENDINIT  SE  is de asserted.        SEISR.IS0 and SEISR.IS1 are updated by these bits only when Safety        ENDINIT  SE  is re asserted. As long as an Safety ENDINIT is cleared         SEISR.IS0 and SEISR.IS1 control the current input frequency of the        Safety ENDINIT Timeout Timer. When Safety ENDINIT SE  is re asserted         SEISR.IS0 and SEISR.IS1 are updated with the new values of IR0 and IR1."]
    #[inline(always)]
    pub fn ir0(self) -> crate::common::RegisterFieldBool<2, 1, 0, Seicon1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Seicon1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable Request Control Bit   DR. This bit can only be modified when Safety ENDINIT  SE  is de asserted.        SEISR.DS is updated when Safety ENDINIT is re asserted. As long as        Safety ENDINIT is deasserted  bit SEISR.DS controls the current        enable disable status of the WDT. When Safety ENDINIT is re asserted         SEISR.DS is updated with the state of DR."]
    #[inline(always)]
    pub fn dr(self) -> crate::common::RegisterFieldBool<3, 1, 0, Seicon1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Seicon1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Frequency Request Control   IR1 IR0. Bit IR0 and IR1 should be programmed together to determine the Safety        ENDINIT Timeout Counter frequency. These bits can only be modified when Safety ENDINIT  SE  is de asserted.        SEISR.IS0 and SEISR.IS1 are updated by these bit only when Safety        ENDINIT  SE  is re asserted. As long as an ENDINIT is cleared  SEISR.IS0        and SEISR.IS1 control the current input frequency of the ENDINIT Timeout        Counter. When Safety ENDINIT SE  is re asserted  SEISR.IS0 and SEISR.IS1        is updated with the new values of IR0 and IR1."]
    #[inline(always)]
    pub fn ir1(self) -> crate::common::RegisterFieldBool<5, 1, 0, Seicon1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Seicon1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Seicon1 {
    #[inline(always)]
    fn default() -> Seicon1 {
        <crate::RegValueT<Seicon1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seisr_SPEC;
impl crate::sealed::RegSpec for Seisr_SPEC {
    type DataType = u32;
}
#[doc = "Safety ENDINIT Timeout Status Register\n resetvalue={Application Reset:0x0FFFC0000}"]
pub type Seisr = crate::RegValueT<Seisr_SPEC>;

impl Seisr {
    #[doc = "SEICON0 Access Error Status Flag   AE. This bit is set when an illegal Password Access or Modify Access to        register SEICON0 was attempted. This bit is only cleared on a valid        SEICON0.ENDINIT Modify Access"]
    #[inline(always)]
    pub fn ae(self) -> crate::common::RegisterFieldBool<0, 1, 0, Seisr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Seisr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SEI Timeout Overflow Error Status Flag   OE. This bit is set when SEISR.TIM overflows from FFFF H to FFFC H . This bit is only cleared on        a valid SEICON0 Modify Access."]
    #[inline(always)]
    pub fn oe(self) -> crate::common::RegisterFieldBool<1, 1, 0, Seisr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Seisr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SEI Timeout Input Clock Status   IS0  IS1. Bit IS0 and IS1 should be programmed together. These bits inidicate the        current Safety ENDINIT Timeout Counter clock period. They are updated        with the state of bits SEICON1.IR0 and SEICON1.IR1 after a valid SEICON0        Modify Access."]
    #[inline(always)]
    pub fn is0(self) -> crate::common::RegisterFieldBool<2, 1, 0, Seisr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Seisr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SEI Enable Disable Status Flag   DS"]
    #[inline(always)]
    pub fn ds(self) -> crate::common::RegisterFieldBool<3, 1, 0, Seisr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Seisr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SEI Time Out Mode Flag   TO"]
    #[inline(always)]
    pub fn to(self) -> crate::common::RegisterFieldBool<4, 1, 0, Seisr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Seisr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SEI Timeout Input Clock Status   IS0  IS1. Bit IS0 and IS1 should be programmed together. These bits inidicate the        current Safety ENDINIT Timeout Counter clock period . They are updated        with the state of bits SEICON1.IR0 and SEICON1.IR1 after a valid SEICON0        Modify Access."]
    #[inline(always)]
    pub fn is1(self) -> crate::common::RegisterFieldBool<5, 1, 0, Seisr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Seisr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer Value   TIM. Reflects the current content of the Safety EINDINIT Timeout Counter.  Only        bits 17 and 16 are implemented in SEISR. Others return   8216 1  8217"]
    #[inline(always)]
    pub fn tim(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Seisr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Seisr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Seisr {
    #[inline(always)]
    fn default() -> Seisr {
        <crate::RegValueT<Seisr_SPEC> as RegisterValue<_>>::new(4294705152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stcon_SPEC;
impl crate::sealed::RegSpec for Stcon_SPEC {
    type DataType = u32;
}
#[doc = "Start up Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Stcon = crate::RegValueT<Stcon_SPEC>;

impl Stcon {
    #[doc = "Set Flash Config. Sector Access Enable   SFCBAE. Setting this bit sets bit STSTAT.FCBAE. Reading this bit returns always        a zero. If bits SFCBAE and CFCBAE are both set during the same access then bit          STSTAT.FCBAE is set."]
    #[inline(always)]
    pub fn sfcbae(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Stcon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, Stcon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Flash Config. Sector Access Enable   CFCBAE. Setting this bit clears bit STSTAT.FCBAE. Reading this bit returns        always a zero."]
    #[inline(always)]
    pub fn cfcbae(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Stcon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, Stcon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Start up Protection Setting   STP. This bit will be always set by FW and can t be reset. This bit is also cleared by an Application Reset. STP is automatically set when a shutdown trap occurs. The start up protection for the system generated out of this bit can be overruled  deactivated  if bit RTID.RT14 is set AND signal tcu nostp request from the TCU is asserted."]
    #[inline(always)]
    pub fn stp(self) -> crate::common::RegisterFieldBool<15, 1, 0, Stcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Stcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Stcon {
    #[inline(always)]
    fn default() -> Stcon {
        <crate::RegValueT<Stcon_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stmem1_SPEC;
impl crate::sealed::RegSpec for Stmem1_SPEC {
    type DataType = u32;
}
#[doc = "Start up Memory Register 1\n resetvalue={PowerOn Reset:0x0}"]
pub type Stmem1 = crate::RegValueT<Stmem1_SPEC>;

impl Stmem1 {
    #[doc = "Memory   MEM. This register is used by the start up software as memory."]
    #[inline(always)]
    pub fn mem(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Stmem1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Stmem1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Stmem1 {
    #[inline(always)]
    fn default() -> Stmem1 {
        <crate::RegValueT<Stmem1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stmem2_SPEC;
impl crate::sealed::RegSpec for Stmem2_SPEC {
    type DataType = u32;
}
#[doc = "Start up Memory Register 2\n resetvalue={System Reset:0x0}"]
pub type Stmem2 = crate::RegValueT<Stmem2_SPEC>;

impl Stmem2 {
    #[doc = "Memory   MEM. This register is used by the start up software as memory."]
    #[inline(always)]
    pub fn mem(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Stmem2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Stmem2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Stmem2 {
    #[inline(always)]
    fn default() -> Stmem2 {
        <crate::RegValueT<Stmem2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stmem3_SPEC;
impl crate::sealed::RegSpec for Stmem3_SPEC {
    type DataType = u32;
}
#[doc = "Start up Memory Register 3\n resetvalue={Application Reset:0x0}"]
pub type Stmem3 = crate::RegValueT<Stmem3_SPEC>;

impl Stmem3 {
    #[doc = "Memory   MEM. This register is used by the start up software as memory."]
    #[inline(always)]
    pub fn mem(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Stmem3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Stmem3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Stmem3 {
    #[inline(always)]
    fn default() -> Stmem3 {
        <crate::RegValueT<Stmem3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stmem4_SPEC;
impl crate::sealed::RegSpec for Stmem4_SPEC {
    type DataType = u32;
}
#[doc = "Start up Memory Register 4\n resetvalue={Cold PowerOn Reset:0x0}"]
pub type Stmem4 = crate::RegValueT<Stmem4_SPEC>;

impl Stmem4 {
    #[doc = "Memory   MEM. This register is used by the start up software as memory."]
    #[inline(always)]
    pub fn mem(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Stmem4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Stmem4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Stmem4 {
    #[inline(always)]
    fn default() -> Stmem4 {
        <crate::RegValueT<Stmem4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stmem5_SPEC;
impl crate::sealed::RegSpec for Stmem5_SPEC {
    type DataType = u32;
}
#[doc = "Start up Memory Register 5\n resetvalue={PowerOn Reset:0x0}"]
pub type Stmem5 = crate::RegValueT<Stmem5_SPEC>;

impl Stmem5 {
    #[doc = "Memory   MEM. This register is used by the start up software as memory."]
    #[inline(always)]
    pub fn mem(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Stmem5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Stmem5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Stmem5 {
    #[inline(always)]
    fn default() -> Stmem5 {
        <crate::RegValueT<Stmem5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stmem6_SPEC;
impl crate::sealed::RegSpec for Stmem6_SPEC {
    type DataType = u32;
}
#[doc = "Start up Memory Register 6\n resetvalue={System Reset:0x0}"]
pub type Stmem6 = crate::RegValueT<Stmem6_SPEC>;

impl Stmem6 {
    #[doc = "Memory   MEM. This register is used by the start up software as memory."]
    #[inline(always)]
    pub fn mem(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Stmem6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Stmem6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Stmem6 {
    #[inline(always)]
    fn default() -> Stmem6 {
        <crate::RegValueT<Stmem6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ststat_SPEC;
impl crate::sealed::RegSpec for Ststat_SPEC {
    type DataType = u32;
}
#[doc = "Start up Status Register\n resetvalue={PowerOn Reset:0x08000}"]
pub type Ststat = crate::RegValueT<Ststat_SPEC>;

impl Ststat {
    #[doc = "Hardware Configuration Setting   HWCFG. This bit field contains the value that is used by the boot software. This bit field is updated in case of an Application Reset with the content by register SWRSTCON.SWCFG if bit SWRSTCON.SWBOOT AND RSTSTAT.SW are set. This bit field is updated in case of an Application Reset with the content of the latches of pins P14.2 P14.5  P10.5  P10.6 if bit SWRSTCON.SWBOOT OR RSTSTAT.SW are cleared and bit STSTAT.LUDIS is cleared. This bit field is left unchanged in case of an Application Reset and is not updated with the content of the latches of pins P14.2 P14.5  P10.5  P10.6 if bit SWRSTCON.SWBOOT OR RSTSTAT.SW are cleared and bit STSTAT.LUDIS is set. The observed reset value after boot depends upon the state of the HWCFG pins"]
    #[inline(always)]
    pub fn hwcfg(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Ststat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Ststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Firmware Test Setting   FTM. This bit field contains the value that is used by the boot software in        Test Mode. This Bit field is updated in Test Mode by the TCU. In        Normal Mode this bit field is updated with 0000000 B and should be ignored by the boot software."]
    #[inline(always)]
    pub fn ftm(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, Ststat_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x7f,1,0,u8, Ststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "MODE   MODE. This bit indicates if the Test Mode is entered or not."]
    #[inline(always)]
    pub fn mode(self) -> crate::common::RegisterFieldBool<15, 1, 0, Ststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Flash Config. Sector Access Enable   FCBAE. This bit can be cleared by setting bit STCON.CFCBAE. This bit can be set by setting bit STCON.SFCBAE. Reset value of this bit is 0"]
    #[inline(always)]
    pub fn fcbae(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Ststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Latch Update Disable   LUDIS. This bit can be set by setting bit SYSCON.SETLUDIS. Reset value of this bit is 0"]
    #[inline(always)]
    pub fn ludis(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Ststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TRSTL Status   TRSTL. This bit simply displays the value of TRST ."]
    #[inline(always)]
    pub fn trstl(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Ststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Single Pin DAP Mode Enable   SPDEN"]
    #[inline(always)]
    pub fn spden(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Ststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RAM Content Security Integrity   RAMINT. In normal operation this bit can be set or cleared by the application         via SYSCON.RAMINTM . If a test boot mode is entered  the bit is        automatically cleared  and cannot be set again in test mode  because the        content may have been altered This bit is reset only by a cold power on reset."]
    #[inline(always)]
    pub fn ramint(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Ststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ststat {
    #[inline(always)]
    fn default() -> Ststat {
        <crate::RegValueT<Ststat_SPEC> as RegisterValue<_>>::new(32768)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swapctrl_SPEC;
impl crate::sealed::RegSpec for Swapctrl_SPEC {
    type DataType = u32;
}
#[doc = "Address Map Control Register\n resetvalue={System Reset:0x1}"]
pub type Swapctrl = crate::RegValueT<Swapctrl_SPEC>;

impl Swapctrl {
    #[doc = "Address Configuration. Configures the currently used address map  standard alternate selection"]
    #[inline(always)]
    pub fn addrcfg(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Swapctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Swapctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Spare address configuration registers. Spare read write bits"]
    #[inline(always)]
    pub fn spare(
        self,
    ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, Swapctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3fff,1,0,u16, Swapctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Swapctrl {
    #[inline(always)]
    fn default() -> Swapctrl {
        <crate::RegValueT<Swapctrl_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swrstcon_SPEC;
impl crate::sealed::RegSpec for Swrstcon_SPEC {
    type DataType = u32;
}
#[doc = "Software Reset Configuration Register\n resetvalue={PowerOn Reset:0x0,PowerOn Reset:0x0}"]
pub type Swrstcon = crate::RegValueT<Swrstcon_SPEC>;

impl Swrstcon {
    #[doc = "Software Reset Request   SWRSTREQ. This bit is automatically cleared and read always as zero."]
    #[inline(always)]
    pub fn swrstreq(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Swrstcon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Swrstcon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Swrstcon {
    #[inline(always)]
    fn default() -> Swrstcon {
        <crate::RegValueT<Swrstcon_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscon_SPEC;
impl crate::sealed::RegSpec for Syscon_SPEC {
    type DataType = u32;
}
#[doc = "System Control Register\n resetvalue={System Reset:0x0}"]
pub type Syscon = crate::RegValueT<Syscon_SPEC>;

impl Syscon {
    #[doc = "Capture Compare Trigger 0   CCTRIG0. This bit is used to trigger the Synchronous Start feature of the CCU6."]
    #[inline(always)]
    pub fn cctrig0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Syscon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Syscon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "RAM Integrity Modify   RAMINTM"]
    #[inline(always)]
    pub fn ramintm(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Syscon_SPEC, crate::common::W> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Syscon_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Latch Update Disable   SETLUDIS. Setting this bit sets bit STSTAT.LUDIS. Clearing this bit has no effect. This bit reads always as zero."]
    #[inline(always)]
    pub fn setludis(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Syscon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Syscon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable DXCPL   DDC. Drives signal to TCU"]
    #[inline(always)]
    pub fn ddc(self) -> crate::common::RegisterFieldBool<8, 1, 0, Syscon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Syscon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Syscon {
    #[inline(always)]
    fn default() -> Syscon {
        <crate::RegValueT<Syscon_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syspllcon0_SPEC;
impl crate::sealed::RegSpec for Syspllcon0_SPEC {
    type DataType = u32;
}
#[doc = "System PLL Configuration 0 Register\n resetvalue={System Reset:0x40003A00}"]
pub type Syspllcon0 = crate::RegValueT<Syspllcon0_SPEC>;

impl Syspllcon0 {
    #[doc = "Modulation Enable   MODEN. This bit controls the activation of the frequency modulation of the        System PLL."]
    #[inline(always)]
    pub fn moden(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Syspllcon0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Syspllcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N Divider Value   NDIV. The value the N Divider operates with is NDIV 1."]
    #[inline(always)]
    pub fn ndiv(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, Syspllcon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8, Syspllcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "System PLL Power Saving Mode   PLLPWD. If the PLL has been powered down and is getting re enabled via PLLPWD   1  a wait period of 1 ms has to be applied until it is stable without jitter."]
    #[inline(always)]
    pub fn pllpwd(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Syspllcon0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Syspllcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Restart DCO Lock Detection   RESLD. Setting this bit will clear bit SYSPLLSTAT.LOCK and restart the DCO lock detection. Reading this bit returns always a zero."]
    #[inline(always)]
    pub fn resld(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Syspllcon0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18,1,0,Syspllcon0_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "P Divider Value   PDIV. The value the P Divider operates with is PDIV 1."]
    #[inline(always)]
    pub fn pdiv(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Syspllcon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Syspllcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Selection   INSEL. This bit field defines as clock source for the two PLLs  System       PLL and Peripheral PLL ."]
    #[inline(always)]
    pub fn insel(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, Syspllcon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, Syspllcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Syspllcon0 {
    #[inline(always)]
    fn default() -> Syspllcon0 {
        <crate::RegValueT<Syspllcon0_SPEC> as RegisterValue<_>>::new(1073756672)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syspllcon1_SPEC;
impl crate::sealed::RegSpec for Syspllcon1_SPEC {
    type DataType = u32;
}
#[doc = "System PLL Configuration 1 Register\n resetvalue={System Reset:0x5}"]
pub type Syspllcon1 = crate::RegValueT<Syspllcon1_SPEC>;

impl Syspllcon1 {
    #[doc = "K2 Divider Value   K2DIV. The value the K2 Divider operates with is K2DIV 1. While SYSPLLSTAT.K2RDY   0  K2DIV is locked."]
    #[inline(always)]
    pub fn k2div(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Syspllcon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Syspllcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Syspllcon1 {
    #[inline(always)]
    fn default() -> Syspllcon1 {
        <crate::RegValueT<Syspllcon1_SPEC> as RegisterValue<_>>::new(5)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syspllcon2_SPEC;
impl crate::sealed::RegSpec for Syspllcon2_SPEC {
    type DataType = u32;
}
#[doc = "System PLL Configuration 2 Register\n resetvalue={System Reset:0x6000}"]
pub type Syspllcon2 = crate::RegValueT<Syspllcon2_SPEC>;

impl Syspllcon2 {
    #[doc = "Modulation Configuration   MODCFG. This bit field defines the modulation. MODCFG 9 0  defines the modulation amplitude. Bits MODCFG 9 5  are treated as integer part and bits MODCFG 4 0  as fractional part. Bits MODCFG 15 10  have to be configured with the following setting  0x111101 B ."]
    #[inline(always)]
    pub fn modcfg(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Syspllcon2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Syspllcon2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Syspllcon2 {
    #[inline(always)]
    fn default() -> Syspllcon2 {
        <crate::RegValueT<Syspllcon2_SPEC> as RegisterValue<_>>::new(24576)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syspllstat_SPEC;
impl crate::sealed::RegSpec for Syspllstat_SPEC {
    type DataType = u32;
}
#[doc = "System PLL Status Register\n resetvalue={System Reset:0x2,System Reset:0x2}"]
pub type Syspllstat = crate::RegValueT<Syspllstat_SPEC>;

impl Syspllstat {
    #[doc = "System PLL Power saving Mode Status   PWDSTAT"]
    #[inline(always)]
    pub fn pwdstat(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Syspllstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,Syspllstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "System PLL Lock Status   LOCK. In case of a loss of lock  the f DCO is kept on the previous constant frequency."]
    #[inline(always)]
    pub fn lock(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Syspllstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,Syspllstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "K2 Divider Ready Status   K2RDY. This bit indicates whether the K2 divider operates on the configured value. This is of interest when the SYSPLLCON1.K2DIV value is changed. The PLL must be enabled and clocked to set the K2RDY field."]
    #[inline(always)]
    pub fn k2rdy(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Syspllstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,Syspllstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Modulation Run   MODRUN. This bit indicates if the frequency modulation of the System PLL is activated or not."]
    #[inline(always)]
    pub fn modrun(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Syspllstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,Syspllstat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Syspllstat {
    #[inline(always)]
    fn default() -> Syspllstat {
        <crate::RegValueT<Syspllstat_SPEC> as RegisterValue<_>>::new(2)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trapclr_SPEC;
impl crate::sealed::RegSpec for Trapclr_SPEC {
    type DataType = u32;
}
#[doc = "Trap Clear Register\n resetvalue={System Reset:0x0}"]
pub type Trapclr = crate::RegValueT<Trapclr_SPEC>;

impl Trapclr {
    #[doc = "Clear Trap Request Flag ESR0T   ESR0T. Setting this bit clears bit TRAPSTAT.ESR0T. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn esr0t(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Trapclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Trapclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Trap Request Flag ESR1T   ESR1T. Setting this bit clears bit TRAPSTAT.ESR1T. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn esr1t(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Trapclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Trapclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Trap Request Flag TRAP2   TRAP2. Setting this bit clears bit TRAPSTAT.TRAP2. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn trap2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Trapclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Trapclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Trap Request Flag SMUT   SMUT. Setting this bit clears bit TRAPSTAT.SMUT. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn smut(self) -> crate::common::RegisterFieldBool<3, 1, 0, Trapclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Trapclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Trapclr {
    #[inline(always)]
    fn default() -> Trapclr {
        <crate::RegValueT<Trapclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trapdis0_SPEC;
impl crate::sealed::RegSpec for Trapdis0_SPEC {
    type DataType = u32;
}
#[doc = "Trap Disable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Trapdis0 = crate::RegValueT<Trapdis0_SPEC>;

impl Trapdis0 {
    #[doc = "Disable Trap Request ESR0T on CPU0   CPU0ESR0T"]
    #[inline(always)]
    pub fn cpu0esr0t(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Trapdis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Trapdis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable Trap Request ESR1T on CPU0   CPU0ESR1T"]
    #[inline(always)]
    pub fn cpu0esr1t(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Trapdis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Trapdis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable Trap Request TRAP2T on CPU0   CPU0TRAP2T"]
    #[inline(always)]
    pub fn cpu0trap2t(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Trapdis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Trapdis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable Trap Request SMUT on CPU0   CPU0SMUT"]
    #[inline(always)]
    pub fn cpu0smut(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Trapdis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Trapdis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable Trap Request ESR0T on CPU1  If product has CPU1    CPU1ESR0T"]
    #[inline(always)]
    pub fn cpu1esr0t(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Trapdis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Trapdis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable Trap Request ESR1T on CPU1  If product has CPU1    CPU1ESR1T"]
    #[inline(always)]
    pub fn cpu1esr1t(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Trapdis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Trapdis0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable Trap Request TRAP2T on CPU1  If product has CPU1    CPU1TRAP2T"]
    #[inline(always)]
    pub fn cpu1trap2t(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Trapdis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Trapdis0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable Trap Request SMUT on CPU1  If product has CPU1    CPU1SMUT"]
    #[inline(always)]
    pub fn cpu1smut(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Trapdis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Trapdis0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable Trap Request ESR0T on CPU2  If product has CPU2    CPU2ESR0T"]
    #[inline(always)]
    pub fn cpu2esr0t(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Trapdis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Trapdis0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable Trap Request ESR1T on CPU2  If product has CPU2    CPU2ESR1T"]
    #[inline(always)]
    pub fn cpu2esr1t(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Trapdis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,Trapdis0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable Trap Request TRAP2T on CPU2  If product has CPU2    CPU2TRAP2T"]
    #[inline(always)]
    pub fn cpu2trap2t(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Trapdis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,Trapdis0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable Trap Request SMUT on CPU2  If product has CPU2    CPU2SMUT"]
    #[inline(always)]
    pub fn cpu2smut(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Trapdis0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,Trapdis0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Trapdis0 {
    #[inline(always)]
    fn default() -> Trapdis0 {
        <crate::RegValueT<Trapdis0_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trapset_SPEC;
impl crate::sealed::RegSpec for Trapset_SPEC {
    type DataType = u32;
}
#[doc = "Trap Set Register\n resetvalue={System Reset:0x0}"]
pub type Trapset = crate::RegValueT<Trapset_SPEC>;

impl Trapset {
    #[doc = "Set Trap Request Flag ESR0T   ESR0T. Setting this bit sets bit TRAPSTAT.ESR0T. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn esr0t(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Trapset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Trapset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Trap Request Flag ESR1T   ESR1T. Setting this bit sets bit TRAPSTAT.ESR1T. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn esr1t(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Trapset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Trapset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Trap Request Flag TRAP2   TRAP2. Setting this bit sets bit TRAPSTAT.TRAP2. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn trap2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Trapset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Trapset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Trap Request Flag SMUT   SMUT. Setting this bit sets bit TRAPSTAT.SMUT. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn smut(self) -> crate::common::RegisterFieldBool<3, 1, 0, Trapset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Trapset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Trapset {
    #[inline(always)]
    fn default() -> Trapset {
        <crate::RegValueT<Trapset_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trapstat_SPEC;
impl crate::sealed::RegSpec for Trapstat_SPEC {
    type DataType = u32;
}
#[doc = "Trap Status Register\n resetvalue={System Reset:0x0}"]
pub type Trapstat = crate::RegValueT<Trapstat_SPEC>;

impl Trapstat {
    #[doc = "ESR0 Trap Request Flag   ESR0T. This bit is set if an ESR0 event is triggered. This bit can be cleared by setting bit TRAPCLR.ESR0T. This bit can be set by setting bit TRAPSET.ESR0T. Observed reset value after boot will depend upon ARI mode because of          ESR pin transition."]
    #[inline(always)]
    pub fn esr0t(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Trapstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Trapstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ESR1 Trap Request Flag   ESR1T. This bit is set if an ESR1 event is triggered. This bit can be cleared by setting bit TRAPCLR.ESR1T. This bit can be set by setting bit TRAPSET.ESR1T. Reset value of this bit is 0"]
    #[inline(always)]
    pub fn esr1t(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Trapstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Trapstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Trap Bit 2 Request Flag   TRAP2. This bit can be cleared by setting bit TRAPCLR.TRAP2. This bit can be set by setting bit TRAPSET.TRAP2. Reset value of this bit is 0"]
    #[inline(always)]
    pub fn trap2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Trapstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Trapstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SMU Alarm Trap Request Flag   SMUT. This bit is set if an SMU Alarm is indicated. This bit can be cleared by setting bit TRAPCLR.SMUT. This bit can be set by setting bit TRAPSET.SMUT. Reset value of this bit is 0"]
    #[inline(always)]
    pub fn smut(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Trapstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Trapstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Trapstat {
    #[inline(always)]
    fn default() -> Trapstat {
        <crate::RegValueT<Trapstat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc = "ESRCFGx"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EsrcfGx(pub(super) *mut u8);
unsafe impl core::marker::Send for EsrcfGx {}
unsafe impl core::marker::Sync for EsrcfGx {}
impl EsrcfGx {
    #[doc = "ESR0 Input Configuration Register\n resetvalue={System Reset:0x100}"]
    #[inline(always)]
    pub const fn esrcfgx(&self) -> crate::common::Reg<esrcfgx::EsrcfGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod esrcfgx {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EsrcfGx_SPEC;
    impl crate::sealed::RegSpec for EsrcfGx_SPEC {
        type DataType = u32;
    }
    #[doc = "ESR0 Input Configuration Register\n resetvalue={System Reset:0x100}"]
    pub type EsrcfGx = crate::RegValueT<EsrcfGx_SPEC>;

    impl EsrcfGx {
        #[doc = "Edge Detection Control   EDCON. This bit field defines the edges that lead to an ESRx trigger of the        synchronous path."]
        #[inline(always)]
        pub fn edcon(
            self,
        ) -> crate::common::RegisterField<7, 0x3, 1, 0, u8, EsrcfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<7,0x3,1,0,u8, EsrcfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for EsrcfGx {
        #[inline(always)]
        fn default() -> EsrcfGx {
            <crate::RegValueT<EsrcfGx_SPEC> as RegisterValue<_>>::new(256)
        }
    }
}
#[doc = "CPU watchdogs"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdtcpu(pub(super) *mut u8);
unsafe impl core::marker::Send for Wdtcpu {}
unsafe impl core::marker::Sync for Wdtcpu {}
impl Wdtcpu {
    #[doc = "CPU0 WDT Control Register 0\n resetvalue={Application Reset:0x0FFFC000E,Application Reset:0x0FFFC000F}"]
    #[inline(always)]
    pub const fn wdtcpuycon0(
        &self,
    ) -> crate::common::Reg<wdtcpu::WdtcpUyCon0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "CPU0 WDT Control Register 1\n resetvalue={Application Reset:0x0,Application Reset:0x08}"]
    #[inline(always)]
    pub const fn wdtcpuycon1(
        &self,
    ) -> crate::common::Reg<wdtcpu::WdtcpUyCon1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "CPU0 WDT Status Register\n resetvalue={Application Reset:0x0FFFC0010,Application Reset:0x0FFFC0008}"]
    #[inline(always)]
    pub const fn wdtcpuysr(&self) -> crate::common::Reg<wdtcpu::WdtcpUySr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
pub mod wdtcpu {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdtcpUyCon0_SPEC;
    impl crate::sealed::RegSpec for WdtcpUyCon0_SPEC {
        type DataType = u32;
    }
    #[doc = "CPU0 WDT Control Register 0\n resetvalue={Application Reset:0x0FFFC000E,Application Reset:0x0FFFC000F}"]
    pub type WdtcpUyCon0 = crate::RegValueT<WdtcpUyCon0_SPEC>;

    impl WdtcpUyCon0 {
        #[doc = "End of Initialization Control Bit   ENDINIT. This bit must be written with a   8216 1  8217  during a Password Access or Check        Access  although this write is only used for the password protection        mechanism and is not stored . This bit must be written with the required        ENDINIT update value during a Modify Access. This bit is intended for accessing registers with   8220 CEy  8221  protection  y         CPU number . It may also be used to access registers with   8220 E  8221         protection  but the alternate register EICON0.ENDINIT is recommended for        this purpose so that the Watchdog Timer is not affected."]
        #[inline(always)]
        pub fn endinit(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, WdtcpUyCon0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<0,1,0,WdtcpUyCon0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Lock Bit to Control Access to WDTxCON0   LCK. The current value of LCK is controlled by hardware. It is cleared after        a valid Password Access to WDTxCON0 when WDTxSR.US is 0  or when        WDTxSR.US is 1 and the SMU is in RUN mode   and it is automatically set        again after a valid Modify Access to WDTxCON0. During a write to        WDTxCON0  the value written to this bit is only used for the        password protection mechanism and is not stored. This bit must be cleared during a Password Access to WDTxCON0  and set        during a Modify Access to WDTxCON0. A Check Access does not clear LCK."]
        #[inline(always)]
        pub fn lck(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, WdtcpUyCon0_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<1,1,0,WdtcpUyCon0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "User Definable Password Field for Access to WDTxCON0   PW. This bit field is written with an initial password value during a Modify        Access. A read from this bitfield returns this initial password  but bits  7 2         are inverted  toggled  to ensure that a simple read write is not        sufficient to service the WDTx.  This        also provides backward compatibility  If corresponding WDTxSR.PAS   0 then this bit field must be written with        its current contents during a Password Access or Check Access. If corresponding WDTxSR.PAS   1 then this bit field must be written with        the next password in the LFSR sequence during a Password Access or Check        Access The default password after Application Reset is 00000000111100 B"]
        #[inline(always)]
        pub fn pw(
            self,
        ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, WdtcpUyCon0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x3fff,1,0,u16, WdtcpUyCon0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Reload Value for the WDT  also Time Check Value    REL. The reload value can be changed during a Modify Access to WDTxCON0         Default after Application Reset        is FFFC H  .If the Watchdog Timer is        enabled and in Normal Timer Mode  it will start counting from this value        after a correct Watchdog service. A read from this bitfield always returns the current reload value. During a Password Access or a Check Access this bitfield may be used for        additional checks. Writes during such checks have no effect upon the        reload value. If corresponding WDTxSR.TCS 0 then this bit field must be written with        its current contents during a Password Access or Check Access. If corresponding WDTxSR.TCS 1 then this bit field must be written with        an inverted estimate of the current WDTxSR.TIM value during a Password        Access or Check Access."]
        #[inline(always)]
        pub fn rel(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, WdtcpUyCon0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, WdtcpUyCon0_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for WdtcpUyCon0 {
        #[inline(always)]
        fn default() -> WdtcpUyCon0 {
            <crate::RegValueT<WdtcpUyCon0_SPEC> as RegisterValue<_>>::new(4294705166)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdtcpUyCon1_SPEC;
    impl crate::sealed::RegSpec for WdtcpUyCon1_SPEC {
        type DataType = u32;
    }
    #[doc = "CPU0 WDT Control Register 1\n resetvalue={Application Reset:0x0,Application Reset:0x08}"]
    pub type WdtcpUyCon1 = crate::RegValueT<WdtcpUyCon1_SPEC>;

    impl WdtcpUyCon1 {
        #[doc = "Input Frequency Request Control   IR1 IR0. Bit IR0 and IR1 should be programmed together to determine the WDT timer        frequency. These bits can only be modified if the corresponding WDTxCON0.ENDINIT is        cleared. WDTxSR.IS0 and WDTxSR.IS1 are updated by these bit only when        ENDINIT is set again. As long as ENDINIT is cleared  WDTxSR.IS0 and        WDTxSR.IS1 controls the current input frequency of the Watchdog Timer.        When ENDINIT is set again  WDTxSR.IS0 andWDTxSR.IS1 are updated with the        values of IR0 and IR1."]
        #[inline(always)]
        pub fn ir0(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, WdtcpUyCon1_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<2,1,0,WdtcpUyCon1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Disable Request Control Bit   DR. This bit can only be modified if the corresponding WDTxCON0.ENDINIT is        cleared. WDTxSR.DS is updated when ENDINIT is set again. As long as the        ENDINIT is cleared  bit WDTxSR.DS controls the current enable disable        status of the WDTx. When the ENDINIT is set again with a Valid Modify        Access  WDTxSR.DS is updated with the state of DR."]
        #[inline(always)]
        pub fn dr(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, WdtcpUyCon1_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<3,1,0,WdtcpUyCon1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Input Frequency Request Control   IR1 IR0. Bit IR0 and IR1 should be programmed together to determine the WDTx        timer frequency These bits can only be modified if the corresponding WDTxCON0.ENDINIT is        cleared. WDTxSR.IS0 and WDTxSR.IS1 are updated by these bit only when        ENDINIT is set again. As long as ENDINIT is cleared  WDTxSR.IS0 and        WDTxSR.IS1 control the current input frequency of the Watchdog Timer.        When ENDINIT is set again  WDTxSR.IS0 and WDTxSR.IS1 are updated with        the values of IR0 and IR1."]
        #[inline(always)]
        pub fn ir1(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, WdtcpUyCon1_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<5,1,0,WdtcpUyCon1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Unlock Restriction Request Control Bit   UR. This bit can only be modified if the corresponding WDTxCON0.ENDINIT is        cleared. WDTxSR.US is updated when ENDINIT is set again. As long as the        ENDINIT is cleared  bit WDTxSR.US controls whether unlocking is possible        at all times or only when the SMU is not in the FAULT state. When the        ENDINIT is set again with a Valid Modify Access  WDTxSR.US is updated        with the state of UR."]
        #[inline(always)]
        pub fn ur(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, WdtcpUyCon1_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<6,1,0,WdtcpUyCon1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Password Auto sequence Request Bit   PAR. This bit can only be modified if the corresponding WDTxCON0.ENDINIT is        cleared. WDTxSR.PAS is updated when ENDINIT is set again. As long as the        ENDINIT is cleared  bit WDTxSR.PAS controls password sequencing. When        the ENDINIT is set again with a Valid Modify Access  WDTxSR.PAS is        updated with the state of PAR."]
        #[inline(always)]
        pub fn par(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, WdtcpUyCon1_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<7,1,0,WdtcpUyCon1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Counter Check Request Bit   TCR. This bit can only be modified if the corresponding WDTxCON0.ENDINIT is        cleared. WDTxSR.TCS is updated when ENDINIT is set again. As long as the        ENDINIT is cleared  bit WDTxSR.TCS controls whether counter check is        enabled. When the ENDINIT is set again with a Valid Modify Access         WDTxSR.TCS is updated with the state of TCR"]
        #[inline(always)]
        pub fn tcr(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, WdtcpUyCon1_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<8,1,0,WdtcpUyCon1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Timer Check Tolerance Request   TCTR. This bit can only be modified if the corresponding WDTxCON0.ENDINIT is        cleared. WDTxSR.TCT is updated when ENDINIT is set again. As long as the        ENDINIT is cleared  bit WDTxSR.TCT controls the tolerance of timer        checks. When the ENDINIT is set again with a Valid Modify Access         WDTxSR.TCT is updated with the state of TCTR"]
        #[inline(always)]
        pub fn tctr(
            self,
        ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, WdtcpUyCon1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<9,0x7f,1,0,u8, WdtcpUyCon1_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for WdtcpUyCon1 {
        #[inline(always)]
        fn default() -> WdtcpUyCon1 {
            <crate::RegValueT<WdtcpUyCon1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdtcpUySr_SPEC;
    impl crate::sealed::RegSpec for WdtcpUySr_SPEC {
        type DataType = u32;
    }
    #[doc = "CPU0 WDT Status Register\n resetvalue={Application Reset:0x0FFFC0010,Application Reset:0x0FFFC0008}"]
    pub type WdtcpUySr = crate::RegValueT<WdtcpUySr_SPEC>;

    impl WdtcpUySr {
        #[doc = "Watchdog Access Error Status Flag   AE. This bit is set when an illegal Password Access or Modify Access to        register WDTxCON0 was attempted. This bit is only cleared when        WDTxCON0.ENDINIT is set during a valid Modify Access"]
        #[inline(always)]
        pub fn ae(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, WdtcpUySr_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0,1,0,WdtcpUySr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Watchdog Overflow Error Status Flag   OE. This bit is set when the WDTx overflows from FFFF H to 0000 H . This bit is only cleared        when WDTxCON0.ENDINIT is set to 1 during a valid Modify Access."]
        #[inline(always)]
        pub fn oe(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, WdtcpUySr_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<1,1,0,WdtcpUySr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Watchdog Input Clock Status   IS1 IS0. Bit IS0 and IS1 should be programmed together. These bits indicate the        current WDTx clock rate. These bits are updated with the state of bits        WDTxCON1.IR0 and WDTxCON1.IR1 after WDTxCON0.ENDINIT is written with 1        during a valid Modify Access to register WDTxCON0."]
        #[inline(always)]
        pub fn is0(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, WdtcpUySr_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<2,1,0,WdtcpUySr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Watchdog Enable Disable Status Flag   DS. This bit is updated with the state of bit WDTxCON1.DR  after        WDTxCON0.ENDINIT is set during a Valid Modify Access to register        WDTxCON0  and it is cleared when Time Out mode is entered."]
        #[inline(always)]
        pub fn ds(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, WdtcpUySr_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<3,1,0,WdtcpUySr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Watchdog Time Out Mode Flag   TO. This bit is set when Time Out Mode is entered. It is automatically        cleared when Time Out Mode is left."]
        #[inline(always)]
        pub fn to(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, WdtcpUySr_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<4,1,0,WdtcpUySr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Watchdog Input Clock Status   IS1 IS0. Bit IS0 and IS1 should be programmed together. These bits indicate the        current WDTx clock rate. These bits are updated with the state of bits        WDTxCON1.IR0 and WDTxCON1.IR1 after WDTxCON0.ENDINIT is written with 1        during a valid Modify Access to register WDTxCON0."]
        #[inline(always)]
        pub fn is1(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, WdtcpUySr_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<5,1,0,WdtcpUySr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "SMU Unlock Restriction Status Flag   US. WDTxCON0.LCK will not be unlocked by a valid Password Access if this bit        is   8216 1  8217  and the SMU is not in the FAULT state"]
        #[inline(always)]
        pub fn us(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, WdtcpUySr_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<6,1,0,WdtcpUySr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Password Auto sequence Status Flag   PAS. This bit is updated with the state of bit WDTxCON1.PAR after        WDTxCON0.ENDINIT is written with 1 during a valid Modify Access to        register WDTxCON0."]
        #[inline(always)]
        pub fn pas(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, WdtcpUySr_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<7,1,0,WdtcpUySr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Timer Check Status Flag   TCS. This bit is updated with the state of bit WDTxCON1.TCR after        WDTxCON0.ENDINIT is written with 1 during a Valid Modify Access to        register WDTxCON0."]
        #[inline(always)]
        pub fn tcs(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, WdtcpUySr_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<8,1,0,WdtcpUySr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Timer Check Tolerance   TCT. This field determines the tolerance of the timer check during Password        or Check Access  See TCS .This bit is updated with the state of bit        WDTxCON1.TCTR after WDTxCON0.ENDINIT is written with 1 during a Valid        Modify Access to register WDTxCON0."]
        #[inline(always)]
        pub fn tct(
            self,
        ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, WdtcpUySr_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<9,0x7f,1,0,u8, WdtcpUySr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Timer Value   TIM. Reflects the current content of the WDTx."]
        #[inline(always)]
        pub fn tim(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, WdtcpUySr_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, WdtcpUySr_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for WdtcpUySr {
        #[inline(always)]
        fn default() -> WdtcpUySr {
            <crate::RegValueT<WdtcpUySr_SPEC> as RegisterValue<_>>::new(4294705152)
        }
    }
}
#[doc = "WDTS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdts(pub(super) *mut u8);
unsafe impl core::marker::Send for Wdts {}
unsafe impl core::marker::Sync for Wdts {}
impl Wdts {
    #[doc = "Safety WDT Control Register 0\n resetvalue={Application Reset:0x0FFFC000E}"]
    #[inline(always)]
    pub const fn wdtscon0(&self) -> crate::common::Reg<wdts::Wdtscon0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Safety WDT Control Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn wdtscon1(&self) -> crate::common::Reg<wdts::Wdtscon1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Safety WDT Status Register\n resetvalue={Application Reset:0x0FFFC0010}"]
    #[inline(always)]
    pub const fn wdtssr(&self) -> crate::common::Reg<wdts::Wdtssr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
pub mod wdts {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdtscon0_SPEC;
    impl crate::sealed::RegSpec for Wdtscon0_SPEC {
        type DataType = u32;
    }
    #[doc = "Safety WDT Control Register 0\n resetvalue={Application Reset:0x0FFFC000E}"]
    pub type Wdtscon0 = crate::RegValueT<Wdtscon0_SPEC>;

    impl Wdtscon0 {
        #[doc = "End of Initialization Control Bit   ENDINIT. This bit must be written with a   8216 1  8217  during a Password Access or Check        Access  although this write is only used for the password protection        mechanism and is not stored . This bit must be written with the required        ENDINIT update value during a Modify Access. This bit may be used to access registers with   8220 SE  8221  protection  but the        alternate register SEICON0.ENDINIT is recommended for this purpose so        that the Watchdog Timer is not affected."]
        #[inline(always)]
        pub fn endinit(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, Wdtscon0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,Wdtscon0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Lock Bit to Control Access to WDTxCON0   LCK. The current value of LCK is controlled by hardware. It is cleared after        a valid Password Access to WDTxCON0 when WDTxSR.US is 0  or when        WDTxSR.US is 1 and the SMU is in RUN mode   and it is automatically set        again after a valid Modify Access to WDTxCON0. During a write to        WDTxCON0  the value written to this bit is only used for the        password protection mechanism and is not stored. This bit must be cleared during a Password Access to WDTxCON0  and set        during a Modify Access to WDTxCON0. A Check Access does not clear LCK."]
        #[inline(always)]
        pub fn lck(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, Wdtscon0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,Wdtscon0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "User Definable Password Field for Access to WDTxCON0   PW. This bit field is written with an initial password value during a Modify        Access. A read from this bitfield returns this initial password  but bits  7 2         are inverted  toggled  to ensure that a simple read write is not        sufficient to service the WDTx.  This        also provides backward compatibility  If corresponding WDTxSR.PAS   0 then this bit field must be written with        its current contents during a Password Access or Check Access. If corresponding WDTxSR.PAS   1 then this bit field must be written with        the next password in the LFSR sequence during a Password Access or Check        Access The default password after Application Reset is 00000000111100 B"]
        #[inline(always)]
        pub fn pw(
            self,
        ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, Wdtscon0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x3fff,1,0,u16, Wdtscon0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Reload Value for the WDT  also Time Check Value    REL. The reload value can be changed during a Modify Access to WDTxCON0         Default after Application Reset        is FFFC H  .If the Watchdog Timer is        enabled and in Normal Timer Mode  it will start counting from this value        after a correct Watchdog service. A read from this bitfield always returns the current reload value. During a Password Access or a Check Access this bitfield may be used for        additional checks. Writes during such checks have no effect upon the        reload value. If corresponding WDTxSR.TCS 0 then this bit field must be written with        its current contents during a Password Access or Check Access. If corresponding WDTxSR.TCS 1 then this bit field must be written with        an inverted estimate of the current WDTxSR.TIM value during a Password        Access or Check Access."]
        #[inline(always)]
        pub fn rel(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Wdtscon0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, Wdtscon0_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Wdtscon0 {
        #[inline(always)]
        fn default() -> Wdtscon0 {
            <crate::RegValueT<Wdtscon0_SPEC> as RegisterValue<_>>::new(4294705166)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdtscon1_SPEC;
    impl crate::sealed::RegSpec for Wdtscon1_SPEC {
        type DataType = u32;
    }
    #[doc = "Safety WDT Control Register 1\n resetvalue={Application Reset:0x0}"]
    pub type Wdtscon1 = crate::RegValueT<Wdtscon1_SPEC>;

    impl Wdtscon1 {
        #[doc = "Clear Internal Reset Flag   CLRIRF. This bit is used to request a clear of the internal flag which indicates        whether a previous SMU reset has already been requested After modification  the internal flag is only cleared when Safety        Endinit  SE  is re asserted. As long as Safety ENDINIT SE  is not        asserted  the internal flag is unchanged and continues to determine the        response to a further SMU reset request. When Safety ENDINIT is        reasserted  the internal flag is cleared together with this bit."]
        #[inline(always)]
        pub fn clrirf(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, Wdtscon1_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,Wdtscon1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Input Frequency Request Control   IR1 IR0. Bit IR0 and IR1 should be programmed together to determine the WDTx        timer frequency. WDTxSR.IS0 and WDTxSR.IS1 are updated by these bits only when Safety        ENDINIT  SE  is re asserted. As long as Safety ENDINIT is de asserted         WDTxSR.IS0 and WDTxSR.IS1 control the current input frequency of the        Safety Watchdog Timer. When Safety ENDINIT is reasserted  WDTxSR.IS0 and        WDTxSR.IS1 are updated with the values of IR0 and IR1."]
        #[inline(always)]
        pub fn ir0(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, Wdtscon1_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,Wdtscon1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Disable Request Control Bit   DR. This bit can only be modified when Safety ENDINIT  SE  is de asserted.        WDTxSR.DS is updated when Safety ENDINIT is re asserted. As long as        Safety ENDINIT is de asserted  bit WDTxSR.DS controls the current        enable disable status of the WDTx. When Safety ENDINIT is reasserted         WDTxSR.DS is updated with the state of DR."]
        #[inline(always)]
        pub fn dr(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, Wdtscon1_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,Wdtscon1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Input Frequency Request Control   IR1 IR0. Bit IR0 and IR1 should be programmed together to determine the WDTx        timer frequency WDTxSR.IS0 and WDTxSR.IS1 are updated by these bits only when Safety        ENDINIT  SE  is re asserted. As long as Safety ENDINIT is de asserted         WDTxSR.IS0 and WDTxSR.IS1 control the current input frequency of the        Safety Watchdog Timer. When Safety ENDINIT is reasserted  WDTxSR.IS0 and        WDTxSR.IS1 are updated with the values of IR0 and IR1."]
        #[inline(always)]
        pub fn ir1(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, Wdtscon1_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,Wdtscon1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Unlock Restriction Request Control Bit   UR. This bit can only be modified when Safety ENDINIT  SE  is de asserted.        WDTxSR.US is updated when Safety ENDINIT is reasserted. As long as the        Safety ENDINIT is cleared  bit WDTxSR.US controls whether unlocking is        possible at all times or only when the SMU is not in the FAULT state.        When Safety ENDINIT is reasserted  WDTxSR.US is updated with the state        of UR."]
        #[inline(always)]
        pub fn ur(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, Wdtscon1_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,Wdtscon1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Password Auto sequence Request Bit   PAR. This bit can only be modified when Safety ENDINIT is de asserted.        WDTxSR.PAS is updated when Safety ENDINIT is reasserted. As long as        Safety ENDINIT is de asserted  bit WDTxSR.PAS controls password        sequencing. When Safety ENDINIT is reasserted  WDTxSR.PAS is updated        with the state of PAR."]
        #[inline(always)]
        pub fn par(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, Wdtscon1_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,Wdtscon1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Counter Check Request Bit   TCR. This bit can only be modified when Safety ENDINIT  SE  is de asserted.        WDTxSR.TCS is updated when Safety ENDINIT is re asserted. As long as        Safety ENDINIT is de asserted  bit WDTxSR.TCS controls whether counter        check is enabled. When Safety ENDINIT is reasserted  WDTxSR.TCS is        updated with the state of TCR"]
        #[inline(always)]
        pub fn tcr(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, Wdtscon1_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,Wdtscon1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Timer Check Tolerance Request   TCTR. This bit can only be modified when Safety ENDINIT is de asserted.        WDTxSR.TCT is updated when Safety ENDINIT is reasserted. As long as        Safety ENDINIT is de asserted  bit WDTxSR.TCT controls the tolerance of        timer checks. When Safety ENDINIT is re asserted  WDTxSR.TCT is updated        with the state of TCTR."]
        #[inline(always)]
        pub fn tctr(
            self,
        ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, Wdtscon1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<9,0x7f,1,0,u8, Wdtscon1_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Wdtscon1 {
        #[inline(always)]
        fn default() -> Wdtscon1 {
            <crate::RegValueT<Wdtscon1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdtssr_SPEC;
    impl crate::sealed::RegSpec for Wdtssr_SPEC {
        type DataType = u32;
    }
    #[doc = "Safety WDT Status Register\n resetvalue={Application Reset:0x0FFFC0010}"]
    pub type Wdtssr = crate::RegValueT<Wdtssr_SPEC>;

    impl Wdtssr {
        #[doc = "Watchdog Access Error Status Flag   AE. This bit is set when an illegal Password Access or Modify Access to        register WDTxCON0 was attempted. This bit is only cleared when        WDTxCON0.ENDINIT is set during a valid Modify Access"]
        #[inline(always)]
        pub fn ae(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, Wdtssr_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0,1,0,Wdtssr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Watchdog Overflow Error Status Flag   OE. This bit is set when the WDTx overflows from FFFF H to 0000 H . This bit is only cleared        when WDTxCON0.ENDINIT is set to 1 during a valid Modify Access."]
        #[inline(always)]
        pub fn oe(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, Wdtssr_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<1,1,0,Wdtssr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Watchdog Input Clock Status   IS1 IS0. Bit IS0 and IS1 should be programmed together. These bits indicate the        current WDTx clock rate. These bits are updated with the state of bits        WDTxCON1.IR0 and WDTxCON1.IR1 after WDTxCON0.ENDINIT is written with 1        during a valid Modify Access to register WDTxCON0."]
        #[inline(always)]
        pub fn is0(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, Wdtssr_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<2,1,0,Wdtssr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Watchdog Enable Disable Status Flag   DS. This bit is updated with the state of bit WDTxCON1.DR  after        WDTxCON0.ENDINIT is set during a Valid Modify Access to register        WDTxCON0  and it is cleared when Time Out mode is entered."]
        #[inline(always)]
        pub fn ds(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, Wdtssr_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<3,1,0,Wdtssr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Watchdog Time Out Mode Flag   TO. This bit is set when Time Out Mode is entered. It is automatically        cleared when Time Out Mode is left."]
        #[inline(always)]
        pub fn to(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, Wdtssr_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<4,1,0,Wdtssr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Watchdog Input Clock Status   IS1 IS0. Bit IS0 and IS1 should be programmed together. These bits indicate the        current WDTx clock rate. These bits are updated with the state of bits        WDTxCON1.IR0 and WDTxCON1.IR1 after WDTxCON0.ENDINIT is written with 1        during a valid Modify Access to register WDTxCON0."]
        #[inline(always)]
        pub fn is1(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, Wdtssr_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<5,1,0,Wdtssr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "SMU Unlock Restriction Status Flag   US. WDTxCON0.LCK will not be unlocked by a valid Password Access if this bit        is   8216 1  8217  and the SMU is not in the FAULT state"]
        #[inline(always)]
        pub fn us(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, Wdtssr_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<6,1,0,Wdtssr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Password Auto sequence Status Flag   PAS. This bit is updated with the state of bit WDTxCON1.PAR after        WDTxCON0.ENDINIT is written with 1 during a valid Modify Access to        register WDTxCON0."]
        #[inline(always)]
        pub fn pas(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, Wdtssr_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<7,1,0,Wdtssr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Timer Check Status Flag   TCS. This bit is updated with the state of bit WDTxCON1.TCR after        WDTxCON0.ENDINIT is written with 1 during a Valid Modify Access to        register WDTxCON0."]
        #[inline(always)]
        pub fn tcs(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, Wdtssr_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<8,1,0,Wdtssr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Timer Check Tolerance   TCT. This field determines the tolerance of the timer check during Password        or Check Access  See TCS .This bit is updated with the state of bit        WDTxCON1.TCTR after WDTxCON0.ENDINIT is written with 1 during a Valid        Modify Access to register WDTxCON0."]
        #[inline(always)]
        pub fn tct(
            self,
        ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, Wdtssr_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<9,0x7f,1,0,u8, Wdtssr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Timer Value   TIM. Reflects the current content of the WDTx."]
        #[inline(always)]
        pub fn tim(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Wdtssr_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, Wdtssr_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for Wdtssr {
        #[inline(always)]
        fn default() -> Wdtssr {
            <crate::RegValueT<Wdtssr_SPEC> as RegisterValue<_>>::new(4294705168)
        }
    }
}
