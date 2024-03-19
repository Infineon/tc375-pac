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
#[doc = r"DMU"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmu(pub(super) *mut u8);
unsafe impl core::marker::Send for Dmu {}
unsafe impl core::marker::Sync for Dmu {}
impl Dmu {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn hf_accen0(&self) -> crate::common::Reg<self::HfAccen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "Cranking Control Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn hf_ccontrol(
        &self,
    ) -> crate::common::Reg<self::HfCcontrol_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }

    #[doc = "Clear Error Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hf_clre(&self) -> crate::common::Reg<self::HfClre_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }

    #[doc = "Flash Confirm Status Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hf_confirm0(&self) -> crate::common::Reg<self::HfConfirm0_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Flash Confirm Status Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hf_confirm1(&self) -> crate::common::Reg<self::HfConfirm1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "Flash Confirm Status Register 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hf_confirm2(&self) -> crate::common::Reg<self::HfConfirm2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }

    #[doc = "Flash Control Register\n resetvalue={Application Reset:0x320}"]
    #[inline(always)]
    pub const fn hf_control(&self) -> crate::common::Reg<self::HfControl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "DFLASH Wait Cycle Register\n resetvalue={System Reset:0x4000B}"]
    #[inline(always)]
    pub const fn hf_dwait(&self) -> crate::common::Reg<self::HfDwait_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(108usize)) }
    }

    #[doc = "DF0 ECC Control Register\n resetvalue={Application Reset:0x0C0000000}"]
    #[inline(always)]
    pub const fn hf_eccc(&self) -> crate::common::Reg<self::HfEccc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }

    #[doc = "DF0 ECC Read Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hf_eccr(&self) -> crate::common::Reg<self::HfEccr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "DF0 ECC Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hf_eccs(&self) -> crate::common::Reg<self::HfEccs_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }

    #[doc = "DF0 ECC Write Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hf_eccw(&self) -> crate::common::Reg<self::HfEccw_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(76usize)) }
    }

    #[doc = "Enable Error Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hf_eer(&self) -> crate::common::Reg<self::HfEer_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }

    #[doc = "Error Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hf_errsr(&self) -> crate::common::Reg<self::HfErrsr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0E7C000}"]
    #[inline(always)]
    pub const fn hf_id(&self) -> crate::common::Reg<self::HfId_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Margin Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hf_margin(&self) -> crate::common::Reg<self::HfMargin_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(244usize)) }
    }

    #[doc = "Flash Operation Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn hf_operation(
        &self,
    ) -> crate::common::Reg<self::HfOperation_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "Power Control Register\n resetvalue={Application Reset:0x30000}"]
    #[inline(always)]
    pub const fn hf_pcontrol(
        &self,
    ) -> crate::common::Reg<self::HfPcontrol_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(100usize)) }
    }

    #[doc = "Debug Interface Protection Configuration\n resetvalue={Application Reset:0x100,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hf_procondbg(
        &self,
    ) -> crate::common::Reg<self::HfProcondbg_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }

    #[doc = "DFLASH Protection Configuration\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hf_procondf(&self) -> crate::common::Reg<self::HfProcondf_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }

    #[doc = "PFLASH Protection Configuration\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hf_proconpf(&self) -> crate::common::Reg<self::HfProconpf_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "RAM Configuration\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hf_proconram(
        &self,
    ) -> crate::common::Reg<self::HfProconram_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(140usize)) }
    }

    #[doc = "Tuning Protection Configuration\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hf_procontp(&self) -> crate::common::Reg<self::HfProcontp_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }

    #[doc = "DF0 User Mode Control\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hf_proconusr(
        &self,
    ) -> crate::common::Reg<self::HfProconusr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(116usize)) }
    }

    #[doc = "Flash Protection Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hf_protect(&self) -> crate::common::Reg<self::HfProtect_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Power Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hf_pstatus(&self) -> crate::common::Reg<self::HfPstatus_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }

    #[doc = "PFLASH Wait Cycle Register\n resetvalue={System Reset:0x716040B}"]
    #[inline(always)]
    pub const fn hf_pwait(&self) -> crate::common::Reg<self::HfPwait_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(104usize)) }
    }

    #[doc = "Flash Status Register\n resetvalue={Application Reset:0x0FF}"]
    #[inline(always)]
    pub const fn hf_status(&self) -> crate::common::Reg<self::HfStatus_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "Suspend Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hf_suspend(&self) -> crate::common::Reg<self::HfSuspend_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(240usize)) }
    }

    #[doc = "HSM Clear Error Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sf_clre(&self) -> crate::common::Reg<self::SfClre_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(131128usize)) }
    }

    #[doc = "HSM Flash Configuration Register\n resetvalue={Application Reset:0x1}"]
    #[inline(always)]
    pub const fn sf_control(&self) -> crate::common::Reg<self::SfControl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(131092usize)) }
    }

    #[doc = "HSM DF1 ECC Control Register\n resetvalue={Application Reset:0x0C0000000}"]
    #[inline(always)]
    pub const fn sf_eccc(&self) -> crate::common::Reg<self::SfEccc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(131144usize)) }
    }

    #[doc = "HSM DF1 ECC Read Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sf_eccr(&self) -> crate::common::Reg<self::SfEccr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(131136usize)) }
    }

    #[doc = "HSM DF1 ECC Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sf_eccs(&self) -> crate::common::Reg<self::SfEccs_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(131140usize)) }
    }

    #[doc = "HSM DF1 ECC Write Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sf_eccw(&self) -> crate::common::Reg<self::SfEccw_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(131148usize)) }
    }

    #[doc = "HSM Enable Error Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sf_eer(&self) -> crate::common::Reg<self::SfEer_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(131120usize)) }
    }

    #[doc = "HSM Error Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sf_errsr(&self) -> crate::common::Reg<self::SfErrsr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(131124usize)) }
    }

    #[doc = "HSM DF1 Margin Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sf_margin(&self) -> crate::common::Reg<self::SfMargin_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(131308usize)) }
    }

    #[doc = "HSM Flash Operation Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn sf_operation(
        &self,
    ) -> crate::common::Reg<self::SfOperation_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(131096usize)) }
    }

    #[doc = "HSM DF1 User Mode Control\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn sf_proconusr(
        &self,
    ) -> crate::common::Reg<self::SfProconusr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(131188usize)) }
    }

    #[doc = "HSM Flash Status Register\n resetvalue={Application Reset:0x2}"]
    #[inline(always)]
    pub const fn sf_status(&self) -> crate::common::Reg<self::SfStatus_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(131088usize)) }
    }

    #[doc = "HSM Suspend Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sf_suspend(&self) -> crate::common::Reg<self::SfSuspend_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(131304usize)) }
    }

    #[doc = "HSM Interface Protection Configuration\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn sp_proconhsm(
        &self,
    ) -> crate::common::Reg<self::SpProconhsm_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(196672usize)) }
    }

    #[doc = "HSM Code Boot Sector\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn sp_proconhsmcbs(
        &self,
    ) -> crate::common::Reg<self::SpProconhsmcbs_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(196612usize)) }
    }

    #[doc = "HSM Protection Configuration\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn sp_proconhsmcfg(
        &self,
    ) -> crate::common::Reg<self::SpProconhsmcfg_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(196608usize)) }
    }

    #[doc = "HSM Code OTP Protection Configuration\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn sp_proconhsmcotp0(
        &self,
    ) -> crate::common::Reg<self::SpProconhsmcotp0_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(196624usize)) }
    }

    #[doc = "HSM Code OTP Protection Configuration\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn sp_proconhsmcotp1(
        &self,
    ) -> crate::common::Reg<self::SpProconhsmcotp1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(196628usize)) }
    }

    #[doc = "HSM Code Exclusive Protection Configuration\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn sp_proconhsmcx0(
        &self,
    ) -> crate::common::Reg<self::SpProconhsmcx0_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(196616usize)) }
    }

    #[doc = "HSM Code Exclusive Protection Configuration\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn sp_proconhsmcx1(
        &self,
    ) -> crate::common::Reg<self::SpProconhsmcx1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(196620usize)) }
    }
    #[doc = "HP"]
    #[inline(always)]
    pub fn hp(self) -> [self::Hp; 2] {
        unsafe {
            [
                self::Hp(self.0.add(0x10000usize + 0x0usize)),
                self::Hp(self.0.add(0x10000usize + 0x100usize)),
            ]
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfAccen0_SPEC;
impl crate::sealed::RegSpec for HfAccen0_SPEC {
    type DataType = u32;
}
#[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type HfAccen0 = crate::RegValueT<HfAccen0_SPEC>;

impl HfAccen0 {
    #[doc = "Access Enable for Master TAG ID 2   EN2. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, HfAccen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 2   EN2. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, HfAccen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 2   EN2. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, HfAccen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 3   EN3. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID 3"]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, HfAccen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, HfAccen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, HfAccen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, HfAccen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, HfAccen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, HfAccen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en9(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, HfAccen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,HfAccen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,HfAccen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,HfAccen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,HfAccen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,HfAccen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,HfAccen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,HfAccen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,HfAccen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,HfAccen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,HfAccen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20,1,0,HfAccen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21,1,0,HfAccen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22,1,0,HfAccen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23,1,0,HfAccen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,HfAccen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,HfAccen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26,1,0,HfAccen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 27   EN27. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,HfAccen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 28   EN28. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID 28"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,HfAccen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 29   EN29. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID 29"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,HfAccen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 30   EN30. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID 30"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,HfAccen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read   write access to the module kernel addresses for transactions with the Master TAG ID 30"]
    #[inline(always)]
    pub fn en31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, HfAccen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,HfAccen0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for HfAccen0 {
    #[inline(always)]
    fn default() -> HfAccen0 {
        <crate::RegValueT<HfAccen0_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfCcontrol_SPEC;
impl crate::sealed::RegSpec for HfCcontrol_SPEC {
    type DataType = u32;
}
#[doc = "Cranking Control Register\n resetvalue={System Reset:0x0}"]
pub type HfCcontrol = crate::RegValueT<HfCcontrol_SPEC>;

impl HfCcontrol {
    #[doc = "Cranking Mode Control   CRANKING. This bit field determins Cranking mode."]
    #[inline(always)]
    pub fn cranking(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, HfCcontrol_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, HfCcontrol_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for HfCcontrol {
    #[inline(always)]
    fn default() -> HfCcontrol {
        <crate::RegValueT<HfCcontrol_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfClre_SPEC;
impl crate::sealed::RegSpec for HfClre_SPEC {
    type DataType = u32;
}
#[doc = "Clear Error Register\n resetvalue={Application Reset:0x0}"]
pub type HfClre = crate::RegValueT<HfClre_SPEC>;

impl HfClre {
    #[doc = "Clear Command Sequence Error   CSQER"]
    #[inline(always)]
    pub fn csqer(self) -> crate::common::RegisterFieldBool<1, 1, 0, HfClre_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, HfClre_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Protection Error   CPROER"]
    #[inline(always)]
    pub fn cproer(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, HfClre_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, HfClre_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Program Verify Error   CPVER"]
    #[inline(always)]
    pub fn cpver(self) -> crate::common::RegisterFieldBool<3, 1, 0, HfClre_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, HfClre_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Erase Verify Error   CEVER"]
    #[inline(always)]
    pub fn cever(self) -> crate::common::RegisterFieldBool<4, 1, 0, HfClre_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, HfClre_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear SRI Bus Address ECC Error   CADER"]
    #[inline(always)]
    pub fn cader(self) -> crate::common::RegisterFieldBool<5, 1, 0, HfClre_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, HfClre_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for HfClre {
    #[inline(always)]
    fn default() -> HfClre {
        <crate::RegValueT<HfClre_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfConfirm0_SPEC;
impl crate::sealed::RegSpec for HfConfirm0_SPEC {
    type DataType = u32;
}
#[doc = "Flash Confirm Status Register 0\n resetvalue={Application Reset:0x0}"]
pub type HfConfirm0 = crate::RegValueT<HfConfirm0_SPEC>;

impl HfConfirm0 {
    #[doc = "UCB BMHD0 ORIG Confirmation   PROINBMHD0O. This bit reflects the confirmed state of UCB BMHD0 ORIG."]
    #[inline(always)]
    pub fn proinbmhd0o(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, HfConfirm0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3,1,0,u8, HfConfirm0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB BMHD1 ORIG Confirmation   PROINBMHD1O. This bit reflects the confirmed state of UCB BMHD1 ORIG."]
    #[inline(always)]
    pub fn proinbmhd1o(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, HfConfirm0_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x3,1,0,u8, HfConfirm0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB BMHD2 ORIG Confirmation   PROINBMHD2O. This bit reflects the confirmed state of UCB BMHD2 ORIG."]
    #[inline(always)]
    pub fn proinbmhd2o(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, HfConfirm0_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x3,1,0,u8, HfConfirm0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB BMHD3 ORIG Confirmation   PROINBMHD3O. This bit reflects the confirmed state of UCB BMHD3 ORIG."]
    #[inline(always)]
    pub fn proinbmhd3o(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, HfConfirm0_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x3,1,0,u8, HfConfirm0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB SSW Confirmation   PROINSSW. This bit reflects the confirmed state of UCB SSW."]
    #[inline(always)]
    pub fn proinssw(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, HfConfirm0_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3,1,0,u8, HfConfirm0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB USER Confirmation   PROINUSER. This bit reflects the confirmed state of UCB USER."]
    #[inline(always)]
    pub fn proinuser(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, HfConfirm0_SPEC, crate::common::R> {
        crate::common::RegisterField::<10,0x3,1,0,u8, HfConfirm0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB TEST Confirmation   PROINTEST. This bit reflects the confirmed state of UCB TEST."]
    #[inline(always)]
    pub fn prointest(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, HfConfirm0_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x3,1,0,u8, HfConfirm0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB HSMCFG Confirmation   PROINHSMCFG. This bit reflects the confirmed state of UCB HSMCFG."]
    #[inline(always)]
    pub fn proinhsmcfg(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, HfConfirm0_SPEC, crate::common::R> {
        crate::common::RegisterField::<14,0x3,1,0,u8, HfConfirm0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB BMHD0 COPY Confirmation   PROINBMHD0C. This bit reflects the confirmed state of UCB BMHD0 COPY."]
    #[inline(always)]
    pub fn proinbmhd0c(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, HfConfirm0_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3,1,0,u8, HfConfirm0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB BMHD1 COPY Confirmation   PROINBMHD1C. This bit reflects the confirmed state of UCB BMHD1 COPY."]
    #[inline(always)]
    pub fn proinbmhd1c(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, HfConfirm0_SPEC, crate::common::R> {
        crate::common::RegisterField::<18,0x3,1,0,u8, HfConfirm0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB BMHD2 COPY Confirmation   PROINBMHD2C. This bit reflects the confirmed state of UCB BMHD2 COPY."]
    #[inline(always)]
    pub fn proinbmhd2c(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, HfConfirm0_SPEC, crate::common::R> {
        crate::common::RegisterField::<20,0x3,1,0,u8, HfConfirm0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB BMHD3 COPY Confirmation   PROINBMHD3C. This bit reflects the confirmed state of UCB BMHD3 COPY."]
    #[inline(always)]
    pub fn proinbmhd3c(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, HfConfirm0_SPEC, crate::common::R> {
        crate::common::RegisterField::<22,0x3,1,0,u8, HfConfirm0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB REDSEC Confirmation   PROINREDSEC. This bit reflects the confirmed state of UCB REDSEC"]
    #[inline(always)]
    pub fn proinredsec(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, HfConfirm0_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x3,1,0,u8, HfConfirm0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB RETEST Confirmation   PROINSRT. This bit reflects the confirmed state of UCB RETEST."]
    #[inline(always)]
    pub fn proinsrt(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, HfConfirm0_SPEC, crate::common::R> {
        crate::common::RegisterField::<30,0x3,1,0,u8, HfConfirm0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for HfConfirm0 {
    #[inline(always)]
    fn default() -> HfConfirm0 {
        <crate::RegValueT<HfConfirm0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfConfirm1_SPEC;
impl crate::sealed::RegSpec for HfConfirm1_SPEC {
    type DataType = u32;
}
#[doc = "Flash Confirm Status Register 1\n resetvalue={Application Reset:0x0}"]
pub type HfConfirm1 = crate::RegValueT<HfConfirm1_SPEC>;

impl HfConfirm1 {
    #[doc = "UCB PFLASH ORIG Confirmation   PROINPO. This bit reflects the confirmed state of UCB PFLASH ORIG."]
    #[inline(always)]
    pub fn proinpo(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, HfConfirm1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3,1,0,u8, HfConfirm1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB DFLASH ORIG Confirmation   PROINDO. This bit reflects the confirmed state of UCB DFLASH ORIG."]
    #[inline(always)]
    pub fn proindo(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, HfConfirm1_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x3,1,0,u8, HfConfirm1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB DBG ORIG Confirmation   PROINDBGO. This bit reflects the confirmed state of UCB DBG ORIG."]
    #[inline(always)]
    pub fn proindbgo(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, HfConfirm1_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x3,1,0,u8, HfConfirm1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB HSM ORIG Confirmation   PROINHSMO. This bit reflects the confirmed state of UCB HSM ORIG."]
    #[inline(always)]
    pub fn proinhsmo(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, HfConfirm1_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x3,1,0,u8, HfConfirm1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB HSMCOTP0 ORIG Protection   PROINHSMCOTP0O. This bit reflects the confirmed state of UCB HSMCOTP0 ORIG."]
    #[inline(always)]
    pub fn proinhsmcotp0o(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, HfConfirm1_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3,1,0,u8, HfConfirm1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB HSMCOTP1 ORIG Protection   PROINHSMCOTP1O. This bit reflects the confirmed state of UCB HSMCOTP1 ORIG."]
    #[inline(always)]
    pub fn proinhsmcotp1o(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, HfConfirm1_SPEC, crate::common::R> {
        crate::common::RegisterField::<10,0x3,1,0,u8, HfConfirm1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB ECPRIO ORIG Confirmation   PROINECO. This bit reflects the confirmed state of UCB ECPRIO ORIG."]
    #[inline(always)]
    pub fn proineco(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, HfConfirm1_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x3,1,0,u8, HfConfirm1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB PFLASH COPY Confirmation   PROINPC. This bit reflects the confirmed state of UCB PFLASH COPY."]
    #[inline(always)]
    pub fn proinpc(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, HfConfirm1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3,1,0,u8, HfConfirm1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB DFLASH COPY Confirmation   PROINDC. This bit reflects the confirmed state of UCB DFLASH COPY."]
    #[inline(always)]
    pub fn proindc(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, HfConfirm1_SPEC, crate::common::R> {
        crate::common::RegisterField::<18,0x3,1,0,u8, HfConfirm1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB DBG COPY Interface Confirmation   PROINDBGC. This bit reflects the confirmed state of UCB DBG COPY."]
    #[inline(always)]
    pub fn proindbgc(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, HfConfirm1_SPEC, crate::common::R> {
        crate::common::RegisterField::<20,0x3,1,0,u8, HfConfirm1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB HSM COPY Confirmation   PROINHSMC. This bit reflects the confirmed state of UCB HSM COPY."]
    #[inline(always)]
    pub fn proinhsmc(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, HfConfirm1_SPEC, crate::common::R> {
        crate::common::RegisterField::<22,0x3,1,0,u8, HfConfirm1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB HSMCOTP0 COPY Protection   PROINHSMCOTP0C. This bit reflects the confirmed state of UCB HSMCOTP0 COPY."]
    #[inline(always)]
    pub fn proinhsmcotp0c(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, HfConfirm1_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x3,1,0,u8, HfConfirm1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB HSMCOTP1 COPY Protection   PROINHSMCOTP1C. This bit reflects the confirmed state of UCB HSMCOTP1 COPY."]
    #[inline(always)]
    pub fn proinhsmcotp1c(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, HfConfirm1_SPEC, crate::common::R> {
        crate::common::RegisterField::<26,0x3,1,0,u8, HfConfirm1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB ECPRIO COPY Confirmation   PROINECC. This bit reflects the confirmed state of UCB ECPRIO COPY"]
    #[inline(always)]
    pub fn proinecc(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, HfConfirm1_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0x3,1,0,u8, HfConfirm1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for HfConfirm1 {
    #[inline(always)]
    fn default() -> HfConfirm1 {
        <crate::RegValueT<HfConfirm1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfConfirm2_SPEC;
impl crate::sealed::RegSpec for HfConfirm2_SPEC {
    type DataType = u32;
}
#[doc = "Flash Confirm Status Register 2\n resetvalue={Application Reset:0x0}"]
pub type HfConfirm2 = crate::RegValueT<HfConfirm2_SPEC>;

impl HfConfirm2 {
    #[doc = "UCB OTP0 ORIG Confirmation   PROINOTP0O. This bit reflects the confirmed state of UCB OTP0 ORIG."]
    #[inline(always)]
    pub fn proinotp0o(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, HfConfirm2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3,1,0,u8, HfConfirm2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB OTP1 ORIG Confirmation   PROINOTP1O. This bit reflects the confirmed state of UCB OTP1 ORIG."]
    #[inline(always)]
    pub fn proinotp1o(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, HfConfirm2_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x3,1,0,u8, HfConfirm2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB OTP2 ORIG Confirmation   PROINOTP2O. This bit reflects the confirmed state of UCB OTP2 ORIG."]
    #[inline(always)]
    pub fn proinotp2o(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, HfConfirm2_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x3,1,0,u8, HfConfirm2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB OTP3 ORIG Confirmation   PROINOTP3O. This bit reflects the confirmed state of UCB OTP3 ORIG."]
    #[inline(always)]
    pub fn proinotp3o(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, HfConfirm2_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x3,1,0,u8, HfConfirm2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB OTP4 ORIG Confirmation   PROINOTP4O. This bit reflects the confirmed state of UCB OTP4 ORIG."]
    #[inline(always)]
    pub fn proinotp4o(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, HfConfirm2_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3,1,0,u8, HfConfirm2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB OTP5 ORIG Confirmation   PROINOTP5O. This bit reflects the confirmed state of UCB OTP5 ORIG."]
    #[inline(always)]
    pub fn proinotp5o(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, HfConfirm2_SPEC, crate::common::R> {
        crate::common::RegisterField::<10,0x3,1,0,u8, HfConfirm2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB OTP6 ORIG Confirmation   PROINOTP6O. This bit reflects the confirmed state of UCB OTP6 ORIG."]
    #[inline(always)]
    pub fn proinotp6o(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, HfConfirm2_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x3,1,0,u8, HfConfirm2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB OTP7 ORIG Confirmation   PROINOTP7O. This bit reflects the confirmed state of UCB OTP7 ORIG."]
    #[inline(always)]
    pub fn proinotp7o(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, HfConfirm2_SPEC, crate::common::R> {
        crate::common::RegisterField::<14,0x3,1,0,u8, HfConfirm2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB OTP0 COPY Confirmation   PROINOTP0C. This bit reflects the confirmed state of UCB OTP0 COPY."]
    #[inline(always)]
    pub fn proinotp0c(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, HfConfirm2_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3,1,0,u8, HfConfirm2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB OTP1 COPY Confirmation   PROINOTP1C. This bit reflects the confirmed state of UCB OTP1 COPY."]
    #[inline(always)]
    pub fn proinotp1c(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, HfConfirm2_SPEC, crate::common::R> {
        crate::common::RegisterField::<18,0x3,1,0,u8, HfConfirm2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB OTP2 COPY Confirmation   PROINOTP2C. This bit reflects the confirmed state of UCB OTP2 COPY."]
    #[inline(always)]
    pub fn proinotp2c(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, HfConfirm2_SPEC, crate::common::R> {
        crate::common::RegisterField::<20,0x3,1,0,u8, HfConfirm2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB OTP3 COPY Confirmation   PROINOTP3C. This bit reflects the confirmed state of UCB OTP3 COPY."]
    #[inline(always)]
    pub fn proinotp3c(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, HfConfirm2_SPEC, crate::common::R> {
        crate::common::RegisterField::<22,0x3,1,0,u8, HfConfirm2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB OTP4 COPY Confirmation   PROINOTP4C. This bit reflects the confirmed state of UCB OTP4 COPY."]
    #[inline(always)]
    pub fn proinotp4c(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, HfConfirm2_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x3,1,0,u8, HfConfirm2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB OTP5 COPY Confirmation   PROINOTP5C. This bit reflects the confirmed state of UCB OTP5 COPY."]
    #[inline(always)]
    pub fn proinotp5c(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, HfConfirm2_SPEC, crate::common::R> {
        crate::common::RegisterField::<26,0x3,1,0,u8, HfConfirm2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB OTP6 COPY Confirmation   PROINOTP6C. This bit reflects the confirmed state of UCB OTP6 COPY."]
    #[inline(always)]
    pub fn proinotp6c(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, HfConfirm2_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0x3,1,0,u8, HfConfirm2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UCB OTP7 COPY Confirmation   PROINOTP7C. This bit reflects the confirmed state of UCB OTP7 COPY."]
    #[inline(always)]
    pub fn proinotp7c(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, HfConfirm2_SPEC, crate::common::R> {
        crate::common::RegisterField::<30,0x3,1,0,u8, HfConfirm2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for HfConfirm2 {
    #[inline(always)]
    fn default() -> HfConfirm2 {
        <crate::RegValueT<HfConfirm2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfControl_SPEC;
impl crate::sealed::RegSpec for HfControl_SPEC {
    type DataType = u32;
}
#[doc = "Flash Control Register\n resetvalue={Application Reset:0x320}"]
pub type HfControl = crate::RegValueT<HfControl_SPEC>;

impl HfControl {
    #[doc = "Enable Program Erase   FSIENPE. The field prevents any Flash program or erase directly in the FSI. It is        set to   8220 Enabled  8221  by the SSW upon finishing the startup. Once in the   8220 enabled  8221  state this field cannot be changed."]
    #[inline(always)]
    pub fn fsienpe(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        hf_control::Fsienpe,
        HfControl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            hf_control::Fsienpe,
            HfControl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "PFlash Wait State ECC error injection   WSERRINJ. Error injection into the ECC logic protecting PFlash Wait states."]
    #[inline(always)]
    pub fn wserrinj(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, HfControl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,HfControl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable Read from PFLASH   DDFP. This bit enables disables the read access to PFLASH. DMU ramp up and SSW start up This bit is automatically set with reset and is cleared during startup         if no Read Protection installed  and during startup SSW in case of        internal start out of Flash. This is realized by allowing full write access with inactive startup        protection. User Software  Clearing PFLASH Read Protection Once set  DMU HF CONTROL .DDFP        can only be cleared when DMU HF PROTECT .PRODISP        or DMU HF PROTECT .PRODISP0 5        or not DMU HF PROCONPF .RPRO User Software  Setting PFLASH Read Protection Software must write DMU HF CONTROL .DDFP        to 1"]
    #[inline(always)]
    pub fn ddfp(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, HfControl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,HfControl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable Data Fetch from DF0 EEPROM   DDFD. This bit enables disables the data fetch from DF0 EEPROM. DMU ramp up and SSW start up This bit is automatically set with reset and is cleared during startup         if no Read Protection installed  and during startup  BROM SW  in case of        internal start out of Flash. This is realized by allowing full write access with inactive startup        protection. User Software  Clearing DF0 EEPROM Read Protection Once set  DMU HF CONTROL .DDFD        can only be cleared when DMU HF PROTECT .PRODISD        or not DMU HF PROCONDF .RPRO User Software  Setting DF0 EEPROM Read Protection Software must write DMU HF CONTROL .DDFD        to 1"]
    #[inline(always)]
    pub fn ddfd(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, HfControl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,HfControl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear Programming State   CPROG"]
    #[inline(always)]
    pub fn cprog(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, HfControl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16,1,0,HfControl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear Erase State   CERASE"]
    #[inline(always)]
    pub fn cerase(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, HfControl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17,1,0,HfControl_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for HfControl {
    #[inline(always)]
    fn default() -> HfControl {
        <crate::RegValueT<HfControl_SPEC> as RegisterValue<_>>::new(800)
    }
}
pub mod hf_control {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Fsienpe_SPEC;
    pub type Fsienpe = crate::EnumBitfieldStruct<u8, Fsienpe_SPEC>;
    impl Fsienpe {
        #[doc = "00 Disabled Program Erase        are disabled in the FSI."]
        pub const DISABLED_0: Self = Self::new(0);
        #[doc = "01 Enabled Program Erase        are enabled in the FSI."]
        pub const ENABLED_1: Self = Self::new(1);
        #[doc = "10 Disabled Program Erase        are disabled in the FSI."]
        pub const DISABLED_2: Self = Self::new(2);
        #[doc = "11 Disabled Program Erase        are disabled in the FSI."]
        pub const DISABLED_3: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfDwait_SPEC;
impl crate::sealed::RegSpec for HfDwait_SPEC {
    type DataType = u32;
}
#[doc = "DFLASH Wait Cycle Register\n resetvalue={System Reset:0x4000B}"]
pub type HfDwait = crate::RegValueT<HfDwait_SPEC>;

impl HfDwait {
    #[doc = "Operation Mode   RFLASH. DFLASH Wait Cycles for Flash This bit field defines the number of FSI clock cycles for a DFLASH Bank        read access."]
    #[inline(always)]
    pub fn rflash(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, HfDwait_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, HfDwait_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Operation Mode   RECC. DFLASH Error Correction Cycles This bit field defines the number of FSI clock cycles for the DFLASH        Bank ECC correction."]
    #[inline(always)]
    pub fn recc(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, HfDwait_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8, HfDwait_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for HfDwait {
    #[inline(always)]
    fn default() -> HfDwait {
        <crate::RegValueT<HfDwait_SPEC> as RegisterValue<_>>::new(262155)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfEccc_SPEC;
impl crate::sealed::RegSpec for HfEccc_SPEC {
    type DataType = u32;
}
#[doc = "DF0 ECC Control Register\n resetvalue={Application Reset:0x0C0000000}"]
pub type HfEccc = crate::RegValueT<HfEccc_SPEC>;

impl HfEccc {
    #[doc = "Clear ECC status bits   CLR"]
    #[inline(always)]
    pub fn clr(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, HfEccc_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x3,1,0,u8, HfEccc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Host Command Interface ECC Correction Disable   ECCCORDIS"]
    #[inline(always)]
    pub fn ecccordis(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x3,
        1,
        0,
        hf_eccc::Ecccordis,
        HfEccc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            hf_eccc::Ecccordis,
            HfEccc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Host Command Interface Uncorrectable ECC Bit Error Trap Disable   TRAPDIS. Includes CFS and UCB uncorrectable errors."]
    #[inline(always)]
    pub fn trapdis(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, HfEccc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, HfEccc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for HfEccc {
    #[inline(always)]
    fn default() -> HfEccc {
        <crate::RegValueT<HfEccc_SPEC> as RegisterValue<_>>::new(3221225472)
    }
}
pub mod hf_eccc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecccordis_SPEC;
    pub type Ecccordis = crate::EnumBitfieldStruct<u8, Ecccordis_SPEC>;
    impl Ecccordis {
        #[doc = "00 Enabled ECC        correction is enabled for the DF0 read path and if DF1 is configured as        not HSM exclusive  then for the DF1 read path."]
        pub const ENABLED_0: Self = Self::new(0);
        #[doc = "01 Enabled ECC        correction is enabled for the DF0 read path and if DF1 is        configured as not HSM exclusive  then for the DF1 read path."]
        pub const ENABLED_1: Self = Self::new(1);
        #[doc = "10 Enabled ECC        correction is enabled for the DF0 read path and if DF1 is        configured as not HSM exclusive  then for the DF1 read path."]
        pub const ENABLED_2: Self = Self::new(2);
        #[doc = "11 Disabled ECC        correction is disabled for the DF0 read path and if DF1 is configured as        not HSM exclusive  then for the DF1 read path."]
        pub const DISABLED_3: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfEccr_SPEC;
impl crate::sealed::RegSpec for HfEccr_SPEC {
    type DataType = u32;
}
#[doc = "DF0 ECC Read Register\n resetvalue={Application Reset:0x0}"]
pub type HfEccr = crate::RegValueT<HfEccr_SPEC>;

impl HfEccr {
    #[doc = "Error Correction Read Code   RCODE. ECC checksum read during the last NVM read access."]
    #[inline(always)]
    pub fn rcode(
        self,
    ) -> crate::common::RegisterField<0, 0x3fffff, 1, 0, u32, HfEccr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3fffff,1,0,u32, HfEccr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for HfEccr {
    #[inline(always)]
    fn default() -> HfEccr {
        <crate::RegValueT<HfEccr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfEccs_SPEC;
impl crate::sealed::RegSpec for HfEccs_SPEC {
    type DataType = u32;
}
#[doc = "DF0 ECC Status Register\n resetvalue={Application Reset:0x0}"]
pub type HfEccs = crate::RegValueT<HfEccs_SPEC>;

impl HfEccs {
    #[doc = "Read Access Single Bit ECC Error   ERR1. The flag reports a single bit ECC failure during the last NVM read        access. Reset by hardware when DMU HF ECCC .CLR          is written to 11 ."]
    #[inline(always)]
    pub fn err1(self) -> crate::common::RegisterFieldBool<0, 1, 0, HfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, HfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Read Access Double Bit ECC Error   ERR2. The flag reports a double bit ECC failure during the last NVM read        access. Reset by hardware when DMU HF ECCC .CLR          is written to 11 ."]
    #[inline(always)]
    pub fn err2(self) -> crate::common::RegisterFieldBool<1, 1, 0, HfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, HfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Read Access Triple Bit ECC Error   ERR3. The flag reports a triple bit ECC failure during the last NVM read        access. Reset by hardware when DMU HF ECCC .CLR          is written to 11 ."]
    #[inline(always)]
    pub fn err3(self) -> crate::common::RegisterFieldBool<2, 1, 0, HfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, HfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Read Access Multi bit ECC Error   ERRM. The flag reports multi bit ECC failure during the last NVM read access. Reset by hardware when DMU HF ECCC .CLR          is written to 11 ."]
    #[inline(always)]
    pub fn errm(self) -> crate::common::RegisterFieldBool<3, 1, 0, HfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, HfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Any Read Access ECC Error   ERRANY. The flag reports any ECC failure during the last NVM read access. Reset by hardware when DMU HF ECCC .CLR          is written to 11 ."]
    #[inline(always)]
    pub fn errany(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, HfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, HfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Read Access Blank Analog   BLANKA. The flag reports that all read data cells have sufficient high current  a program of new data without prior erase is possible. Under certain operation history  a valid complement data entry may also appear as blank. Data qualifiers like headers or footers  which are usually used in EEPROM emulation  can be used to distinguish expected valid data from unknown data fields. Only blank failures in Complement Sensing mode are reported in this flag and is intended for use only in this mode. Blank failures in Single Ended mode are also reported when in CFTM. Reset by hardware when DMU HF ECCC .CLR is written to 11 ."]
    #[inline(always)]
    pub fn blanka(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, HfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, HfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated Single Bit ECC Errors   AER1. The status flag accumulates single bit failures during NVM read operations. Reset by hardware when DMU HF ECCC .CLR is written to 11 ."]
    #[inline(always)]
    pub fn aer1(self) -> crate::common::RegisterFieldBool<16, 1, 0, HfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, HfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated Double Bit ECC Errors   AER2. The status flag accumulates double bit failures during NVM read operations. Reset by hardware when DMU HF ECCC .CLR is written to 11 ."]
    #[inline(always)]
    pub fn aer2(self) -> crate::common::RegisterFieldBool<17, 1, 0, HfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, HfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated Triple Bit ECC Errors   AER3. The status flag accumulates triple bit failures during NVM read operations. Reset by hardware when DMU HF ECCC .CLR is written to 11 ."]
    #[inline(always)]
    pub fn aer3(self) -> crate::common::RegisterFieldBool<18, 1, 0, HfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, HfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated Multi bit ECC Errors   AERM. The status bit accumulates multi bit failures during NVM read accesses. Reset by hardware when DMU HF ECCC .CLR is written to 11 ."]
    #[inline(always)]
    pub fn aerm(self) -> crate::common::RegisterFieldBool<19, 1, 0, HfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, HfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated Any Read Access ECC Error   AERANY. The status bit accumulates ECC failures during NVM read accesses. Reset by hardware when DMU HF ECCC .CLR          is written to 11 ."]
    #[inline(always)]
    pub fn aerany(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, HfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, HfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated Blank Analog   ABLANKA. The flag accumulates analog evaluated blank failures during NVM read accesses. It reports that all read data cells have sufficient high current  a program of new data without prior erase is possible. Under certain operation history  a valid complement data entry may also appear as blank. Data qualifiers like headers or footers  which are usually used in EEPROM emulation  can be used to distinguish expected valid data from unknown data fields. Only blank failures in Complement Sensing mode are reported in this flag and is intended for use only in this mode. Blank failures in Single Ended mode is also reported when in CFTM. Reset by hardware when DMU HF ECCC .CLR is written to 11 ."]
    #[inline(always)]
    pub fn ablanka(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, HfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, HfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for HfEccs {
    #[inline(always)]
    fn default() -> HfEccs {
        <crate::RegValueT<HfEccs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfEccw_SPEC;
impl crate::sealed::RegSpec for HfEccw_SPEC {
    type DataType = u32;
}
#[doc = "DF0 ECC Write Register\n resetvalue={Application Reset:0x0}"]
pub type HfEccw = crate::RegValueT<HfEccw_SPEC>;

impl HfEccw {
    #[doc = "Error Correction Write Code   WCODE. 22 bit ECC code for the current 64 bit  for DFLASH  or 256 bit  for        PFLASH  write buffer to be written into the assembly buffer instead of a        generated ECC."]
    #[inline(always)]
    pub fn wcode(
        self,
    ) -> crate::common::RegisterField<0, 0x3fffff, 1, 0, u32, HfEccw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3fffff,1,0,u32, HfEccw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PFLASH ECC Encoding Disable   PECENCDIS"]
    #[inline(always)]
    pub fn pecencdis(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, HfEccw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, HfEccw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DFLASH ECC Encoding Disable   DECENCDIS"]
    #[inline(always)]
    pub fn decencdis(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, HfEccw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, HfEccw_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for HfEccw {
    #[inline(always)]
    fn default() -> HfEccw {
        <crate::RegValueT<HfEccw_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfEer_SPEC;
impl crate::sealed::RegSpec for HfEer_SPEC {
    type DataType = u32;
}
#[doc = "Enable Error Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
pub type HfEer = crate::RegValueT<HfEer_SPEC>;

impl HfEer {
    #[doc = "Operation Error Interrupt Mask   OPERM"]
    #[inline(always)]
    pub fn operm(self) -> crate::common::RegisterFieldBool<0, 1, 0, HfEer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, HfEer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Command Sequence Error Interrupt Mask   SQERM"]
    #[inline(always)]
    pub fn sqerm(self) -> crate::common::RegisterFieldBool<1, 1, 0, HfEer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, HfEer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Protection Error Interrupt Mask   PROERM"]
    #[inline(always)]
    pub fn proerm(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, HfEer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, HfEer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Program Verify Error Interrupt Mask   PVERM"]
    #[inline(always)]
    pub fn pverm(self) -> crate::common::RegisterFieldBool<3, 1, 0, HfEer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, HfEer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Erase Verify Error Interrupt Mask   EVERM"]
    #[inline(always)]
    pub fn everm(self) -> crate::common::RegisterFieldBool<4, 1, 0, HfEer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, HfEer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "End of Busy Interrupt Mask   EOBM"]
    #[inline(always)]
    pub fn eobm(self) -> crate::common::RegisterFieldBool<31, 1, 0, HfEer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, HfEer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for HfEer {
    #[inline(always)]
    fn default() -> HfEer {
        <crate::RegValueT<HfEer_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfErrsr_SPEC;
impl crate::sealed::RegSpec for HfErrsr_SPEC {
    type DataType = u32;
}
#[doc = "Error Status Register\n resetvalue={Application Reset:0x0}"]
pub type HfErrsr = crate::RegValueT<HfErrsr_SPEC>;

impl HfErrsr {
    #[doc = "Flash Operation Error   OPER. Cleared with system reset."]
    #[inline(always)]
    pub fn oper(self) -> crate::common::RegisterFieldBool<0, 1, 0, HfErrsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, HfErrsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Command Sequence Error   SQER. A sequence error is not indicated if the Reset to Read command aborts a command sequence. Cleared with application reset  commands  Reset to Read  and  Clear Status  or writing DMU HF CLRE .CSQER   1 ."]
    #[inline(always)]
    pub fn sqer(self) -> crate::common::RegisterFieldBool<1, 1, 0, HfErrsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, HfErrsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Protection Error   PROER. A Protection Error is reported e.g. because of a not allowed command  for example an Erase or Write Page command addressing a locked sector  or because of wrong password s  in a protected command sequence such as  Disable Read Protection . A Protection Error is also reported if the safety protection prevented a program erase operation in Flash. Cleared with application reset  with the command  Clear Status  or writing DMU HF CLRE .CPROER   1 ."]
    #[inline(always)]
    pub fn proer(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, HfErrsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, HfErrsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Program Verify Error   PVER. A verify error was reported on completion of a Flash program operation  even when the operation was directly started in the FSI  Cleared with application reset  with the command   8220 Clear Status  8221  or          writing DMU HF CLRE .CPVER            1 ."]
    #[inline(always)]
    pub fn pver(self) -> crate::common::RegisterFieldBool<3, 1, 0, HfErrsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, HfErrsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Erase Verify Error   EVER. A verify error was reported on completion of a Flash erase operation  even when the operation was directly started in the FSI  . Cleared with application reset  with the command  Clear Status  or writing DMU HF CLRE .CEVER   1 ."]
    #[inline(always)]
    pub fn ever(self) -> crate::common::RegisterFieldBool<4, 1, 0, HfErrsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, HfErrsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SRI Bus Address ECC Error   ADER. This flag is set when the DMU detects an ECC error in the address phase        bus transaction on the SRI bus. Cleared with application reset or writing DMU HF CLRE .CADER            1 ."]
    #[inline(always)]
    pub fn ader(self) -> crate::common::RegisterFieldBool<5, 1, 0, HfErrsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, HfErrsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Original Error   ORIER. If a UCB contains both ORIG and COPY confirmation codes and during        startup an ERRORED value or an uncorrectable ECC error is detected in        the ORIG confirmation code then the original error flag is set by        hardware. Cleared with application reset."]
    #[inline(always)]
    pub fn orier(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, HfErrsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, HfErrsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for HfErrsr {
    #[inline(always)]
    fn default() -> HfErrsr {
        <crate::RegValueT<HfErrsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfId_SPEC;
impl crate::sealed::RegSpec for HfId_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0E7C000}"]
pub type HfId = crate::RegValueT<HfId_SPEC>;

impl HfId {
    #[doc = "Module Revision Number   MOD REV. This bit field defines the module revision number reflecting major        changes in the module."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, HfId_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, HfId_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Module Type   MOD TYPE. The bit field is set to C0 which defines        the module as a 32 bit module."]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, HfId_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, HfId_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Module Number Value   MOD NUMBER. This bit field defines the module identification number."]
    #[inline(always)]
    pub fn mod_number(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, HfId_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, HfId_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Flash Revision   FLASH REV. This bit field defines the flash revision number.  31 28  identifies the product. For example  0x9h indicates TC39x        product  0x8h indicates TC38x product etc.  27 24  identifies the        product revision."]
    #[inline(always)]
    pub fn flash_rev(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, HfId_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, HfId_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for HfId {
    #[inline(always)]
    fn default() -> HfId {
        <crate::RegValueT<HfId_SPEC> as RegisterValue<_>>::new(15187968)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfMargin_SPEC;
impl crate::sealed::RegSpec for HfMargin_SPEC {
    type DataType = u32;
}
#[doc = "Margin Control Register\n resetvalue={Application Reset:0x0}"]
pub type HfMargin = crate::RegValueT<HfMargin_SPEC>;

impl HfMargin {
    #[doc = "DF0 Margin Read Selection   SELD0. If a change between the standard and hard read margin setting is done           the system must wait a delay time t FL MarginDel until the next read is requested."]
    #[inline(always)]
    pub fn seld0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, HfMargin_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, HfMargin_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Hard Margin Selection   HMARGIN. Suboptimal 0 bits are read as 1s. Suboptimal 1 bits are read as 0s. The concrete margin values are restored from the configuration sector        and are determined by Infineon."]
    #[inline(always)]
    pub fn hmargin(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        hf_margin::Hmargin,
        HfMargin_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            hf_margin::Hmargin,
            HfMargin_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for HfMargin {
    #[inline(always)]
    fn default() -> HfMargin {
        <crate::RegValueT<HfMargin_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod hf_margin {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hmargin_SPEC;
    pub type Hmargin = crate::EnumBitfieldStruct<u8, Hmargin_SPEC>;
    impl Hmargin {
        #[doc = "0 Tight0 Tight        margin for 0  low  level."]
        pub const TIGHT_00: Self = Self::new(0);
        #[doc = "1 Tight1 Tight        margin for 1  high  level."]
        pub const TIGHT_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfOperation_SPEC;
impl crate::sealed::RegSpec for HfOperation_SPEC {
    type DataType = u32;
}
#[doc = "Flash Operation Register\n resetvalue={System Reset:0x0}"]
pub type HfOperation = crate::RegValueT<HfOperation_SPEC>;

impl HfOperation {
    #[doc = "Programming State   PROG. HW controlled status flag. Set with last cycle of Write Page Burst and Replace Logical Sector        command sequences. If one BUSY flag is coincidently set  PROG indicates the type of busy        state. If OPER is coincidently set  PROG indicates the type of erroneous        operation. Otherwise  PROG indicates  that operation is still requested        or finished. May be also cleared by writing   8216 1  8217  to DMU HF CONTROL .CPROG. This bit is not set for by program operations initiated by the HSM        interface. Cleared with command   8220 Clear Status  8221 ."]
    #[inline(always)]
    pub fn prog(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, HfOperation_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,HfOperation_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Erase State   ERASE. HW controlled status flag. Set with last cycle of Erase Verify command sequence. Indications are analogous to PROG flag. May be cleared by writing  1  to DMU HF CONTROL .CERASE. This bit is not set for by erase operations initiated by the HSM interface. Cleared with command  Clear Status ."]
    #[inline(always)]
    pub fn erase(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, HfOperation_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,HfOperation_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for HfOperation {
    #[inline(always)]
    fn default() -> HfOperation {
        <crate::RegValueT<HfOperation_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfPcontrol_SPEC;
impl crate::sealed::RegSpec for HfPcontrol_SPEC {
    type DataType = u32;
}
#[doc = "Power Control Register\n resetvalue={Application Reset:0x30000}"]
pub type HfPcontrol = crate::RegValueT<HfPcontrol_SPEC>;

impl HfPcontrol {
    #[doc = "Sleep Mode Control   SLEEP. This bit field determines the Sleep mode."]
    #[inline(always)]
    pub fn sleep(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, HfPcontrol_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, HfPcontrol_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Dynamic Idle Enable   IDLE"]
    #[inline(always)]
    pub fn idle(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, HfPcontrol_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0x3,1,0,u8, HfPcontrol_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Demand Enable   DEMAND"]
    #[inline(always)]
    pub fn demand(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, HfPcontrol_SPEC, crate::common::W> {
        crate::common::RegisterField::<10,0x3,1,0,u8, HfPcontrol_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "External Sleep Mode Request Disable   ESLDIS. Used for Sleep Mode control. The Sleep Mode request is generated by the SCU."]
    #[inline(always)]
    pub fn esldis(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, HfPcontrol_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, HfPcontrol_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Programming Supply 5V   PR5V. Selects the supply for programming."]
    #[inline(always)]
    pub fn pr5v(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        hf_pcontrol::Pr5V,
        HfPcontrol_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            hf_pcontrol::Pr5V,
            HfPcontrol_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for HfPcontrol {
    #[inline(always)]
    fn default() -> HfPcontrol {
        <crate::RegValueT<HfPcontrol_SPEC> as RegisterValue<_>>::new(196608)
    }
}
pub mod hf_pcontrol {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pr5V_SPEC;
    pub type Pr5V = crate::EnumBitfieldStruct<u8, Pr5V_SPEC>;
    impl Pr5V {
        #[doc = "00 P3V The programming voltage is internally generated."]
        pub const P_3_V_0: Self = Self::new(0);
        #[doc = "01 P3V The programming voltage is internally generated."]
        pub const P_3_V_1: Self = Self::new(1);
        #[doc = "10 P3V The programming voltage is internally generated."]
        pub const P_3_V_2: Self = Self::new(2);
        #[doc = "11 P5V As programming voltage the external 5 V supply is used."]
        pub const P_5_V_3: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfProcondbg_SPEC;
impl crate::sealed::RegSpec for HfProcondbg_SPEC {
    type DataType = u32;
}
#[doc = "Debug Interface Protection Configuration\n resetvalue={Application Reset:0x100,CFS Value:0x0}"]
pub type HfProcondbg = crate::RegValueT<HfProcondbg_SPEC>;

impl HfProcondbg {
    #[doc = "OCDS Disabled   OCDSDIS. This bit indicates whether the OCDS is configured as locked."]
    #[inline(always)]
    pub fn ocdsdis(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, HfProcondbg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,HfProcondbg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Debug Interface Locked   DBGIFLCK. This bit indicates whether the debug interface is configured as locked."]
    #[inline(always)]
    pub fn dbgiflck(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, HfProcondbg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,HfProcondbg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Entered Debug Mode   EDM. This bit indicates whether the debug interface has been opened via Destructive Debug Entry. Consequently the CAN and FlexRay operation is made impossible"]
    #[inline(always)]
    pub fn edm(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, HfProcondbg_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x3,1,0,u8, HfProcondbg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Tool Interface Control   TIC. DAP over CAN Physical Layer  DXCPL . Others Reserved."]
    #[inline(always)]
    pub fn tic(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, HfProcondbg_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xf,1,0,u8, HfProcondbg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for HfProcondbg {
    #[inline(always)]
    fn default() -> HfProcondbg {
        <crate::RegValueT<HfProcondbg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfProcondf_SPEC;
impl crate::sealed::RegSpec for HfProcondf_SPEC {
    type DataType = u32;
}
#[doc = "DFLASH Protection Configuration\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
pub type HfProcondf = crate::RegValueT<HfProcondf_SPEC>;

impl HfProcondf {
    #[doc = "DF0 EEPROM Locked for Write Protection   L. This bit indicates whether the DFLASH sectors EEPROMx are write        protected."]
    #[inline(always)]
    pub fn l(self) -> crate::common::RegisterFieldBool<0, 1, 0, HfProcondf_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,HfProcondf_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Hysteresis enable. When enabled by OSCCFG these fields are copied to SCU OSCCON.HYSEN"]
    #[inline(always)]
    pub fn hysen(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, HfProcondf_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,HfProcondf_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Hysteresis Control. When enabled by OSCCFG these fields are copied to SCU OSCCON.HYSCTL"]
    #[inline(always)]
    pub fn hysctl(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, HfProcondf_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x3,1,0,u8, HfProcondf_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Amplitude Control. When enabled by OSCCFG these fields are copied to SCU OSCCON.AMPCTL"]
    #[inline(always)]
    pub fn ampctl(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, HfProcondf_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x3,1,0,u8, HfProcondf_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "OSC Configuration by SSW   OSCCFG. This bit indicates whether the oscillator configuration  fields  CAPxEN         APREN  MODE  HYSEN  HYSCTL  AMPCTL  are installed by the SSW in        SCU OSCCON."]
    #[inline(always)]
    pub fn osccfg(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, HfProcondf_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,HfProcondf_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "OSC Mode   MODE. When enabled by OSCCFG this field is copied to SCU OSCCON.MODE."]
    #[inline(always)]
    pub fn mode(
        self,
    ) -> crate::common::RegisterField<9, 0x3, 1, 0, u8, HfProcondf_SPEC, crate::common::R> {
        crate::common::RegisterField::<9,0x3,1,0,u8, HfProcondf_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "OSC Amplitude Regulation Enable   APREN. When enabled by OSCCFG this field is copied to SCU OSCCON.APREN."]
    #[inline(always)]
    pub fn apren(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, HfProcondf_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,HfProcondf_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "OSC Capacitance 3 Enable  x 0 3    CAP3EN. When enabled by OSCCFG these fields are copied to SCU OSCCON.CAPxEN."]
    #[inline(always)]
    pub fn cap0en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, HfProcondf_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,HfProcondf_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "OSC Capacitance 3 Enable  x 0 3    CAP3EN. When enabled by OSCCFG these fields are copied to SCU OSCCON.CAPxEN."]
    #[inline(always)]
    pub fn cap1en(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, HfProcondf_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,HfProcondf_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "OSC Capacitance 3 Enable  x 0 3    CAP3EN. When enabled by OSCCFG these fields are copied to SCU OSCCON.CAPxEN."]
    #[inline(always)]
    pub fn cap2en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, HfProcondf_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,HfProcondf_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "OSC Capacitance 3 Enable  x 0 3    CAP3EN. When enabled by OSCCFG these fields are copied to SCU OSCCON.CAPxEN."]
    #[inline(always)]
    pub fn cap3en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, HfProcondf_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,HfProcondf_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ESR0 Prolongation Counter   ESR0CNT. Used to configure the ESR0 delay. Evaluation by SSW."]
    #[inline(always)]
    pub fn esr0cnt(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, HfProcondf_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xfff,1,0,u16, HfProcondf_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Read Protection Configuration   RPRO. This bit indicates whether read protection is configured for DFLASH sectors."]
    #[inline(always)]
    pub fn rpro(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, HfProcondf_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,HfProcondf_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for HfProcondf {
    #[inline(always)]
    fn default() -> HfProcondf {
        <crate::RegValueT<HfProcondf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfProconpf_SPEC;
impl crate::sealed::RegSpec for HfProconpf_SPEC {
    type DataType = u32;
}
#[doc = "PFLASH Protection Configuration\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
pub type HfProconpf = crate::RegValueT<HfProconpf_SPEC>;

impl HfProconpf {
    #[doc = "Read Protection Configuration   RPRO. This bit indicates whether read protection is configured for PFLASH sectors."]
    #[inline(always)]
    pub fn rpro(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, HfProconpf_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,HfProconpf_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for HfProconpf {
    #[inline(always)]
    fn default() -> HfProconpf {
        <crate::RegValueT<HfProconpf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfProconram_SPEC;
impl crate::sealed::RegSpec for HfProconram_SPEC {
    type DataType = u32;
}
#[doc = "RAM Configuration\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
pub type HfProconram = crate::RegValueT<HfProconram_SPEC>;

impl HfProconram {
    #[doc = "RAM Initialization by SSW Control   RAMIN. These bits defined whether the RAMs selected by the field RAMINSEL are initialized. This field determines also if a RAM is initialized before MBIST access is granted. In all  Init    cases a RAM is initialized before MBIST access is enabled  in the  No Init  case a RAM is not erased. This is independent of the memory selection with RAMINSEL for initialization during startup  see MTU chapter ."]
    #[inline(always)]
    pub fn ramin(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        hf_proconram::Ramin,
        HfProconram_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            hf_proconram::Ramin,
            HfProconram_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "RAM Initialization Selection   RAMINSEL. These bits select which memories are initialized when the RAM initialization is configured with RAMIN. See CROSSREFERENCE ."]
    #[inline(always)]
    pub fn raminsel(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, HfProconram_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, HfProconram_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "LMU Initialization Selection   LMUINSEL. These bits select which memories are initialized when the RAM initialization is configured with RAMIN. See CROSSREFERENCE ."]
    #[inline(always)]
    pub fn lmuinsel(
        self,
    ) -> crate::common::RegisterField<24, 0x7f, 1, 0, u8, HfProconram_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x7f,1,0,u8, HfProconram_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for HfProconram {
    #[inline(always)]
    fn default() -> HfProconram {
        <crate::RegValueT<HfProconram_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod hf_proconram {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ramin_SPEC;
    pub type Ramin = crate::EnumBitfieldStruct<u8, Ramin_SPEC>;
    impl Ramin {
        #[doc = "00 Init All RAM initialization is performed after cold power on  reset and warm power on reset."]
        pub const INIT_ALL_0: Self = Self::new(0);
        #[doc = "01 Init Warm RAM initialization is performed after warm power on resets but not after cold power on reset  not recommended ."]
        pub const INIT_WARM_1: Self = Self::new(1);
        #[doc = "10 Init Cold RAM initialization is performed after cold power on reset."]
        pub const INIT_COLD_2: Self = Self::new(2);
        #[doc = "11 No Init No RAM initialization is performed."]
        pub const NO_INIT_3: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfProcontp_SPEC;
impl crate::sealed::RegSpec for HfProcontp_SPEC {
    type DataType = u32;
}
#[doc = "Tuning Protection Configuration\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
pub type HfProcontp = crate::RegValueT<HfProcontp_SPEC>;

impl HfProcontp {
    #[doc = "Tuning Protection   TP. This bit indicates whether tuning protection is installed or not."]
    #[inline(always)]
    pub fn tp(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, HfProcontp_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,HfProcontp_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Boot Mode Lock   BML. Used by the SSW to restrict the boot mode selection."]
    #[inline(always)]
    pub fn bml(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, HfProcontp_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3,1,0,u8, HfProcontp_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Enable SOTA mode   SWAPEN. This field enables the entry into  Software update Over the Air SOTA  mode . In this mode  an alternate PFLASH address map can be selected. Please refer to the SOTA section of the Introduction chapter for more details."]
    #[inline(always)]
    pub fn swapen(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        hf_procontp::Swapen,
        HfProcontp_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            hf_procontp::Swapen,
            HfProcontp_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Disable direct LPB access. Disable direct LPB access by the CPU to the Local PFlash Bank  LPB ."]
    #[inline(always)]
    pub fn cpu0ddis(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, HfProcontp_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18,1,0,HfProcontp_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Disable direct LPB access. Disable direct LPB access by the CPU to the Local PFlash Bank  LPB ."]
    #[inline(always)]
    pub fn cpu1ddis(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, HfProcontp_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19,1,0,HfProcontp_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for HfProcontp {
    #[inline(always)]
    fn default() -> HfProcontp {
        <crate::RegValueT<HfProcontp_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod hf_procontp {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Swapen_SPEC;
    pub type Swapen = crate::EnumBitfieldStruct<u8, Swapen_SPEC>;
    impl Swapen {
        #[doc = "00 Disabled SOTA        mode disabled."]
        pub const DISABLED_0: Self = Self::new(0);
        #[doc = "01 Disabled SOTA        mode disabled."]
        pub const DISABLED_1: Self = Self::new(1);
        #[doc = "10 Disabled SOTA        mode disabled."]
        pub const DISABLED_2: Self = Self::new(2);
        #[doc = "11 Enabled SOTA        mode enabled."]
        pub const ENABLED_3: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfProconusr_SPEC;
impl crate::sealed::RegSpec for HfProconusr_SPEC {
    type DataType = u32;
}
#[doc = "DF0 User Mode Control\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
pub type HfProconusr = crate::RegValueT<HfProconusr_SPEC>;

impl HfProconusr {
    #[doc = "DF0 User Mode Control   MODE. Configures the DF0 mode when the user has control."]
    #[inline(always)]
    pub fn mode(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, HfProconusr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3,1,0,u8, HfProconusr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for HfProconusr {
    #[inline(always)]
    fn default() -> HfProconusr {
        <crate::RegValueT<HfProconusr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfProtect_SPEC;
impl crate::sealed::RegSpec for HfProtect_SPEC {
    type DataType = u32;
}
#[doc = "Flash Protection Status Register\n resetvalue={Application Reset:0x0}"]
pub type HfProtect = crate::RegValueT<HfProtect_SPEC>;

impl HfProtect {
    #[doc = "PFLASH Protection Disabled   PRODISP. The protection configured by UCB PFLASH ORIG and UCB PFLASH COPY was        successfully disabled by supplying the correct password to   8220 Disable        Protection  8221 . Cleared with command   8220 Resume Protection  8221 ."]
    #[inline(always)]
    pub fn prodisp(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, HfProtect_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, HfProtect_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "DFLASH Protection Disabled   PRODISD. The protection configured by UCB DFLASH ORIG and UCB DFLASH COPY was        successfully disabled by supplying the correct password to   8220 Disable        Protection  8221 . Cleared with command   8220 Resume Protection  8221 ."]
    #[inline(always)]
    pub fn prodisd(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, HfProtect_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, HfProtect_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Debug Interface Password Protection Disabled   PRODISDBG. The password configured by UCB DBG ORIG and UCB DBG COPY was correctly        received with   8220 Disable Protection  8221 . When DMU SP PROCONHSMCFG.DESTDBG is   8220 destructive  8221  then only the SSW can        disable this protection. This means PRODISDBG is only set when the password matches and  DESTDBG            8220 non destructive  8221  or SCU STCON.STP   0 . Cleared with command   8220 Resume Protection  8221 ."]
    #[inline(always)]
    pub fn prodisdbg(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, HfProtect_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, HfProtect_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Erase Counter Priority Protection Disabled   PRODISEC. The protection configured by UCB ECPRIO ORIG and UCB ECPRIO COPY was        successfully disabled by supplying the correct password to   8220 Disable        Protection  8221 . Cleared with command  quot Resume Protection quot ."]
    #[inline(always)]
    pub fn prodisec(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, HfProtect_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, HfProtect_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "BMHD Protection Disabled   PRODISBMHD. The protection configured by UCB BMHD0 ORIG and UCB BMHD0 COPY was        successfully disabled by supplying the correct password to   8220 Disable        Protection  8221 . Cleared with command  quot Resume Protection quot ."]
    #[inline(always)]
    pub fn prodisbmhd(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, HfProtect_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, HfProtect_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "UCB SWAP protection Disabled   PRODISSWAP. The protection configured by UCB SWAP ORIG and UCB SWAP COPY was        successfully disabled by supplying the correct password to   8220 Disable        Protection  8221 . Cleared with command  quot Resume Protection quot ."]
    #[inline(always)]
    pub fn prodisswap(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, HfProtect_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, HfProtect_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Program Flash Protection Disable PRODISP5. The protection configured for PFx by UCB PFLASH ORIG and UCB PFLASH COPY        was successfully disabled by supplying the correct password to   8220 Disable        Protection  8221 . Cleared with command  quot Resume Protection quot ."]
    #[inline(always)]
    pub fn prodisp0(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, HfProtect_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, HfProtect_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Program Flash Protection Disable PRODISP5. The protection configured for PFx by UCB PFLASH ORIG and UCB PFLASH COPY        was successfully disabled by supplying the correct password to   8220 Disable        Protection  8221 . Cleared with command  quot Resume Protection quot ."]
    #[inline(always)]
    pub fn prodisp1(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, HfProtect_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, HfProtect_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Secure Retest Password Protection Disabled   SRT. Cleared with command  Resume Protection ."]
    #[inline(always)]
    pub fn srt(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, HfProtect_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24,1,0,HfProtect_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for HfProtect {
    #[inline(always)]
    fn default() -> HfProtect {
        <crate::RegValueT<HfProtect_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfPstatus_SPEC;
impl crate::sealed::RegSpec for HfPstatus_SPEC {
    type DataType = u32;
}
#[doc = "Power Status Register\n resetvalue={Application Reset:0x0}"]
pub type HfPstatus = crate::RegValueT<HfPstatus_SPEC>;

impl HfPstatus {
    #[doc = "Sleep Mode   SLEEP"]
    #[inline(always)]
    pub fn sleep(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, HfPstatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, HfPstatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Dynamic Idle   IDLE"]
    #[inline(always)]
    pub fn idle(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, HfPstatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, HfPstatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Demand   DEMAND. For Cranking Mode  DMU HF PSTATUS .DEMAND   1"]
    #[inline(always)]
    pub fn demand(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, HfPstatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, HfPstatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for HfPstatus {
    #[inline(always)]
    fn default() -> HfPstatus {
        <crate::RegValueT<HfPstatus_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfPwait_SPEC;
impl crate::sealed::RegSpec for HfPwait_SPEC {
    type DataType = u32;
}
#[doc = "PFLASH Wait Cycle Register\n resetvalue={System Reset:0x716040B}"]
pub type HfPwait = crate::RegValueT<HfPwait_SPEC>;

impl HfPwait {
    #[doc = "Operation Mode   RFLASH. PFLASH Wait Cycles This bit field defines the number of SRI clock cycles for a PFLASH Bank read access."]
    #[inline(always)]
    pub fn rflash(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, HfPwait_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, HfPwait_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Operation Mode   RECC. PFLASH Error Correction Cycles This bit field defines the number of SRI clock cycles for the PFLASH Bank ECC correction."]
    #[inline(always)]
    pub fn recc(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, HfPwait_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8, HfPwait_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Cranking Mode   CFLASH. PFLASH Wait Cycles This bit field defines the number of SRI clock cycles for a PFLASH Bank read access."]
    #[inline(always)]
    pub fn cflash(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, HfPwait_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, HfPwait_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Cranking Mode   CECC. PFLASH Error Correction Cycles This bit field defines the number of SRI clock cycles for the PFLASH Bank ECC correction."]
    #[inline(always)]
    pub fn cecc(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, HfPwait_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, HfPwait_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for HfPwait {
    #[inline(always)]
    fn default() -> HfPwait {
        <crate::RegValueT<HfPwait_SPEC> as RegisterValue<_>>::new(118883339)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfStatus_SPEC;
impl crate::sealed::RegSpec for HfStatus_SPEC {
    type DataType = u32;
}
#[doc = "Flash Status Register\n resetvalue={Application Reset:0x0FF}"]
pub type HfStatus = crate::RegValueT<HfStatus_SPEC>;

impl HfStatus {
    #[doc = "Data Flash Bank 0 Busy   D0BUSY. HW controlled status flag. Indication of busy state of DFLASH bank  160 0 because of active execution of        an operation  DF0 busy state is also indicated during Flash startup        after reset or in sleep mode  while in busy state the DF0 does not allow        read access."]
    #[inline(always)]
    pub fn d0busy(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, HfStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, HfStatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Data Flash Bank 1 Busy   D1BUSY. HW controlled status flag. Indication of busy state of DFLASH bank  160 1 because of active execution of        an operation  DF1 busy state is also indicated during Flash startup        after reset or in sleep mode  while in busy state the DF1 does not allow        read access. Bit is not set for program erase operations initiated by the HSM        interface."]
    #[inline(always)]
    pub fn d1busy(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, HfStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, HfStatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Program Flash PF5BUSY   P5BUSY. HW controlled status flag. Indication of busy state of PFx because of active execution of an        operation  PFx busy state is also indicated during Flash startup after        reset or in sleep mode  while in busy state the PFx does not allow read        access."]
    #[inline(always)]
    pub fn p0busy(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, HfStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, HfStatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Program Flash PF5BUSY   P5BUSY. HW controlled status flag. Indication of busy state of PFx because of active execution of an        operation  PFx busy state is also indicated during Flash startup after        reset or in sleep mode  while in busy state the PFx does not allow read        access."]
    #[inline(always)]
    pub fn p1busy(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, HfStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, HfStatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Data Flash in Page Mode   DFPAGE. HW controlled status flag. Set with Enter Page Mode for DFLASH  cleared with Write Page command. This bit is not set by   8220 Enter Page Mode  8221  initiated by the HSM interface. Read accesses are allowed while in page mode."]
    #[inline(always)]
    pub fn dfpage(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, HfStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, HfStatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Program Flash in Page Mode   PFPAGE. HW controlled status flag. Set with Enter Page Mode for Flash  cleared with Write Page command This bit is not set by   8220 Enter Page Mode  8221  initiated by the HSM interface. Read accesses are allowed while in page mode."]
    #[inline(always)]
    pub fn pfpage(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, HfStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, HfStatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for HfStatus {
    #[inline(always)]
    fn default() -> HfStatus {
        <crate::RegValueT<HfStatus_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HfSuspend_SPEC;
impl crate::sealed::RegSpec for HfSuspend_SPEC {
    type DataType = u32;
}
#[doc = "Suspend Control Register\n resetvalue={Application Reset:0x0}"]
pub type HfSuspend = crate::RegValueT<HfSuspend_SPEC>;

impl HfSuspend {
    #[doc = "Suspend Request   REQ"]
    #[inline(always)]
    pub fn req(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, HfSuspend_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,HfSuspend_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Suspend Clear   CLR. Software write only active high clear of Suspend Error."]
    #[inline(always)]
    pub fn clr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, HfSuspend_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, HfSuspend_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Flash Operation Suspended   SPND. Suspension of a Flash program or erase operation."]
    #[inline(always)]
    pub fn spnd(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, HfSuspend_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16,1,0,HfSuspend_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Suspend Error   ERR. Reset by hardware when DMU HF SUSPEND .CLR is written to 1 ."]
    #[inline(always)]
    pub fn err(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, HfSuspend_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17,1,0,HfSuspend_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for HfSuspend {
    #[inline(always)]
    fn default() -> HfSuspend {
        <crate::RegValueT<HfSuspend_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SfClre_SPEC;
impl crate::sealed::RegSpec for SfClre_SPEC {
    type DataType = u32;
}
#[doc = "HSM Clear Error Register\n resetvalue={Application Reset:0x0}"]
pub type SfClre = crate::RegValueT<SfClre_SPEC>;

impl SfClre {
    #[doc = "Clear Command Sequence Error   CSQER"]
    #[inline(always)]
    pub fn csqer(self) -> crate::common::RegisterFieldBool<1, 1, 0, SfClre_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, SfClre_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Program Verify Error   CPVER"]
    #[inline(always)]
    pub fn cpver(self) -> crate::common::RegisterFieldBool<3, 1, 0, SfClre_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, SfClre_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Erase Verify Error   CEVER"]
    #[inline(always)]
    pub fn cever(self) -> crate::common::RegisterFieldBool<4, 1, 0, SfClre_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, SfClre_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for SfClre {
    #[inline(always)]
    fn default() -> SfClre {
        <crate::RegValueT<SfClre_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SfControl_SPEC;
impl crate::sealed::RegSpec for SfControl_SPEC {
    type DataType = u32;
}
#[doc = "HSM Flash Configuration Register\n resetvalue={Application Reset:0x1}"]
pub type SfControl = crate::RegValueT<SfControl_SPEC>;

impl SfControl {
    #[doc = "Lock Access to UCB HSMCFG   LCKHSMUCB. Trap door register. This field can only be written to the   x201c Locked  x201d  state. Other writes are ignored."]
    #[inline(always)]
    pub fn lckhsmucb(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        sf_control::Lckhsmucb,
        SfControl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            sf_control::Lckhsmucb,
            SfControl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Clear Programming State   CPROG"]
    #[inline(always)]
    pub fn cprog(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, SfControl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16,1,0,SfControl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear Erase State   CERASE"]
    #[inline(always)]
    pub fn cerase(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, SfControl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17,1,0,SfControl_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for SfControl {
    #[inline(always)]
    fn default() -> SfControl {
        <crate::RegValueT<SfControl_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod sf_control {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lckhsmucb_SPEC;
    pub type Lckhsmucb = crate::EnumBitfieldStruct<u8, Lckhsmucb_SPEC>;
    impl Lckhsmucb {
        #[doc = "00 Locked Reads to UCB HSMCFG forbidden."]
        pub const LOCKED_0: Self = Self::new(0);
        #[doc = "01 Unlocked Reads by HSM to UCB HSMCFG allowed."]
        pub const UNLOCKED_1: Self = Self::new(1);
        #[doc = "10 Locked Reads to UCB HSMCFG forbidden."]
        pub const LOCKED_2: Self = Self::new(2);
        #[doc = "11 Locked Reads to UCB HSMCFG forbidden."]
        pub const LOCKED_3: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SfEccc_SPEC;
impl crate::sealed::RegSpec for SfEccc_SPEC {
    type DataType = u32;
}
#[doc = "HSM DF1 ECC Control Register\n resetvalue={Application Reset:0x0C0000000}"]
pub type SfEccc = crate::RegValueT<SfEccc_SPEC>;

impl SfEccc {
    #[doc = "Clear ECC status bits   CLR"]
    #[inline(always)]
    pub fn clr(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, SfEccc_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x3,1,0,u8, SfEccc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "HSM Command Interface ECC Correction Disable   ECCCORDIS"]
    #[inline(always)]
    pub fn ecccordis(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x3,
        1,
        0,
        sf_eccc::Ecccordis,
        SfEccc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            sf_eccc::Ecccordis,
            SfEccc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "HSM Command Interface Uncorrectable ECC Bit Error Trap Disable   TRAPDIS"]
    #[inline(always)]
    pub fn trapdis(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, SfEccc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, SfEccc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for SfEccc {
    #[inline(always)]
    fn default() -> SfEccc {
        <crate::RegValueT<SfEccc_SPEC> as RegisterValue<_>>::new(3221225472)
    }
}
pub mod sf_eccc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ecccordis_SPEC;
    pub type Ecccordis = crate::EnumBitfieldStruct<u8, Ecccordis_SPEC>;
    impl Ecccordis {
        #[doc = "00 Enabled If        DF1 is configured as HSM exclusive  then  ECC correction for the DF1        read path is enabled."]
        pub const ENABLED_0: Self = Self::new(0);
        #[doc = "01 Enabled If        DF1 is configured as HSM exclusive  then  ECC correction for the DF1        read path is enabled."]
        pub const ENABLED_1: Self = Self::new(1);
        #[doc = "10 Enabled If        DF1 is configured as HSM exclusive  then  ECC correction for the DF1        read path is enabled."]
        pub const ENABLED_2: Self = Self::new(2);
        #[doc = "11 Disabled If        DF1 is configured as HSM exclusive  then  ECC correction for the DF1        read path is disabled."]
        pub const DISABLED_3: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SfEccr_SPEC;
impl crate::sealed::RegSpec for SfEccr_SPEC {
    type DataType = u32;
}
#[doc = "HSM DF1 ECC Read Register\n resetvalue={Application Reset:0x0}"]
pub type SfEccr = crate::RegValueT<SfEccr_SPEC>;

impl SfEccr {
    #[doc = "Error Correction Read Code   RCODE. ECC checksum read during the last NVM read access."]
    #[inline(always)]
    pub fn rcode(
        self,
    ) -> crate::common::RegisterField<0, 0x3fffff, 1, 0, u32, SfEccr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3fffff,1,0,u32, SfEccr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for SfEccr {
    #[inline(always)]
    fn default() -> SfEccr {
        <crate::RegValueT<SfEccr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SfEccs_SPEC;
impl crate::sealed::RegSpec for SfEccs_SPEC {
    type DataType = u32;
}
#[doc = "HSM DF1 ECC Status Register\n resetvalue={Application Reset:0x0}"]
pub type SfEccs = crate::RegValueT<SfEccs_SPEC>;

impl SfEccs {
    #[doc = "Read Access Single Bit ECC Error   ERR1. The flag reports a single bit ECC failure during the last NVM read access. Reset by hardware when DMU SF ECCC .CLR is written to 11 ."]
    #[inline(always)]
    pub fn err1(self) -> crate::common::RegisterFieldBool<0, 1, 0, SfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, SfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Read Access Double Bit ECC Error   ERR2. The flag reports a double bit ECC failure during the last NVM read        access. Reset by hardware when DMU SF ECCC .CLR          is written to 11 ."]
    #[inline(always)]
    pub fn err2(self) -> crate::common::RegisterFieldBool<1, 1, 0, SfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, SfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Read Access Triple Bit ECC Error   ERR3. The flag reports a triple bit ECC failure during the last NVM read        access. Reset by hardware when DMU SF ECCC .CLR          is written to 11 ."]
    #[inline(always)]
    pub fn err3(self) -> crate::common::RegisterFieldBool<2, 1, 0, SfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, SfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Read Access Multi bit ECC Error   ERRM. The flag reports multi bit ECC failure during the last NVM read access. Reset by hardware when DMU SF ECCC .CLR          is written to 11 ."]
    #[inline(always)]
    pub fn errm(self) -> crate::common::RegisterFieldBool<3, 1, 0, SfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, SfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Any Read Access ECC Error   ERRANY. The flag reports any ECC failure during the last NVM read access. Reset by hardware when DMU SF ECCC .CLR          is written to 11 ."]
    #[inline(always)]
    pub fn errany(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, SfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Read Access Blank Analog   BLANKA. The flag reports that all read data cells have sufficient high current  a program of new data without prior erase is possible. Under certain operation history  a valid complement data entry may also appear as blank. Data qualifiers like headers or footers  which are usually used in EEPROM emulation  can be used to distinguish expected valid data from unknown data fields. Only blank failures in Complement Sensing mode are reported in this flag and is intended for use only in this mode. Blank failures in Single Ended mode is also reported when in CFTM. Reset by hardware when DMU SF ECCC .CLR is written to 11 ."]
    #[inline(always)]
    pub fn blanka(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, SfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, SfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated Single Bit ECC Errors   AER1. The status flag accumulates single bit failures during NVM read        operations. Reset by hardware when DMU SF ECCC .CLR          is written to 11 ."]
    #[inline(always)]
    pub fn aer1(self) -> crate::common::RegisterFieldBool<16, 1, 0, SfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, SfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated Double Bit ECC Errors   AER2. The status flag accumulates double bit failures during NVM read        operations. Reset by hardware when DMU SF ECCC .CLR          is written to 11 ."]
    #[inline(always)]
    pub fn aer2(self) -> crate::common::RegisterFieldBool<17, 1, 0, SfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, SfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated Triple Bit ECC Errors   AER3. The status flag accumulates triple bit failures during NVM read operations. Reset by hardware when DMU SF ECCC .CLR is written to 11 ."]
    #[inline(always)]
    pub fn aer3(self) -> crate::common::RegisterFieldBool<18, 1, 0, SfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, SfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated Multi bit ECC Errors   AERM. The status bit accumulates multi bit failures during NVM read accesses. Reset by hardware when DMU SF ECCC .CLR is written to 11 ."]
    #[inline(always)]
    pub fn aerm(self) -> crate::common::RegisterFieldBool<19, 1, 0, SfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, SfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated Any Read Access ECC Error   AERANY. The status bit accumulates ECC failures during NVM read accesses. Reset by hardware when DMU SF ECCC .CLR          is written to 11 ."]
    #[inline(always)]
    pub fn aerany(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, SfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, SfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated Blank Analog   ABLANKA. The flag accumulates analog evaluated blank failures during NVM read accesses. It reports that all read data cells have sufficient high current  a program of new data without prior erase is possible. Under certain operation history  a valid complement data entry may also appear as blank. Data qualifiers like headers or footers  which are usually used in EEPROM emulation  can be used to distinguish expected valid data from unknown data fields. Only blank failures in Complement Sensing mode are reported in this flag and is intended for use only in this mode. Blank failures in Single Ended mode is also reported when in CFTM. Reset by hardware when DMU SF ECCC .CLR is written to 11 ."]
    #[inline(always)]
    pub fn ablanka(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, SfEccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, SfEccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for SfEccs {
    #[inline(always)]
    fn default() -> SfEccs {
        <crate::RegValueT<SfEccs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SfEccw_SPEC;
impl crate::sealed::RegSpec for SfEccw_SPEC {
    type DataType = u32;
}
#[doc = "HSM DF1 ECC Write Register\n resetvalue={Application Reset:0x0}"]
pub type SfEccw = crate::RegValueT<SfEccw_SPEC>;

impl SfEccw {
    #[doc = "Error Correction Write Code   WCODE. 22 bit ECC code for the current 64 bit  for DFLASH  or 256 bit  for        PFLASH  write buffer to be written into the assembly buffer instead of a        generated ECC."]
    #[inline(always)]
    pub fn wcode(
        self,
    ) -> crate::common::RegisterField<0, 0x3fffff, 1, 0, u32, SfEccw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3fffff,1,0,u32, SfEccw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ECC Encoding Disable   ECCENCDIS"]
    #[inline(always)]
    pub fn eccencdis(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, SfEccw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, SfEccw_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for SfEccw {
    #[inline(always)]
    fn default() -> SfEccw {
        <crate::RegValueT<SfEccw_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SfEer_SPEC;
impl crate::sealed::RegSpec for SfEer_SPEC {
    type DataType = u32;
}
#[doc = "HSM Enable Error Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
pub type SfEer = crate::RegValueT<SfEer_SPEC>;

impl SfEer {
    #[doc = "Operation Error Interrupt Mask   OPERM"]
    #[inline(always)]
    pub fn operm(self) -> crate::common::RegisterFieldBool<0, 1, 0, SfEer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, SfEer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Command Sequence Error Interrupt Mask   SQERM"]
    #[inline(always)]
    pub fn sqerm(self) -> crate::common::RegisterFieldBool<1, 1, 0, SfEer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, SfEer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Program Verify Error Interrupt Mask   PVERM"]
    #[inline(always)]
    pub fn pverm(self) -> crate::common::RegisterFieldBool<3, 1, 0, SfEer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, SfEer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Erase Verify Error Interrupt Mask   EVERM"]
    #[inline(always)]
    pub fn everm(self) -> crate::common::RegisterFieldBool<4, 1, 0, SfEer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, SfEer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "End of Busy Interrupt Mask   EOBM"]
    #[inline(always)]
    pub fn eobm(self) -> crate::common::RegisterFieldBool<31, 1, 0, SfEer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, SfEer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for SfEer {
    #[inline(always)]
    fn default() -> SfEer {
        <crate::RegValueT<SfEer_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SfErrsr_SPEC;
impl crate::sealed::RegSpec for SfErrsr_SPEC {
    type DataType = u32;
}
#[doc = "HSM Error Status Register\n resetvalue={Application Reset:0x0}"]
pub type SfErrsr = crate::RegValueT<SfErrsr_SPEC>;

impl SfErrsr {
    #[doc = "Flash Operation Error   OPER. Cleared with system reset."]
    #[inline(always)]
    pub fn oper(self) -> crate::common::RegisterFieldBool<0, 1, 0, SfErrsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, SfErrsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Command Sequence Error   SQER. A sequence error is not indicated if the Reset to Read command aborts a command sequence. Cleared with application reset  commands  Reset to Read  and  Clear Status  or writing DMU SF CLRE .CSQER   1 ."]
    #[inline(always)]
    pub fn sqer(self) -> crate::common::RegisterFieldBool<1, 1, 0, SfErrsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, SfErrsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Program Verify Error   PVER. A verify error was reported on completion of a Flash program operation  even when the operation was directly started in the FSI  . Cleared with application reset  with the command  Clear Status  or writing DMU SF CLRE .CPVER   1 ."]
    #[inline(always)]
    pub fn pver(self) -> crate::common::RegisterFieldBool<3, 1, 0, SfErrsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, SfErrsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Erase Verify Error   EVER. A verify error was reported on completion of a Flash erase operation  even when the operation was directly started in the FSI  . Cleared with application reset  with the command  Clear Status  or writing DMU SF CLRE .CEVER   1 ."]
    #[inline(always)]
    pub fn ever(self) -> crate::common::RegisterFieldBool<4, 1, 0, SfErrsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, SfErrsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for SfErrsr {
    #[inline(always)]
    fn default() -> SfErrsr {
        <crate::RegValueT<SfErrsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SfMargin_SPEC;
impl crate::sealed::RegSpec for SfMargin_SPEC {
    type DataType = u32;
}
#[doc = "HSM DF1 Margin Control Register\n resetvalue={Application Reset:0x0}"]
pub type SfMargin = crate::RegValueT<SfMargin_SPEC>;

impl SfMargin {
    #[doc = "DF1 Margin Read Selection   SELD1. If a change between the standard and hard read margin setting is done           the system must wait a delay time tFL MarginDel until the next read is          requested."]
    #[inline(always)]
    pub fn seld1(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, SfMargin_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, SfMargin_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Hard Margin Selection   HMARGIN. Suboptimal 0 bits are read as 1s. Suboptimal 1 bits are read as 0s. The concrete margin values are restored from the configuration sector        and are determined by Infineon."]
    #[inline(always)]
    pub fn hmargin(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        sf_margin::Hmargin,
        SfMargin_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            sf_margin::Hmargin,
            SfMargin_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for SfMargin {
    #[inline(always)]
    fn default() -> SfMargin {
        <crate::RegValueT<SfMargin_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sf_margin {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hmargin_SPEC;
    pub type Hmargin = crate::EnumBitfieldStruct<u8, Hmargin_SPEC>;
    impl Hmargin {
        #[doc = "0 Tight0 Tight        margin for 0  low  level."]
        pub const TIGHT_00: Self = Self::new(0);
        #[doc = "1 Tight1 Tight        margin for 1  high  level."]
        pub const TIGHT_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SfOperation_SPEC;
impl crate::sealed::RegSpec for SfOperation_SPEC {
    type DataType = u32;
}
#[doc = "HSM Flash Operation Register\n resetvalue={System Reset:0x0}"]
pub type SfOperation = crate::RegValueT<SfOperation_SPEC>;

impl SfOperation {
    #[doc = "Programming State   PROG. HW controlled status flag. Set with last cycle of Write Page Burst command sequence. If one BUSY flag is coincidently set  PROG indicates the type of busy        state. If OPER is coincidently set  PROG indicates the type of erroneous        operation. Otherwise  PROG indicates  that operation is still requested        or finished. Can be also cleared by writing   8216 11  8217  to DMU SF CONTROL .CPROG. This bit is not set for by program operations initiated by the Host        interface. Cleared with command   8220 Clear Status  8221 ."]
    #[inline(always)]
    pub fn prog(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SfOperation_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,SfOperation_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Erase State   ERASE. HW controlled status flag. Set with last cycle of Erase Verify command sequence. Indications are analogous to PROG flag. Can be also cleared by writing   8216 11  8217  to DMU SF CONTROL .CERASE. This bit is not set for by erase operations initiated by the Host        interface. Cleared with command   8220 Clear Status  8221 ."]
    #[inline(always)]
    pub fn erase(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SfOperation_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,SfOperation_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for SfOperation {
    #[inline(always)]
    fn default() -> SfOperation {
        <crate::RegValueT<SfOperation_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SfProconusr_SPEC;
impl crate::sealed::RegSpec for SfProconusr_SPEC {
    type DataType = u32;
}
#[doc = "HSM DF1 User Mode Control\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
pub type SfProconusr = crate::RegValueT<SfProconusr_SPEC>;

impl SfProconusr {
    #[doc = "DF1 User Mode Control   MODE. Configures the DF1 mode when the user has control."]
    #[inline(always)]
    pub fn mode(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, SfProconusr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3,1,0,u8, SfProconusr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for SfProconusr {
    #[inline(always)]
    fn default() -> SfProconusr {
        <crate::RegValueT<SfProconusr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SfStatus_SPEC;
impl crate::sealed::RegSpec for SfStatus_SPEC {
    type DataType = u32;
}
#[doc = "HSM Flash Status Register\n resetvalue={Application Reset:0x2}"]
pub type SfStatus = crate::RegValueT<SfStatus_SPEC>;

impl SfStatus {
    #[doc = "Data Flash Bank 1 Busy   D1BUSY. HW controlled status flag. Indication of busy state of DFLASH bank 1 because of active execution of an operation  DF1 busy state is also indicated during Flash startup after reset or in sleep mode  while in busy state the DF1 does not allow read access."]
    #[inline(always)]
    pub fn d1busy(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SfStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, SfStatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Data Flash in Page Mode   DFPAGE. HW controlled status flag. Set with Enter Page Mode for DFLASH  cleared with Write Page command. Read accesses are allowed while in page mode."]
    #[inline(always)]
    pub fn dfpage(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, SfStatus_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, SfStatus_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for SfStatus {
    #[inline(always)]
    fn default() -> SfStatus {
        <crate::RegValueT<SfStatus_SPEC> as RegisterValue<_>>::new(2)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SfSuspend_SPEC;
impl crate::sealed::RegSpec for SfSuspend_SPEC {
    type DataType = u32;
}
#[doc = "HSM Suspend Control Register\n resetvalue={Application Reset:0x0}"]
pub type SfSuspend = crate::RegValueT<SfSuspend_SPEC>;

impl SfSuspend {
    #[doc = "Suspend Request   REQ"]
    #[inline(always)]
    pub fn req(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SfSuspend_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,SfSuspend_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Suspend Clear   CLR. Software write only active high clear of Suspend Error."]
    #[inline(always)]
    pub fn clr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SfSuspend_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, SfSuspend_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Flash Operation Suspended   SPND. Suspension of a Flash program or erase operation."]
    #[inline(always)]
    pub fn spnd(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, SfSuspend_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16,1,0,SfSuspend_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Suspend Error   ERR. Reset by hardware when DMU SF SUSPEND .CLR is written to 1 ."]
    #[inline(always)]
    pub fn err(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, SfSuspend_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17,1,0,SfSuspend_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for SfSuspend {
    #[inline(always)]
    fn default() -> SfSuspend {
        <crate::RegValueT<SfSuspend_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpProconhsm_SPEC;
impl crate::sealed::RegSpec for SpProconhsm_SPEC {
    type DataType = u32;
}
#[doc = "HSM Interface Protection Configuration\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
pub type SpProconhsm = crate::RegValueT<SpProconhsm_SPEC>;

impl SpProconhsm {
    #[doc = "HSM Debug Disable   HSMDBGDIS. This bit indicates whether HSM debug is configured as   8220 disabled  8221 ."]
    #[inline(always)]
    pub fn hsmdbgdis(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SpProconhsm_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,SpProconhsm_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Debug Interface Locked   DBGIFLCK. This bit indicates whether the chip debug interface is configured as          8220 locked  8221 ."]
    #[inline(always)]
    pub fn dbgiflck(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SpProconhsm_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,SpProconhsm_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Test Interface Locked   TSTIFLCK. This bit indicates whether the chip test interface is configured as  locked . TSTIFLCK is not used."]
    #[inline(always)]
    pub fn tstiflck(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SpProconhsm_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,SpProconhsm_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "HSM Test Disable   HSMTSTDIS. This bit indicates whether the HSM test is configured as   8220 disabled  8221 . HSMTSTDIS is not used."]
    #[inline(always)]
    pub fn hsmtstdis(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SpProconhsm_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,SpProconhsm_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "HSM Trace Disable   HSMTRDIS. This bit field indicates whether the HSM tracing and capturing of        transactions for debug and in error cases via BCU is configured as          8220 disabled  8221 . In order to ensure that with UCB delivery state HSM tracing is fully          enabled the encoding for this bit field is inverted compared to the          corresponding HSM control registers."]
    #[inline(always)]
    pub fn hsmtrdis(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        sp_proconhsm::Hsmtrdis,
        SpProconhsm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            sp_proconhsm::Hsmtrdis,
            SpProconhsm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "HSM Type of Trace   HSMTRTYPE. This bit field indicates which information can be captured by the BCU        for HSM transactions. In order to ensure that with UCB delivery state HSM tracing is fully          enabled the encoding for this bit field is inverted compared to the          corresponding HSM control registers."]
    #[inline(always)]
    pub fn hsmtrtype(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        sp_proconhsm::Hsmtrtype,
        SpProconhsm_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            sp_proconhsm::Hsmtrtype,
            SpProconhsm_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for SpProconhsm {
    #[inline(always)]
    fn default() -> SpProconhsm {
        <crate::RegValueT<SpProconhsm_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sp_proconhsm {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hsmtrdis_SPEC;
    pub type Hsmtrdis = crate::EnumBitfieldStruct<u8, Hsmtrdis_SPEC>;
    impl Hsmtrdis {
        #[doc = "00 TRACEEN Tracing enabled."]
        pub const TRACEEN_0: Self = Self::new(0);
        #[doc = "01 TRACEDIS2 Tracing disabled."]
        pub const TRACEDIS_21: Self = Self::new(1);
        #[doc = "10 TRACEDIS1 Tracing disabled."]
        pub const TRACEDIS_12: Self = Self::new(2);
        #[doc = "11 TRACEDIS0 Tracing disabled."]
        pub const TRACEDIS_03: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hsmtrtype_SPEC;
    pub type Hsmtrtype = crate::EnumBitfieldStruct<u8, Hsmtrtype_SPEC>;
    impl Hsmtrtype {
        #[doc = "0 TRDATA Trace addresses and data."]
        pub const TRDATA_0: Self = Self::new(0);
        #[doc = "1 TRADDR Trace addresses only."]
        pub const TRADDR_1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpProconhsmcbs_SPEC;
impl crate::sealed::RegSpec for SpProconhsmcbs_SPEC {
    type DataType = u32;
}
#[doc = "HSM Code Boot Sector\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
pub type SpProconhsmcbs = crate::RegValueT<SpProconhsmcbs_SPEC>;

impl SpProconhsmcbs {
    #[doc = "Boot Sector Selection   BOOTSEL0. This field controls which of the HSM code sectors is searched for boot code. See CROSSREFERENCE ."]
    #[inline(always)]
    pub fn bootsel0(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, SpProconhsmcbs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,u8, SpProconhsmcbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Boot Sector Selection   BOOTSEL1. This field controls which of the HSM code sectors is searched for boot code. See CROSSREFERENCE ."]
    #[inline(always)]
    pub fn bootsel1(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, SpProconhsmcbs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x3f,1,0,u8, SpProconhsmcbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Boot Sector Selection   BOOTSEL2. This field controls which of the HSM code sectors is searched for boot        code. See CROSSREFERENCE ."]
    #[inline(always)]
    pub fn bootsel2(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, SpProconhsmcbs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x3f,1,0,u8, SpProconhsmcbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Boot Sector Selection   BOOTSEL3. This field controls which of the HSM code sectors is searched for boot code. See CROSSREFERENCE ."]
    #[inline(always)]
    pub fn bootsel3(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, SpProconhsmcbs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0x3f,1,0,u8, SpProconhsmcbs_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for SpProconhsmcbs {
    #[inline(always)]
    fn default() -> SpProconhsmcbs {
        <crate::RegValueT<SpProconhsmcbs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpProconhsmcfg_SPEC;
impl crate::sealed::RegSpec for SpProconhsmcfg_SPEC {
    type DataType = u32;
}
#[doc = "HSM Protection Configuration\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
pub type SpProconhsmcfg = crate::RegValueT<SpProconhsmcfg_SPEC>;

impl SpProconhsmcfg {
    #[doc = "HSM Boot Enable   HSMBOOTEN"]
    #[inline(always)]
    pub fn hsmbooten(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SpProconhsmcfg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,SpProconhsmcfg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SSW Wait   SSWWAIT. Defines if the SSW waits for the HSM to release the jump of CPU0 to user code."]
    #[inline(always)]
    pub fn sswwait(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SpProconhsmcfg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,SpProconhsmcfg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "HSM Data Sectors Exclusive   HSMDX. This bit indicates whether the DFLASH1 logical sectors EEPROMx are configured as   x201c HSM exclusive  x201d ."]
    #[inline(always)]
    pub fn hsmdx(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SpProconhsmcfg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,SpProconhsmcfg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "HSM RAM Clear   HSMRAMKEEP"]
    #[inline(always)]
    pub fn hsmramkeep(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, SpProconhsmcfg_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x3,1,0,u8, SpProconhsmcfg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Enable HSM Forcing of Pins HSM1 2   HSMENPINS. This bit indicates whether HSM may force the value of the pins HSM1 2  i.e. overrule the value driven by the application ."]
    #[inline(always)]
    pub fn hsmenpins(
        self,
    ) -> crate::common::RegisterField<7, 0x3, 1, 0, u8, SpProconhsmcfg_SPEC, crate::common::R> {
        crate::common::RegisterField::<7,0x3,1,0,u8, SpProconhsmcfg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Enable HSM Triggering Resets   HSMENRES. This bit indicates whether HSM may trigger application or system resets."]
    #[inline(always)]
    pub fn hsmenres(
        self,
    ) -> crate::common::RegisterField<9, 0x3, 1, 0, u8, SpProconhsmcfg_SPEC, crate::common::R> {
        crate::common::RegisterField::<9,0x3,1,0,u8, SpProconhsmcfg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Destructive Debug Entry   DESTDBG. This field configures the destructive debug entry. Security  functionally not part of the HSM."]
    #[inline(always)]
    pub fn destdbg(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, SpProconhsmcfg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<11,0x3,1,0,u8, SpProconhsmcfg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Block Flash Analysis   BLKFLAN. The commands   x201c Verify Erased Page  x201d     x201c Verify Erased WL  x201d     x201c Verify Erased Logical Sector Range  x201d   are blocked on the HSM code ranges. Security  functionally not part of the HSM. Also blocks User Margin Count  User Vth Count and User Content Count ."]
    #[inline(always)]
    pub fn blkflan(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, SpProconhsmcfg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,SpProconhsmcfg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for SpProconhsmcfg {
    #[inline(always)]
    fn default() -> SpProconhsmcfg {
        <crate::RegValueT<SpProconhsmcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpProconhsmcotp0_SPEC;
impl crate::sealed::RegSpec for SpProconhsmcotp0_SPEC {
    type DataType = u32;
}
#[doc = "HSM Code OTP Protection Configuration\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
pub type SpProconhsmcotp0 = crate::RegValueT<SpProconhsmcotp0_SPEC>;

impl SpProconhsmcotp0 {
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm0rom(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm1rom(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm2rom(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm3rom(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm4rom(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm5rom(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm6rom(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm7rom(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm8rom(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm9rom(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm10rom(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm11rom(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm12rom(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm13rom(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm14rom(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm15rom(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm16rom(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm17rom(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm18rom(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm19rom(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm20rom(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm21rom(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm22rom(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm23rom(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm24rom(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm25rom(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm26rom(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm27rom(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm28rom(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm29rom(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm30rom(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Locked Forever   HSM31ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm31rom(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, SpProconhsmcotp0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,SpProconhsmcotp0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for SpProconhsmcotp0 {
    #[inline(always)]
    fn default() -> SpProconhsmcotp0 {
        <crate::RegValueT<SpProconhsmcotp0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpProconhsmcotp1_SPEC;
impl crate::sealed::RegSpec for SpProconhsmcotp1_SPEC {
    type DataType = u32;
}
#[doc = "HSM Code OTP Protection Configuration\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
pub type SpProconhsmcotp1 = crate::RegValueT<SpProconhsmcotp1_SPEC>;

impl SpProconhsmcotp1 {
    #[doc = "PFLASH Sector 39 HSM Code Locked Forever   HSM39ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm32rom(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SpProconhsmcotp1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,SpProconhsmcotp1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 39 HSM Code Locked Forever   HSM39ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm33rom(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SpProconhsmcotp1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,SpProconhsmcotp1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 39 HSM Code Locked Forever   HSM39ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm34rom(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SpProconhsmcotp1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,SpProconhsmcotp1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 39 HSM Code Locked Forever   HSM39ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm35rom(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SpProconhsmcotp1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,SpProconhsmcotp1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 39 HSM Code Locked Forever   HSM39ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm36rom(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SpProconhsmcotp1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,SpProconhsmcotp1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 39 HSM Code Locked Forever   HSM39ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm37rom(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SpProconhsmcotp1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,SpProconhsmcotp1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 39 HSM Code Locked Forever   HSM39ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm38rom(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SpProconhsmcotp1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,SpProconhsmcotp1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 39 HSM Code Locked Forever   HSM39ROM. This bit indicates whether PFLASH sector is an HSM Code OTP protected sector with read only functionality."]
    #[inline(always)]
    pub fn hsm39rom(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SpProconhsmcotp1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,SpProconhsmcotp1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for SpProconhsmcotp1 {
    #[inline(always)]
    fn default() -> SpProconhsmcotp1 {
        <crate::RegValueT<SpProconhsmcotp1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpProconhsmcx0_SPEC;
impl crate::sealed::RegSpec for SpProconhsmcx0_SPEC {
    type DataType = u32;
}
#[doc = "HSM Code Exclusive Protection Configuration\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
pub type SpProconhsmcx0 = crate::RegValueT<SpProconhsmcx0_SPEC>;

impl SpProconhsmcx0 {
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm0x(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm1x(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm2x(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm3x(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm4x(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm5x(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm6x(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm7x(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm8x(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm9x(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm10x(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm11x(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm12x(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm13x(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm14x(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm15x(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm16x(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm17x(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm18x(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm19x(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm20x(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm21x(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm22x(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm23x(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm24x(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm25x(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm26x(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm27x(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm28x(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm29x(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm30x(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 31 HSM Code Exclusive   HSM31X. This bit indicates whether the PFLASH logical sector is   8220 HSM exclusive  8221 ."]
    #[inline(always)]
    pub fn hsm31x(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, SpProconhsmcx0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,SpProconhsmcx0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for SpProconhsmcx0 {
    #[inline(always)]
    fn default() -> SpProconhsmcx0 {
        <crate::RegValueT<SpProconhsmcx0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpProconhsmcx1_SPEC;
impl crate::sealed::RegSpec for SpProconhsmcx1_SPEC {
    type DataType = u32;
}
#[doc = "HSM Code Exclusive Protection Configuration\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
pub type SpProconhsmcx1 = crate::RegValueT<SpProconhsmcx1_SPEC>;

impl SpProconhsmcx1 {
    #[doc = "PFLASH Sector 39 HSM Code Exclusive   HSM39X. This bit indicates whether the PFLASH logical sector is  HSM exclusive ."]
    #[inline(always)]
    pub fn hsm32x(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SpProconhsmcx1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,SpProconhsmcx1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 39 HSM Code Exclusive   HSM39X. This bit indicates whether the PFLASH logical sector is  HSM exclusive ."]
    #[inline(always)]
    pub fn hsm33x(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SpProconhsmcx1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,SpProconhsmcx1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 39 HSM Code Exclusive   HSM39X. This bit indicates whether the PFLASH logical sector is  HSM exclusive ."]
    #[inline(always)]
    pub fn hsm34x(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SpProconhsmcx1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,SpProconhsmcx1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 39 HSM Code Exclusive   HSM39X. This bit indicates whether the PFLASH logical sector is  HSM exclusive ."]
    #[inline(always)]
    pub fn hsm35x(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, SpProconhsmcx1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,SpProconhsmcx1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 39 HSM Code Exclusive   HSM39X. This bit indicates whether the PFLASH logical sector is  HSM exclusive ."]
    #[inline(always)]
    pub fn hsm36x(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, SpProconhsmcx1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,SpProconhsmcx1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 39 HSM Code Exclusive   HSM39X. This bit indicates whether the PFLASH logical sector is  HSM exclusive ."]
    #[inline(always)]
    pub fn hsm37x(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, SpProconhsmcx1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,SpProconhsmcx1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 39 HSM Code Exclusive   HSM39X. This bit indicates whether the PFLASH logical sector is  HSM exclusive ."]
    #[inline(always)]
    pub fn hsm38x(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, SpProconhsmcx1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,SpProconhsmcx1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PFLASH Sector 39 HSM Code Exclusive   HSM39X. This bit indicates whether the PFLASH logical sector is  HSM exclusive ."]
    #[inline(always)]
    pub fn hsm39x(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, SpProconhsmcx1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,SpProconhsmcx1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for SpProconhsmcx1 {
    #[inline(always)]
    fn default() -> SpProconhsmcx1 {
        <crate::RegValueT<SpProconhsmcx1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc = "HP"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hp(pub(super) *mut u8);
unsafe impl core::marker::Send for Hp {}
unsafe impl core::marker::Sync for Hp {}
impl Hp {
    #[doc = "PFLASH Bank 0 Erase Counter Priority configuration 0\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_ecprioi0(&self) -> crate::common::Reg<hp::HpEcpriOi0_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }
    #[doc = "PFLASH Bank 0 Erase Counter Priority Configuration 1\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_ecprioi1(&self) -> crate::common::Reg<hp::HpEcpriOi1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(164usize)) }
    }
    #[doc = "PFLASH Bank 0 Erase Counter Priority Configuration 2\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_ecprioi2(&self) -> crate::common::Reg<hp::HpEcpriOi2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(168usize)) }
    }
    #[doc = "PFLASH Bank 0 Erase Counter Priority Configuration 3\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_ecprioi3(&self) -> crate::common::Reg<hp::HpEcpriOi3_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(172usize)) }
    }
    #[doc = "PFLASH Bank 0 Erase Counter Priority Configuration 4\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_ecprioi4(&self) -> crate::common::Reg<hp::HpEcpriOi4_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(176usize)) }
    }
    #[doc = "PFLASH Bank 0 Erase Counter Priority Configuration 5\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_ecprioi5(&self) -> crate::common::Reg<hp::HpEcpriOi5_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(180usize)) }
    }
    #[doc = "PFLASH Bank 0 OTP Protection Configuration 0\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_proconotpi0(
        &self,
    ) -> crate::common::Reg<hp::HpProconotPi0_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }
    #[doc = "PFLASH Bank 0 OTP Protection Configuration 1\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_proconotpi1(
        &self,
    ) -> crate::common::Reg<hp::HpProconotPi1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }
    #[doc = "PFLASH Bank 0 OTP Protection Configuration 2\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_proconotpi2(
        &self,
    ) -> crate::common::Reg<hp::HpProconotPi2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }
    #[doc = "PFLASH Bank 0 OTP Protection Configuration 3\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_proconotpi3(
        &self,
    ) -> crate::common::Reg<hp::HpProconotPi3_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(76usize)) }
    }
    #[doc = "PFLASH Bank 0 OTP Protection Configuration 4\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_proconotpi4(
        &self,
    ) -> crate::common::Reg<hp::HpProconotPi4_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }
    #[doc = "PFLASH Bank 0 OTP Protection Configuration 5\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_proconotpi5(
        &self,
    ) -> crate::common::Reg<hp::HpProconotPi5_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(84usize)) }
    }
    #[doc = "PFLASH Bank 0 Protection Configuration 0\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_proconpi0(&self) -> crate::common::Reg<hp::HpProconPi0_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "PFLASH Bank 0 Protection Configuration 1\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_proconpi1(&self) -> crate::common::Reg<hp::HpProconPi1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "PFLASH Bank 0 Protection Configuration 2\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_proconpi2(&self) -> crate::common::Reg<hp::HpProconPi2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "PFLASH Bank 0 Protection Configuration 3\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_proconpi3(&self) -> crate::common::Reg<hp::HpProconPi3_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "PFLASH Bank 0 Protection Configuration 4\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_proconpi4(&self) -> crate::common::Reg<hp::HpProconPi4_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "PFLASH Bank 0 Protection Configuration 5\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_proconpi5(&self) -> crate::common::Reg<hp::HpProconPi5_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "PFLASH Bank 0 WOP Configuration 0\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_proconwopi0(
        &self,
    ) -> crate::common::Reg<hp::HpProconwoPi0_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }
    #[doc = "PFLASH Bank 0 WOP Configuration 1\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_proconwopi1(
        &self,
    ) -> crate::common::Reg<hp::HpProconwoPi1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }
    #[doc = "PFLASH Bank 0 WOP Configuration 2\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_proconwopi2(
        &self,
    ) -> crate::common::Reg<hp::HpProconwoPi2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }
    #[doc = "PFLASH Bank 0 WOP Configuration 3\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_proconwopi3(
        &self,
    ) -> crate::common::Reg<hp::HpProconwoPi3_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(140usize)) }
    }
    #[doc = "PFLASH Bank 0 WOP Configuration 4\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_proconwopi4(
        &self,
    ) -> crate::common::Reg<hp::HpProconwoPi4_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }
    #[doc = "PFLASH Bank 0 WOP Configuration 5\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn hp_proconwopi5(
        &self,
    ) -> crate::common::Reg<hp::HpProconwoPi5_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }
}
pub mod hp {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpEcpriOi0_SPEC;
    impl crate::sealed::RegSpec for HpEcpriOi0_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 Erase Counter Priority configuration 0\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpEcpriOi0 = crate::RegValueT<HpEcpriOi0_SPEC>;

    impl HpEcpriOi0 {
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s0l(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s1l(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<1,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s2l(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<2,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s3l(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<3,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s4l(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<4,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s5l(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<5,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s6l(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<6,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s7l(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<7,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s8l(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<8,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s9l(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<9,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s10l(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<10,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s11l(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<11,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s12l(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<12,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s13l(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<13,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s14l(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<14,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s15l(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<15,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s16l(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<16,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s17l(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<17,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s18l(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<18,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s19l(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<19,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s20l(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<20,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s21l(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<21,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s22l(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<22,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s23l(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<23,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s24l(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<24,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s25l(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<25,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s26l(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<26,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s27l(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<27,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s28l(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<28,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s29l(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<29,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s30l(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<30,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Erase Counter priority   S31L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s31l(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpEcpriOi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<31,1,0,HpEcpriOi0_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpEcpriOi0 {
        #[inline(always)]
        fn default() -> HpEcpriOi0 {
            <crate::RegValueT<HpEcpriOi0_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpEcpriOi1_SPEC;
    impl crate::sealed::RegSpec for HpEcpriOi1_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 Erase Counter Priority Configuration 1\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpEcpriOi1 = crate::RegValueT<HpEcpriOi1_SPEC>;

    impl HpEcpriOi1 {
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s32l(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s33l(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<1,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s34l(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<2,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s35l(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<3,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s36l(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<4,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s37l(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<5,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s38l(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<6,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s39l(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<7,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s40l(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<8,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s41l(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<9,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s42l(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<10,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s43l(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<11,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s44l(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<12,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s45l(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<13,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s46l(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<14,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s47l(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<15,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s48l(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<16,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s49l(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<17,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s50l(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<18,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s51l(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<19,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s52l(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<20,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s53l(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<21,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s54l(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<22,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s55l(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<23,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s56l(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<24,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s57l(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<25,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s58l(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<26,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s59l(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<27,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s60l(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<28,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s61l(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<29,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s62l(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<30,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Erase Counter priority   S63L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s63l(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpEcpriOi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<31,1,0,HpEcpriOi1_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpEcpriOi1 {
        #[inline(always)]
        fn default() -> HpEcpriOi1 {
            <crate::RegValueT<HpEcpriOi1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpEcpriOi2_SPEC;
    impl crate::sealed::RegSpec for HpEcpriOi2_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 Erase Counter Priority Configuration 2\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpEcpriOi2 = crate::RegValueT<HpEcpriOi2_SPEC>;

    impl HpEcpriOi2 {
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s64l(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s65l(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<1,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s66l(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<2,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s67l(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<3,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s68l(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<4,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s69l(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<5,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s70l(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<6,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s71l(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<7,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s72l(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<8,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s73l(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<9,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s74l(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<10,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s75l(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<11,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s76l(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<12,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s77l(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<13,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s78l(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<14,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s79l(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<15,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s80l(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<16,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s81l(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<17,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s82l(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<18,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s83l(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<19,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s84l(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<20,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s85l(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<21,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s86l(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<22,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s87l(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<23,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s88l(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<24,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s89l(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<25,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s90l(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<26,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s91l(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<27,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s92l(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<28,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s93l(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<29,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s94l(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<30,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Erase Counter priority   S95L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s95l(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpEcpriOi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<31,1,0,HpEcpriOi2_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpEcpriOi2 {
        #[inline(always)]
        fn default() -> HpEcpriOi2 {
            <crate::RegValueT<HpEcpriOi2_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpEcpriOi3_SPEC;
    impl crate::sealed::RegSpec for HpEcpriOi3_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 Erase Counter Priority Configuration 3\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpEcpriOi3 = crate::RegValueT<HpEcpriOi3_SPEC>;

    impl HpEcpriOi3 {
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s96l(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s97l(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<1,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s98l(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<2,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s99l(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<3,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s100l(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<4,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s101l(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<5,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s102l(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<6,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s103l(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<7,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s104l(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<8,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s105l(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<9,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s106l(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<10,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s107l(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<11,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s108l(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<12,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s109l(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<13,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s110l(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<14,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s111l(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<15,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s112l(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<16,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s113l(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<17,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s114l(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<18,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s115l(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<19,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s116l(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<20,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s117l(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<21,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s118l(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<22,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s119l(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<23,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s120l(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<24,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s121l(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<25,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s122l(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<26,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s123l(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<27,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s124l(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<28,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s125l(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<29,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s126l(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<30,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Erase Counter priority   S127L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s127l(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpEcpriOi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<31,1,0,HpEcpriOi3_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpEcpriOi3 {
        #[inline(always)]
        fn default() -> HpEcpriOi3 {
            <crate::RegValueT<HpEcpriOi3_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpEcpriOi4_SPEC;
    impl crate::sealed::RegSpec for HpEcpriOi4_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 Erase Counter Priority Configuration 4\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpEcpriOi4 = crate::RegValueT<HpEcpriOi4_SPEC>;

    impl HpEcpriOi4 {
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s128l(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s129l(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<1,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s130l(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<2,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s131l(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<3,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s132l(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<4,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s133l(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<5,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s134l(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<6,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s135l(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<7,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s136l(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<8,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s137l(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<9,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s138l(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<10,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s139l(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<11,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s140l(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<12,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s141l(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<13,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s142l(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<14,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s143l(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<15,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s144l(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<16,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s145l(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<17,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s146l(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<18,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s147l(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<19,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s148l(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<20,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s149l(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<21,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s150l(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<22,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s151l(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<23,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s152l(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<24,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s153l(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<25,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s154l(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<26,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s155l(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<27,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s156l(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<28,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s157l(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<29,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s158l(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<30,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Erase Counter priority   S159L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s159l(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpEcpriOi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<31,1,0,HpEcpriOi4_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpEcpriOi4 {
        #[inline(always)]
        fn default() -> HpEcpriOi4 {
            <crate::RegValueT<HpEcpriOi4_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpEcpriOi5_SPEC;
    impl crate::sealed::RegSpec for HpEcpriOi5_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 Erase Counter Priority Configuration 5\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpEcpriOi5 = crate::RegValueT<HpEcpriOi5_SPEC>;

    impl HpEcpriOi5 {
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s160l(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s161l(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<1,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s162l(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<2,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s163l(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<3,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s164l(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<4,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s165l(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<5,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s166l(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<6,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s167l(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<7,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s168l(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<8,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s169l(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<9,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s170l(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<10,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s171l(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<11,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s172l(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<12,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s173l(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<13,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s174l(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<14,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s175l(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<15,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s176l(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<16,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s177l(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<17,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s178l(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<18,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s179l(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<19,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s180l(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<20,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s181l(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<21,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s182l(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<22,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s183l(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<23,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s184l(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<24,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s185l(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<25,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s186l(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<26,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s187l(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<27,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s188l(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<28,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s189l(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<29,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s190l(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<30,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Erase Counter priority   S191L. These bits indicate whether PFLASH sector x has high or low priority for        Erase Counter recording."]
        #[inline(always)]
        pub fn s191l(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpEcpriOi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<31,1,0,HpEcpriOi5_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpEcpriOi5 {
        #[inline(always)]
        fn default() -> HpEcpriOi5 {
            <crate::RegValueT<HpEcpriOi5_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpProconotPi0_SPEC;
    impl crate::sealed::RegSpec for HpProconotPi0_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 OTP Protection Configuration 0\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpProconotPi0 = crate::RegValueT<HpProconotPi0_SPEC>;

    impl HpProconotPi0 {
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s0rom(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<0,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s1rom(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<1,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s2rom(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<2,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s3rom(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<3,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s4rom(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<4,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s5rom(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<5,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s6rom(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<6,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s7rom(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<7,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s8rom(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<8,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s9rom(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<9,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s10rom(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<10,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s11rom(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<11,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s12rom(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<12,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s13rom(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<13,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s14rom(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<14,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s15rom(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<15,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s16rom(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<16,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s17rom(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<17,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s18rom(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<18,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s19rom(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<19,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s20rom(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<20,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s21rom(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<21,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s22rom(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<22,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s23rom(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<23,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s24rom(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<24,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s25rom(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<25,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s26rom(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<26,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s27rom(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s28rom(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<28,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s29rom(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<29,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s30rom(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<30,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Forever   S31ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s31rom(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpProconotPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,HpProconotPi0_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpProconotPi0 {
        #[inline(always)]
        fn default() -> HpProconotPi0 {
            <crate::RegValueT<HpProconotPi0_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpProconotPi1_SPEC;
    impl crate::sealed::RegSpec for HpProconotPi1_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 OTP Protection Configuration 1\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpProconotPi1 = crate::RegValueT<HpProconotPi1_SPEC>;

    impl HpProconotPi1 {
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s32rom(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<0,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s33rom(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<1,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s34rom(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<2,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s35rom(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<3,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s36rom(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<4,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s37rom(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<5,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s38rom(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<6,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s39rom(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<7,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s40rom(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<8,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s41rom(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<9,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s42rom(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<10,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s43rom(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<11,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s44rom(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<12,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s45rom(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<13,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s46rom(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<14,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s47rom(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<15,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s48rom(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<16,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s49rom(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<17,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s50rom(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<18,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s51rom(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<19,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s52rom(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<20,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s53rom(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<21,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s54rom(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<22,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s55rom(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<23,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s56rom(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<24,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s57rom(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<25,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s58rom(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<26,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s59rom(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s60rom(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<28,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s61rom(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<29,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s62rom(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<30,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Forever   S63ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s63rom(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpProconotPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,HpProconotPi1_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpProconotPi1 {
        #[inline(always)]
        fn default() -> HpProconotPi1 {
            <crate::RegValueT<HpProconotPi1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpProconotPi2_SPEC;
    impl crate::sealed::RegSpec for HpProconotPi2_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 OTP Protection Configuration 2\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpProconotPi2 = crate::RegValueT<HpProconotPi2_SPEC>;

    impl HpProconotPi2 {
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s64rom(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<0,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s65rom(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<1,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s66rom(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<2,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s67rom(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<3,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s68rom(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<4,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s69rom(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<5,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s70rom(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<6,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s71rom(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<7,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s72rom(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<8,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s73rom(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<9,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s74rom(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<10,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s75rom(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<11,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s76rom(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<12,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s77rom(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<13,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s78rom(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<14,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s79rom(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<15,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s80rom(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<16,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s81rom(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<17,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s82rom(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<18,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s83rom(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<19,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s84rom(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<20,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s85rom(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<21,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s86rom(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<22,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s87rom(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<23,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s88rom(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<24,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s89rom(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<25,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s90rom(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<26,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s91rom(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s92rom(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<28,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s93rom(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<29,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s94rom(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<30,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Forever   S95ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s95rom(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpProconotPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,HpProconotPi2_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpProconotPi2 {
        #[inline(always)]
        fn default() -> HpProconotPi2 {
            <crate::RegValueT<HpProconotPi2_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpProconotPi3_SPEC;
    impl crate::sealed::RegSpec for HpProconotPi3_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 OTP Protection Configuration 3\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpProconotPi3 = crate::RegValueT<HpProconotPi3_SPEC>;

    impl HpProconotPi3 {
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s96rom(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<0,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s97rom(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<1,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s98rom(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<2,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s99rom(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<3,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s100rom(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<4,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s101rom(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<5,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s102rom(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<6,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s103rom(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<7,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s104rom(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<8,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s105rom(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<9,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s106rom(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<10,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s107rom(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<11,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s108rom(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<12,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s109rom(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<13,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s110rom(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<14,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s111rom(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<15,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s112rom(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<16,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s113rom(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<17,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s114rom(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<18,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s115rom(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<19,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s116rom(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<20,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s117rom(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<21,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s118rom(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<22,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s119rom(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<23,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s120rom(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<24,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s121rom(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<25,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s122rom(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<26,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s123rom(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s124rom(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<28,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s125rom(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<29,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s126rom(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<30,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Forever   S127ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s127rom(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpProconotPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,HpProconotPi3_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpProconotPi3 {
        #[inline(always)]
        fn default() -> HpProconotPi3 {
            <crate::RegValueT<HpProconotPi3_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpProconotPi4_SPEC;
    impl crate::sealed::RegSpec for HpProconotPi4_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 OTP Protection Configuration 4\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpProconotPi4 = crate::RegValueT<HpProconotPi4_SPEC>;

    impl HpProconotPi4 {
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s128rom(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<0,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s129rom(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<1,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s130rom(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<2,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s131rom(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<3,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s132rom(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<4,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s133rom(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<5,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s134rom(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<6,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s135rom(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<7,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s136rom(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<8,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s137rom(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<9,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s138rom(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<10,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s139rom(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<11,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s140rom(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<12,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s141rom(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<13,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s142rom(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<14,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s143rom(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<15,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s144rom(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<16,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s145rom(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<17,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s146rom(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<18,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s147rom(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<19,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s148rom(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<20,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s149rom(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<21,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s150rom(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<22,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s151rom(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<23,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s152rom(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<24,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s153rom(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<25,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s154rom(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<26,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s155rom(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s156rom(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<28,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s157rom(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<29,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s158rom(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<30,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Forever   S159ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s159rom(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpProconotPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,HpProconotPi4_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpProconotPi4 {
        #[inline(always)]
        fn default() -> HpProconotPi4 {
            <crate::RegValueT<HpProconotPi4_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpProconotPi5_SPEC;
    impl crate::sealed::RegSpec for HpProconotPi5_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 OTP Protection Configuration 5\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpProconotPi5 = crate::RegValueT<HpProconotPi5_SPEC>;

    impl HpProconotPi5 {
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s160rom(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<0,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s161rom(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<1,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s162rom(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<2,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s163rom(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<3,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s164rom(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<4,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s165rom(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<5,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s166rom(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<6,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s167rom(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<7,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s168rom(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<8,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s169rom(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<9,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s170rom(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<10,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s171rom(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<11,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s172rom(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<12,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s173rom(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<13,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s174rom(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<14,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s175rom(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<15,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s176rom(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<16,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s177rom(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<17,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s178rom(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<18,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s179rom(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<19,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s180rom(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<20,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s181rom(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<21,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s182rom(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<22,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s183rom(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<23,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s184rom(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<24,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s185rom(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<25,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s186rom(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<26,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s187rom(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s188rom(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<28,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s189rom(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<29,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s190rom(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<30,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Forever   S191ROM. These bits indicate whether PFLASH p sector x is an OTP protected sector with read only functionality."]
        #[inline(always)]
        pub fn s191rom(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpProconotPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,HpProconotPi5_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpProconotPi5 {
        #[inline(always)]
        fn default() -> HpProconotPi5 {
            <crate::RegValueT<HpProconotPi5_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpProconPi0_SPEC;
    impl crate::sealed::RegSpec for HpProconPi0_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 Protection Configuration 0\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpProconPi0 = crate::RegValueT<HpProconPi0_SPEC>;

    impl HpProconPi0 {
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s0l(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpProconPi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s1l(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpProconPi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<1,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s2l(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpProconPi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<2,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s3l(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpProconPi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<3,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s4l(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpProconPi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<4,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s5l(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpProconPi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<5,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s6l(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpProconPi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<6,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s7l(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpProconPi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<7,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s8l(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpProconPi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<8,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s9l(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpProconPi0_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<9,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s10l(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpProconPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<10,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s11l(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpProconPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<11,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s12l(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpProconPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<12,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s13l(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpProconPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<13,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s14l(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpProconPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<14,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s15l(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpProconPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<15,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s16l(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpProconPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<16,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s17l(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpProconPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<17,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s18l(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpProconPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<18,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s19l(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpProconPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<19,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s20l(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpProconPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<20,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s21l(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpProconPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<21,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s22l(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpProconPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<22,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s23l(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpProconPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<23,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s24l(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpProconPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<24,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s25l(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpProconPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<25,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s26l(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpProconPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<26,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s27l(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpProconPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s28l(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpProconPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<28,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s29l(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpProconPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<29,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s30l(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpProconPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<30,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Locked for Write Protection   S31L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s31l(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpProconPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,HpProconPi0_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpProconPi0 {
        #[inline(always)]
        fn default() -> HpProconPi0 {
            <crate::RegValueT<HpProconPi0_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpProconPi1_SPEC;
    impl crate::sealed::RegSpec for HpProconPi1_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 Protection Configuration 1\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpProconPi1 = crate::RegValueT<HpProconPi1_SPEC>;

    impl HpProconPi1 {
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s32l(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpProconPi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s33l(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpProconPi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<1,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s34l(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpProconPi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<2,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s35l(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpProconPi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<3,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s36l(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpProconPi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<4,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s37l(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpProconPi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<5,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s38l(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpProconPi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<6,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s39l(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpProconPi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<7,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s40l(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpProconPi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<8,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s41l(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpProconPi1_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<9,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s42l(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpProconPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<10,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s43l(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpProconPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<11,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s44l(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpProconPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<12,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s45l(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpProconPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<13,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s46l(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpProconPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<14,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s47l(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpProconPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<15,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s48l(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpProconPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<16,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s49l(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpProconPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<17,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s50l(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpProconPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<18,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s51l(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpProconPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<19,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s52l(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpProconPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<20,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s53l(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpProconPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<21,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s54l(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpProconPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<22,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s55l(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpProconPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<23,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s56l(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpProconPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<24,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s57l(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpProconPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<25,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s58l(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpProconPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<26,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s59l(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpProconPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s60l(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpProconPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<28,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s61l(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpProconPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<29,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s62l(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpProconPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<30,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Locked for Write Protection   S63L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s63l(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpProconPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,HpProconPi1_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpProconPi1 {
        #[inline(always)]
        fn default() -> HpProconPi1 {
            <crate::RegValueT<HpProconPi1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpProconPi2_SPEC;
    impl crate::sealed::RegSpec for HpProconPi2_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 Protection Configuration 2\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpProconPi2 = crate::RegValueT<HpProconPi2_SPEC>;

    impl HpProconPi2 {
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s64l(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpProconPi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s65l(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpProconPi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<1,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s66l(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpProconPi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<2,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s67l(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpProconPi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<3,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s68l(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpProconPi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<4,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s69l(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpProconPi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<5,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s70l(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpProconPi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<6,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s71l(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpProconPi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<7,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s72l(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpProconPi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<8,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s73l(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpProconPi2_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<9,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s74l(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpProconPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<10,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s75l(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpProconPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<11,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s76l(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpProconPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<12,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s77l(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpProconPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<13,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s78l(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpProconPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<14,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s79l(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpProconPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<15,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s80l(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpProconPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<16,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s81l(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpProconPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<17,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s82l(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpProconPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<18,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s83l(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpProconPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<19,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s84l(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpProconPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<20,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s85l(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpProconPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<21,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s86l(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpProconPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<22,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s87l(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpProconPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<23,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s88l(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpProconPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<24,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s89l(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpProconPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<25,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s90l(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpProconPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<26,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s91l(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpProconPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s92l(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpProconPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<28,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s93l(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpProconPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<29,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s94l(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpProconPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<30,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Locked for Write Protection   S95L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s95l(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpProconPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,HpProconPi2_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpProconPi2 {
        #[inline(always)]
        fn default() -> HpProconPi2 {
            <crate::RegValueT<HpProconPi2_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpProconPi3_SPEC;
    impl crate::sealed::RegSpec for HpProconPi3_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 Protection Configuration 3\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpProconPi3 = crate::RegValueT<HpProconPi3_SPEC>;

    impl HpProconPi3 {
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s96l(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpProconPi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s97l(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpProconPi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<1,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s98l(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpProconPi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<2,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s99l(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpProconPi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<3,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s100l(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpProconPi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<4,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s101l(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpProconPi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<5,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s102l(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpProconPi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<6,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s103l(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpProconPi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<7,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s104l(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpProconPi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<8,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s105l(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpProconPi3_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<9,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s106l(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpProconPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<10,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s107l(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpProconPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<11,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s108l(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpProconPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<12,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s109l(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpProconPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<13,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s110l(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpProconPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<14,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s111l(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpProconPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<15,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s112l(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpProconPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<16,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s113l(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpProconPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<17,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s114l(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpProconPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<18,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s115l(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpProconPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<19,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s116l(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpProconPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<20,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s117l(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpProconPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<21,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s118l(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpProconPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<22,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s119l(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpProconPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<23,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s120l(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpProconPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<24,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s121l(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpProconPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<25,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s122l(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpProconPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<26,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s123l(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpProconPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s124l(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpProconPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<28,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s125l(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpProconPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<29,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s126l(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpProconPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<30,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Locked for Write Protection   S127L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s127l(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpProconPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,HpProconPi3_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpProconPi3 {
        #[inline(always)]
        fn default() -> HpProconPi3 {
            <crate::RegValueT<HpProconPi3_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpProconPi4_SPEC;
    impl crate::sealed::RegSpec for HpProconPi4_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 Protection Configuration 4\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpProconPi4 = crate::RegValueT<HpProconPi4_SPEC>;

    impl HpProconPi4 {
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s128l(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpProconPi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s129l(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpProconPi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<1,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s130l(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpProconPi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<2,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s131l(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpProconPi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<3,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s132l(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpProconPi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<4,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s133l(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpProconPi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<5,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s134l(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpProconPi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<6,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s135l(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpProconPi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<7,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s136l(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpProconPi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<8,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s137l(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpProconPi4_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<9,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s138l(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpProconPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<10,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s139l(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpProconPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<11,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s140l(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpProconPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<12,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s141l(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpProconPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<13,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s142l(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpProconPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<14,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s143l(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpProconPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<15,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s144l(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpProconPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<16,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s145l(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpProconPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<17,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s146l(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpProconPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<18,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s147l(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpProconPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<19,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s148l(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpProconPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<20,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s149l(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpProconPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<21,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s150l(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpProconPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<22,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s151l(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpProconPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<23,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s152l(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpProconPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<24,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s153l(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpProconPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<25,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s154l(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpProconPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<26,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s155l(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpProconPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s156l(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpProconPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<28,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s157l(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpProconPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<29,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s158l(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpProconPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<30,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Locked for Write Protection   S159L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s159l(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpProconPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,HpProconPi4_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpProconPi4 {
        #[inline(always)]
        fn default() -> HpProconPi4 {
            <crate::RegValueT<HpProconPi4_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpProconPi5_SPEC;
    impl crate::sealed::RegSpec for HpProconPi5_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 Protection Configuration 5\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpProconPi5 = crate::RegValueT<HpProconPi5_SPEC>;

    impl HpProconPi5 {
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s160l(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpProconPi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s161l(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpProconPi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<1,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s162l(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpProconPi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<2,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s163l(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpProconPi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<3,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s164l(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpProconPi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<4,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s165l(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpProconPi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<5,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s166l(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpProconPi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<6,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s167l(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpProconPi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<7,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s168l(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpProconPi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<8,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s169l(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpProconPi5_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<9,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s170l(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpProconPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<10,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s171l(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpProconPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<11,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s172l(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpProconPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<12,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s173l(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpProconPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<13,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s174l(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpProconPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<14,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s175l(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpProconPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<15,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s176l(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpProconPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<16,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s177l(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpProconPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<17,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s178l(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpProconPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<18,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s179l(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpProconPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<19,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s180l(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpProconPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<20,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s181l(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpProconPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<21,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s182l(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpProconPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<22,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s183l(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpProconPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<23,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s184l(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpProconPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<24,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s185l(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpProconPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<25,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s186l(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpProconPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<26,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s187l(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpProconPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s188l(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpProconPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<28,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s189l(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpProconPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<29,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s190l(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpProconPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<30,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Locked for Write Protection   S191L. These bits indicate whether PFLASH sector x is write protected."]
        #[inline(always)]
        pub fn s191l(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpProconPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,HpProconPi5_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpProconPi5 {
        #[inline(always)]
        fn default() -> HpProconPi5 {
            <crate::RegValueT<HpProconPi5_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpProconwoPi0_SPEC;
    impl crate::sealed::RegSpec for HpProconwoPi0_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 WOP Configuration 0\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpProconwoPi0 = crate::RegValueT<HpProconwoPi0_SPEC>;

    impl HpProconwoPi0 {
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s0wop(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<0,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s1wop(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<1,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s2wop(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<2,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s3wop(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<3,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s4wop(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<4,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s5wop(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<5,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s6wop(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<6,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s7wop(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<7,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s8wop(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<8,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s9wop(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<9,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s10wop(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<10,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s11wop(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<11,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s12wop(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<12,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s13wop(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<13,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s14wop(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<14,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s15wop(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<15,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s16wop(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<16,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s17wop(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<17,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s18wop(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<18,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s19wop(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<19,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s20wop(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<20,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s21wop(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<21,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s22wop(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<22,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s23wop(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<23,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s24wop(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<24,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s25wop(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<25,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s26wop(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<26,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s27wop(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s28wop(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<28,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s29wop(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<29,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s30wop(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<30,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 31 Configured for Write Once Protection   S31WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s31wop(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpProconwoPi0_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,HpProconwoPi0_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpProconwoPi0 {
        #[inline(always)]
        fn default() -> HpProconwoPi0 {
            <crate::RegValueT<HpProconwoPi0_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpProconwoPi1_SPEC;
    impl crate::sealed::RegSpec for HpProconwoPi1_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 WOP Configuration 1\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpProconwoPi1 = crate::RegValueT<HpProconwoPi1_SPEC>;

    impl HpProconwoPi1 {
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s32wop(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<0,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s33wop(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<1,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s34wop(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<2,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s35wop(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<3,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s36wop(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<4,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s37wop(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<5,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s38wop(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<6,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s39wop(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<7,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s40wop(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<8,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s41wop(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<9,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s42wop(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<10,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s43wop(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<11,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s44wop(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<12,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s45wop(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<13,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s46wop(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<14,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s47wop(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<15,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s48wop(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<16,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s49wop(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<17,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s50wop(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<18,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s51wop(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<19,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s52wop(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<20,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s53wop(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<21,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s54wop(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<22,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s55wop(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<23,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s56wop(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<24,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s57wop(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<25,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s58wop(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<26,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s59wop(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s60wop(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<28,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s61wop(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<29,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s62wop(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<30,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 63 Configured for Write Once Protection   S63WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s63wop(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpProconwoPi1_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,HpProconwoPi1_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpProconwoPi1 {
        #[inline(always)]
        fn default() -> HpProconwoPi1 {
            <crate::RegValueT<HpProconwoPi1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpProconwoPi2_SPEC;
    impl crate::sealed::RegSpec for HpProconwoPi2_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 WOP Configuration 2\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpProconwoPi2 = crate::RegValueT<HpProconwoPi2_SPEC>;

    impl HpProconwoPi2 {
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s64wop(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<0,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s65wop(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<1,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s66wop(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<2,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s67wop(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<3,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s68wop(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<4,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s69wop(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<5,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s70wop(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<6,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s71wop(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<7,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s72wop(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<8,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s73wop(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<9,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s74wop(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<10,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s75wop(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<11,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s76wop(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<12,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s77wop(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<13,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s78wop(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<14,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s79wop(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<15,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s80wop(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<16,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s81wop(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<17,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s82wop(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<18,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s83wop(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<19,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s84wop(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<20,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s85wop(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<21,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s86wop(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<22,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s87wop(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<23,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s88wop(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<24,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s89wop(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<25,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s90wop(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<26,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s91wop(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s92wop(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<28,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s93wop(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<29,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s94wop(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<30,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 95 Configured for Write Once Protection   S95WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s95wop(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpProconwoPi2_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,HpProconwoPi2_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpProconwoPi2 {
        #[inline(always)]
        fn default() -> HpProconwoPi2 {
            <crate::RegValueT<HpProconwoPi2_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpProconwoPi3_SPEC;
    impl crate::sealed::RegSpec for HpProconwoPi3_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 WOP Configuration 3\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpProconwoPi3 = crate::RegValueT<HpProconwoPi3_SPEC>;

    impl HpProconwoPi3 {
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s96wop(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<0,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s97wop(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<1,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s98wop(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<2,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s99wop(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<3,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s100wop(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<4,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s101wop(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<5,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s102wop(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<6,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s103wop(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<7,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s104wop(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<8,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s105wop(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<9,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s106wop(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<10,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s107wop(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<11,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s108wop(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<12,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s109wop(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<13,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s110wop(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<14,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s111wop(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<15,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s112wop(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<16,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s113wop(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<17,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s114wop(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<18,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s115wop(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<19,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s116wop(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<20,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s117wop(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<21,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s118wop(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<22,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s119wop(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<23,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s120wop(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<24,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s121wop(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<25,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s122wop(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<26,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s123wop(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s124wop(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<28,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s125wop(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<29,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s126wop(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<30,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 127 Configured for Write Once Protection   S127WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s127wop(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpProconwoPi3_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,HpProconwoPi3_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpProconwoPi3 {
        #[inline(always)]
        fn default() -> HpProconwoPi3 {
            <crate::RegValueT<HpProconwoPi3_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpProconwoPi4_SPEC;
    impl crate::sealed::RegSpec for HpProconwoPi4_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 WOP Configuration 4\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpProconwoPi4 = crate::RegValueT<HpProconwoPi4_SPEC>;

    impl HpProconwoPi4 {
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s128wop(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<0,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s129wop(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<1,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s130wop(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<2,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s131wop(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<3,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s132wop(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<4,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s133wop(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<5,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s134wop(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<6,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s135wop(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<7,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s136wop(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<8,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s137wop(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<9,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s138wop(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<10,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s139wop(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<11,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s140wop(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<12,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s141wop(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<13,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s142wop(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<14,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s143wop(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<15,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s144wop(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<16,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s145wop(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<17,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s146wop(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<18,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s147wop(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<19,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s148wop(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<20,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s149wop(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<21,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s150wop(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<22,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s151wop(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<23,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s152wop(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<24,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s153wop(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<25,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s154wop(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<26,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s155wop(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s156wop(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<28,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s157wop(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<29,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s158wop(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<30,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 159 Configured for Write Once Protection   S159WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s159wop(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpProconwoPi4_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,HpProconwoPi4_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpProconwoPi4 {
        #[inline(always)]
        fn default() -> HpProconwoPi4 {
            <crate::RegValueT<HpProconwoPi4_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpProconwoPi5_SPEC;
    impl crate::sealed::RegSpec for HpProconwoPi5_SPEC {
        type DataType = u32;
    }
    #[doc = "PFLASH Bank 0 WOP Configuration 5\n resetvalue={Application Reset:0x0,CFS Value:0x0}"]
    pub type HpProconwoPi5 = crate::RegValueT<HpProconwoPi5_SPEC>;

    impl HpProconwoPi5 {
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s160wop(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<0,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s161wop(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<1,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s162wop(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<2,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s163wop(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<3,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s164wop(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<4,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s165wop(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<5,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s166wop(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<6,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s167wop(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<7,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s168wop(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<8,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s169wop(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<9,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s170wop(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<10,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s171wop(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<11,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s172wop(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<12,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s173wop(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<13,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s174wop(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<14,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s175wop(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<15,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s176wop(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<16,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s177wop(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<17,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s178wop(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<18,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s179wop(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<19,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s180wop(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<20,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s181wop(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<21,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s182wop(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<22,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s183wop(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<23,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s184wop(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<24,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s185wop(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<25,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s186wop(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<26,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s187wop(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s188wop(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<28,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s189wop(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<29,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s190wop(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<30,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "PFLASH p Sector 191 Configured for Write Once Protection   S191WOP. These bits indicate whether PFLASH p sector x is an WOP protected sector."]
        #[inline(always)]
        pub fn s191wop(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, HpProconwoPi5_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,HpProconwoPi5_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpProconwoPi5 {
        #[inline(always)]
        fn default() -> HpProconwoPi5 {
            <crate::RegValueT<HpProconwoPi5_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
