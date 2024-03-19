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
#[doc = r"PMS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pms(pub(super) *mut u8);
unsafe impl core::marker::Send for Pms {}
unsafe impl core::marker::Sync for Pms {}
impl Pms {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(508usize)) }
    }

    #[doc = "SMU stdby FSP Configuration Register\n resetvalue={LVD Reset:0x0,Warm PORST:0x0}"]
    #[inline(always)]
    pub const fn ag2ifsp_stdby(
        &self,
    ) -> [crate::common::Reg<self::Ag2IFspStdby_SPEC, crate::common::RW>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x1a4usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a4usize + 0x4usize)),
            ]
        }
    }

    #[doc = "Alarm Status Register\n resetvalue={LVD Reset:0x0}"]
    #[inline(always)]
    pub const fn ag2i_stdby(
        &self,
    ) -> [crate::common::Reg<self::Ag2IStdby_SPEC, crate::common::RW>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x188usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x188usize + 0x4usize)),
            ]
        }
    }

    #[doc = "SMU stdby Command Register\n resetvalue={LVD Reset:0x0,Warm PORST:0x0}"]
    #[inline(always)]
    pub const fn cmd_stdby(&self) -> crate::common::Reg<self::CmdStdby_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(412usize)) }
    }

    #[doc = "Die Temperature Sensor Limit Register\n resetvalue={LVD Reset:0x0CD806D6,Cold PORST:0x0CD806D6}"]
    #[inline(always)]
    pub const fn dtslim(&self) -> crate::common::Reg<self::Dtslim_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(456usize)) }
    }

    #[doc = "Die Temperature Sensor Status Register\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
    #[inline(always)]
    pub const fn dtsstat(&self) -> crate::common::Reg<self::Dtsstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(448usize)) }
    }

    #[doc = "EVR33 Control Register\n resetvalue={LVD Reset:0x407ED,Cold PORST:0x407ED,After SSW execution:0x407ED}"]
    #[inline(always)]
    pub const fn evr33con(&self) -> crate::common::Reg<self::Evr33Con_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }

    #[doc = "EVR Primary ADC Status Register\n resetvalue={LVD Reset:0x0}"]
    #[inline(always)]
    pub const fn evradcstat(&self) -> crate::common::Reg<self::Evradcstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "EVR Secondary Monitor Control Register\n resetvalue={LVD Reset:0x0A5A5A5,Cold PORST:0x0A5A5A5,After SSW execution:0x0A5A5A5}"]
    #[inline(always)]
    pub const fn evrmonctrl(&self) -> crate::common::Reg<self::Evrmonctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(104usize)) }
    }

    #[doc = "EVR Secondary Monitor Filter Register\n resetvalue={LVD Reset:0x300,Cold PORST:0x300,After SSW execution:0x10301}"]
    #[inline(always)]
    pub const fn evrmonfilt(&self) -> crate::common::Reg<self::Evrmonfilt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(112usize)) }
    }

    #[doc = "EVR Secondary ADC Status Register 1\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
    #[inline(always)]
    pub const fn evrmonstat1(
        &self,
    ) -> crate::common::Reg<self::Evrmonstat1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }

    #[doc = "EVR Secondary ADC Status Register 2\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
    #[inline(always)]
    pub const fn evrmonstat2(
        &self,
    ) -> crate::common::Reg<self::Evrmonstat2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(100usize)) }
    }

    #[doc = "EVR Oscillator Control Register\n resetvalue={LVD Reset:0x1F,After SSW execution:0x2000001F}"]
    #[inline(always)]
    pub const fn evroscctrl(&self) -> crate::common::Reg<self::Evroscctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }

    #[doc = "EVR Secondary Over voltage Monitor Register\n resetvalue={LVD Reset:0x0FEFEFE,Cold PORST:0x0FEFEFE,After SSW execution:0x0FEFEFE}"]
    #[inline(always)]
    pub const fn evrovmon(&self) -> crate::common::Reg<self::Evrovmon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(124usize)) }
    }

    #[doc = "EVR Secondary Over voltage Monitor Register 2\n resetvalue={LVD Reset:0x0FEFEFE,Cold PORST:0x0FEFEFE,After SSW execution:0x0FEFEFE}"]
    #[inline(always)]
    pub const fn evrovmon2(&self) -> crate::common::Reg<self::Evrovmon2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }

    #[doc = "EVR Reset Control Register\n resetvalue={LVD Reset:0x597F4A,Cold PORST:0x597F4A,After SSW execution:0x5C834B}"]
    #[inline(always)]
    pub const fn evrrstcon(&self) -> crate::common::Reg<self::Evrrstcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "EVR Reset Status Register\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
    #[inline(always)]
    pub const fn evrrststat(&self) -> crate::common::Reg<self::Evrrststat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }

    #[doc = "EVRC SD Coefficient Register 0\n resetvalue={LVD Reset:0x0B50873B6,Cold PORST:0x0B50873B6,After SSW execution:0x0B50873B6}"]
    #[inline(always)]
    pub const fn evrsdcoeff0(
        &self,
    ) -> crate::common::Reg<self::Evrsdcoeff0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(328usize)) }
    }

    #[doc = "EVRC SD Coefficient Register 1\n resetvalue={LVD Reset:0x0A2946C46,Cold PORST:0x0A2946C46,After SSW execution:0x0A2946C46}"]
    #[inline(always)]
    pub const fn evrsdcoeff1(
        &self,
    ) -> crate::common::Reg<self::Evrsdcoeff1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(332usize)) }
    }

    #[doc = "EVRC SD Coefficient Register 2\n resetvalue={LVD Reset:0x3408710E,Cold PORST:0x3408710E,After SSW execution:0x3408710E}"]
    #[inline(always)]
    pub const fn evrsdcoeff2(
        &self,
    ) -> crate::common::Reg<self::Evrsdcoeff2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(336usize)) }
    }

    #[doc = "EVRC SD Coefficient Register 3\n resetvalue={LVD Reset:0x2946C44,Cold PORST:0x2946C44,After SSW execution:0x2946C44}"]
    #[inline(always)]
    pub const fn evrsdcoeff3(
        &self,
    ) -> crate::common::Reg<self::Evrsdcoeff3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(340usize)) }
    }

    #[doc = "EVRC SD Coefficient Register 4\n resetvalue={LVD Reset:0x1B0822B6,Cold PORST:0x1B0822B6,After SSW execution:0x1B0822B6}"]
    #[inline(always)]
    pub const fn evrsdcoeff4(
        &self,
    ) -> crate::common::Reg<self::Evrsdcoeff4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(344usize)) }
    }

    #[doc = "EVRC SD Coefficient Register 5\n resetvalue={LVD Reset:0x2946C46,Cold PORST:0x2946C46,After SSW execution:0x2946C46}"]
    #[inline(always)]
    pub const fn evrsdcoeff5(
        &self,
    ) -> crate::common::Reg<self::Evrsdcoeff5_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(348usize)) }
    }

    #[doc = "EVRC SD Coefficient Register 6\n resetvalue={LVD Reset:0x080971802,Cold PORST:0x080971802,After SSW execution:0x080971802}"]
    #[inline(always)]
    pub const fn evrsdcoeff6(
        &self,
    ) -> crate::common::Reg<self::Evrsdcoeff6_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(352usize)) }
    }

    #[doc = "EVRC SD Coefficient Register 7\n resetvalue={LVD Reset:0x08000D8F7,Cold PORST:0x08000D8F7,After SSW execution:0x08000D8F7}"]
    #[inline(always)]
    pub const fn evrsdcoeff7(
        &self,
    ) -> crate::common::Reg<self::Evrsdcoeff7_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(356usize)) }
    }

    #[doc = "EVRC SD Coefficient Register 8\n resetvalue={LVD Reset:0x080171002,Cold PORST:0x080171002,After SSW execution:0x080171002}"]
    #[inline(always)]
    pub const fn evrsdcoeff8(
        &self,
    ) -> crate::common::Reg<self::Evrsdcoeff8_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(360usize)) }
    }

    #[doc = "EVRC SD Coefficient Register 9\n resetvalue={LVD Reset:0x08000A0AF,Cold PORST:0x08000A0AF,After SSW execution:0x08000A0AF}"]
    #[inline(always)]
    pub const fn evrsdcoeff9(
        &self,
    ) -> crate::common::Reg<self::Evrsdcoeff9_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(364usize)) }
    }

    #[doc = "EVRC SD Control Register 0\n resetvalue={LVD Reset:0x0F0390001,Cold PORST:0x0F0390001,After SSW execution:0x0F0390001}"]
    #[inline(always)]
    pub const fn evrsdctrl0(&self) -> crate::common::Reg<self::Evrsdctrl0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(264usize)) }
    }

    #[doc = "EVRC SD Control Register 1\n resetvalue={LVD Reset:0x086690708,Cold PORST:0x086690708,After SSW execution:0x086690708}"]
    #[inline(always)]
    pub const fn evrsdctrl1(&self) -> crate::common::Reg<self::Evrsdctrl1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(268usize)) }
    }

    #[doc = "EVRC SD Control Register 10\n resetvalue={LVD Reset:0x5A82,Cold PORST:0x5A82,After SSW execution:0x5A82}"]
    #[inline(always)]
    pub const fn evrsdctrl10(
        &self,
    ) -> crate::common::Reg<self::Evrsdctrl10_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(304usize)) }
    }

    #[doc = "EVRC SD Control Register 11\n resetvalue={LVD Reset:0x092070909,Cold PORST:0x092070909,After SSW execution:0x092070909}"]
    #[inline(always)]
    pub const fn evrsdctrl11(
        &self,
    ) -> crate::common::Reg<self::Evrsdctrl11_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(308usize)) }
    }

    #[doc = "EVRC SD Control Register 2\n resetvalue={LVD Reset:0x36033B,Cold PORST:0x36033B,After SSW execution:0x36033B}"]
    #[inline(always)]
    pub const fn evrsdctrl2(&self) -> crate::common::Reg<self::Evrsdctrl2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(272usize)) }
    }

    #[doc = "EVRC SD Control Register 3\n resetvalue={LVD Reset:0x0B690810,Cold PORST:0x0B690810,After SSW execution:0x0B690810}"]
    #[inline(always)]
    pub const fn evrsdctrl3(&self) -> crate::common::Reg<self::Evrsdctrl3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(276usize)) }
    }

    #[doc = "EVRC SD Control Register 4\n resetvalue={LVD Reset:0x360009,Cold PORST:0x360009,After SSW execution:0x360009}"]
    #[inline(always)]
    pub const fn evrsdctrl4(&self) -> crate::common::Reg<self::Evrsdctrl4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(280usize)) }
    }

    #[doc = "EVRC SD Control Register 5\n resetvalue={LVD Reset:0x0B690808,Cold PORST:0x0B690808,After SSW execution:0x0B690808}"]
    #[inline(always)]
    pub const fn evrsdctrl5(&self) -> crate::common::Reg<self::Evrsdctrl5_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(284usize)) }
    }

    #[doc = "EVRC SD Control Register 6\n resetvalue={LVD Reset:0x080231C94,Cold PORST:0x080231C94,After SSW execution:0x080231C94}"]
    #[inline(always)]
    pub const fn evrsdctrl6(&self) -> crate::common::Reg<self::Evrsdctrl6_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(288usize)) }
    }

    #[doc = "EVRC SD Control Register 7\n resetvalue={LVD Reset:0x0800000FE,Cold PORST:0x0800000FE,After SSW execution:0x0800000FE}"]
    #[inline(always)]
    pub const fn evrsdctrl7(&self) -> crate::common::Reg<self::Evrsdctrl7_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(292usize)) }
    }

    #[doc = "EVRC SD Control Register 8\n resetvalue={LVD Reset:0x09121048E,Cold PORST:0x09121048E,After SSW execution:0x09121048E}"]
    #[inline(always)]
    pub const fn evrsdctrl8(&self) -> crate::common::Reg<self::Evrsdctrl8_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(296usize)) }
    }

    #[doc = "EVRC SD Control Register 9\n resetvalue={LVD Reset:0x080000434,Cold PORST:0x080000434,After SSW execution:0x080000434}"]
    #[inline(always)]
    pub const fn evrsdctrl9(&self) -> crate::common::Reg<self::Evrsdctrl9_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(300usize)) }
    }

    #[doc = "EVR SD Status Register 0\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
    #[inline(always)]
    pub const fn evrsdstat0(&self) -> crate::common::Reg<self::Evrsdstat0_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "EVR Status Register\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
    #[inline(always)]
    pub const fn evrstat(&self) -> crate::common::Reg<self::Evrstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }

    #[doc = "EVR Trim Control Register\n resetvalue={LVD Reset:0x080006C9E,Cold PORST:0x080006C9E,After SSW execution:0x6C9E}"]
    #[inline(always)]
    pub const fn evrtrim(&self) -> crate::common::Reg<self::Evrtrim_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(76usize)) }
    }

    #[doc = "EVR Trim Status Register\n resetvalue={LVD Reset:0x6C9E,Cold PORST:0x6C9E}"]
    #[inline(always)]
    pub const fn evrtrimstat(
        &self,
    ) -> crate::common::Reg<self::Evrtrimstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }

    #[doc = "EVR Secondary Under voltage Monitor Register\n resetvalue={LVD Reset:0x75A7B8,Cold PORST:0x75A7B8,After SSW execution:0x75A7B8}"]
    #[inline(always)]
    pub const fn evruvmon(&self) -> crate::common::Reg<self::Evruvmon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(120usize)) }
    }

    #[doc = "EVR Secondary Under voltage Monitor Register 2\n resetvalue={LVD Reset:0x2A7000BC,Cold PORST:0x2A7000BC,After SSW execution:0x2A7000BC}"]
    #[inline(always)]
    pub const fn evruvmon2(&self) -> crate::common::Reg<self::Evruvmon2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "EVR Primary HSM Over voltage Monitor Register\n resetvalue={LVD Reset:0x0E1B586,Cold PORST:0x0E1B586,After SSW execution:0x0E1B586}"]
    #[inline(always)]
    pub const fn hsmovmon(&self) -> crate::common::Reg<self::Hsmovmon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(140usize)) }
    }

    #[doc = "EVR Primary HSM Under voltage Monitor Register\n resetvalue={LVD Reset:0x5C824D,Cold PORST:0x5C824D,After SSW execution:0x5C824D}"]
    #[inline(always)]
    pub const fn hsmuvmon(&self) -> crate::common::Reg<self::Hsmuvmon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }

    #[doc = "Identification Register\n resetvalue={Application Reset:0x0E8C001}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "SMU stdby BIST Control Register\n resetvalue={LVD Reset:0x0,Warm PORST:0x0}"]
    #[inline(always)]
    pub const fn monbistctrl(
        &self,
    ) -> crate::common::Reg<self::Monbistctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(408usize)) }
    }

    #[doc = "SMU stdby BIST Status Register\n resetvalue={LVD Reset:0x0,Warm PORST:0x0}"]
    #[inline(always)]
    pub const fn monbiststat(
        &self,
    ) -> crate::common::Reg<self::Monbiststat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(400usize)) }
    }

    #[doc = "OCDS Trigger Set Control 0 Register\n resetvalue={LVD Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn otsc0(&self) -> crate::common::Reg<self::Otsc0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(484usize)) }
    }

    #[doc = "OCDS Trigger Set Control 1 Register\n resetvalue={LVD Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn otsc1(&self) -> crate::common::Reg<self::Otsc1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(488usize)) }
    }

    #[doc = "OCDS Trigger Set Select Register\n resetvalue={LVD Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn otss(&self) -> crate::common::Reg<self::Otss_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(480usize)) }
    }

    #[doc = "PMS Interrupt Enable Register\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
    #[inline(always)]
    pub const fn pmsien(&self) -> crate::common::Reg<self::Pmsien_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(116usize)) }
    }

    #[doc = "Standby and Wake up Control Register 0\n resetvalue={LVD Reset:0x1002D0}"]
    #[inline(always)]
    pub const fn pmswcr0(&self) -> crate::common::Reg<self::Pmswcr0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(180usize)) }
    }

    #[doc = "Standby and Wake up Control Register 2\n resetvalue={LVD Reset:0x4000000}"]
    #[inline(always)]
    pub const fn pmswcr2(&self) -> crate::common::Reg<self::Pmswcr2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(184usize)) }
    }

    #[doc = "Standby and Wake up Control Register 3\n resetvalue={LVD Reset:0x0}"]
    #[inline(always)]
    pub const fn pmswcr3(&self) -> crate::common::Reg<self::Pmswcr3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(192usize)) }
    }

    #[doc = "Standby and Wake up Control Register 4\n resetvalue={LVD Reset:0x20,After SSW execution:0x2000020}"]
    #[inline(always)]
    pub const fn pmswcr4(&self) -> crate::common::Reg<self::Pmswcr4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(196usize)) }
    }

    #[doc = "Standby and Wake up Control Register 5\n resetvalue={LVD Reset:0x0}"]
    #[inline(always)]
    pub const fn pmswcr5(&self) -> crate::common::Reg<self::Pmswcr5_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(200usize)) }
    }

    #[doc = "Standby and Wake up Status Register\n resetvalue={LVD Reset:0x0A0000}"]
    #[inline(always)]
    pub const fn pmswstat(&self) -> crate::common::Reg<self::Pmswstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(212usize)) }
    }

    #[doc = "Standby and Wake up Status Register 2\n resetvalue={LVD Reset:0x100000}"]
    #[inline(always)]
    pub const fn pmswstat2(&self) -> crate::common::Reg<self::Pmswstat2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(216usize)) }
    }

    #[doc = "Standby and Wake up Status Clear Register\n resetvalue={LVD Reset:0x0}"]
    #[inline(always)]
    pub const fn pmswstatclr(
        &self,
    ) -> crate::common::Reg<self::Pmswstatclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(232usize)) }
    }

    #[doc = "Standby WUT Counter Register\n resetvalue={LVD Reset:0x0}"]
    #[inline(always)]
    pub const fn pmswutcnt(&self) -> crate::common::Reg<self::Pmswutcnt_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(220usize)) }
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
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
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
pub struct Ag2IFspStdby_SPEC;
impl crate::sealed::RegSpec for Ag2IFspStdby_SPEC {
    type DataType = u32;
}
#[doc = "SMU stdby FSP Configuration Register\n resetvalue={LVD Reset:0x0,Warm PORST:0x0}"]
pub type Ag2IFspStdby = crate::RegValueT<Ag2IFspStdby_SPEC>;

impl Ag2IFspStdby {
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ag2IFspStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Ag2IFspStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ag2IFspStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Ag2IFspStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ag2IFspStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Ag2IFspStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ag2IFspStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Ag2IFspStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Ag2IFspStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Ag2IFspStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe9(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ag2IFspStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Ag2IFspStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ag2IFspStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Ag2IFspStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ag2IFspStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Ag2IFspStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Ag2IFspStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Ag2IFspStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Ag2IFspStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Ag2IFspStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Ag2IFspStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Ag2IFspStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Ag2IFspStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Ag2IFspStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "AG2iFSP STDBY register bits protection. Setting this bit enables that bits FE z  can be changed in this write        operation. This bit is read as zero."]
    #[inline(always)]
    pub fn bitprot(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Ag2IFspStdby_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30,1,0,Ag2IFspStdby_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Ag2IFspStdby {
    #[inline(always)]
    fn default() -> Ag2IFspStdby {
        <crate::RegValueT<Ag2IFspStdby_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ag2IStdby_SPEC;
impl crate::sealed::RegSpec for Ag2IStdby_SPEC {
    type DataType = u32;
}
#[doc = "Alarm Status Register\n resetvalue={LVD Reset:0x0}"]
pub type Ag2IStdby = crate::RegValueT<Ag2IStdby_SPEC>;

impl Ag2IStdby {
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ag2IStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Ag2IStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ag2IStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Ag2IStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Ag2IStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Ag2IStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Ag2IStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Ag2IStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Ag2IStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Ag2IStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf9(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ag2IStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,Ag2IStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Ag2IStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,Ag2IStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ag2IStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,Ag2IStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Ag2IStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Ag2IStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Ag2IStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Ag2IStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Ag2IStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Ag2IStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Ag2IStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Ag2IStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Pin Fault State Status Bit   FSPERR. The bit indicates that Error pin was set into fault state by the SMU stdby. Reset by setting to 1. If the Error Pins were set in fault state by the SMU stdby  reseting this bit sets the Error Pins back in fault free state"]
    #[inline(always)]
    pub fn fsperr(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Ag2IStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,Ag2IStdby_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ag2IStdby {
    #[inline(always)]
    fn default() -> Ag2IStdby {
        <crate::RegValueT<Ag2IStdby_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdStdby_SPEC;
impl crate::sealed::RegSpec for CmdStdby_SPEC {
    type DataType = u32;
}
#[doc = "SMU stdby Command Register\n resetvalue={LVD Reset:0x0,Warm PORST:0x0}"]
pub type CmdStdby = crate::RegValueT<CmdStdby_SPEC>;

impl CmdStdby {
    #[doc = "SMU stdby Module Enable   SMUEN. This bit enables SMU stdby to issue a FSP reaction when an alarm is received. Also  SMUEN needs to be set to enter the SMU stdby BIST mode."]
    #[inline(always)]
    pub fn smuen(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, CmdStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, CmdStdby_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SMU stdby FSP0 Error pin enable   FSP0EN. This bit enables SMU stdby Error pin function to be able set P33.8 to        fault state."]
    #[inline(always)]
    pub fn fsp0en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, CmdStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, CmdStdby_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SMU stdby FSP1 Error pin enable   FSP1EN. This bit enables SMU stdby Error pin function to be able set P33.10 to        fault state."]
    #[inline(always)]
    pub fn fsp1en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, CmdStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, CmdStdby_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SMU stdby alarm status clear enable   ASCE. This bit controls if a status flag set in an AGx register upon detection        of the alarm event can be cleared by software or not. When ASCE is        enabled  software shall write a 1 to bit position in AGx to clear the        bit  W1C . When a W1C action takes place the ASCE bit is automatically        cleared to 0 by hardware and software shall set the ASCE bit again."]
    #[inline(always)]
    pub fn asce(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, CmdStdby_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, CmdStdby_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CMD STDBY register bits protection. Setting this bit enables that bits SMUEN  FSP0EN  FSP1EN or and ASCE can        be changed in this write operation. This bit is read as zero."]
    #[inline(always)]
    pub fn bitprot(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, CmdStdby_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, CmdStdby_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for CmdStdby {
    #[inline(always)]
    fn default() -> CmdStdby {
        <crate::RegValueT<CmdStdby_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtslim_SPEC;
impl crate::sealed::RegSpec for Dtslim_SPEC {
    type DataType = u32;
}
#[doc = "Die Temperature Sensor Limit Register\n resetvalue={LVD Reset:0x0CD806D6,Cold PORST:0x0CD806D6}"]
pub type Dtslim = crate::RegValueT<Dtslim_SPEC>;

impl Dtslim {
    #[doc = "Lower Limit   LOWER. This bit field defines the lower limit of the DTS temperature check. The        DTS measurement result is compared against this value and if the        measurement result is less than or equal to the configured LOWER        bitfield value  flag LLU is set."]
    #[inline(always)]
    pub fn lower(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, Dtslim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16, Dtslim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lower Limit Underflow   LLU. When this bit is set  a HSM temperature underflow trigger is generated.        When this bit is set the related SMU DTS alarm trigger is generated.        This bit has to be written with zero in order to clear it. Writing a one        has no effect. This bit is set when a DTS measurement is finished and        the result is below the lower limit  i.e. DTSLIM.LOWER ."]
    #[inline(always)]
    pub fn llu(self) -> crate::common::RegisterFieldBool<15, 1, 0, Dtslim_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Dtslim_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Upper Limit   UPPER. This bit field defines the upper limit of the DTS temperature check. The        DTS measurement result is compared against this value and if the        measurement result is greater than or equal to the configured UPPER        bitfield value  flag UOF is set."]
    #[inline(always)]
    pub fn upper(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, Dtslim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xfff,1,0,u16, Dtslim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Dtslim_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Dtslim_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Upper Limit Overflow   UOF. When this bit is set  a HSM temperature overflow trigger is generated. When this bit is set  the related SMU DTS alarm trigger is generated. This bit has to be written with zero in order to clear it. Writing a one        has no effect. This bit is set when a DTS measurement is finished and the result is        exceeding the upper limit  i.e. DTSLIM.UPPER ."]
    #[inline(always)]
    pub fn uof(self) -> crate::common::RegisterFieldBool<31, 1, 0, Dtslim_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Dtslim_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Dtslim {
    #[inline(always)]
    fn default() -> Dtslim {
        <crate::RegValueT<Dtslim_SPEC> as RegisterValue<_>>::new(215484118)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtsstat_SPEC;
impl crate::sealed::RegSpec for Dtsstat_SPEC {
    type DataType = u32;
}
#[doc = "Die Temperature Sensor Status Register\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
pub type Dtsstat = crate::RegValueT<Dtsstat_SPEC>;

impl Dtsstat {
    #[doc = "Result of the DTS Measurement   RESULT. This bit field shows the result of the DTS measurement. The value given        is directly related to the die temperature and can be evaluated using        the following formula. T    176 C     RESULT   Gnom    273.15 T   176 K   RESULT  G nom RESULT  G nom    T    176 C    273.15    G nom   T    176 K  G nom   7.505"]
    #[inline(always)]
    pub fn result(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, Dtsstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xfff,1,0,u16, Dtsstat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Dtsstat {
    #[inline(always)]
    fn default() -> Dtsstat {
        <crate::RegValueT<Dtsstat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evr33Con_SPEC;
impl crate::sealed::RegSpec for Evr33Con_SPEC {
    type DataType = u32;
}
#[doc = "EVR33 Control Register\n resetvalue={LVD Reset:0x407ED,Cold PORST:0x407ED,After SSW execution:0x407ED}"]
pub type Evr33Con = crate::RegValueT<Evr33Con_SPEC>;

impl Evr33Con {
    #[doc = "Short to Supply Voltage Threshold x i    SHVH33. This field defines the upper threshold level VDDP3 supply. EVR33 short to supply alarm has the nominal values of SHVH33   4.5V and t33SHHV   3ms. Do not change the reset value SHVH33     VDDx   937.5 mV    LSB  LSB   15 mV"]
    #[inline(always)]
    pub fn shvh33(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evr33Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evr33Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Short to High Detection Enable   SHHVEN"]
    #[inline(always)]
    pub fn shhven(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Evr33Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,Evr33Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Short to Low Detection Enable   SHLVEN"]
    #[inline(always)]
    pub fn shlven(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Evr33Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,Evr33Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Short to Ground Voltage Threshold x i    SHVL33. This field defines the lower threshold level VDDP3 supply. EVR33 short to ground alarm has the nominal values of SHVL33   1V and t33SHLV   3ms. Do not change the reset value SHVL33     VDDx   937.5 mV    LSB  LSB   15 mV"]
    #[inline(always)]
    pub fn shvl33(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evr33Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evr33Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Evr33Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,Evr33Con_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Evr33Con {
    #[inline(always)]
    fn default() -> Evr33Con {
        <crate::RegValueT<Evr33Con_SPEC> as RegisterValue<_>>::new(264173)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evradcstat_SPEC;
impl crate::sealed::RegSpec for Evradcstat_SPEC {
    type DataType = u32;
}
#[doc = "EVR Primary ADC Status Register\n resetvalue={LVD Reset:0x0}"]
pub type Evradcstat = crate::RegValueT<Evradcstat_SPEC>;

impl Evradcstat {
    #[doc = "ADC VDD Core Voltage Conversion Result   ADCCV. This bit field contains the last filtered conversion result of the ADC measurement of the VDD  xa0    xa0 EVRC supply by the Primary Monitor. VIN  xa0    xa0  0.7125    ADCCV  xa0    xa0 LSB    xa0 V LSB   5  xa0 mV Eg. 1.25 V   6C"]
    #[inline(always)]
    pub fn adccv(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evradcstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evradcstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ADC VDDP3 Voltage Conversion Result   ADC33V. This bit field contains the last filtered conversion result of the ADC measurement of the VDDP3  xa0    xa0 EVR33 supply by the Primary Monitor. VIN  xa0    xa0  0.9375    ADC33V  xa0    xa0 LSB    xa0 V LSB   15  xa0 mV Eg. 3.3 V   9E"]
    #[inline(always)]
    pub fn adc33v(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evradcstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evradcstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ADC VEXT Supply Conversion Result   ADCSWDV. This bit field contains the last filtered conversion result of the ADC measurement of the external VEXT  3.3V  xa0    xa0 5V  supply by the Primary Monitor. VIN  xa0    xa0  1.050    ADCSWDV  xa0    xa0 LSB    xa0 V  xa0  LSB   20  xa0 mV Eg. 5 V   C6"]
    #[inline(always)]
    pub fn adcswdv(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evradcstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evradcstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EVRC Regulator or VDD Over voltage event flag   OVC. This bit is set if VDD primary voltage monitor recognizes a over voltage        event. An alarm is raised to the HSM and SMU."]
    #[inline(always)]
    pub fn ovc(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Evradcstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24,1,0,Evradcstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EVR33 Regulator or VDDP3 Over voltage event flag   OV33. This bit is set if VDDP3 primary voltage monitor recognizes a        over voltage event. An alarm is raised to the HSM and SMU."]
    #[inline(always)]
    pub fn ov33(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Evradcstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25,1,0,Evradcstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Supply Watchdog  SWD  or VEXT Over voltage event flag   OVSWD. This bit is set if VEXT primary voltage monitor recognizes an        over voltage event. An alarm is raised to the HSM and SMU."]
    #[inline(always)]
    pub fn ovswd(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Evradcstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26,1,0,Evradcstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EVRC Regulator or VDD Under voltage event flag   UVC. This bit is set if VDD primary voltage monitor recognizes a        under voltage event. An alarm is raised to the HSM and SMU."]
    #[inline(always)]
    pub fn uvc(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Evradcstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27,1,0,Evradcstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EVR33 Regulator or VDDP3 Under voltage event flag   UV33. This bit is set if VDDP3 primary voltage monitor recognizes a        under voltage event. An alarm is raised to the HSM and SMU."]
    #[inline(always)]
    pub fn uv33(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Evradcstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28,1,0,Evradcstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Supply Watchdog  SWD  or VEXT Under voltage event flag   UVSWD. This bit is set if VEXT primary voltage monitor recognizes an        under voltage event. An alarm is raised to the HSM and SMU."]
    #[inline(always)]
    pub fn uvswd(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Evradcstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29,1,0,Evradcstat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evradcstat {
    #[inline(always)]
    fn default() -> Evradcstat {
        <crate::RegValueT<Evradcstat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrmonctrl_SPEC;
impl crate::sealed::RegSpec for Evrmonctrl_SPEC {
    type DataType = u32;
}
#[doc = "EVR Secondary Monitor Control Register\n resetvalue={LVD Reset:0x0A5A5A5,Cold PORST:0x0A5A5A5,After SSW execution:0x0A5A5A5}"]
pub type Evrmonctrl = crate::RegValueT<Evrmonctrl_SPEC>;

impl Evrmonctrl {
    #[doc = "VDD Over voltage monitoring mode   EVRCOVMOD. Incase both EVRCOVMOD  160    160  00  amp         EVRCUVMOD  160    160  00   then ADC        conversion for the respective supply rail does not take place."]
    #[inline(always)]
    pub fn evrcovmod(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Evrmonctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Evrmonctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "EVRPR or VDDPD Over voltage monitoring mode   PREOVMOD. Incase both PREOVMOD  160    160  00  amp         PREUVMOD  160    160  00   then ADC        conversion for the respective supply rail does not take place."]
    #[inline(always)]
    pub fn preovmod(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Evrmonctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Evrmonctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDD Under voltage monitoring mode   EVRCUVMOD. Incase both EVRCOVMOD  160    160  00  amp         EVRCUVMOD  160    160  00   then ADC        conversion for the respective supply rail does not take place."]
    #[inline(always)]
    pub fn evrcuvmod(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Evrmonctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Evrmonctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "EVRPR or VDDPD Under voltage monitoring mode   PREUVMOD. Incase both PREOVMOD  160    160  00  amp         PREUVMOD  160    160  00   then ADC        conversion for the respective supply rail does not take place."]
    #[inline(always)]
    pub fn preuvmod(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Evrmonctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Evrmonctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDP3 Supply Over voltage monitoring mode   EVR33OVMOD. Incase both EVR33OVMOD  160    160  00  amp         EVR33UVMOD  160    160  00   then ADC        conversion for the respective supply rail does not take place."]
    #[inline(always)]
    pub fn evr33ovmod(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Evrmonctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Evrmonctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDM ADC Supply Over voltage monitoring mode   VDDMOVMOD. Incase both VDDMOVMOD  160    160  00  amp         VDDMUVMOD  160    160  00   then ADC        conversion for the VDDM supply rail continues to run as used for ADC        function."]
    #[inline(always)]
    pub fn vddmovmod(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Evrmonctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Evrmonctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDP3 Supply Under voltage monitoring mode   EVR33UVMOD. Incase both EVR33OVMOD  160    160  00  amp         EVR33UVMOD  160    160  00   then ADC        conversion for the respective supply rail does not take place."]
    #[inline(always)]
    pub fn evr33uvmod(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Evrmonctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Evrmonctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDM ADC Supply Under voltage monitoring mode   VDDMUVMOD. Incase both VDDMOVMOD  160    160  00  amp         VDDMUVMOD  160    160  00   then ADC        conversion for the VDDM supply rail continues to run as used for ADC        function."]
    #[inline(always)]
    pub fn vddmuvmod(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Evrmonctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Evrmonctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEXT Over voltage monitoring mode   SWDOVMOD. Incase both SWDOVMOD  160    160  00  amp         SWDUVMOD  160    160  00   then ADC        conversion for the respective supply rail does not take place."]
    #[inline(always)]
    pub fn swdovmod(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Evrmonctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Evrmonctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "EVR Standby Supply or VEVRSB Over voltage monitoring mode   SBOVMOD. Incase both SBOVMOD  160    160  00  amp         SBUVMOD  160    160  00   then ADC        conversion for the respective supply rail does not take place."]
    #[inline(always)]
    pub fn sbovmod(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, Evrmonctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, Evrmonctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEXT Under voltage monitoring mode   SWDUVMOD. Incase both SWDOVMOD  160    160  00  amp         SWDUVMOD  160    160  00   then ADC        conversion for the respective supply rail does not take place."]
    #[inline(always)]
    pub fn swduvmod(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Evrmonctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Evrmonctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "EVR Standby Supply or VEVRSB Under voltage monitoring mode   SBUVMOD. Incase both SBOVMOD  160    160  00  amp         SBUVMOD  160    160  00   then ADC        conversion for the respective supply rail does not take place."]
    #[inline(always)]
    pub fn sbuvmod(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, Evrmonctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8, Evrmonctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Evrmonctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,Evrmonctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Evrmonctrl {
    #[inline(always)]
    fn default() -> Evrmonctrl {
        <crate::RegValueT<Evrmonctrl_SPEC> as RegisterValue<_>>::new(10855845)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrmonfilt_SPEC;
impl crate::sealed::RegSpec for Evrmonfilt_SPEC {
    type DataType = u32;
}
#[doc = "EVR Secondary Monitor Filter Register\n resetvalue={LVD Reset:0x300,Cold PORST:0x300,After SSW execution:0x10301}"]
pub type Evrmonfilt = crate::RegValueT<Evrmonfilt_SPEC>;

impl Evrmonfilt {
    #[doc = "VDD Secondary ADC Supply Filter   EVRCFIL"]
    #[inline(always)]
    pub fn evrcfil(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Evrmonfilt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Evrmonfilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDPD Secondary ADC Supply Filter   PREFIL"]
    #[inline(always)]
    pub fn prefil(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Evrmonfilt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Evrmonfilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDP3 Secondary ADC Supply Filter   EVR33FIL"]
    #[inline(always)]
    pub fn evr33fil(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Evrmonfilt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Evrmonfilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDM Secondary ADC Supply Filter   VDDMFIL"]
    #[inline(always)]
    pub fn vddmfil(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Evrmonfilt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Evrmonfilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEXT Secondary ADC Supply Filter   SWDFIL"]
    #[inline(always)]
    pub fn swdfil(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Evrmonfilt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Evrmonfilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEVRSB Secondary ADC Supply Filter   SBFIL"]
    #[inline(always)]
    pub fn sbfil(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Evrmonfilt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Evrmonfilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear all Spike Filters   CLRFIL. To avoid spurious alarms during change of configuration or start up         CLRFIL shall be set followed by alarm reconfiguration followed by        activation of filter logic by clearing CLRFIL register bit."]
    #[inline(always)]
    pub fn clrfil(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Evrmonfilt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,Evrmonfilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Evrmonfilt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,Evrmonfilt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Evrmonfilt {
    #[inline(always)]
    fn default() -> Evrmonfilt {
        <crate::RegValueT<Evrmonfilt_SPEC> as RegisterValue<_>>::new(768)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrmonstat1_SPEC;
impl crate::sealed::RegSpec for Evrmonstat1_SPEC {
    type DataType = u32;
}
#[doc = "EVR Secondary ADC Status Register 1\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
pub type Evrmonstat1 = crate::RegValueT<Evrmonstat1_SPEC>;

impl Evrmonstat1 {
    #[doc = "VDD Supply Secondary ADC Conversion Result   ADCCV. This bit field contains the last conversion result of the ADC        measurement of the VDD  160    160 EVRC supply by the Secondary Monitor. This        bitfield is updated if secondary over  or under voltage monitoring is        activated via EVRMONCTRL.EVRCxxMOD. The        update of the result is taking place at   6  160    160    160 12  160    160 active channels  160                160 fBACK   4  160    clock cycles. VIN  160    160  LSB  160    160  ADCx 1    160    160 Ideal LSB   5.7692  160 mV Full Range  160    160 1465  160 mV E.g. 1.25  160 V   DA"]
    #[inline(always)]
    pub fn adccv(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrmonstat1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrmonstat1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VDDP3 Supply Secondary ADC Conversion Result   ADC33V. This bit field contains the last conversion result of the ADC        measurement of the VDDP3  160    160 EVR33 supply by the Secondary Monitor. This        bitfield is updated if secondary over  or under voltage monitoring is        activated via EVRMONCTRL.EVR33xxMOD. The        update of the result is taking place at   6  160    160    160 12  160    160 active channels  160                160 fBACK   4  160    clock cycles. VIN  160    160  LSB  160    160  ADCx 1    160    160 Ideal LSB   15.00  160 mV Full Range  160    160 3810  160 mV E.g. 3.30  160 V   DD"]
    #[inline(always)]
    pub fn adc33v(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrmonstat1_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrmonstat1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VEXT Supply Secondary ADC Conversion Result   ADCSWDV. This bit field contains the last conversion result of the ADC        measurement of the external VEXT  3.3V  160    160 5V  supply by the Secondary        Monitor. This bitfield is updated if secondary over  or under voltage        monitoring is activated via EVRMONCTRL.SWDxxMOD. The        update of the result is taking place at   6  160    160    160 12  160    160 active channels  160                160 fBACK   4  160    clock cycles. VIN  160    160  LSB  160    160  ADCx 1    160    160 LSB   23.077  160 mV Full Range  160    160 5861  160 mV E.g. 5.01  160 V   DA   160   160   160   160   160    160 3.3  160 V   90"]
    #[inline(always)]
    pub fn adcswdv(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evrmonstat1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evrmonstat1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Secondary Monitor Activity Counter   ACTVCNT. This bit field cumulatively counts the end of conversion signals in a        single Secondary Monitor Background Scan over all channels and        respective filter configurations. The total number of conversions ConvTot     8721   ChX  ChXFIL . The counter is reset to 0 on a ConvTot overflow."]
    #[inline(always)]
    pub fn actvcnt(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, Evrmonstat1_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x3f,1,0,u8, Evrmonstat1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrmonstat1 {
    #[inline(always)]
    fn default() -> Evrmonstat1 {
        <crate::RegValueT<Evrmonstat1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrmonstat2_SPEC;
impl crate::sealed::RegSpec for Evrmonstat2_SPEC {
    type DataType = u32;
}
#[doc = "EVR Secondary ADC Status Register 2\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
pub type Evrmonstat2 = crate::RegValueT<Evrmonstat2_SPEC>;

impl Evrmonstat2 {
    #[doc = "VDDPD Supply Secondary ADC Conversion Result   ADCPRE. This bit field contains the last conversion result of the ADC        measurement of the VDDPD supply by the Secondary Monitor. This bitfield        is updated if secondary over  or under voltage monitoring is activated        via EVRMONCTRL.PRExxMOD. The        update of the result is taking place at   6  160    160    160 12  160    160 active channels  160                160 fBACK   4  160    clock cycles. VIN  160    160  LSB  160    160  ADCx 1    160    160 Ideal LSB   5.7692  160 mV Full Range  160    160 1465  160 mV E.g. 1.25  160 V   DA"]
    #[inline(always)]
    pub fn adcpre(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrmonstat2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrmonstat2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VEVRSB Supply Secondary ADC Conversion Result   ADCSB. This bit field contains the last conversion result of the ADC        measurement of the external VEVRSB  3.3V  160    160 5V  standby supply by the        Secondary Monitor. This bitfield is updated if secondary over  or        under voltage monitoring is activated via EVRMONCTRL.SBxxMOD. The        update of the result is taking place at   6  160    160    160 12  160    160 active channels  160                160 fBACK   4  160    clock cycles. VIN  160    160  LSB  160    160  ADCx 1    160    160 Ideal LSB   23.077  160 mV Full Range  160    160 5861  160 mV E.g. 5.01  160 V   DA   160   160   160   160   160   160 3.0  160 V   90"]
    #[inline(always)]
    pub fn adcsb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrmonstat2_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrmonstat2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VDDM Supply Secondary ADC Conversion Result   ADCVDDM. This bit field contains the last conversion result of the ADC        measurement of the VDDM ADC supply by the Secondary Monitor. This        bitfield is updated if secondary over  or under voltage monitoring is        activated via EVRMONCTRL.VDDMxxMOD. The        update of the result is taking place at   6  160    160    160 12  160    160 active channels  160                160 fBACK   4  160    clock cycles. VIN  160    160  LSB  160    160  ADCx 1    160    160 Ideal LSB   23.077  160 mV Full Range  160    160 5861  160 mV E.g. 5.01  160 V   DA   160   160   160   160   160   160 3.0  160 V   90"]
    #[inline(always)]
    pub fn adcvddm(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evrmonstat2_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evrmonstat2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrmonstat2 {
    #[inline(always)]
    fn default() -> Evrmonstat2 {
        <crate::RegValueT<Evrmonstat2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evroscctrl_SPEC;
impl crate::sealed::RegSpec for Evroscctrl_SPEC {
    type DataType = u32;
}
#[doc = "EVR Oscillator Control Register\n resetvalue={LVD Reset:0x1F,After SSW execution:0x2000001F}"]
pub type Evroscctrl = crate::RegValueT<Evroscctrl_SPEC>;

impl Evroscctrl {
    #[doc = "Back up Clock Fine Trim Value   OSCFTRIM. This thermometer coded bit field contains information about the 100MHz        OSC fine trimming. fBACK ftrim     OSCFTRIM    OSCFPTRIM           OSCTRx if OSCTRIMEN 1       LSBFT  MHz  LSBFT   110kHz Back up Clock accuracy is documented in datasheet. It is recommended to        wait 1  160 us after every fine trim step so that the clock source settles at        the new frequency. fBACK ftrim        value is saturated to range of 64."]
    #[inline(always)]
    pub fn oscftrim(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Evroscctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Evroscctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OSC Fine Trim Signed Value   OSCFPTRIM. This bit field allows device individual trimming of the oscillator trim        value during application. After updating the trim value  a waiting time        of 1  160 us is required for the change to take effect. OSCTRx        is implicitly added to OSCFPTRIM and saturated to range of  32 to 31"]
    #[inline(always)]
    pub fn oscfptrim(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Evroscctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Evroscctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Oscillator Temperature Offset Coefficient   OSCTEMPOFFS. This bitfield enables the centering function of the HPOSC temperature        coefficient to compensate for technology variations."]
    #[inline(always)]
    pub fn osctempoffs(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Evroscctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,Evroscctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Dynamic Oscillator Trim Enable   OSCTRIMEN. Based on temperature  Oscillator can be trimmed."]
    #[inline(always)]
    pub fn osctrimen(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Evroscctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,Evroscctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Evroscctrl {
    #[inline(always)]
    fn default() -> Evroscctrl {
        <crate::RegValueT<Evroscctrl_SPEC> as RegisterValue<_>>::new(31)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrovmon_SPEC;
impl crate::sealed::RegSpec for Evrovmon_SPEC {
    type DataType = u32;
}
#[doc = "EVR Secondary Over voltage Monitor Register\n resetvalue={LVD Reset:0x0FEFEFE,Cold PORST:0x0FEFEFE,After SSW execution:0x0FEFEFE}"]
pub type Evrovmon = crate::RegValueT<Evrovmon_SPEC>;

impl Evrovmon {
    #[doc = "VDD Supply Secondary Monitor Over voltage threshold   EVRCOVVAL. This field defines the over voltage monitoring threshold level of the        EVRC regulator output or VDD supply. Ideal Threshold     VIN  160    160 LSB   160    160 1  Ideal LSB   5.7692  160 mV"]
    #[inline(always)]
    pub fn evrcovval(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrovmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrovmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDP3 Supply Secondary Monitor Over voltage threshold   EVR33OVVAL. This field defines the over voltage monitoring threshold level of the        EVR33 regulator output or VDDP3 supply. Ideal Threshold     VIN  160    160 LSB   160    160 1  Ideal LSB   15.00  160 mV"]
    #[inline(always)]
    pub fn evr33ovval(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrovmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrovmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEXT Supply Secondary Monitor Over voltage threshold   SWDOVVAL. This field defines the over voltage threshold level of the external VEXT        supply monitor. Ideal Threshold     VIN  160    160 LSB   160    160 1  Ideal LSB   23.077  160 mV."]
    #[inline(always)]
    pub fn swdovval(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evrovmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evrovmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Evrovmon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,Evrovmon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Evrovmon {
    #[inline(always)]
    fn default() -> Evrovmon {
        <crate::RegValueT<Evrovmon_SPEC> as RegisterValue<_>>::new(16711422)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrovmon2_SPEC;
impl crate::sealed::RegSpec for Evrovmon2_SPEC {
    type DataType = u32;
}
#[doc = "EVR Secondary Over voltage Monitor Register 2\n resetvalue={LVD Reset:0x0FEFEFE,Cold PORST:0x0FEFEFE,After SSW execution:0x0FEFEFE}"]
pub type Evrovmon2 = crate::RegValueT<Evrovmon2_SPEC>;

impl Evrovmon2 {
    #[doc = "VDDPD Supply Secondary Monitor Over voltage threshold   PREOVVAL. This field defines the over voltage monitoring threshold level of the        VDDPD supply or EVRPR output. Ideal Threshold     VIN  160    160 LSB   160    160 1  Ideal LSB   5.7692  160 mV"]
    #[inline(always)]
    pub fn preovval(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrovmon2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrovmon2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDM Supply Secondary Monitor Over voltage threshold   VDDMOVVAL. This field defines the over voltage monitoring threshold level of the        VDDM ADC supply Ideal Threshold     VIN  160    160 LSB   160    160 1  Ideal LSB   23.077  160 mV"]
    #[inline(always)]
    pub fn vddmovval(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrovmon2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrovmon2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEVRSB Supply Secondary Monitor Over voltage threshold   SBOVVAL. This field defines the over voltage threshold level of the external        VEVRSB  3.3V  160    160 5V  standby supply monitor. Ideal Threshold     VIN  160    160 LSB   160    160 1  Ideal LSB   23.077  160 mV"]
    #[inline(always)]
    pub fn sbovval(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evrovmon2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evrovmon2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Evrovmon2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,Evrovmon2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Evrovmon2 {
    #[inline(always)]
    fn default() -> Evrovmon2 {
        <crate::RegValueT<Evrovmon2_SPEC> as RegisterValue<_>>::new(16711422)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrrstcon_SPEC;
impl crate::sealed::RegSpec for Evrrstcon_SPEC {
    type DataType = u32;
}
#[doc = "EVR Reset Control Register\n resetvalue={LVD Reset:0x597F4A,Cold PORST:0x597F4A,After SSW execution:0x5C834B}"]
pub type Evrrstcon = crate::RegValueT<Evrrstcon_SPEC>;

impl Evrrstcon {
    #[doc = "VDD Supply Reset Trim Value   RSTCTRIM. This bit field selects the hard reset generation level of VDD supply        rail. This bit field is trimmed by Firmware. RSTCTRIM  160    160   VDDx   712.5 mV    LSB   if        RSTCPTRIM 0  VDDPRIUV  160    160 712.5 mV   LSB   RSTCTRIM          LSB   RSTCPTRIM  signed value  LSB   5 mV"]
    #[inline(always)]
    pub fn rstctrim(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrrstcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrrstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDP3 Supply Reset Trim Value   RST33TRIM. This bit field selects the hard reset generation level of VDDP3 supply        rail. This bit field is trimmed by Firmware. RST33TRIM     VDDx   937.5 mV    LSB   if        RST33PTRIM 0  VDDP3PRIUV  160    160 937.5 mV   LSB   RST33TRIM   LSB   RST33PTRIM  signed        value  LSB   15 mV"]
    #[inline(always)]
    pub fn rst33trim(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrrstcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrrstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEXT Supply Reset Trim Value   RSTSWDTRIM. This bit field selects the hard reset generation level of the external        VEXT supply rail. This bitfield is trimmed by Firmware. RSTSWDTRIM     VDDx   1050 mV    LSB   if RSTSWDPTRIM 0  VEXTPRIUV   1050 mV  LSB   RSTSWDTRIM          LSB   RSTSWDPTRIM  signed value  LSB   20 mV"]
    #[inline(always)]
    pub fn rstswdtrim(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evrrstcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evrrstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDD Reset Enable   RSTCOFF. This bit can only be changed if bit BPRSTCOFF is set in parallel.        RSTCOFF is intended to be used only for internal test purposes and the        primary reset generation is not to be disabled in customer application."]
    #[inline(always)]
    pub fn rstcoff(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Evrrstcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,Evrrstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bit Protection RSTCOFF   BPRSTCOFF. Setting this bit enables that bit RSTCOFF can be changed in this write        operation. This bit is read as zero."]
    #[inline(always)]
    pub fn bprstcoff(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Evrrstcon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25,1,0,Evrrstcon_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "VDDP3 Reset Enable   RST33OFF. This bit can only be changed if bit BPRST33OFF is set in parallel. The        VDDP3 reset is disabled by application to support voltage drop up to        nominal 3.0  160 V during cranking. RST33OFF is intended to be used only for        internal test purposes and the primary reset generation is not to be        disabled in customer application."]
    #[inline(always)]
    pub fn rst33off(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Evrrstcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,Evrrstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bit Protection RST33OFF   BPRST33OFF. Setting this bit enables that bit RST33OFF can be changed in this write        operation. This bit read also as zero."]
    #[inline(always)]
    pub fn bprst33off(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Evrrstcon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27,1,0,Evrrstcon_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "VEXT Reset Enable   RSTSWDOFF. This bit can only be changed if bit BPRSTSWDOFF is set in parallel.        RSTSWDOFF is intended to be used only for internal test purposes and the        primary reset generation is not to be disabled in customer application."]
    #[inline(always)]
    pub fn rstswdoff(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Evrrstcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,Evrrstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bit Protection RSTSWDOFF   BPRSTSWDOFF. Setting this bit enables that bit RSTSWDOFF can be changed in this write        operation. This bit is read as zero."]
    #[inline(always)]
    pub fn bprstswdoff(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Evrrstcon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29,1,0,Evrrstcon_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Evrrstcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,Evrrstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Evrrstcon {
    #[inline(always)]
    fn default() -> Evrrstcon {
        <crate::RegValueT<Evrrstcon_SPEC> as RegisterValue<_>>::new(5768010)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrrststat_SPEC;
impl crate::sealed::RegSpec for Evrrststat_SPEC {
    type DataType = u32;
}
#[doc = "EVR Reset Status Register\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
pub type Evrrststat = crate::RegValueT<Evrrststat_SPEC>;

impl Evrrststat {
    #[doc = "VDD Supply Reset Value Status   RSTC. This bit field indicates the actual cold PORST reset trim setpoint for        core voltage supply rail used by the Primary monitors. The value is        updated via PMS EVRRSTCON .RSTCTRIM and PMS EVRRSTCON2 .RSTCPTRIM register. RSTC   RSTCTRIM   RSTCPTRIM  signed value  RSTC range  160    160 0 up to 255 VDDPRIUV   712.5 mV   LSB   RSTC LSB   5 mV"]
    #[inline(always)]
    pub fn rstc(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrrststat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrrststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VDDP3 Supply Reset Value Status   RST33. This bit field indicates the actual cold PORST reset trim setpoint for        3.3  160 V supply rail used by the Primary monitors. The value is updated via PMS EVRRSTCON .RST33TRIM and PMS EVRRSTCON2 .RST33PTRIM register. RST33   RST33TRIM   RST33PTRIM  signed value  RST33 range  160    160 0 up to 255 VDDP3PRIUV   937.5 mV   LSB   RST33 LSB   15 mV"]
    #[inline(always)]
    pub fn rst33(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrrststat_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrrststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VEXT Supply Reset Value Status   RSTSWD. This bit field indicates the actual cold PORST reset trim setpoint for        5  160 V supply rail used by the Primary monitors. The value is updated via PMS EVRRSTCON .RSTSWDTRIM and PMS EVRRSTCON2 .        RSTSWDPTRIM register. RSTSWD   RSTSWDTRIM          RSTSWDPTRIM  signed value  RSTSWD range  160    160 0 up to 255 VEXTPRIUV   1050 mV  LSB   RSTSWD LSB   20 mV"]
    #[inline(always)]
    pub fn rstswd(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evrrststat_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evrrststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EVRC Reset Enable Status   RSTCOFF. The value is updated via PMS EVRRSTCON .RSTCOFF        register bit."]
    #[inline(always)]
    pub fn rstcoff(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Evrrststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24,1,0,Evrrststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EVR33 Reset Enable Status   RST33OFF. The value is updated via PMS EVRRSTCON .RST33OFF        register bit."]
    #[inline(always)]
    pub fn rst33off(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Evrrststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26,1,0,Evrrststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EVR SWD Reset Enable   RSTSWDOFF. The value is updated via PMS EVRRSTCON .RSTSWDOFF        register bit."]
    #[inline(always)]
    pub fn rstswdoff(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Evrrststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28,1,0,Evrrststat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrrststat {
    #[inline(always)]
    fn default() -> Evrrststat {
        <crate::RegValueT<Evrrststat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdcoeff0_SPEC;
impl crate::sealed::RegSpec for Evrsdcoeff0_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Coefficient Register 0\n resetvalue={LVD Reset:0x0B50873B6,Cold PORST:0x0B50873B6,After SSW execution:0x0B50873B6}"]
pub type Evrsdcoeff0 = crate::RegValueT<Evrsdcoeff0_SPEC>;

impl Evrsdcoeff0 {
    #[doc = "S0 Enable m0en s0en i    M0S0EN. This bitfield enables the fast forward error term."]
    #[inline(always)]
    pub fn m0s0en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Enable m0en s2en i    M0S2EN. This bitfield enables the digital reconstruction of the inductor current."]
    #[inline(always)]
    pub fn m0s2en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Enable m0en s3en i    M0S3EN. This bitfield enables the integrator."]
    #[inline(always)]
    pub fn m0s3en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Clip m0en s3clip i    M0S3CLIP. This bitfield specifies the clipping of the integrator state to negative        values."]
    #[inline(always)]
    pub fn m0s3clip(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S4 Enable m0en s4en i    M0S4EN. This bitfield enables the double integrator branch."]
    #[inline(always)]
    pub fn m0s4en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Ramp Enable m0en rampen i    M0RAMPEN. This bitfield enables the artificial ramp in order to avoid        instabilities at high duty cycles."]
    #[inline(always)]
    pub fn m0rampen(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SFRGET m0en sfrget i    M0SFRGET. This bitfield enables the compensation of parasitic effects in the        inductor current reconstruction."]
    #[inline(always)]
    pub fn m0sfrget(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Skip Enable m0en skipen i    M0SKIPEN. This bitfield enables the skip pulse logic."]
    #[inline(always)]
    pub fn m0skipen(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Coefficient m0s3 coeff i    M0S3COEFF. Configuration register of S3   integrator coefficient."]
    #[inline(always)]
    pub fn m0s3coeff(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S4 Coefficient m0s4 coeff i    M0S4COEFF. Configuration register of S4   double integrator coefficient."]
    #[inline(always)]
    pub fn m0s4coeff(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S Ramp Coefficient m0srmp coeff i    M0SRMPCOEFF. Configuration register of S Ramp   artificial ramp coefficient."]
    #[inline(always)]
    pub fn m0srmpcoeff(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Forgetting Factor m0fget coeff i    M0FGETCOEFF. This bitfield specifies the forgetting factor for compensation of        parasitic effects."]
    #[inline(always)]
    pub fn m0fgetcoeff(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Coefficient m0s2 coeff i    M0S2COEFF. Inductor current reconstruction coefficient."]
    #[inline(always)]
    pub fn m0s2coeff(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Vin Source m0s2 vinsrc i    M0S2VINSRC. This bitfield specifies the source of the input voltage used for the        inductor current reconstruction."]
    #[inline(always)]
    pub fn m0s2vinsrc(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Vout Source m0s2 vosrc i    M0S2VOSRC. This bitfield specifies the source of the output voltage used for the        inductor current reconstruction."]
    #[inline(always)]
    pub fn m0s2vosrc(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S Ramp Fractional Coefficient. This bitfield specifies the S Ramp fractional coefficient."]
    #[inline(always)]
    pub fn m0srmpcoefffrac(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Evrsdcoeff0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Evrsdcoeff0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdcoeff0 {
    #[inline(always)]
    fn default() -> Evrsdcoeff0 {
        <crate::RegValueT<Evrsdcoeff0_SPEC> as RegisterValue<_>>::new(3037230006)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdcoeff1_SPEC;
impl crate::sealed::RegSpec for Evrsdcoeff1_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Coefficient Register 1\n resetvalue={LVD Reset:0x0A2946C46,Cold PORST:0x0A2946C46,After SSW execution:0x0A2946C46}"]
pub type Evrsdcoeff1 = crate::RegValueT<Evrsdcoeff1_SPEC>;

impl Evrsdcoeff1 {
    #[doc = "LPF Coefficient m0vocf lpf i    M0VOCFLPF. This bitfield reflects LPF coefficient used in the LPF applied to the        FB ADC counter value or the programmed register value. y  160  k   160    160    160 y  160  k 1   160    160  1 a   160    160    160    160 x  160  k   160    160 a   160   y  160  k  is filter output         x  160  k  is ADC output a  160    160  1  160    160  2  160    160 LPF  .If LPF  160    160 0  the filter output is the same as ADC        output."]
    #[inline(always)]
    pub fn m0vocflpf(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Evrsdcoeff1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Evrsdcoeff1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Output Voltage Ramp Coefficient m0vocf inc i    M0VOCFINC. This bitfield reflects increment for the output voltage ramp used in the        inductor current reconstruction. Step applied to ramp   2  160    160 M0VOCFINC."]
    #[inline(always)]
    pub fn m0vocfinc(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Evrsdcoeff1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Evrsdcoeff1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Digital representation of the target voltage m0vo lb i    M0VOUT. This bitfield can be used for the inductor current reconstruction        instead of the FBADC value."]
    #[inline(always)]
    pub fn m0vout(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdcoeff1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdcoeff1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Digital representation of the input voltage m0vinh vin i m0vinl vin i     M0VIN. This bitfield is used for the inductor current reconstruction instead of        the FFADC value. Absolute value including ADC offset."]
    #[inline(always)]
    pub fn m0vin(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, Evrsdcoeff1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16, Evrsdcoeff1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Fractional Coefficient. This bitfield specifies the S3 fractional integrator coefficient. 00   no fractional coefficient used 01 ... fractional coefficient 1 4 used  S3   0.25  10 ... fractional coefficient 1 2 used  S3   0.5  11 ... fractional coefficient 3 4 used  S3   0.75"]
    #[inline(always)]
    pub fn m0s3coefffrac(
        self,
    ) -> crate::common::RegisterField<27, 0x3, 1, 0, u8, Evrsdcoeff1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x3,1,0,u8, Evrsdcoeff1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Fractional Coefficient. This bitfield specifies the S2 fractional coefficient of the inductor        current reconstruction coefficient. 00   no fractional coefficient used 01 ... fractional coefficient 1 4 used  S2   0.25  10 ... fractional coefficient 1 2 used  S2   0.5  11 ... fractional coefficient 3 4 used  S2   0.75"]
    #[inline(always)]
    pub fn m0s2coefffrac(
        self,
    ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, Evrsdcoeff1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x3,1,0,u8, Evrsdcoeff1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Evrsdcoeff1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Evrsdcoeff1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdcoeff1 {
    #[inline(always)]
    fn default() -> Evrsdcoeff1 {
        <crate::RegValueT<Evrsdcoeff1_SPEC> as RegisterValue<_>>::new(2727636038)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdcoeff2_SPEC;
impl crate::sealed::RegSpec for Evrsdcoeff2_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Coefficient Register 2\n resetvalue={LVD Reset:0x3408710E,Cold PORST:0x3408710E,After SSW execution:0x3408710E}"]
pub type Evrsdcoeff2 = crate::RegValueT<Evrsdcoeff2_SPEC>;

impl Evrsdcoeff2 {
    #[doc = "S0 Enable m1en s0en i    M1S0EN. This bitfield enables the fast forward error term."]
    #[inline(always)]
    pub fn m1s0en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Enable m1en s2en i    M1S2EN. This bitfield enables the digital reconstruction of the inductor current."]
    #[inline(always)]
    pub fn m1s2en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Enable m1en s3en i    M1S3EN. This bitfield enables the integrator."]
    #[inline(always)]
    pub fn m1s3en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Clip m1en s3clip i    M1S3CLIP. This bitfield specifies the clipping of the integrator state to negative        values."]
    #[inline(always)]
    pub fn m1s3clip(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S4 Enable m1en s4en i    M1S4EN. This bitfield enables the double integrator branch."]
    #[inline(always)]
    pub fn m1s4en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Ramp Enable m1en rampen i    M1RAMPEN. This bitfield enables the artificial ramp in order to avoid        instabilities at high duty cycles."]
    #[inline(always)]
    pub fn m1rampen(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SFRGET m1en sfrget i    M1SFRGET. This bitfield enables the compensation of parasitic effects in the        inductor current reconstruction."]
    #[inline(always)]
    pub fn m1sfrget(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Skip Enable m1en skipen i    M1SKIPEN. This bitfield enables the skip pulse logic."]
    #[inline(always)]
    pub fn m1skipen(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Coefficient m1s3 coeff i    M1S3COEFF. Configuration register of S3   integrator coefficient."]
    #[inline(always)]
    pub fn m1s3coeff(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S4 Coefficient m1s4 coeff i    M1S4COEFF. Configuration register of S4   double integrator coefficient."]
    #[inline(always)]
    pub fn m1s4coeff(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S Ramp Coefficient m1srmp coeff i    M1SRMPCOEFF. Configuration register of S Ramp   artificial ramp coefficient."]
    #[inline(always)]
    pub fn m1srmpcoeff(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Forgetting Factor m1fget coeff i    M1FGETCOEFF. This bitfield specifies the forgetting factor for compensation of        parasitic effects."]
    #[inline(always)]
    pub fn m1fgetcoeff(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Coefficient m1s2 coeff i    M1S2COEFF. Inductor current reconstruction coefficient."]
    #[inline(always)]
    pub fn m1s2coeff(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Vin Source m1s2 vinsrc i    M1S2VINSRC. This bitfield specifies the source of the input voltage used for the        inductor current reconstruction."]
    #[inline(always)]
    pub fn m1s2vinsrc(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Vout Source m1s2 vosrc i    M1S2VOSRC. This bitfield specifies the source of the output voltage used for the        inductor current reconstruction."]
    #[inline(always)]
    pub fn m1s2vosrc(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdcoeff2 {
    #[inline(always)]
    fn default() -> Evrsdcoeff2 {
        <crate::RegValueT<Evrsdcoeff2_SPEC> as RegisterValue<_>>::new(872968462)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdcoeff3_SPEC;
impl crate::sealed::RegSpec for Evrsdcoeff3_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Coefficient Register 3\n resetvalue={LVD Reset:0x2946C44,Cold PORST:0x2946C44,After SSW execution:0x2946C44}"]
pub type Evrsdcoeff3 = crate::RegValueT<Evrsdcoeff3_SPEC>;

impl Evrsdcoeff3 {
    #[doc = "LPF Coefficient m1vocf lpf i    M1VOCFLPF. This bitfield reflects LPF coefficient used in the LPF applied to the        FB ADC counter value or the programmed register value. y  160  k   160    160    160 y  160  k 1   160    160  1 a   160    160    160    160 x  160  k   160    160 a   160   y  160  k  is filter output         x  160  k  is ADC output a  160    160  1  160    160  2  160    160 LPF  .If LPF  160    160 0  the filter output is the same as ADC        output."]
    #[inline(always)]
    pub fn m1vocflpf(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Evrsdcoeff3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Evrsdcoeff3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Output Voltage Ramp Coefficient m1vocf inc i    M1VOCFINC. This bitfield reflects increment for the output voltage ramp used in the        inductor current reconstruction. Step applied to ramp   2  160    160 M1VOCFINC."]
    #[inline(always)]
    pub fn m1vocfinc(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Evrsdcoeff3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Evrsdcoeff3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Digital representation of the target voltage m1vo lb i    M1VOUT. This bitfield can be used for the inductor current reconstruction        instead of the FBADC value."]
    #[inline(always)]
    pub fn m1vout(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdcoeff3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdcoeff3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Digital representation of the input voltage m1vinh vin i m1vinl vin i    M1VIN. This bitfield can be used for the inductor current reconstruction        instead of the FFADC value. Absolute value including ADC offset."]
    #[inline(always)]
    pub fn m1vin(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, Evrsdcoeff3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16, Evrsdcoeff3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Fractional Coefficient. This bitfield specifies the S3 fractional integrator coefficient. 00   no fractional coefficient used 01 ... fractional coefficient 1 4 used  S3   0.25  10 ... fractional coefficient 1 2 used  S3   0.5  11 ... fractional coefficient 3 4 used  S3   0.75"]
    #[inline(always)]
    pub fn m1s3coefffrac(
        self,
    ) -> crate::common::RegisterField<27, 0x3, 1, 0, u8, Evrsdcoeff3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x3,1,0,u8, Evrsdcoeff3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Fractional Coefficient. This bitfield specifies the S2 fractional coefficient of the inductor        current reconstruction coefficient. 00   no fractional coefficient used 01 ... fractional coefficient 1 4 used  S2   0.25  10 ... fractional coefficient 1 2 used  S2   0.5  11 ... fractional coefficient 3 4 used  S2   0.75"]
    #[inline(always)]
    pub fn m1s2coefffrac(
        self,
    ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, Evrsdcoeff3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x3,1,0,u8, Evrsdcoeff3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S Ramp Fractional Coefficient. This bitfield specifies the S Ramp fractional coefficient."]
    #[inline(always)]
    pub fn m1srmpcoefffrac(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Evrsdcoeff3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,Evrsdcoeff3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdcoeff3 {
    #[inline(always)]
    fn default() -> Evrsdcoeff3 {
        <crate::RegValueT<Evrsdcoeff3_SPEC> as RegisterValue<_>>::new(43281476)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdcoeff4_SPEC;
impl crate::sealed::RegSpec for Evrsdcoeff4_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Coefficient Register 4\n resetvalue={LVD Reset:0x1B0822B6,Cold PORST:0x1B0822B6,After SSW execution:0x1B0822B6}"]
pub type Evrsdcoeff4 = crate::RegValueT<Evrsdcoeff4_SPEC>;

impl Evrsdcoeff4 {
    #[doc = "S0 Enable m2en s0en i    M2S0EN. This bitfield enables the fast forward error term."]
    #[inline(always)]
    pub fn m2s0en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Enable m2en s2en i    M2S2EN. This bitfield enables the digital reconstruction of the inductor current."]
    #[inline(always)]
    pub fn m2s2en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Enable m2en s3en i    M2S3EN. This bitfield enables the integrator."]
    #[inline(always)]
    pub fn m2s3en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Clip m2en s3clip i    M2S3CLIP. This bitfield specifies the clipping of the integrator state to negative        values."]
    #[inline(always)]
    pub fn m2s3clip(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S4 Enable m2en s4en i    M2S4EN. This bitfield enables the double integrator branch."]
    #[inline(always)]
    pub fn m2s4en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Ramp Enable m2en rampen i    M2RAMPEN. This bitfield enables the artificial ramp in order to avoid        instabilities at high duty cycles."]
    #[inline(always)]
    pub fn m2rampen(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SFRGET m2en sfrget i    M2SFRGET. This bitfield enables the compensation of parasitic effects in the        inductor current reconstruction."]
    #[inline(always)]
    pub fn m2sfrget(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Skip Enable m2en skipen i    M2SKIPEN. This bitfield enables the skip pulse logic."]
    #[inline(always)]
    pub fn m2skipen(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Coefficient m2s3 coeff i    M2S3COEFF. Configuration register of S3   integrator coefficient."]
    #[inline(always)]
    pub fn m2s3coeff(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S4 Coefficient m2s4 coeff i    M2S4COEFF. Configuration register of S4   double integrator coefficient."]
    #[inline(always)]
    pub fn m2s4coeff(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S Ramp Coefficient m2srmp coeff i    M2SRMPCOEFF. Configuration register of S Ramp   artificial ramp coefficient."]
    #[inline(always)]
    pub fn m2srmpcoeff(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Forgetting Factor m2fget coeff i    M2FGETCOEFF. This bitfield specifies the forgetting factor for compensation of        parasitic effects."]
    #[inline(always)]
    pub fn m2fgetcoeff(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Coefficient m2s2 coeff i    M2S2COEFF. Inductor current reconstruction coefficient."]
    #[inline(always)]
    pub fn m2s2coeff(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Vin Source m2s2 vinsrc i    M2S2VINSRC. This bitfield specifies the source of the input voltage used for the        inductor current reconstruction."]
    #[inline(always)]
    pub fn m2s2vinsrc(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Vout Source m2s2 vosrc i    M2S2VOSRC. This bitfield specifies the source of the output voltage used for the        inductor current reconstruction."]
    #[inline(always)]
    pub fn m2s2vosrc(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdcoeff4 {
    #[inline(always)]
    fn default() -> Evrsdcoeff4 {
        <crate::RegValueT<Evrsdcoeff4_SPEC> as RegisterValue<_>>::new(453518006)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdcoeff5_SPEC;
impl crate::sealed::RegSpec for Evrsdcoeff5_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Coefficient Register 5\n resetvalue={LVD Reset:0x2946C46,Cold PORST:0x2946C46,After SSW execution:0x2946C46}"]
pub type Evrsdcoeff5 = crate::RegValueT<Evrsdcoeff5_SPEC>;

impl Evrsdcoeff5 {
    #[doc = "LPF Coefficient m2vocf lpf i    M2VOCFLPF. This bitfield reflects LPF coefficient used in the LPF applied to the        FB ADC counter value or the programmed register value. y  160  k   160    160    160 y  160  k 1   160    160  1 a   160    160    160    160 x  160  k   160    160 a   160   y  160  k  is filter output         x  160  k  is ADC output a  160    160  1  160    160  2  160    160 LPF  .If LPF  160    160 0  the filter output is the same as ADC        output."]
    #[inline(always)]
    pub fn m2vocflpf(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Evrsdcoeff5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Evrsdcoeff5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Output Voltage Ramp Coefficient m2vocf inc i    M2VOCFINC. This bitfield reflects the increment for the output voltage ramp used in        the inductor current reconstruction. Step applied to ramp   2  160    160 M2VOCFINC."]
    #[inline(always)]
    pub fn m2vocfinc(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Evrsdcoeff5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Evrsdcoeff5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Digital representation of the target voltage m2vo lb i    M2VOUT. This bitfield can be used for the inductor current reconstruction        instead of the FBADC value."]
    #[inline(always)]
    pub fn m2vout(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdcoeff5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdcoeff5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Digital representation of the input voltage m2vinh vin i m2vinl vin i    M2VIN. This bitfield can be used for the inductor current reconstruction        instead of the FFADC value. Absolute value including ADC offset."]
    #[inline(always)]
    pub fn m2vin(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, Evrsdcoeff5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16, Evrsdcoeff5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Fractional Coefficient. This bitfield specifies the S3 fractional integrator coefficient. 00   no fractional coefficient used 01 ... fractional coefficient 1 4 used  S3   0.25  10 ... fractional coefficient 1 2 used  S3   0.5  11 ... fractional coefficient 3 4 used  S3   0.75"]
    #[inline(always)]
    pub fn m2s3coefffrac(
        self,
    ) -> crate::common::RegisterField<27, 0x3, 1, 0, u8, Evrsdcoeff5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x3,1,0,u8, Evrsdcoeff5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Fractional Coefficient. This bitfield specifies the S2 fractional coefficient of the inductor        current reconstruction coefficient. 00   no fractional coefficient used 01 ... fractional coefficient 1 4 used  S2   0.25  10 ... fractional coefficient 1 2 used  S2   0.5  11 ... fractional coefficient 3 4 used  S2   0.75"]
    #[inline(always)]
    pub fn m2s2coefffrac(
        self,
    ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, Evrsdcoeff5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x3,1,0,u8, Evrsdcoeff5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S Ramp Fractional Coefficient. This bitfield specifies the S Ramp fractional coefficient."]
    #[inline(always)]
    pub fn m2srmpcoefffrac(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Evrsdcoeff5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,Evrsdcoeff5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdcoeff5 {
    #[inline(always)]
    fn default() -> Evrsdcoeff5 {
        <crate::RegValueT<Evrsdcoeff5_SPEC> as RegisterValue<_>>::new(43281478)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdcoeff6_SPEC;
impl crate::sealed::RegSpec for Evrsdcoeff6_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Coefficient Register 6\n resetvalue={LVD Reset:0x080971802,Cold PORST:0x080971802,After SSW execution:0x080971802}"]
pub type Evrsdcoeff6 = crate::RegValueT<Evrsdcoeff6_SPEC>;

impl Evrsdcoeff6 {
    #[doc = "Commutation trimming and Slope Control drv5v0 trim i    CT5REG0. Trimming of the commutation parameters of the external driver  5V ."]
    #[inline(always)]
    pub fn ct5reg0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdcoeff6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdcoeff6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Commutation trimming drv5v1 trim i    CT5REG1. Trimming of the commutation parameters of the external driver  5V ."]
    #[inline(always)]
    pub fn ct5reg1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdcoeff6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdcoeff6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Commutation trimming drv5v2 trim i    CT5REG2. Trimming of the commutation parameters of the external driver  5V ."]
    #[inline(always)]
    pub fn ct5reg2(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evrsdcoeff6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evrsdcoeff6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Evrsdcoeff6_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Evrsdcoeff6_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdcoeff6 {
    #[inline(always)]
    fn default() -> Evrsdcoeff6 {
        <crate::RegValueT<Evrsdcoeff6_SPEC> as RegisterValue<_>>::new(2157385730)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdcoeff7_SPEC;
impl crate::sealed::RegSpec for Evrsdcoeff7_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Coefficient Register 7\n resetvalue={LVD Reset:0x08000D8F7,Cold PORST:0x08000D8F7,After SSW execution:0x08000D8F7}"]
pub type Evrsdcoeff7 = crate::RegValueT<Evrsdcoeff7_SPEC>;

impl Evrsdcoeff7 {
    #[doc = "Commutation trimming drv5v3 trim i    CT5REG3. Trimming of the commutation parameters of the external driver  5V ."]
    #[inline(always)]
    pub fn ct5reg3(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdcoeff7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdcoeff7_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Commutation trimming drv5v4 trim i    CT5REG4. Trimming of the commutation parameters of the external driver  5V ."]
    #[inline(always)]
    pub fn ct5reg4(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdcoeff7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdcoeff7_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Evrsdcoeff7_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Evrsdcoeff7_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdcoeff7 {
    #[inline(always)]
    fn default() -> Evrsdcoeff7 {
        <crate::RegValueT<Evrsdcoeff7_SPEC> as RegisterValue<_>>::new(2147539191)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdcoeff8_SPEC;
impl crate::sealed::RegSpec for Evrsdcoeff8_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Coefficient Register 8\n resetvalue={LVD Reset:0x080171002,Cold PORST:0x080171002,After SSW execution:0x080171002}"]
pub type Evrsdcoeff8 = crate::RegValueT<Evrsdcoeff8_SPEC>;

impl Evrsdcoeff8 {
    #[doc = "Commutation trimming drv3v0 trim i    CT33REG0. Trimming of the commutation parameters of the external driver  3.3V ."]
    #[inline(always)]
    pub fn ct33reg0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdcoeff8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdcoeff8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Commutation trimming drv3v1 trim i    CT33REG1. Trimming of the commutation parameters of the external driver  3.3V ."]
    #[inline(always)]
    pub fn ct33reg1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdcoeff8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdcoeff8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Commutation trimming drv3v2 trim i    CT33REG2. Trimming of the commutation parameters of the external driver  3.3V ."]
    #[inline(always)]
    pub fn ct33reg2(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evrsdcoeff8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evrsdcoeff8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Evrsdcoeff8_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Evrsdcoeff8_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdcoeff8 {
    #[inline(always)]
    fn default() -> Evrsdcoeff8 {
        <crate::RegValueT<Evrsdcoeff8_SPEC> as RegisterValue<_>>::new(2148995074)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdcoeff9_SPEC;
impl crate::sealed::RegSpec for Evrsdcoeff9_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Coefficient Register 9\n resetvalue={LVD Reset:0x08000A0AF,Cold PORST:0x08000A0AF,After SSW execution:0x08000A0AF}"]
pub type Evrsdcoeff9 = crate::RegValueT<Evrsdcoeff9_SPEC>;

impl Evrsdcoeff9 {
    #[doc = "Commutation trimming drv3v3 trim i    CT33REG3. Trimming of the commutation parameters of the external driver  3.3V ."]
    #[inline(always)]
    pub fn ct33reg3(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdcoeff9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdcoeff9_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Commutation trimming drv3v4 trim i    CT33REG4. Trimming of the commutation parameters of the external driver  3.3V ."]
    #[inline(always)]
    pub fn ct33reg4(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdcoeff9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdcoeff9_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Evrsdcoeff9_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Evrsdcoeff9_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdcoeff9 {
    #[inline(always)]
    fn default() -> Evrsdcoeff9 {
        <crate::RegValueT<Evrsdcoeff9_SPEC> as RegisterValue<_>>::new(2147524783)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl0_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl0_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 0\n resetvalue={LVD Reset:0x0F0390001,Cold PORST:0x0F0390001,After SSW execution:0x0F0390001}"]
pub type Evrsdctrl0 = crate::RegValueT<Evrsdctrl0_SPEC>;

impl Evrsdctrl0 {
    #[doc = "Frequency Spread Threshold freqsp coeff i    SDFREQSPRD. This bit field defines the additional frequency spread to the nominal        EVRC regulator switching frequency during operation"]
    #[inline(always)]
    pub fn sdfreqsprd(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Evrsdctrl0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Evrsdctrl0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Regulator Switching Frequency or Over sampling Factor m0osfl fact i m0osfh fact i    SDFREQ. This bit field configures the EVRC regulator switching frequency during        closed loop operation.The switching frequency is equal to  100  160 MHz           SDFREQ 1   value. SDFREQ represents the corresponding over sampling        factor or clock cycles in a period."]
    #[inline(always)]
    pub fn sdfreq(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, Evrsdctrl0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16, Evrsdctrl0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "NMOS level during OFF state drvslo ngoff i    NGOFF. This bit field configures the state of N ch. MOSFET driver during        start up and shut down phases."]
    #[inline(always)]
    pub fn ngoff(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Evrsdctrl0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,Evrsdctrl0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PMOS level during OFF state drvslo pgoff i    PGOFF. This bitfield configures the state of Pch. MOSFET driver during start up        and shut down phases."]
    #[inline(always)]
    pub fn pgoff(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Evrsdctrl0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,Evrsdctrl0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Update request for SMPS register values   UP. This bitfield triggers the update of the current register values from        PMS FPI EVRC registers to the local SMPS module registers. It shall be ensured that ALL EVRSDCTRLx and EVRSDCOEFFx registers have        correct and coherent values across the various registers before the        update request is issued. Incase of singular register update  the other        register values should match and be consistent. After a cold PORST  the        UP bit is set as default reset value to ensure that the complete SMPS        regulator parameter set is set back to its reset state. Consequently         the UP bit is reset and a read delivers 0. The parameter update via UP        bit is not allowed in start up and low power mode."]
    #[inline(always)]
    pub fn up(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Evrsdctrl0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,Evrsdctrl0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When UP bit is set to 1  register is locked for shadow register update        and LCK bit is set to 1. Once shadow register update is done  the lock        is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Evrsdctrl0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Evrsdctrl0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdctrl0 {
    #[inline(always)]
    fn default() -> Evrsdctrl0 {
        <crate::RegValueT<Evrsdctrl0_SPEC> as RegisterValue<_>>::new(4030267393)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl1_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl1_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 1\n resetvalue={LVD Reset:0x086690708,Cold PORST:0x086690708,After SSW execution:0x086690708}"]
pub type Evrsdctrl1 = crate::RegValueT<Evrsdctrl1_SPEC>;

impl Evrsdctrl1 {
    #[doc = "Minimum Off Time m0toff mintof i    M0TOFF. This bitfield configures the minimum off time within one period in        100MHz clock cycle periods during closed loop operation."]
    #[inline(always)]
    pub fn m0toff(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdctrl1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdctrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Minimum On Time m0ton minton i    M0TON. This bitfield configures the minimum on time within one period in 100MHz        clock cycle periods during closed loop operation."]
    #[inline(always)]
    pub fn m0ton(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdctrl1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdctrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S0 coefficient m0s0 coeff i    M0S0COEFF. This bitfield indicates the S0 coefficient during closed loop operation."]
    #[inline(always)]
    pub fn m0s0coeff(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Evrsdctrl1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Evrsdctrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Dead Band m0s0 deadbd i    M0DEADBD. This bitfield specifies the dead band to block the ADC ripple during        closed loop operation."]
    #[inline(always)]
    pub fn m0deadbd(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Evrsdctrl1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Evrsdctrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ADC Zero Bin m0fcfg adczb i    M0ADCZB. This bitfield specifies the zero error bin during closed loop operation."]
    #[inline(always)]
    pub fn m0adczb(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, Evrsdctrl1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8, Evrsdctrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Skip Pulse Threshold m0skip thres i    M0SKIP. This bitfield specifies the threshold to detect a skip pulse condition        during closed loop operation.  N channel MOSFET ."]
    #[inline(always)]
    pub fn m0skip(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Evrsdctrl1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Evrsdctrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "EVRC Synchronization input enable synci0 en i    SYNCEN. This bitfield enables the input synchronization logic of EVRC SMPS        regulator. When set to 1  the DCDC will start to lock to the external        synchronization input signal. This EVRC Synchronization status is indicated in EVRSTAT.SYNCLCK status        bits."]
    #[inline(always)]
    pub fn syncen(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Evrsdctrl1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,Evrsdctrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Evrsdctrl1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Evrsdctrl1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdctrl1 {
    #[inline(always)]
    fn default() -> Evrsdctrl1 {
        <crate::RegValueT<Evrsdctrl1_SPEC> as RegisterValue<_>>::new(2255030024)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl10_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl10_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 10\n resetvalue={LVD Reset:0x5A82,Cold PORST:0x5A82,After SSW execution:0x5A82}"]
pub type Evrsdctrl10 = crate::RegValueT<Evrsdctrl10_SPEC>;

impl Evrsdctrl10 {
    #[doc = "Short to High Voltage Threshold shrth1 shvh i    SHVH. High Voltage Threshold    SDVOUTSEL  160    160 SHVH  160 x  160 5  160 mV . EVRC short to supply        alarm has the nominal values of SHVH of 1.9V and tCSHHV of 3ms."]
    #[inline(always)]
    pub fn shvh(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdctrl10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdctrl10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Short to Low Voltage Threshold shrtl1 shvl i    SHVL. Low Voltage Threshold    SDVOUTSEL  160    160 SHVL  160 x  160 5  160 mV . EVRC short to ground        alarm has the nominal values of SHVL of 0.8V and tCSHLV of 3ms."]
    #[inline(always)]
    pub fn shvl(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdctrl10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdctrl10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Short to High Detection Enable shrth0 shhven i    SHHVEN"]
    #[inline(always)]
    pub fn shhven(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Evrsdctrl10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,Evrsdctrl10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Short to Low Detection Enable shrtl0 shlven i    SHLVEN"]
    #[inline(always)]
    pub fn shlven(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Evrsdctrl10_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,Evrsdctrl10_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdctrl10 {
    #[inline(always)]
    fn default() -> Evrsdctrl10 {
        <crate::RegValueT<Evrsdctrl10_SPEC> as RegisterValue<_>>::new(23170)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl11_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl11_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 11\n resetvalue={LVD Reset:0x092070909,Cold PORST:0x092070909,After SSW execution:0x092070909}"]
pub type Evrsdctrl11 = crate::RegValueT<Evrsdctrl11_SPEC>;

impl Evrsdctrl11 {
    #[doc = "High VDD Limit for Droop request droopvh thres i    DROOPVH. This bitfield defines the VDD high voltage limit above which a positive        droop request on VDD voltage shall be ignored. VDD Droop High Limit   712.5 mV   LSB    SDVOUTSEL  SDVOUTTRIM         DROOPVH   LSB   5 mV"]
    #[inline(always)]
    pub fn droopvh(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, Evrsdctrl11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, Evrsdctrl11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Low VDD Limit for Droop request droopvl thres i    DROOPVL. This bitfield defines the VDD low voltage limit below which a negative        droop request on VDD voltage shall be ignored. VDD Droop Low Limit   712.5 mV   LSB    SDVOUTSEL  SDVOUTTRIM  DROOPVL          LSB   5 mV"]
    #[inline(always)]
    pub fn droopvl(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, Evrsdctrl11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, Evrsdctrl11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Maximum Deviation of the Synchronization Input Frequency synci1 maxdev i    SYNCMAXDEV. This bitfield defines the maximum allowed frequency deviation of the        synchronization input signal frequency from the programmed nominal DCDC        switching frequency  EVRSDCTRL0.SDFREQ . For locking         EVRSDCTRL11.SYNCMAXDEV has to be chosen to be greater or equal to the        value of EVRSDCTRL11.SYNCHYST  and unequal to zero. Violation of limit        leads to loss of synchronization.The frequency window is defined as        follows d  160  f MAXDEV   160   100 MHz   2 SYNCMAXDEV            SDFREQ 2 SYNCMAXDEV 2  SYNCMAXDEV  160   round    100 MHz   d  160  f MAXDEV     sqrt   100 MHz   d  160  f MAXDEV   2          SDFREQ 2"]
    #[inline(always)]
    pub fn syncmaxdev(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Evrsdctrl11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Evrsdctrl11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Unlock Hysteresis Window synci0 hyst i    SYNCHYST. This bitfield defines the hysteresis window for synchronization locking        and unlocking. For locking  EVRSDCTRL11.SYNCHYST has to be chosen to be        lower or equal to the value of EVRSDCTRL11.SYNCMAXDEV  and unequal to        zero. The limit is applied to the period counter running at 100 MHz. Upper unlock condition  SDFREQ          SYNCMAXDEV Upper lock condition  SDFREQ          SYNCMAXDEV   SYNCHYST Lower unlock condition   SDFREQ          SYNCMAXDEV Lower lock condition   SDFREQ          SYNCMAXDEV   SYNCHYST SYNCHYST   round   d  160  f HYST    SDFREQ   177  SYNCMAXDEV  2       d  160  f HYST    SDFREQ   177  SYNCMAXDEV    100 MHz"]
    #[inline(always)]
    pub fn synchyst(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Evrsdctrl11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Evrsdctrl11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Synchronisation Input Multiplexer   SYNCMUXSEL. This bitfield selects synchronisation input either from CCU6 or GTM        inputs to be forwarded to EVRC SMPS regulator."]
    #[inline(always)]
    pub fn syncmuxsel(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, Evrsdctrl11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, Evrsdctrl11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Evrsdctrl11_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Evrsdctrl11_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdctrl11 {
    #[inline(always)]
    fn default() -> Evrsdctrl11 {
        <crate::RegValueT<Evrsdctrl11_SPEC> as RegisterValue<_>>::new(2449934601)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl2_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl2_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 2\n resetvalue={LVD Reset:0x36033B,Cold PORST:0x36033B,After SSW execution:0x36033B}"]
pub type Evrsdctrl2 = crate::RegValueT<Evrsdctrl2_SPEC>;

impl Evrsdctrl2 {
    #[doc = "Low Power Mode Hysteresis OFFSET lpbnd offset i    LPBNDOFFSET. This bitfield defines the turn on threshold in LP mode"]
    #[inline(always)]
    pub fn lpbndoffset(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Evrsdctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Evrsdctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Low Power Mode Hysteresis Band Width lpbnd width i    LPBNDWIDTH. This bitfield defines the turn on threshold in LP mode."]
    #[inline(always)]
    pub fn lpbndwidth(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Evrsdctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Evrsdctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Low Pass Filter Coefficient lplpf coeff i    LPLPFCOEFF. This bit field configures the low pass filter coefficient for the        setting of the turn on threshold of the Sliding function."]
    #[inline(always)]
    pub fn lplpfcoeff(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Evrsdctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Evrsdctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Regulator Over sampling Factor m1osfl fact i m1osfh fact i    SDFREQLP. This bitfield configures the EVRC regulator FB ADC sampling period        during low power mode. The switching frequency is not constant."]
    #[inline(always)]
    pub fn sdfreqlp(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, Evrsdctrl2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16, Evrsdctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LPM or PWM EVRC Mode Activation   EVRCMOD. This bit switches operation mode between PWM and LPM mode."]
    #[inline(always)]
    pub fn evrcmod(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Evrsdctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,Evrsdctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Evrsdctrl2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Evrsdctrl2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdctrl2 {
    #[inline(always)]
    fn default() -> Evrsdctrl2 {
        <crate::RegValueT<Evrsdctrl2_SPEC> as RegisterValue<_>>::new(3539771)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl3_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl3_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 3\n resetvalue={LVD Reset:0x0B690810,Cold PORST:0x0B690810,After SSW execution:0x0B690810}"]
pub type Evrsdctrl3 = crate::RegValueT<Evrsdctrl3_SPEC>;

impl Evrsdctrl3 {
    #[doc = "Minimum Off Time m1toff mintof i    M1TOFF. This bitfield configures the minimum off time within one period in        100MHz clock cycle periods during LP mode."]
    #[inline(always)]
    pub fn m1toff(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdctrl3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdctrl3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Minimum On Time m1ton minton i    M1TON. This bitfield configures the minimum on time within one period in 100MHz        clock cycle periods during LP mode."]
    #[inline(always)]
    pub fn m1ton(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdctrl3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdctrl3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S0 coefficient m1s0 coeff i    M1S0COEFF. This bitfield indicates the S0 coefficient during LP mode."]
    #[inline(always)]
    pub fn m1s0coeff(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Evrsdctrl3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Evrsdctrl3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Dead Band m1s0 deadbd i    M1DEADBD. This bitfield specifies the dead band to block the ADC ripple during LP        mode."]
    #[inline(always)]
    pub fn m1deadbd(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Evrsdctrl3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Evrsdctrl3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ADC Zero Bin m1fcfg adczb i    M1ADCZB. This bitfield specifies the zero error bin during LP mode."]
    #[inline(always)]
    pub fn m1adczb(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, Evrsdctrl3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8, Evrsdctrl3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Skip Pulse Threshold m1skip thres i    M1SKIP. This bitfield is disabled in LPM mode as PFM applied by control itself."]
    #[inline(always)]
    pub fn m1skip(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Evrsdctrl3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Evrsdctrl3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdctrl3 {
    #[inline(always)]
    fn default() -> Evrsdctrl3 {
        <crate::RegValueT<Evrsdctrl3_SPEC> as RegisterValue<_>>::new(191432720)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl4_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl4_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 4\n resetvalue={LVD Reset:0x360009,Cold PORST:0x360009,After SSW execution:0x360009}"]
pub type Evrsdctrl4 = crate::RegValueT<Evrsdctrl4_SPEC>;

impl Evrsdctrl4 {
    #[doc = "Voltage OK Circuit Configuration vokcfg config i    VOKCFG. t.b.d."]
    #[inline(always)]
    pub fn vokcfg(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Evrsdctrl4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Evrsdctrl4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Regulator Switching Frequency or Over sampling Factor m2osfl fact i m2osfh fact i    SDFREQST. This bit field configures the EVRC regulator switching frequency during        closed loop start up.The switching frequency is equal to  100  160 MHz          SDFREQ  value. SDFREQ represents the corresponding over sampling factor."]
    #[inline(always)]
    pub fn sdfreqst(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, Evrsdctrl4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xfff,1,0,u16, Evrsdctrl4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdctrl4 {
    #[inline(always)]
    fn default() -> Evrsdctrl4 {
        <crate::RegValueT<Evrsdctrl4_SPEC> as RegisterValue<_>>::new(3538953)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl5_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl5_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 5\n resetvalue={LVD Reset:0x0B690808,Cold PORST:0x0B690808,After SSW execution:0x0B690808}"]
pub type Evrsdctrl5 = crate::RegValueT<Evrsdctrl5_SPEC>;

impl Evrsdctrl5 {
    #[doc = "Minimum Off Time m2toff mintof i    M2TOFF. This bitfield configures the minimum off time within one period in        100MHz clock cycle periods during closed loop operation."]
    #[inline(always)]
    pub fn m2toff(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdctrl5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdctrl5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Minimum On Time m2ton minton i    M2TON. This bitfield configures the minimum on time within one period in 100MHz        clock cycle periods during closed loop operation."]
    #[inline(always)]
    pub fn m2ton(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdctrl5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdctrl5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S0 coefficient m2s0 coeff i    M2S0COEFF. This bitfield indicates the S0 coefficient during closed loop operation."]
    #[inline(always)]
    pub fn m2s0coeff(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Evrsdctrl5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Evrsdctrl5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Dead Band m2s0 deadbd i    M2DEADBD. This bitfield specifies the dead band to block the ADC ripple during        closed loop operation."]
    #[inline(always)]
    pub fn m2deadbd(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Evrsdctrl5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Evrsdctrl5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ADC Zero Bin m2fcfg adczb i    M2ADCZB. This bitfield specifies the zero error bin during closed loop operation."]
    #[inline(always)]
    pub fn m2adczb(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, Evrsdctrl5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8, Evrsdctrl5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Skip Pulse Threshold m2skip thres i    M2SKIP. This bitfield specifies the threshold to detect a skip pulse condition        during closed loop operation.  N channel MOSFET ."]
    #[inline(always)]
    pub fn m2skip(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Evrsdctrl5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Evrsdctrl5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdctrl5 {
    #[inline(always)]
    fn default() -> Evrsdctrl5 {
        <crate::RegValueT<Evrsdctrl5_SPEC> as RegisterValue<_>>::new(191432712)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl6_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl6_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 6\n resetvalue={LVD Reset:0x080231C94,Cold PORST:0x080231C94,After SSW execution:0x080231C94}"]
pub type Evrsdctrl6 = crate::RegValueT<Evrsdctrl6_SPEC>;

impl Evrsdctrl6 {
    #[doc = "Vin threshold to switch between SINCLO or SINCHI. svinth thres i    SVINTH. This bit field specifies the threshold to decide on the ramp up        increment during startup. If Vin is below the threshold  SINCLO is taken        as ramp up increment  else if Vin is equal or above the threshold         SINCHI is taken as ramp up increment.The threshold is compared to the        FF ADC counter value  without offset."]
    #[inline(always)]
    pub fn svinth(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdctrl6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdctrl6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Vout threshold to switch from open loop start up to closed loop mode. svoth thres i    SVOTH. This bit field specifies the threshold to decide when to switch from        open loop mode to closed loop mode during startup. If Vout is below the        threshold  open loop ramp up is executed. if Vout is equal or above the        threshold  closed loop PWM in start up configuration is executed. The        threshold is compared to the low pass filtered FB ADC counter value         without offset. The switch happens only in one direction during startup        and the system does not switch back into start up mode even if threshold        is crossed in other direction."]
    #[inline(always)]
    pub fn svoth(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdctrl6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdctrl6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Increment for low input voltage. sinc sinclo i    SINCLO. This bitfield specifies the increment of the on time during open loop        ramp up during startup. If Vin is below the threshold  SVINTH   SINCLO        is taken as ramp up increment. if Vin is equal or above the threshold         SVINTH   SINCHI is taken as ramp up increment"]
    #[inline(always)]
    pub fn sinclo(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Evrsdctrl6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Evrsdctrl6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Increment for high input voltage. sinc sinchi i    SINCHI. This bitfield specifies the increment of the on time during open loop        ramp up during startup. If Vin is below the threshold  SVINTH   SINCLO        is taken as ramp up increment. if Vin is equal or above the threshold         SVINTH   SINCHI is taken as ramp up increment"]
    #[inline(always)]
    pub fn sinchi(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, Evrsdctrl6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x7,1,0,u8, Evrsdctrl6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Evrsdctrl6_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Evrsdctrl6_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdctrl6 {
    #[inline(always)]
    fn default() -> Evrsdctrl6 {
        <crate::RegValueT<Evrsdctrl6_SPEC> as RegisterValue<_>>::new(2149784724)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl7_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl7_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 7\n resetvalue={LVD Reset:0x0800000FE,Cold PORST:0x0800000FE,After SSW execution:0x0800000FE}"]
pub type Evrsdctrl7 = crate::RegValueT<Evrsdctrl7_SPEC>;

impl Evrsdctrl7 {
    #[doc = "Selection of N driver current   DRVNI. Adjustable driver strength of the N driver current"]
    #[inline(always)]
    pub fn drvni(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Evrsdctrl7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Evrsdctrl7_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "P Driver Current Boost Factor drvp strgth i    DRVPCBF. Adjustable boost factor for the P driver current"]
    #[inline(always)]
    pub fn drvpcbf(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Evrsdctrl7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Evrsdctrl7_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "P Driver Current drvp strgth i    DRVP. Base drive current of the P channel MOSFET when driven with 3.3V  160    160  5V."]
    #[inline(always)]
    pub fn drvp(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Evrsdctrl7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Evrsdctrl7_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Switching Configuration drvslo mode i    DRVSLOMODE. This bitfield configure the type of switching."]
    #[inline(always)]
    pub fn drvslomode(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Evrsdctrl7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Evrsdctrl7_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Spare bits drvspr x i    DRVSPR"]
    #[inline(always)]
    pub fn drvspr(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evrsdctrl7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evrsdctrl7_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Switching frequency division factor for external synchronisation synco divfac i    SYNCDIVFAC. This bit field defines the divider factor for the SMPS switching output        to generate DCDCSYNCO output to synchronize external regulator to the        internal EVRC regulator. The sigal is routed to pin if enabled via        PMSWCR5.DCDCSYNCO bit. All other combinations are reserved."]
    #[inline(always)]
    pub fn syncdivfac(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Evrsdctrl7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Evrsdctrl7_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Evrsdctrl7_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Evrsdctrl7_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdctrl7 {
    #[inline(always)]
    fn default() -> Evrsdctrl7 {
        <crate::RegValueT<Evrsdctrl7_SPEC> as RegisterValue<_>>::new(2147483902)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl8_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl8_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 8\n resetvalue={LVD Reset:0x09121048E,Cold PORST:0x09121048E,After SSW execution:0x09121048E}"]
pub type Evrsdctrl8 = crate::RegValueT<Evrsdctrl8_SPEC>;

impl Evrsdctrl8 {
    #[doc = "Feedback Converted Counter Value Offset fbadc2 offset i    FBADCOFFS. This bitfield configures the offset of the converted counter value of        the feedback ADC measuring the core voltage."]
    #[inline(always)]
    pub fn fbadcoffs(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdctrl8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdctrl8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FB ADC Sampling period fbadc1 smpthr i    FBADCSMP. This bitfield configures the sampling period in 100 MHz clock cycles for        the feedback ADC measuring the core voltage."]
    #[inline(always)]
    pub fn fbadcsmp(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, Evrsdctrl8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3f,1,0,u8, Evrsdctrl8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FB ADC Blanked Samples Number fbadc0 blank i    FBADCBLNK. This bitfield configures the number of feedback ADC samples that are        blanked in case of a transition of the PWM drive output to minimise        switching noise influence."]
    #[inline(always)]
    pub fn fbadcblnk(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Evrsdctrl8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Evrsdctrl8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FB ADC Counter LPF Coefficient fbadc0 lpfcnt i    FBADCLPF. This bit field configures the coefficient of the Low Pass Filter of the        feedback ADC counter value measuring the core voltage. y  160  k   160    160    160 y  160  k 1   160    160  1 a   160    160    160    160 x  160  k   160    160 a   160   y  160  k  is filter output         x  160  k  is ADC output a  160    160  1  160    160  2  160    160 LPF  .If LPF  160    160 0  the filter output is the same as ADC        output."]
    #[inline(always)]
    pub fn fbadclpf(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Evrsdctrl8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Evrsdctrl8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FB ADC Error LPF Coefficient fbadc3 lpferr i    FBADCERR. This bitfield configures the coefficient of the Low Pass Filter of the        output voltage error signal of the feedback ADC."]
    #[inline(always)]
    pub fn fbadcerr(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, Evrsdctrl8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, Evrsdctrl8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FB ADC LSB for Error Computation fbadc3 lsb i    FBADCLSB. This bitfield configures the LSB of the feedback ADC counter value used        for the error computation."]
    #[inline(always)]
    pub fn fbadclsb(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Evrsdctrl8_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,Evrsdctrl8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Evrsdctrl8_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Evrsdctrl8_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdctrl8 {
    #[inline(always)]
    fn default() -> Evrsdctrl8 {
        <crate::RegValueT<Evrsdctrl8_SPEC> as RegisterValue<_>>::new(2434860174)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl9_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl9_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 9\n resetvalue={LVD Reset:0x080000434,Cold PORST:0x080000434,After SSW execution:0x080000434}"]
pub type Evrsdctrl9 = crate::RegValueT<Evrsdctrl9_SPEC>;

impl Evrsdctrl9 {
    #[doc = "Feed Forward Converted Counter Value Offset ffadc1 offset i    FFADCOFFS. This bit field configures the offset of the converted counter value of        the feed forward ADC measuring the input VEXT voltage."]
    #[inline(always)]
    pub fn ffadcoffs(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdctrl9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdctrl9_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FF ADC Counter LPF Coefficient ffadc0 lpfcnt i    FFADCLPF. This bit field configures the coefficient of the Low Pass Filter of the        feed forward ADC counter value measuring the input VEXT voltage. y  160  k   160    160    160 y  160  k 1   160    160  1 a   160    160    160    160 x  160  k   160    160 a   160   y  160  k  is filter output         x  160  k  is ADC output a  160    160  1  160    160  2  160    160 LPF  .If LPF  160    160 0  the filter output is the same as ADC        output."]
    #[inline(always)]
    pub fn ffadclpf(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Evrsdctrl9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Evrsdctrl9_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Evrsdctrl9_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Evrsdctrl9_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdctrl9 {
    #[inline(always)]
    fn default() -> Evrsdctrl9 {
        <crate::RegValueT<Evrsdctrl9_SPEC> as RegisterValue<_>>::new(2147484724)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdstat0_SPEC;
impl crate::sealed::RegSpec for Evrsdstat0_SPEC {
    type DataType = u32;
}
#[doc = "EVR SD Status Register 0\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
pub type Evrsdstat0 = crate::RegValueT<Evrsdstat0_SPEC>;

impl Evrsdstat0 {
    #[doc = "Step Down Converter Core Voltage Feedback ADC Conversion Result   ADCFBCV. This bit field indicates the last ADC conversion result of the step down converter feedback ADC measuring VDD core voltage. VIN  xa0    xa0  LSB  xa0    xa0  ADCFBCV  xa0    xa0 EVRTRIM.SDVOUTTRIM    0.7125   xa0 V  xa0    xa0 LSB   5  xa0 mV E.g. 1.20  xa0 V   62   98"]
    #[inline(always)]
    pub fn adcfbcv(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdstat0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdstat0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DPWM Control Output Status   DPWMOUT. This bit field reflects the actual PWM output of the controller provided        to the external MOSFET switches."]
    #[inline(always)]
    pub fn dpwmout(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, Evrsdstat0_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xfff,1,0,u16, Evrsdstat0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdstat0 {
    #[inline(always)]
    fn default() -> Evrsdstat0 {
        <crate::RegValueT<Evrsdstat0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrstat_SPEC;
impl crate::sealed::RegSpec for Evrstat_SPEC {
    type DataType = u32;
}
#[doc = "EVR Status Register\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
pub type Evrstat = crate::RegValueT<Evrstat_SPEC>;

impl Evrstat {
    #[doc = "EVRC status   EVRC. This bit is set if the internal EVRC regulator is currently active. EVRC        is activated if HWCFG 2  pin level is latched high during start up        phase. The bit reflects the        DCDC regulator enable bit provided from PMS to DCDC."]
    #[inline(always)]
    pub fn evrc(self) -> crate::common::RegisterFieldBool<0, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "VDD Over voltage event flag   OVC. This bit is set if VDD secondary voltage monitor recognizes a        over voltage event. An alarm is raised to the SMU and the status bit        remains set until violation disappears."]
    #[inline(always)]
    pub fn ovc(self) -> crate::common::RegisterFieldBool<1, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "EVR33 status   EVR33. This bit is set if the internal EVR33 LDO regulator is active. EVR33 is        activated if HWCFG 1  pin level is latched high during start up phase. The bit reflects the EVR33 enable bit provided from PMS to PID."]
    #[inline(always)]
    pub fn evr33(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "VDDP3 Over voltage event flag   OV33. This bit is set if VDDP3 secondary voltage monitor recognizes a        over voltage event. An alarm is raised to the SMU and the status bit        remains set until violation disappears."]
    #[inline(always)]
    pub fn ov33(self) -> crate::common::RegisterFieldBool<3, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "VEXT Over voltage event flag   OVSWD. This bit is set if VEXT secondary voltage monitor recognizes an        over voltage event. An alarm is raised to the SMU and the status bit        remains set until violation disappears."]
    #[inline(always)]
    pub fn ovswd(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "VDD Under voltage event flag   UVC. This bit is set if VDD secondary voltage monitor recognizes a        under voltage event. An alarm is raised to the SMU and the status bit        remains set until violation disappears."]
    #[inline(always)]
    pub fn uvc(self) -> crate::common::RegisterFieldBool<5, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "VDDP3 Under voltage event flag   UV33. This bit is set if VDDP3 secondary voltage monitor recognizes a        under voltage event. An alarm is raised to the SMU and the status bit        remains set until violation disappears."]
    #[inline(always)]
    pub fn uv33(self) -> crate::common::RegisterFieldBool<6, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "VEXT Under voltage event flag   UVSWD. This bit is set if VEXT secondary voltage monitor recognizes an        under voltage event. An alarm is raised to the SMU and the status bit        remains set until violation disappears."]
    #[inline(always)]
    pub fn uvswd(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "EVRC Synchronization Input Locked status sd sync in locked o    SYNCLCK. This bitfield indicates the current synchronization status of EVRC SMPS        regulator to external DCDCSYNCI input signal. When the EVRC switching        frequency  edge is locked to the synchronization input  the SYNCLCK bit        is set to HIGH indicating the locked state. When the synchronization is        lost owing to frequency deviations beyond MAXDEV or the feature is        disabled via SYNCEN  the SYNCLCK bit is set to LOW. This EVRC Synchronization status is indicated in EVRSDSTAT0.SYNCLCK        status bits."]
    #[inline(always)]
    pub fn synclck(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "EVR33 Regulator Voltage OK status   EVR33VOK. This bit is set after the soft ramp up time of the EVR33 voltage OK ramp        detector has elapsed and is not based on the measured VDDP3 voltage at        the end of ramp phase.."]
    #[inline(always)]
    pub fn evr33vok(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "EVRC Reset Trigger   RSTC"]
    #[inline(always)]
    pub fn rstc(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "EVR33 Reset Trigger   RST33"]
    #[inline(always)]
    pub fn rst33(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "EVR SWD Reset Trigger   RSTSWD"]
    #[inline(always)]
    pub fn rstswd(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Short to ground   EVRCSHLV. This bit is set if a short condition to ground has been detected. The        measured EVRC output is below the operational supply range and the upper        controller limits are reached.The feature is supported only during        closed loop operation or EVRCMOD  160    160 00b.  evr sd short to lv"]
    #[inline(always)]
    pub fn evrcshlv(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Short to supply   EVRCSHHV. This bit is set if a short condition to supply has been detected. The        measured EVRC output exceeds the allowed supply range and the lower        controller limits are reached. The feature is supported only during        closed loop operation or EVRCMOD  160    160 00b.  evr sd short to hv"]
    #[inline(always)]
    pub fn evrcshhv(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Short to ground   EVR33SHLV. This bit is set if a short condition to ground has been detected. The        measured EVR33 output is below the operational supply range and the        lower gate drive threshold voltage driving P ch. MOSFET  VGATEL 0 1V  is reached."]
    #[inline(always)]
    pub fn evr33shlv(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Short to supply   EVR33SHHV. This bit is set if a short condition to supply has been detected. The        measured EVR33 output exceeds the allowed supply range and the upper        gate drive threshold voltage driving P ch. MOSFET  VGATEL 4 9V  is reached."]
    #[inline(always)]
    pub fn evr33shhv(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "VEXT External Supply Level Status   SWDLVL. This bit indicates that the VEXT voltage has dropped below  4  160 V to        indicate EVRC parameter switch to differentiate 5V or 3.3V external        supply. A hysteresis of  120  160 mV is implemented on this detector.  drv low vdd . The primary SWD ADC monitor value is used for EVRC SWDLVL        comparator."]
    #[inline(always)]
    pub fn swdlvl(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "EVRC Regulator Voltage OK status   SDVOK. This bit is set by the EVRC voltage OK detector to indicate that the new        regulator output value has been reached. This bit is reset incase        EVRTRIM. SDVOUTSEL or SDVOUTTRIM values are adapted to scale core        voltage and is set when the new output setpoint is reached. This bit is        also reset incase droop compensation is requested before a load jump        event. A time out period of x  160 us shall be waited when polling SDVOK bit.  status voltok o"]
    #[inline(always)]
    pub fn sdvok(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "EVRC Mode   EVRCMOD. This bit indicates the current operation mode of LC   PWM  LPM  STRT. Depending        on whether EVRC is activated the status bit is consequently updated.         status mode o"]
    #[inline(always)]
    pub fn evrcmod(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<22,0x3,1,0,u8, Evrstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Pre Regulator VDDPD Over voltage event flag   OVPRE. This bit is set if VDDPD supply secondary voltage monitor recognizes an        over voltage event. An alarm is raised to the SMU."]
    #[inline(always)]
    pub fn ovpre(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Standby Supply or VEVRSB Over voltage event flag   OVSB. This bit is set if VEVRSB supply secondary voltage monitor recognizes an        over voltage event. An alarm is raised to the SMU."]
    #[inline(always)]
    pub fn ovsb(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ADC VDDM Supply Over voltage event flag   OVDDM. This bit is set if VDDM ADC supply secondary voltage monitor recognizes        an over voltage event. An alarm is raised to the SMU."]
    #[inline(always)]
    pub fn ovddm(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pre Regulator VDDPD Under voltage event flag   UVPRE. This bit is set if VDDPD supply secondary voltage monitor recognizes an        under voltage event. An alarm is raised to the SMU."]
    #[inline(always)]
    pub fn uvpre(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Standby Supply or VEVRSB Under voltage event flag   UVSB. This bit is set if VEVRSB supply secondary voltage monitor recognizes an        under voltage event. An alarm is raised to the SMU."]
    #[inline(always)]
    pub fn uvsb(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ADC VDDM Supply Under voltage event flag   UVDDM. This bit is set if VDDM ADC supply secondary voltage monitor recognizes        an under voltage event. An alarm is raised to the SMU."]
    #[inline(always)]
    pub fn uvddm(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Evrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Evrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Evrstat {
    #[inline(always)]
    fn default() -> Evrstat {
        <crate::RegValueT<Evrstat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrtrim_SPEC;
impl crate::sealed::RegSpec for Evrtrim_SPEC {
    type DataType = u32;
}
#[doc = "EVR Trim Control Register\n resetvalue={LVD Reset:0x080006C9E,Cold PORST:0x080006C9E,After SSW execution:0x6C9E}"]
pub type Evrtrim = crate::RegValueT<Evrtrim_SPEC>;

impl Evrtrim {
    #[doc = "EVR33 Regulator Output Voltage Target Value   EVR33VOUTSEL. The VDDP3 output level of the EVR33 LDO regulator. The ramp up        completion to the new target value is indicated via EVRSTAT.EVR33VOK        bit. The  EVR33VOUTSEL   EVR33VOUTTRIM  setpoint value shall be        programmed between 0x24 and 0xDA for valid closed loop PID regulator        function. 3.3 V   9E   158 EVR33VOUTSEL     VDDP3   937.5 mV    LSB  VDDP3   937.5 mV   LSB   EVR33VOUTSEL LSB   15 mV"]
    #[inline(always)]
    pub fn evr33voutsel(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrtrim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrtrim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "EVRC Regulator Output Voltage Target Value   SDVOUTSEL. The VDD output level of the Step down regulator.  vosel target i  1.25 V   6C   108 SDVOUTSEL     VDD   712.5 mV    LSB   VDD   712.5 mV   LSB   SDVOUTSEL  LSB   5 mV. This register bitfield requires a parameter update via EVRSDCTRL0.UP for        transfer to EVRC SMPS shadow register. The reaching of the new target        value is indicated via EVRSTAT.SDVOK bit."]
    #[inline(always)]
    pub fn sdvoutsel(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrtrim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrtrim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "EVR33 Regulator Output Voltage Trim Value   EVR33VOUTTRIM. The 6 bit ADC BIST trimming value offset added to the EVR33 output level        value installed by firmware from the flash. This        field is meant for trimming during production tests. VDDP3 Setpoint   EVR33VOUTSEL   EVR33VOUTTRIM  signed value  EVR33OUTTRIM RANGE    32 to 31 LSB LSB   15 mV"]
    #[inline(always)]
    pub fn evr33vouttrim(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Evrtrim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Evrtrim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "EVRC Regulator Output Voltage Trim Value vtrim trim i    SDVOUTTRIM. The 6 bit ADC BIST trimming value offset added to the EVRC output level        value installed by firmware from the flash. The reaching of the new        setpoint is indicated via EVRSTAT.SDVOK bits VDD Setpoint   SDVOUTSEL   SDVOUTTRIM  signed value  SDVOUTTRIM RANGE    32 to 31 LSB LSB   5 mV This register bitfield requires a parameter update via EVRSDCTRL0.UP for        transfer to SMPS shadow register."]
    #[inline(always)]
    pub fn sdvouttrim(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, Evrtrim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3f,1,0,u8, Evrtrim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Evrtrim_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Evrtrim_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(self) -> crate::common::RegisterFieldBool<31, 1, 0, Evrtrim_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Evrtrim_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Evrtrim {
    #[inline(always)]
    fn default() -> Evrtrim {
        <crate::RegValueT<Evrtrim_SPEC> as RegisterValue<_>>::new(27806)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrtrimstat_SPEC;
impl crate::sealed::RegSpec for Evrtrimstat_SPEC {
    type DataType = u32;
}
#[doc = "EVR Trim Status Register\n resetvalue={LVD Reset:0x6C9E,Cold PORST:0x6C9E}"]
pub type Evrtrimstat = crate::RegValueT<Evrtrimstat_SPEC>;

impl Evrtrimstat {
    #[doc = "EVR33 Regulator Output Voltage Target Value   EVR33VOUTSEL. This bitfield indicates EVR33 output target value as configured in        EVTRIM.EVR33VOUTSEL."]
    #[inline(always)]
    pub fn evr33voutsel(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrtrimstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrtrimstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EVRC Regulator Output Voltage Target Value   SDVOUTSEL. This bit field indicates the EVRC output level of the Step down        regulator as configured in EVTRIM.SDVOUTSEL.  vosel target o"]
    #[inline(always)]
    pub fn sdvoutsel(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrtrimstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrtrimstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EVR33 Regulator Output Voltage Trim Value   EVR33VOUTTRIM. This bit field indicates the 6 bit ADC BIST trimming value offset added        to the EVR33 output level value installed by firmware from flash        configuration sector if production trimming is required."]
    #[inline(always)]
    pub fn evr33vouttrim(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Evrtrimstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Evrtrimstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EVRC Regulator Output Voltage Trim Value vtrim trim o    SDVOUTTRIM. This bit field indicates the 5 bit ADC BIST trimming value offset added        to the EVRC output level value installed by firmware from flash        configuration sector as configured in EVTRIM.SDVOUTTRIM."]
    #[inline(always)]
    pub fn sdvouttrim(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, Evrtrimstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x3f,1,0,u8, Evrtrimstat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrtrimstat {
    #[inline(always)]
    fn default() -> Evrtrimstat {
        <crate::RegValueT<Evrtrimstat_SPEC> as RegisterValue<_>>::new(27806)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evruvmon_SPEC;
impl crate::sealed::RegSpec for Evruvmon_SPEC {
    type DataType = u32;
}
#[doc = "EVR Secondary Under voltage Monitor Register\n resetvalue={LVD Reset:0x75A7B8,Cold PORST:0x75A7B8,After SSW execution:0x75A7B8}"]
pub type Evruvmon = crate::RegValueT<Evruvmon_SPEC>;

impl Evruvmon {
    #[doc = "VDD Supply Secondary Monitor Under voltage threshold   EVRCUVVAL. This field defines the under voltage monitoring threshold level of the        EVRC regulator output or VDD supply. Ideal Threshold     VIN  160    160 LSB   160    160 1  Ideal LSB   5.7692  160 mV"]
    #[inline(always)]
    pub fn evrcuvval(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evruvmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evruvmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDP3 Supply Secondary Monitor Under voltage threshold   EVR33UVVAL. This field defines the under voltage monitoring threshold level of the        EVR33 regulator output or VDDP3 supply. Ideal Threshold     VIN  160    160 LSB   160    160 1 . Ideal LSB   15.00  160 mV"]
    #[inline(always)]
    pub fn evr33uvval(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evruvmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evruvmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEXT Supply Secondary Monitor Under voltage threshold   SWDUVVAL. This field defines the under voltage threshold level of the external        VEXT supply monitor. Ideal Threshold     VIN  160    160 LSB   160    160 1  Ideal LSB   23.077  160 mV."]
    #[inline(always)]
    pub fn swduvval(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evruvmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evruvmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Evruvmon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,Evruvmon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Evruvmon {
    #[inline(always)]
    fn default() -> Evruvmon {
        <crate::RegValueT<Evruvmon_SPEC> as RegisterValue<_>>::new(7710648)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evruvmon2_SPEC;
impl crate::sealed::RegSpec for Evruvmon2_SPEC {
    type DataType = u32;
}
#[doc = "EVR Secondary Under voltage Monitor Register 2\n resetvalue={LVD Reset:0x2A7000BC,Cold PORST:0x2A7000BC,After SSW execution:0x2A7000BC}"]
pub type Evruvmon2 = crate::RegValueT<Evruvmon2_SPEC>;

impl Evruvmon2 {
    #[doc = "VDDPD Supply Secondary Monitor Under voltage threshold   PREUVVAL. This field defines the under voltage monitoring threshold level of the        VDDPD supply or EVRPR output. Ideal Threshold     VIN  160    160 LSB   160    160 1  Ideal LSB   5.7692  160 mV"]
    #[inline(always)]
    pub fn preuvval(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evruvmon2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evruvmon2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDM Supply Secondary Monitor Under voltage threshold   VDDMUVVAL. This field defines the under voltage monitoring threshold level of the        VDDM ADC supply. Ideal Threshold     VIN  160    160 LSB   160    160 1  Ideal LSB   23.077  160 mV"]
    #[inline(always)]
    pub fn vddmuvval(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evruvmon2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evruvmon2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEVRSB Supply Secondary Monitor Under voltage threshold   SBUVVAL. This field defines the under voltage threshold level of the external        VEVRSB  3.3V  160    160 5V  standby supply monitor. Ideal Threshold     VIN  160    160 LSB   160    160 1  Ideal LSB   23.077  160 mV."]
    #[inline(always)]
    pub fn sbuvval(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evruvmon2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evruvmon2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDM Level Select   VDDMLVLSEL. This field defines the under voltage monitoring threshold level required        by EVADC  160    160 EDSADC modules to differentiate between 5  160 V or 3.3  160 V VDDM        supply level to adjust analog behavior to the actual voltage level. The        6 MSB bits of the ADC result is compared against VDDMLVLSEL with 4 LSB        hysteresis. Ideal Threshold     VIN   LSB    1  Ideal LSB   92.308 mV"]
    #[inline(always)]
    pub fn vddmlvlsel(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, Evruvmon2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3f,1,0,u8, Evruvmon2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Evruvmon2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,Evruvmon2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Evruvmon2 {
    #[inline(always)]
    fn default() -> Evruvmon2 {
        <crate::RegValueT<Evruvmon2_SPEC> as RegisterValue<_>>::new(711983292)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsmovmon_SPEC;
impl crate::sealed::RegSpec for Hsmovmon_SPEC {
    type DataType = u32;
}
#[doc = "EVR Primary HSM Over voltage Monitor Register\n resetvalue={LVD Reset:0x0E1B586,Cold PORST:0x0E1B586,After SSW execution:0x0E1B586}"]
pub type Hsmovmon = crate::RegValueT<Hsmovmon_SPEC>;

impl Hsmovmon {
    #[doc = "VDD Supply Primary Monitor Alarm Over voltage threshold   EVRCOVVAL. This field defines the over voltage monitoring threshold level of the        EVRC regulator output or VDD supply. EVRCOVVAL     VDDx   712.5 mV    LSB  LSB   5 mV"]
    #[inline(always)]
    pub fn evrcovval(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Hsmovmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Hsmovmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDP3 Supply Primary Monitor Alarm Over voltage threshold   EVR33OVVAL. This field defines the over voltage monitoring threshold level of the        EVR33 regulator output or VDDP3 supply. EVR33OVVAL     VDDx   937.5 mV    LSB  LSB   15 mV"]
    #[inline(always)]
    pub fn evr33ovval(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Hsmovmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Hsmovmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEXT Supply Primary Monitor Alarm Over voltage threshold   SWDOVVAL. This field defines the over voltage threshold level of the external VEXT        supply monitor. SWDOVVAL     VDDx   1050 mV    LSB  LSB   20 mV"]
    #[inline(always)]
    pub fn swdovval(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Hsmovmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Hsmovmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDD Primary Monitor OV Alarm Disable   EVRCOFF"]
    #[inline(always)]
    pub fn evrcoff(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Hsmovmon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,Hsmovmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDP3 Primary Monitor OV Alarm Disable   EVR33OFF"]
    #[inline(always)]
    pub fn evr33off(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Hsmovmon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,Hsmovmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEXT Primary Monitor OV Alarm Disable   SWDOFF"]
    #[inline(always)]
    pub fn swdoff(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Hsmovmon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,Hsmovmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Hsmovmon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,Hsmovmon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Hsmovmon {
    #[inline(always)]
    fn default() -> Hsmovmon {
        <crate::RegValueT<Hsmovmon_SPEC> as RegisterValue<_>>::new(14792070)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsmuvmon_SPEC;
impl crate::sealed::RegSpec for Hsmuvmon_SPEC {
    type DataType = u32;
}
#[doc = "EVR Primary HSM Under voltage Monitor Register\n resetvalue={LVD Reset:0x5C824D,Cold PORST:0x5C824D,After SSW execution:0x5C824D}"]
pub type Hsmuvmon = crate::RegValueT<Hsmuvmon_SPEC>;

impl Hsmuvmon {
    #[doc = "VDD Supply Primary Monitor Alarm Under voltage threshold   EVRCUVVAL. This field defines the under voltage monitoring threshold level of the        EVRC regulator output or VDD supply.EVRCUVVAL     VDDx   712.5 mV    LSB  LSB   5 mV"]
    #[inline(always)]
    pub fn evrcuvval(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Hsmuvmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Hsmuvmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDP3 Supply Primary Monitor Alarm Under voltage threshold   EVR33UVVAL. This field defines the under voltage monitoring threshold level of the        EVR33 regulator output or VDDP3 supply. EVR33UVVAL     VDDx   937.5 mV    LSB  LSB   15 mV"]
    #[inline(always)]
    pub fn evr33uvval(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Hsmuvmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Hsmuvmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEXT Supply Primary Monitor Alarm Under voltage threshold   SWDUVVAL. This field defines the under voltage threshold level of the external        VEXT supply monitor. SWDUVVAL     VDDx   1050 mV    LSB  LSB   20 mV"]
    #[inline(always)]
    pub fn swduvval(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Hsmuvmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Hsmuvmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDD Primary Monitor UV Alarm Disable   EVRCOFF"]
    #[inline(always)]
    pub fn evrcoff(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Hsmuvmon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,Hsmuvmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDP3 Primary Monitor UV Alarm Disable   EVR33OFF"]
    #[inline(always)]
    pub fn evr33off(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Hsmuvmon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,Hsmuvmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEXT Primary Monitor UV Alarm Disable   SWDOFF"]
    #[inline(always)]
    pub fn swdoff(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Hsmuvmon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,Hsmuvmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Voltage Filter   HSMFIL"]
    #[inline(always)]
    pub fn hsmfil(
        self,
    ) -> crate::common::RegisterField<27, 0xf, 1, 0, u8, Hsmuvmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0xf,1,0,u8, Hsmuvmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Hsmuvmon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,Hsmuvmon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Hsmuvmon {
    #[inline(always)]
    fn default() -> Hsmuvmon {
        <crate::RegValueT<Hsmuvmon_SPEC> as RegisterValue<_>>::new(6062669)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Identification Register\n resetvalue={Application Reset:0x0E8C001}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MODREV. This bit field indicates the revision number of the PMS module. This is intended as a continuous numbering scheme defined by Design."]
    #[inline(always)]
    pub fn modrev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MODTYPE. This bit field is fixed coded as C0 H .        It defines a 32 bit module."]
    #[inline(always)]
    pub fn modtype(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number   MODNUMBER. This bit field defines the module identification number. The identification number for the PMS is 00E8 H ."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(15253505)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Monbistctrl_SPEC;
impl crate::sealed::RegSpec for Monbistctrl_SPEC {
    type DataType = u32;
}
#[doc = "SMU stdby BIST Control Register\n resetvalue={LVD Reset:0x0,Warm PORST:0x0}"]
pub type Monbistctrl = crate::RegValueT<Monbistctrl_SPEC>;

impl Monbistctrl {
    #[doc = "SMU stdby alarm BIST enable   TSTEN. If SMUEN in the CMD stdby register is set to 1  setting the TSTEN bit triggers SMU stdby BIST tests to test the alarm path and PMS components. This bit can only be changed if bit BITPROT is set in parallel. This bit is cleared by hardware at the end of the SMU stdby BIST operation."]
    #[inline(always)]
    pub fn tsten(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Monbistctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Monbistctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SMU stdby BIST flag clear   TSTCLR. Setting this bit enables the clearing of the bitfields TSTOK  TSTDONE  TSTRUN  SMUERR and PMSERR of register MONBISTSTAT."]
    #[inline(always)]
    pub fn tstclr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Monbistctrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,Monbistctrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Bit Protection TSTEN   BITPROT. Setting this bit enables that bit TSTEN can be changed in this write        operation. This bit is read as zero."]
    #[inline(always)]
    pub fn bitprot(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Monbistctrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30,1,0,Monbistctrl_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Monbistctrl {
    #[inline(always)]
    fn default() -> Monbistctrl {
        <crate::RegValueT<Monbistctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Monbiststat_SPEC;
impl crate::sealed::RegSpec for Monbiststat_SPEC {
    type DataType = u32;
}
#[doc = "SMU stdby BIST Status Register\n resetvalue={LVD Reset:0x0,Warm PORST:0x0}"]
pub type Monbiststat = crate::RegValueT<Monbiststat_SPEC>;

impl Monbiststat {
    #[doc = "SMU stdby BIST ok bit   TSTOK. This status bit indicates that MONBIST test was successful and the        result is as expected."]
    #[inline(always)]
    pub fn tstok(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Monbiststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,Monbiststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SMU stdby BIST run bit   TSTRUN. The status bit indicates that MONBIST test is ongoing."]
    #[inline(always)]
    pub fn tstrun(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Monbiststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,Monbiststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SMU stdby BIST done bit   TSTDONE. The status bit indicates that MONBIST test is completed."]
    #[inline(always)]
    pub fn tstdone(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Monbiststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,Monbiststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Error found in SMU stdby found by SMU stdby BIST. This status bit indicates that the MONBIST test found an error in the        SMU stdby."]
    #[inline(always)]
    pub fn smuerr(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Monbiststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,Monbiststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Error found in PMS SARADC by SMU stdby BIST. This status bit indicates that SMU stdby BIST found an error in the PMS        SARADC."]
    #[inline(always)]
    pub fn pmserr(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Monbiststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,Monbiststat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Monbiststat {
    #[inline(always)]
    fn default() -> Monbiststat {
        <crate::RegValueT<Monbiststat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otsc0_SPEC;
impl crate::sealed::RegSpec for Otsc0_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Trigger Set Control 0 Register\n resetvalue={LVD Reset:0x0,Debug Reset:0x0}"]
pub type Otsc0 = crate::RegValueT<Otsc0_SPEC>;

impl Otsc0 {
    #[doc = "OTGB0 TS16 ADCMON Low Byte   B0LAM"]
    #[inline(always)]
    pub fn b0lam(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Otsc0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Otsc0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OTGB0 TS16 ADCMON High Byte   B0HAM"]
    #[inline(always)]
    pub fn b0ham(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Otsc0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Otsc0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OTGB1 TS16 ADCMON Low Byte   B1LAM"]
    #[inline(always)]
    pub fn b1lam(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Otsc0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Otsc0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OTGB1 TS16 ADCMON High Byte   B1HAM"]
    #[inline(always)]
    pub fn b1ham(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Otsc0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Otsc0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Otsc0 {
    #[inline(always)]
    fn default() -> Otsc0 {
        <crate::RegValueT<Otsc0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otsc1_SPEC;
impl crate::sealed::RegSpec for Otsc1_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Trigger Set Control 1 Register\n resetvalue={LVD Reset:0x0,Debug Reset:0x0}"]
pub type Otsc1 = crate::RegValueT<Otsc1_SPEC>;

impl Otsc1 {
    #[doc = "OTGB0 TS16 EVRCON   B0EC"]
    #[inline(always)]
    pub fn b0ec(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Otsc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Otsc1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OTGB1 TS16 EVRCON   B1EC"]
    #[inline(always)]
    pub fn b1ec(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Otsc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Otsc1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OTGB0 TS16 EVRCON DMONAD   DMONAD. The multiplexer signal selection documented in DMONAD coding table."]
    #[inline(always)]
    pub fn dmonad(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Otsc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Otsc1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OTGB0 TS16 EVRCON SMCDBG   SMCDBG. Reserved for future extensions."]
    #[inline(always)]
    pub fn smcdbg(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Otsc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Otsc1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Otsc1 {
    #[inline(always)]
    fn default() -> Otsc1 {
        <crate::RegValueT<Otsc1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otss_SPEC;
impl crate::sealed::RegSpec for Otss_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Trigger Set Select Register\n resetvalue={LVD Reset:0x0,Debug Reset:0x0}"]
pub type Otss = crate::RegValueT<Otss_SPEC>;

impl Otss {
    #[doc = "Trigger Set for OTGB0   OTGB0"]
    #[inline(always)]
    pub fn otgb0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Otss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Otss_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Set for OTGB1   OTGB1"]
    #[inline(always)]
    pub fn otgb1(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Otss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Otss_SPEC,crate::common::RW>::from_register(self,0)
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
pub struct Pmsien_SPEC;
impl crate::sealed::RegSpec for Pmsien_SPEC {
    type DataType = u32;
}
#[doc = "PMS Interrupt Enable Register\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
pub type Pmsien = crate::RegValueT<Pmsien_SPEC>;

impl Pmsien {
    #[doc = "OVSWD Interrupt enable   OVSWD. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn ovswd(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "UVSWD Interrupt enable   UVSWD. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn uvswd(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "OV33 Interrupt enable   OV33. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn ov33(self) -> crate::common::RegisterFieldBool<2, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "UV33 Interrupt enable   UV33. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn uv33(self) -> crate::common::RegisterFieldBool<3, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "OVC Interrupt enable   OVC. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn ovc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "UVC Interrupt enable   UVC. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn uvc(self) -> crate::common::RegisterFieldBool<5, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "OVPRE Interrupt enable   OVPRE. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn ovpre(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "UVPRE Interrupt enable   UVPRE. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn uvpre(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "OVDDM Interrupt enable   OVDDM. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn ovddm(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "UVDDM Interrupt enable   UVDDM. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn uvddm(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "OVSB Interrupt enable   OVSB. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn ovsb(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "UVSB Interrupt enable   UVSB. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn uvsb(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "EVRCMOD Interrupt enable   EVRCMOD. Interrupt triggered on a state change of EVRSTAT.EVRCMOD 0  bitfield."]
    #[inline(always)]
    pub fn evrcmod(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SDVOK Interrupt enable   SDVOK. Interrupt triggered on EVRSTAT.SDVOK rising edge event."]
    #[inline(always)]
    pub fn sdvok(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD SYNCLCK Interrupt enable   SYNCLCK. Interrupt triggered on a state change of EVRSTAT.SYNCLCK bitfield."]
    #[inline(always)]
    pub fn synclck(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SWDLVL Interrupt enable   SWDLVL. Interrupt triggered on a state change of EVRSTAT.SWDLVL bitfield."]
    #[inline(always)]
    pub fn swdlvl(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "WUTWKP Interrupt enable   WUTWKP. Interrupt triggered on a WUTCNT underflow event."]
    #[inline(always)]
    pub fn wutwkp(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ESR0WKP Interrupt enable   ESR0WKP. Interrupt triggered on a ESR0WKP event."]
    #[inline(always)]
    pub fn esr0wkp(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ESR1WKP Interrupt enable   ESR1WKP. Interrupt triggered on a ESR1WKP event."]
    #[inline(always)]
    pub fn esr1wkp(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PINAWKP Interrupt enable   PINAWKP. Interrupt triggered on a PINAWKP event."]
    #[inline(always)]
    pub fn pinawkp(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PINBWKP Interrupt enable   PINBWKP. Interrupt triggered on a PINBWKP event."]
    #[inline(always)]
    pub fn pinbwkp(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SCRINT Interrupt enable   SCRINT. Interrupt triggered on a SCRINT event triggered by SCR to PMS to decode        information in PMSWCR2.SCRINT register."]
    #[inline(always)]
    pub fn scrint(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SCRRST Interrupt enable   SCRRST. Interrupt triggered by SCR to PMS on an internal SCR software reset."]
    #[inline(always)]
    pub fn scrrst(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SCRECC Interrupt enable   SCRECC. Interrupt triggered by SCR to PMS on an internal RAM double bit ECC        error."]
    #[inline(always)]
    pub fn screcc(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SCRWDT Interrupt enable   SCRWDT. Interrupt triggered by SCR to PMS on an internal SCR watchdog timeout        error."]
    #[inline(always)]
    pub fn scrwdt(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Pmsien_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Pmsien_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Pmsien {
    #[inline(always)]
    fn default() -> Pmsien {
        <crate::RegValueT<Pmsien_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmswcr0_SPEC;
impl crate::sealed::RegSpec for Pmswcr0_SPEC {
    type DataType = u32;
}
#[doc = "Standby and Wake up Control Register 0\n resetvalue={LVD Reset:0x1002D0}"]
pub type Pmswcr0 = crate::RegValueT<Pmswcr0_SPEC>;

impl Pmswcr0 {
    #[doc = "Standby Entry on VEXT Supply ramp down   VEXTSTBYEN. This bit field enables Standby Entry on VEXT supply ramp down. This is        supported only in case Standby domain is supplied separately via VEVRSB        supply pin and VEXT rail is switched off during Standby. The voltage        threshold for entry is configured in EVRUVMON register. Current        configuration is reflected in PMSWSTAT2.VEXTSTBYEN register bit."]
    #[inline(always)]
    pub fn vextstbyen(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Pmswcr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Pmswcr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Standby Entry on VDD Supply ramp down   VDDSTBYEN. This bit field enables Standby Entry on VDD supply ramp down. This is        supported only in case Standby domain is supplied separately via VEVRSB        supply pin and VDD rail is switched off during Standby. The voltage        threshold for entry is configured in EVRUVMON register. Current        configuration is reflected in PMSWSTAT2.VDDSTBYEN register bit."]
    #[inline(always)]
    pub fn vddstbyen(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Pmswcr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Pmswcr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ESR0 Digital Filter Enable   ESR0DFEN. This bit activates digital spike filter. If the digital filter  majority        filter of 3 consecutive values  is enabled during normal RUN mode  then        pulses less than 30ns are suppressed and pulses longer than 100ns will        always result in a trigger. If the back up clock is disabled in Standby        mode and filter is running on 70  160 KHz Standby clock  then pulses less        than 5 us are suppressed and pulses longer than 50 us will always result        in a trigger."]
    #[inline(always)]
    pub fn esr0dfen(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Pmswcr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pmswcr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ESR0 Edge Detection Control   ESR0EDCON. This bit field defines the edge of a ESR0 wake up trigger"]
    #[inline(always)]
    pub fn esr0edcon(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, Pmswcr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8, Pmswcr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ESR1 Digital Filter Enable   ESR1DFEN. This bit activates digital spike filter. If the digital filter  majority        filter of 3 consecutive values  is enabled during normal RUN mode  then        pulses less than 30ns are suppressed and pulses longer than 100ns will        always result in a trigger. If the back up clock is disabled in Standby        mode and filter is running on 70 KHz Standby clock  then pulses less        than 5 us are suppressed and pulses longer than 50 us will always result        in a trigger."]
    #[inline(always)]
    pub fn esr1dfen(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Pmswcr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Pmswcr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ESR1 Edge Detection Control   ESR1EDCON. This bit field defines the edge of a ESR1 wake up trigger"]
    #[inline(always)]
    pub fn esr1edcon(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Pmswcr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Pmswcr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PINA Digital Filter Enable   PINADFEN. This bit activates digital spike filter. If the digital filter  majority        filter of 3 consecutive values  is enabled during normal RUN mode  then        pulses less than 40ns are suppressed and pulses longer than 100ns will        always result in a trigger. If the back up clock is disabled in Standby        mode and filter is running on 70 KHz Standby clock  then pulses less        than 5 us are suppressed and pulses longer than 50 us will always result        in a trigger."]
    #[inline(always)]
    pub fn pinadfen(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Pmswcr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Pmswcr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PINA Edge Detection Control   PINAEDCON. This bit field defines the edge of a Pin A wake up trigger"]
    #[inline(always)]
    pub fn pinaedcon(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, Pmswcr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x3,1,0,u8, Pmswcr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PINB Digital Filter Enable   PINBDFEN. This bit activates digital spike filter. If the digital filter  majority        filter of 3 consecutive values  is enabled during normal RUN mode  then        pulses less than 40ns are suppressed and pulses longer than 100ns will        always result in a trigger. If the back up clock is disabled in Standby        mode and filter is running on 70 KHz Standby clock  then pulses less        than 5 us are suppressed and pulses longer than 50 us will always result        in a trigger."]
    #[inline(always)]
    pub fn pinbdfen(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Pmswcr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Pmswcr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PINB Edge Detection Control   PINBEDCON. This bit field defines the edge of a Pin B wake up trigger"]
    #[inline(always)]
    pub fn pinbedcon(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Pmswcr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Pmswcr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Standby RAM supply in Standby Mode   STBYRAMSEL. This bit field configures the Standby RAM blocks to be kept supplied        during Standby Mode from VDDPD supply rail. The current configuration is        reflected in PMSWSTAT2.STBYRAM bitfield. All other bit combinations are reserved."]
    #[inline(always)]
    pub fn stbyramsel(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Pmswcr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Pmswcr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Blanking Filter delay for Wake up   BLNKFIL. This bitfield enables a nominal blanking filter delay time immediately        after Standby entry only after which a valid wake up event is recognized        and reacted upon. The actual delay may vary     160 30  to this nominal        value. Current configuration is reflected in PMSWSTAT2.BLNKFIL bitfield. All other bit combinations are reserved. Incase WUT is used as a          wake up source  the blanking filter should be configured for a period          greater than 3x 70kHz clock cycles."]
    #[inline(always)]
    pub fn blnkfil(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Pmswcr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Pmswcr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ESR0 Wake up enable from Standby   ESR0WKEN. This bit configures wake up via ESR0 pin from STANDBY mode and current        configuration is reflected in PMSWSTAT2.ESR0WKEN register bit."]
    #[inline(always)]
    pub fn esr0wken(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Pmswcr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Pmswcr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ESR1 Wake up enable from Standby   ESR1WKEN. This bit configures wake up via ESR1 pin from STANDBY mode and current        configuration is reflected in PMSWSTAT2.ESR1WKEN register bit."]
    #[inline(always)]
    pub fn esr1wken(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Pmswcr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Pmswcr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pin A Wake up enable from Standby   PINAWKEN. This bit configures wake up via PINA pin from STANDBY mode and current        configuration is reflected in PMSWSTAT2.PINAWKEN register bit."]
    #[inline(always)]
    pub fn pinawken(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Pmswcr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Pmswcr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pin B Wake up enable from Standby   PINBWKEN. This bit configures wake up via PINB pin from STANDBY mode and current        configuration is reflected in PMSWSTAT2.PINBWKEN register bit."]
    #[inline(always)]
    pub fn pinbwken(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Pmswcr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Pmswcr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Standby Wake up Enable on VEXT Supply ramp up   PWRWKEN. This bit field enables wake up on VEXT supply ramp up after blanking        filter time has expired. This is supported only in case Standby domain        is supplied separately via VEVRSB supply pin and VEXT rail is switched        off during Standby. Current configuration is reflected in        PMSWSTAT2.PWRWKEN register bit."]
    #[inline(always)]
    pub fn pwrwken(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Pmswcr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Pmswcr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Standby Controller Wake up enable from Standby   SCRWKEN. This bit configures wake up via SCR from STANDBY mode and current        configuration is reflected in PMSWSTAT2.SCRWKEN register bit."]
    #[inline(always)]
    pub fn scrwken(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Pmswcr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Pmswcr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PORST pin Wake up enable from Standby   PORSTWKEN. This bit configures wake up via PORST pin from STANDBY mode and current        configuration is reflected in PMSWSTAT2.PORSTWKEN register bit."]
    #[inline(always)]
    pub fn porstwken(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Pmswcr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Pmswcr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "WUT Wake up enable from Standby   WUTWKEN. This bit configures wake up via WUT from STANDBY mode and current        configuration is reflected in PMSWSTAT2.WUTWKEN register bit."]
    #[inline(always)]
    pub fn wutwken(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Pmswcr0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Pmswcr0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Pmswcr0 {
    #[inline(always)]
    fn default() -> Pmswcr0 {
        <crate::RegValueT<Pmswcr0_SPEC> as RegisterValue<_>>::new(1049296)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmswcr2_SPEC;
impl crate::sealed::RegSpec for Pmswcr2_SPEC {
    type DataType = u32;
}
#[doc = "Standby and Wake up Control Register 2\n resetvalue={LVD Reset:0x4000000}"]
pub type Pmswcr2 = crate::RegValueT<Pmswcr2_SPEC>;

impl Pmswcr2 {
    #[doc = "Data exchange from Standby Controller to PMS main domain.   SCRINT. This bit field allows fast data exchange from SCR to PMS CPUx. The data        maybe read by CPUx consequent to an interrupt from the SCR to decode the        interrupt. Incase SCR is enabled  at the end of the SCR Firmware        routine  a value of 80H is set in SCRINT register to indicate that SCR        has finished executing the startup code."]
    #[inline(always)]
    pub fn scrint(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Pmswcr2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Pmswcr2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SCR RAM ECC error   reset flag   SCRECC. The flag is set by SCR and cleared by explicit write to the register          bit. The flag is not cleared by SCR.While the SCR is being reset          triggered by SCR RAM ECC error  this flag is set and clearing the flag          is not possible for that duration."]
    #[inline(always)]
    pub fn screcc(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Pmswcr2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Pmswcr2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SCR Watchdog Timer error   reset flag   SCRWDT. The flag is set by SCR and cleared by explicit write to the register          bit. The flag is not cleared by SCR.While the SCR is being reset          triggered by SCR watchdog  this flag is set and clearing the flag is          not possible for that duration."]
    #[inline(always)]
    pub fn scrwdt(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Pmswcr2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Pmswcr2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SCR Software reset flag   SCRRST. The flag is set by SCR and cleared by explicit write to the register          bit. The flag is not cleared by SCR. While the SCR is being reset          triggered by SCR software  this flag is set and clearing  the flag is not possible for that duration."]
    #[inline(always)]
    pub fn scrrst(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Pmswcr2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Pmswcr2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Data exchange from PMS main domain to Standby Controller.   TCINT. This bit field allows fast data exchange from PMS to SCR. The data may        be read by SCR consequent to an interrupt request  TCINTREQ  from        PMS CPUx to SCR to decode the interrupt."]
    #[inline(always)]
    pub fn tcint(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Pmswcr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Pmswcr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SW Interrupt request from PMS to Standby Controller.   TCINTREQ. Setting this bit triggers an interrupt to the 8 bit Standby controller. The TCINTREQ bit is cleared after SCR has latched the TCINT value and          SCR NMI is issued."]
    #[inline(always)]
    pub fn tcintreq(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Pmswcr2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Pmswcr2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SMU Reset indication flag   SMURST. SCR is explicitly informed in case reset issued by SMU. The flag is          cleared after SCR has latched the signal and SCR NMI is issued."]
    #[inline(always)]
    pub fn smurst(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Pmswcr2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Pmswcr2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Application or System Reset indication flag   RST. SCR is explicitly informed in case of an application or system reset.          The flag is cleared after SCR has latched the signal and SCR NMI is          issued."]
    #[inline(always)]
    pub fn rst(self) -> crate::common::RegisterFieldBool<26, 1, 0, Pmswcr2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Pmswcr2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Pmswcr2 {
    #[inline(always)]
    fn default() -> Pmswcr2 {
        <crate::RegValueT<Pmswcr2_SPEC> as RegisterValue<_>>::new(67108864)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmswcr3_SPEC;
impl crate::sealed::RegSpec for Pmswcr3_SPEC {
    type DataType = u32;
}
#[doc = "Standby and Wake up Control Register 3\n resetvalue={LVD Reset:0x0}"]
pub type Pmswcr3 = crate::RegValueT<Pmswcr3_SPEC>;

impl Pmswcr3 {
    #[doc = "WUT reload value.   WUTREL. The counter starts counting down from WUTREL value. The current value of        counter is indicated in WUTCNT. On WUTCNT underflow  a reload        WUTCNT  160    160 WUTREL takes place in auto reload mode."]
    #[inline(always)]
    pub fn wutrel(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, Pmswcr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffff,1,0,u32, Pmswcr3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "WUT enable   WUTEN. This bit enables the Wake up Timer. The status bit PMSWSTAT.WUTEN is set        once Wake up Timer is enabled."]
    #[inline(always)]
    pub fn wuten(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Pmswcr3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Pmswcr3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Lock Status   LCK. This bit indicates that the register is busy owing to ongoing bus        access. The register can be updated with a new value when BUSY bit is        cleared. The register requires synchronization to the 70kHz clock domain        on a register update."]
    #[inline(always)]
    pub fn busy(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Pmswcr3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Pmswcr3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "WUT clock divider   WUTDIV. A write to this register bitfield may trigger immediate update        irrespective of the status of BUSY bit."]
    #[inline(always)]
    pub fn wutdiv(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Pmswcr3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Pmswcr3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "WUT mode selection   WUTMODE. This bit configures the Wake up Timer mode. The status bit        PMSWSTAT.WUTMODE is respectively updated. A write to this register        bitfield may trigger immediate update irrespective of the status of BUSY        bit."]
    #[inline(always)]
    pub fn wutmode(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Pmswcr3_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Pmswcr3_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Pmswcr3 {
    #[inline(always)]
    fn default() -> Pmswcr3 {
        <crate::RegValueT<Pmswcr3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmswcr4_SPEC;
impl crate::sealed::RegSpec for Pmswcr4_SPEC {
    type DataType = u32;
}
#[doc = "Standby and Wake up Control Register 4\n resetvalue={LVD Reset:0x20,After SSW execution:0x2000020}"]
pub type Pmswcr4 = crate::RegValueT<Pmswcr4_SPEC>;

impl Pmswcr4 {
    #[doc = "Standby Controller Reset request enable   SCRSTEN"]
    #[inline(always)]
    pub fn bpscrstreq(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Pmswcr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pmswcr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Standby Controller Reset request   SCRSTREQ"]
    #[inline(always)]
    pub fn scrstreq(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Pmswcr4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pmswcr4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Bit Protection for PORSTREQ   PORSTEN"]
    #[inline(always)]
    pub fn bpporstreq(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Pmswcr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pmswcr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "SCR Reset behavior on warm PORST in Normal RUN   SLEEP mode   PORSTREQ"]
    #[inline(always)]
    pub fn porstreq(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Pmswcr4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Pmswcr4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Default Clock selection on Standby Mode Entry   SCRCLKSEL"]
    #[inline(always)]
    pub fn scrclksel(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Pmswcr4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Pmswcr4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Hardware configuration of the 8 bit SCR controller.   SCRCFG. Any change in SCRCFG is followed by a SCRSTREQ reset request of the 8          bit controller to start off in the chosen mode. All other bit          combinations are reserved. Writing to PMSWCR4.SCRCFG with values             USERMODE1 0 will have an immediate effect on the enabling of debug          pins."]
    #[inline(always)]
    pub fn scrcfg(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Pmswcr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Pmswcr4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Standby Controller Reset request enable   BPSCREN"]
    #[inline(always)]
    pub fn bpscren(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Pmswcr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Pmswcr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Standby Controller Enable request   SCREN. SCR MBIST maybe activated independent of this bit."]
    #[inline(always)]
    pub fn scren(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Pmswcr4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Pmswcr4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Pmswcr4 {
    #[inline(always)]
    fn default() -> Pmswcr4 {
        <crate::RegValueT<Pmswcr4_SPEC> as RegisterValue<_>>::new(32)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmswcr5_SPEC;
impl crate::sealed::RegSpec for Pmswcr5_SPEC {
    type DataType = u32;
}
#[doc = "Standby and Wake up Control Register 5\n resetvalue={LVD Reset:0x0}"]
pub type Pmswcr5 = crate::RegValueT<Pmswcr5_SPEC>;

impl Pmswcr5 {
    #[doc = "Bit protection for Tristate request bit  TRISTREQ    BPTRISTREQ. Setting this bit enables that bit TRISTREQ can be changed by a write        operation."]
    #[inline(always)]
    pub fn bptristreq(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Pmswcr5_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pmswcr5_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Tristate enable   TRISTREQ. This bit decides whether pads behave as inputs with weak pull up or        tristate on reset assertion de assertion or Standby  Wake up transition.        After supply ramp up or LVD reset  TRISTREQ  160    160    160 HWCFG6."]
    #[inline(always)]
    pub fn tristreq(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Pmswcr5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pmswcr5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ESR0 Tristate enable   ESR0TRIST. This bit configures ESR0 pin behavior either as reset output or tristate        during Standby mode if VEXT is supplied."]
    #[inline(always)]
    pub fn esr0trist(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Pmswcr5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Pmswcr5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PORST Digital Filter enable   PORSTDF. This bit field enables additional PORST digital filter    160 tPORSTDF        parameter  160   to provide enhanced immunity against spurious spikes. The filter is implemented over 20 cycles x 40 ns clock which results in        a total delay between 800ns    2   trimmed fback clock  and is active        only in operational mode. In Standby mode  the filter is implemented        over 2 x 70kHz clock such that pulses less than 8 us are suppressed and        pulses longer than 70 us will result in a wake up trigger."]
    #[inline(always)]
    pub fn porstdf(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Pmswcr5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pmswcr5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "DC DC Synchronisation Output. This bitfield enables the synchronisation output to synchronize the external SMPS regulator with respect to the internal EVRC regulator. This enable bit controls the ENPS signal to the port pin  so that the synchronisation signal is also available during active warm PORST phase. If enabled  this signal need to be also routed to HW DIR  amp  HWEN of the port pin."]
    #[inline(always)]
    pub fn dcdcsynco(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Pmswcr5_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Pmswcr5_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Pmswcr5 {
    #[inline(always)]
    fn default() -> Pmswcr5 {
        <crate::RegValueT<Pmswcr5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmswstat_SPEC;
impl crate::sealed::RegSpec for Pmswstat_SPEC {
    type DataType = u32;
}
#[doc = "Standby and Wake up Status Register\n resetvalue={LVD Reset:0x0A0000}"]
pub type Pmswstat = crate::RegValueT<Pmswstat_SPEC>;

impl Pmswstat {
    #[doc = "EVR Hardware Configuration status   HWCFGEVR. This bit field indicates the supply configuration latched by the EVR        from HWCFG 2 1  during a cold startup based on which EVRx regulators are        consequently started. The latched configuration is used during        STANDBY RUN transition to reselect EVR mode. The HWCFG pins are latched continuously during a period of 60us as        measure to improve robustness owing to noise injection."]
    #[inline(always)]
    pub fn hwcfgevr(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, Pmswstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x3,1,0,u8, Pmswstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Hardware Configuration Pin 4 status   HWCFG4. This bit field indicates the latched level of HWCFG 4  during a cold        startup. The HWCFG pin is latched continuously during a period of 60us as measure        to improve robustness owing to noise injection."]
    #[inline(always)]
    pub fn hwcfg4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Pmswstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pmswstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Hardware Configuration Pin 5 status   HWCFG5. This bit field indicates the latched level of HWCFG 5  during a cold        startup. The HWCFG pin is latched continuously during a period of 60us as measure        to improve robustness owing to noise injection."]
    #[inline(always)]
    pub fn hwcfg5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Pmswstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Pmswstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pad Tristate   Pull up status   TRIST. This bit indicates whether pads are configured as inputs with weak        pull up or as tristate during after reset or after wake up. At start up         the value latched from HWCFG 6  pin decides the default state and is        reflected in TRIST status bit. This bit may be later updated when        PMSWCR5.TRISTREQ is set to override initial latched status from HWCFG 6 ."]
    #[inline(always)]
    pub fn trist(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Pmswstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Pmswstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TESTMODE Pin status   TESTMODE. This bit field indicates the latched level of TESTMODE pin during a cold        startup. The HWCFG pin is latched continuously during a period of 60us as measure        to improve robustness owing to noise injection."]
    #[inline(always)]
    pub fn testmode(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Pmswstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Pmswstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ESR0 pin status during Standby   ESR0TRIST. This bit indicates if ESR0 pin is configured as reset output or tristate        during Standby mode  amp  transitions if VEXT is supplied.This bit is        updated when PMSWCR5. ESR0TRIST is set."]
    #[inline(always)]
    pub fn esr0trist(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Pmswstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Pmswstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PORST Digital Filter status   PORSTDF. This bit field indicates whether additional PORST digital filter is        activated. This bit is updated when PMSWCR5.PORSTDF is set."]
    #[inline(always)]
    pub fn porstdf(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Pmswstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Pmswstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Standby Controller status   SCR. This bit indicates whether SCR is enabled. This bit is updated when        PMSWCR4.SCREN bit is set."]
    #[inline(always)]
    pub fn scr(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Pmswstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Pmswstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Standby Controller Reset Indication flag   SCRST. This bit is set after a power on reset as SCR is in reset state. This bit is consequently set when a reset is issued via PMSWCR4.SCRSTREQ bit. This status flag is set on every SCR reset caused by any SCR reset source."]
    #[inline(always)]
    pub fn scrst(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Pmswstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Pmswstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Current Clock configuration for SCR before Standby Mode Entry   SCRCLK. This bit is updated when PMSWCR4.SCRCLKSEL bit is set."]
    #[inline(always)]
    pub fn scrclk(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Pmswstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Pmswstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Standby Controller Reset on warm PORST   PORSTREQ. This bit is updated when PMSWCR4.PORSTREQ bit is set."]
    #[inline(always)]
    pub fn porstreq(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Pmswstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Pmswstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "WUT Enable status   WUTEN. This bit indicates whether WUT is enabled. This bit is updated when        PMSWCR3.WUTEN bit is updated."]
    #[inline(always)]
    pub fn wuten(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Pmswstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Pmswstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "WUT Run status   WUTRUN. This bit indicates whether WUT is currently running. Due to        synchronization to 70 KHz    160 fSB  160   WUT clock  setting of flag after        enable may take up to 55  160 us."]
    #[inline(always)]
    pub fn wutrun(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Pmswstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Pmswstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "WUT Mode status   WUTMODE. This bit indicates the current WUT mode. This bit is updated when        PMSWCR3.WUTMODE bit is updated."]
    #[inline(always)]
    pub fn wutmode(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Pmswstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Pmswstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ESR0 Interrupt flag   ESR0INT. In case interrupt was triggered by ESR0 pin event during RUN mode  this        flag is set. The bit shall be cleared explicitly via        PMSWSTATCLR.ESR0INTCLR bit after interrupt is serviced."]
    #[inline(always)]
    pub fn esr0int(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Pmswstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Pmswstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ESR1 Interrupt flag   ESR1INT. In case interrupt was triggered by ESR1 pin event during RUN mode  this        flag is set. The bit shall be cleared explicitly via        PMSWSTATCLR.ESR1INTCLR bit after interrupt is serviced."]
    #[inline(always)]
    pub fn esr1int(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Pmswstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Pmswstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pin A Interrupt flag   PINAINT. In case interrupt was triggered by PINA pin event during RUN mode  this        flag is set. The bit shall be cleared explicitly via        PMSWSTATCLR.PINAINTCLR bit after interrupt is serviced."]
    #[inline(always)]
    pub fn pinaint(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Pmswstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Pmswstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pin B Interrupt flag   PINBINT. In case interrupt was triggered by PINB pin event during RUN mode  this        flag is set. The bit shall be cleared explicitly via        PMSWSTATCLR.PINBINTCLR bit after interrupt is serviced."]
    #[inline(always)]
    pub fn pinbint(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Pmswstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Pmswstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Pmswstat {
    #[inline(always)]
    fn default() -> Pmswstat {
        <crate::RegValueT<Pmswstat_SPEC> as RegisterValue<_>>::new(655360)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmswstat2_SPEC;
impl crate::sealed::RegSpec for Pmswstat2_SPEC {
    type DataType = u32;
}
#[doc = "Standby and Wake up Status Register 2\n resetvalue={LVD Reset:0x100000}"]
pub type Pmswstat2 = crate::RegValueT<Pmswstat2_SPEC>;

impl Pmswstat2 {
    #[doc = "ESR0 Wake up flag   ESR0WKP. In case wake up was triggered by ESR0 pin during STANDBY  this flag is        set. The bit shall be cleared explicitly after wakeup via        PMSWSTATCLR.ESR0WKPCLR bit before next STANDBY entry."]
    #[inline(always)]
    pub fn esr0wkp(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pmswstat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ESR1 Wake up flag   ESR1WKP. In case wake up was triggered by ESR1 pin during STANDBY  this flag is        set. The bit shall be cleared explicitly after wakeup via        PMSWSTATCLR.ESR1WKPCLR bit before next STANDBY entry."]
    #[inline(always)]
    pub fn esr1wkp(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pmswstat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pin Wake up flag   PINAWKP. In case wake up was triggered by PINA pin during STANDBY  this flag is        set. The bit shall be cleared explicitly after wakeup via        PMSWSTATCLR.PINAWKPCLR bit before next STANDBY entry."]
    #[inline(always)]
    pub fn pinawkp(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Pmswstat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pin B Wake up flag   PINBWKP. In case wake up was triggered by PINB pin during STANDBY  this flag is        set. The bit shall be cleared explicitly after wakeup via        PMSWSTATCLR.PINBWKPCLR bit before next STANDBY entry."]
    #[inline(always)]
    pub fn pinbwkp(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Pmswstat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wake up event on VEXT Supply ramp up   PWRWKP. In case wake up was triggered by VEXT ramp up pin during STANDBY  this        flag is set. The bit shall be cleared explicitly after wakeup via        PMSWSTATCLR.PWRWKPCLR bit before next STANDBY entry."]
    #[inline(always)]
    pub fn pwrwkp(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pmswstat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SCR Wake up flag   SCRWKP. In case wake up is triggered by SCR to the main controller during        STANDBY  this flag is set. The bit shall be cleared explicitly after        wakeup via PMSWSTATCLR.SCRWKPCLR bit before next STANDBY entry."]
    #[inline(always)]
    pub fn scrwkp(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Pmswstat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PORST Wake up flag   PORSTWKP. In case wake up was triggered by PORST pin during STANDBY  this flag is        set. The bit shall be cleared explicitly after wakeup via        PMSWSTATCLR.PORSTWKPCLR bit before next STANDBY entry."]
    #[inline(always)]
    pub fn porstwkp(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Pmswstat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "WUT Wake up flag   WUTWKP. In case wake up was triggered by Wake up timer during STANDBY  this flag        is set. The bit shall be cleared explicitly after wakeup via        PMSWSTATCLR.WUTWKPCLR bit before next STANDBY entry."]
    #[inline(always)]
    pub fn wutwkp(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Pmswstat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ESR0 Overrun status flag   ESR0OVRUN. This flag indicates that a consecutive ESR0 wake up event occurred while        ESR0WKP flag was already set during STANDBY. The bit shall be cleared        explicitly after wakeup via PMSWSTATCLR.ESR0OVRUNCLR bit before next        STANDBY entry."]
    #[inline(always)]
    pub fn esr0ovrun(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Pmswstat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ESR1 Overrun status flag   ESR1OVRUN. This flag indicates that a consecutive ESR1 wake up event occurred while        ESR1WKP flag was already set during STANDBY. The bit shall be cleared        explicitly after wakeup via PMSWSTATCLR.ESR1OVRUNCLR bit before next        STANDBY entry."]
    #[inline(always)]
    pub fn esr1ovrun(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Pmswstat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pin A Overrun status flag   PINAOVRUN. This flag indicates that a consecutive PINA wake up event occurred while        PINAWKP flag was already set during STANDBY. The bit shall be cleared        explicitly after wakeup via PMSWSTATCLR.PINAOVRUNCLR bit before next        STANDBY entry."]
    #[inline(always)]
    pub fn pinaovrun(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,Pmswstat2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Pin B Overrun status flag   PINBOVRUN. This flag indicates that a consecutive PINB wake up event occurred while        PINBWKP flag was already set during STANDBY. The bit shall be cleared        explicitly after wakeup via PMSWSTATCLR.PINBOVRUNCLR bit before next        STANDBY entry."]
    #[inline(always)]
    pub fn pinbovrun(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,Pmswstat2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Standby Entry Enable status on VDD Supply ramp down   VDDSTBYWKEN. This bit indicates that Standby Entry may be triggered on a VDD Supply        undervoltage event  VDDUV . This is supported only when Standby domain        is supplied separately by VEVRSB Standby supply pin.This bit is updated        when PMSWCR0.VDDSTBYWKEN bit is updated."]
    #[inline(always)]
    pub fn vddstbyen(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,Pmswstat2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SCR Overrun status flag   SCROVRUN. This flag indicates that a consecutive SCR wake up event occurred while        SCRWKP flag was already set during STANDBY. The bit shall be cleared        explicitly after wakeup via PMSWSTATCLR.SCROVRUNCLR bit before next        STANDBY entry."]
    #[inline(always)]
    pub fn scrovrun(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,Pmswstat2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PORST Overrun status flag   PORSTOVRUN. This flag indicates that a consecutive PORST wake up event occurred        while PORSTWKP flag was already set during STANDBY. The bit shall be        cleared explicitly after wakeup via PMSWSTATCLR.PORSTOVRUNCLR bit before        next STANDBY entry."]
    #[inline(always)]
    pub fn porstovrun(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,Pmswstat2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "WUT Overrun status flag   WUTOVRUN. This flag indicates that a consecutive WUT wake up event occurred while        WUTWKP flag was already set during STANDBY. The bit shall be cleared        explicitly after wakeup via PMSWSTATCLR.WUTOVRUNCLR bit before next        STANDBY entry. WUTREL need to be greater than 10 during Standby mode to        be able to latch consecutive WUT underflow events and update the        WUTOVRRUN register bitfield."]
    #[inline(always)]
    pub fn wutovrun(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,Pmswstat2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Standby RAM Supply status   STBYRAM. This bit field indicates whether Standby RAM was supplied during Standby        Mode and to infer status after a wake up event. This bit is updated when        PMSWCR0.STBYRAMSEL is set. All other bit combinations are reserved. In case of VDDPD Standby          supply fail or VEVRSB supply fail leading to LVD reset  indicated also          in RSTSTAT.STBYR   the STBYRAM status bit is reset to 000b to indicate          that Standby RAM contents may be corrupted."]
    #[inline(always)]
    pub fn stbyram(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Pmswstat2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Standby Entry Enable status on VEXT Supply ramp down   VEXTSTBYWKEN. This bit indicates that Standby Entry may be triggered on a VEXT Supply        undervoltage event  SWDUV . This is supported only when Standby domain        is supplied separately by VEVRSB Standby supply pin.This bit is updated        when PMSWCR0.VEXTSTBYWKEN bit is updated."]
    #[inline(always)]
    pub fn vextstbyen(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19,1,0,Pmswstat2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Blanking Filter Delay for VEXT Supply Wake up   BLNKFIL. This bit field indicates the Blanking filter configuration. This bit        field is updated with the value configured in PMSWCR0.BLNKFIL bitfield. All other bit combinations are reserved."]
    #[inline(always)]
    pub fn blnkfil(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Pmswstat2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ESR0 Wake up enable status   ESR0WKEN. This bit indicates that ESR0 is enabled to trigger wake up from Standby.        This bit is updated when PMSWCR0.ESR0WKEN bit is updated."]
    #[inline(always)]
    pub fn esr0wken(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24,1,0,Pmswstat2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ESR1 Wake up enable status   ESR1WKEN. This bit indicates that ESR1 is enabled to trigger wake up from Standby.        This bit is updated when PMSWCR0.ESR1WKEN bit is updated."]
    #[inline(always)]
    pub fn esr1wken(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25,1,0,Pmswstat2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Pin A Wake up enable status   PINAWKEN. This bit indicates that PINA is enabled to trigger wake up from Standby.        This bit is updated when PMSWCR0.PINAWKEN bit is updated."]
    #[inline(always)]
    pub fn pinawken(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26,1,0,Pmswstat2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Pin B Wake up enable status   PINBWKEN. This bit indicates that PINB is enabled to trigger wake up from Standby.        This bit is updated when PMSWCR0.PINBWKEN bit is updated."]
    #[inline(always)]
    pub fn pinbwken(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27,1,0,Pmswstat2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Standby Wake up Enable status on VEXT Supply ramp up   PWRWKEN. This bit indicates that VEXT detector is enabled to trigger wake up from        Standby during VEXT supply ramp up after blanking filter time has        expired. This is supported only when Standby domain is supplied        separately by VEVRSB Standby supply pin.This bit is updated when        PMSWCR0.PWRWKEN bit is updated."]
    #[inline(always)]
    pub fn pwrwken(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28,1,0,Pmswstat2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Standby Controller Wake up Enable status   SCRWKEN. This bit indicates that SCR is enabled to trigger wake up from Standby.        This bit is updated when PMSWCR0.SCRWKEN bit is updated."]
    #[inline(always)]
    pub fn scrwken(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29,1,0,Pmswstat2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PORST pin Wake up enable status from Standby   PORSTWKEN. This bit indicates that wake up via PORST pin is enabled during STANDBY        mode. This bit is updated when PMSWCR0. PORSTWKEN bit is updated."]
    #[inline(always)]
    pub fn porstwken(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30,1,0,Pmswstat2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "WUT Wake up enable status   WUTWKEN. This bit indicates that WUT is enabled to trigger wake up from Standby.        This bit is updated when PMSWCR0.WUTWKEN bit is updated."]
    #[inline(always)]
    pub fn wutwken(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Pmswstat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,Pmswstat2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pmswstat2 {
    #[inline(always)]
    fn default() -> Pmswstat2 {
        <crate::RegValueT<Pmswstat2_SPEC> as RegisterValue<_>>::new(1048576)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmswstatclr_SPEC;
impl crate::sealed::RegSpec for Pmswstatclr_SPEC {
    type DataType = u32;
}
#[doc = "Standby and Wake up Status Clear Register\n resetvalue={LVD Reset:0x0}"]
pub type Pmswstatclr = crate::RegValueT<Pmswstatclr_SPEC>;

impl Pmswstatclr {
    #[doc = "ESR0 Wake up indication flag clear   ESR0WKPCLR"]
    #[inline(always)]
    pub fn esr0wkpclr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Pmswstatclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,Pmswstatclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "ESR1 Wake up indication flag clear   ESR1WKPCLR"]
    #[inline(always)]
    pub fn esr1wkpclr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Pmswstatclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,Pmswstatclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "PINA Wake up indication flag clear   PINAWKPCLR"]
    #[inline(always)]
    pub fn pinawkpclr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Pmswstatclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,Pmswstatclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "PINB Wake up indication flag clear   PINBWKPCLR"]
    #[inline(always)]
    pub fn pinbwkpclr(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Pmswstatclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,Pmswstatclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "PWRWKP Wake up indication flag clear   PWRWKPCLR"]
    #[inline(always)]
    pub fn pwrwkpclr(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Pmswstatclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,Pmswstatclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "SCR Wake up indication flag clear   SCRWKPCLR"]
    #[inline(always)]
    pub fn scrwkpclr(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Pmswstatclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5,1,0,Pmswstatclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "PORST Wake up indication flag clear   PORSTWKPCLR"]
    #[inline(always)]
    pub fn porstwkpclr(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Pmswstatclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6,1,0,Pmswstatclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "WUT Wake up indication flag clear   WUTWKPCLR"]
    #[inline(always)]
    pub fn wutwkpclr(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Pmswstatclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7,1,0,Pmswstatclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "ESR0 Overrun status indication flag clear   ESR0OVRUNCLR"]
    #[inline(always)]
    pub fn esr0ovrunclr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Pmswstatclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8,1,0,Pmswstatclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "ESR1 Overrun status indication flag clear   ESR1OVRUNCLR"]
    #[inline(always)]
    pub fn esr1ovrunclr(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Pmswstatclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9,1,0,Pmswstatclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "PINA Overrun status indication flag clear   PINAOVRUNCLR"]
    #[inline(always)]
    pub fn pinaovrunclr(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Pmswstatclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10,1,0,Pmswstatclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "PINB Overrun status indication flag clear   PINBOVRUNCLR"]
    #[inline(always)]
    pub fn pinbovrunclr(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Pmswstatclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11,1,0,Pmswstatclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "SCR Overrun status indication flag clear   SCROVRUNCLR"]
    #[inline(always)]
    pub fn scrovrunclr(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Pmswstatclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13,1,0,Pmswstatclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "PORST Overrun status indication flag clear   PORSTOVRUNCLR"]
    #[inline(always)]
    pub fn porstovrunclr(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Pmswstatclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14,1,0,Pmswstatclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "WUT Overrun status indication flag clear   WUTOVRUNCLR"]
    #[inline(always)]
    pub fn wutovrunclr(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Pmswstatclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15,1,0,Pmswstatclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Standby controller SCRST indication flag clear   SCRSTCLR"]
    #[inline(always)]
    pub fn scrstclr(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Pmswstatclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16,1,0,Pmswstatclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "ESR0 Interrupt indication flag clear   ESR0INTCLR"]
    #[inline(always)]
    pub fn esr0intclr(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Pmswstatclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28,1,0,Pmswstatclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "ESR1 Interrupt indication flag clear   ESR1INTCLR"]
    #[inline(always)]
    pub fn esr1intclr(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Pmswstatclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29,1,0,Pmswstatclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "PINA Interrupt indication flag clear   PINAINTCLR"]
    #[inline(always)]
    pub fn pinaintclr(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Pmswstatclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30,1,0,Pmswstatclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "PINB Interrupt indication flag clear   PINBINTCLR"]
    #[inline(always)]
    pub fn pinbintclr(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Pmswstatclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31,1,0,Pmswstatclr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Pmswstatclr {
    #[inline(always)]
    fn default() -> Pmswstatclr {
        <crate::RegValueT<Pmswstatclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmswutcnt_SPEC;
impl crate::sealed::RegSpec for Pmswutcnt_SPEC {
    type DataType = u32;
}
#[doc = "Standby WUT Counter Register\n resetvalue={LVD Reset:0x0}"]
pub type Pmswutcnt = crate::RegValueT<Pmswutcnt_SPEC>;

impl Pmswutcnt {
    #[doc = "WUT counter value.   WUTCNT. The current WUT counter value is indicated in this register bitfield.        The WUTCNT value may have a deviation of 3 additional clock cycles to        the expected counter value owing to synchronization overheads. The WUT        clock is based on standby 70 kHz clock with       160 30  variation. The        counter depending on the mode can run through a RUN to STANDBY to RUN        mode transition without interruption."]
    #[inline(always)]
    pub fn wutcnt(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, Pmswutcnt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32, Pmswutcnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pmswutcnt {
    #[inline(always)]
    fn default() -> Pmswutcnt {
        <crate::RegValueT<Pmswutcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
