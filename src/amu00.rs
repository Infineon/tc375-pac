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
#[doc = r"AMU"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amu00(pub(super) *mut u8);
unsafe impl core::marker::Send for Amu00 {}
unsafe impl core::marker::Sync for Amu00 {}
impl Amu00 {
    #[doc = "Control Register 0\n resetvalue={Application Reset:0x7000}"]
    #[inline(always)]
    pub const fn ctrl0(&self) -> crate::common::Reg<self::Ctrl0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }

    #[doc = "Control Register 0 Mirror\n resetvalue={Application Reset:0x7000}"]
    #[inline(always)]
    pub const fn ctrl0mir(&self) -> crate::common::Reg<self::Ctrl0Mir_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(140usize)) }
    }

    #[doc = "Control Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> crate::common::Reg<self::Ctrl1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Error Enable Register\n resetvalue={Application Reset:0x40007F}"]
    #[inline(always)]
    pub const fn eer(&self) -> crate::common::Reg<self::Eer_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }

    #[doc = "Error Status Register\n resetvalue={Application Reset:0x0F00}"]
    #[inline(always)]
    pub const fn errsr(&self) -> crate::common::Reg<self::Errsr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(156usize)) }
    }

    #[doc = "Current Coefficient Matrix L Pointer Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn lcurrpfloat(
        &self,
    ) -> crate::common::Reg<self::Lcurrpfloat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(188usize)) }
    }

    #[doc = "Coefficient Array L Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn lpfloat(&self) -> crate::common::Reg<self::Lpfloat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "Parameter p8 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn olcomp(&self) -> crate::common::Reg<self::Olcomp_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "Parameter p10 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn p10sint(&self) -> crate::common::Reg<self::P10Sint_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Current Parameter p1 Pointer Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn p1currpfloat(
        &self,
    ) -> crate::common::Reg<self::P1Currpfloat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(172usize)) }
    }

    #[doc = "Parameter p1 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn p1pfloat(&self) -> crate::common::Reg<self::P1Pfloat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }

    #[doc = "Current Parameter p2 Pointer Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn p2currpfloat(
        &self,
    ) -> crate::common::Reg<self::P2Currpfloat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(176usize)) }
    }

    #[doc = "Parameter p2 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn p2pfloat(&self) -> crate::common::Reg<self::P2Pfloat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }

    #[doc = "Current Parameter p3 Pointer Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn p3currpfloat(
        &self,
    ) -> crate::common::Reg<self::P3Currpfloat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(180usize)) }
    }

    #[doc = "Parameter p3 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn p3pfloat(&self) -> crate::common::Reg<self::P3Pfloat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }

    #[doc = "Parameter p4 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn p4float(&self) -> crate::common::Reg<self::P4Float_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "Parameter p5 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn p5float(&self) -> crate::common::Reg<self::P5Float_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }

    #[doc = "Parameter p6 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn p6sint(&self) -> crate::common::Reg<self::P6Sint_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }

    #[doc = "Parameter p7 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn p7sint(&self) -> crate::common::Reg<self::P7Sint_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "Parameter p8 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn p8sint(&self) -> crate::common::Reg<self::P8Sint_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Interrupt Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn p8sintmir(&self) -> crate::common::Reg<self::P8Sintmir_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }

    #[doc = "Parameter p9 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn p9doublehigh32(
        &self,
    ) -> crate::common::Reg<self::P9Doublehigh32_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Parameter p9 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn p9doublelow32(
        &self,
    ) -> crate::common::Reg<self::P9Doublelow32_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "Current Input Vector Pointer Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ucurrpfloat(
        &self,
    ) -> crate::common::Reg<self::Ucurrpfloat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(168usize)) }
    }

    #[doc = "Input Vector u Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn upfloat(&self) -> crate::common::Reg<self::Upfloat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "Current Coefficient Matrix V Pointer Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn vcurrpfloat(
        &self,
    ) -> crate::common::Reg<self::Vcurrpfloat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(184usize)) }
    }

    #[doc = "Coefficient Matrix V Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn vpfloat(&self) -> crate::common::Reg<self::Vpfloat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Intermediate Result yRegister\n resetvalue={Application Reset:0x7FFFFFFF}"]
    #[inline(always)]
    pub const fn ydoublehigh32(
        &self,
    ) -> crate::common::Reg<self::Ydoublehigh32_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }

    #[doc = "Intermediate Result yRegister\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn ydoublelow32(
        &self,
    ) -> crate::common::Reg<self::Ydoublelow32_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }

    #[doc = "Intermediate Result y Float Register\n resetvalue={Application Reset:0x7FFFFFFF}"]
    #[inline(always)]
    pub const fn yfloat(&self) -> crate::common::Reg<self::Yfloat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(152usize)) }
    }

    #[doc = "Return Value Register\n resetvalue={Application Reset:0x7FFFFFFF}"]
    #[inline(always)]
    pub const fn zdoublehigh32(
        &self,
    ) -> crate::common::Reg<self::Zdoublehigh32_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }

    #[doc = "Return Value Register\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn zdoublelow32(
        &self,
    ) -> crate::common::Reg<self::Zdoublelow32_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "Return Value Register\n resetvalue={Application Reset:0x7FFFFFFF}"]
    #[inline(always)]
    pub const fn zfloat(&self) -> crate::common::Reg<self::Zfloat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0_SPEC;
impl crate::sealed::RegSpec for Ctrl0_SPEC {
    type DataType = u32;
}
#[doc = "Control Register 0\n resetvalue={Application Reset:0x7000}"]
pub type Ctrl0 = crate::RegValueT<Ctrl0_SPEC>;

impl Ctrl0 {
    #[doc = "Enable Bit   EN. Write 1 to enable the algorithm. Write 0 to disable the algorithm. The flag will return a 1 if read while        the algorithm is running and 0 is the        algorithm is stopped or finished."]
    #[inline(always)]
    pub fn en(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ctrl0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ctrl0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Condition Bit   ENCN. This bit must be written with 1 to        change the EN field. If 0 is written the        value written to the EN field is ignored. Always read as 0 ."]
    #[inline(always)]
    pub fn encn(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ctrl0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ctrl0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Ready Flag   READY. This bit is cleared by hardware when a computation starts and set by hardware upon successful completion of the computation."]
    #[inline(always)]
    pub fn ready(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ctrl0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ctrl0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Slowdown Flag   SDOWN. The flags can be used to slow down the state machine which controls the algorithm."]
    #[inline(always)]
    pub fn sdown(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, Ctrl0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x3,1,0,u8, Ctrl0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slowdown Condition Bit   SDOWNCN. This bit must be written with 1 to change the SDOWN field. If 0 is written the value written to the SDOWN field is ignored. Always read as 0 ."]
    #[inline(always)]
    pub fn sdowncn(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Ctrl0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ctrl0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Limit Interrupt   LI. This flag shows the state of the limit interrupt. This interrupt is raised when the AMU outer loop counter reaches the value provided in p10. Cleared by hardware when read or on the start of the next execution   0 to 1 transition on EN ."]
    #[inline(always)]
    pub fn li(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ctrl0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ctrl0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Interrupt   ERR. This flag signifies that an error during the computation has occurred. An interrupt is raised. Cleared by hardware when read or on the start of the next execution   0 to 1 transition on EN ."]
    #[inline(always)]
    pub fn err(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ctrl0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ctrl0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Limit to Ready Interrupt   LIM2RDY. If this flag is set  a limit interrupt is also signaled at the ready interrupt line."]
    #[inline(always)]
    pub fn lim2rdy(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Ctrl0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ctrl0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Ready to Limit Interrupt   RDY2LIM. If this flag is set  the ready interrupt is also signaled the limit interrupt line."]
    #[inline(always)]
    pub fn rdy2lim(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ctrl0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ctrl0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Stop on Limit Interrupt   SL. If this flag is set  AMU stops on limit interrupt"]
    #[inline(always)]
    pub fn sl(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ctrl0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ctrl0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Allow Register Write when Active   REGACCEN. If this flag is set  writes to the P10SINT  P6SINT and OLCOMP registers when the AMU is active do not cause an error."]
    #[inline(always)]
    pub fn regaccen(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ctrl0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ctrl0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Maximum Speed Control   MAX. The field is used to define the speed at which arithmetic operations complete. It can set the speed to between 1 clock cycle  when written with 0   and 8 clock cycles  when written with 7  ."]
    #[inline(always)]
    pub fn max(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, Ctrl0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8, Ctrl0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Maximum Speed Condition Bit   MAXCN. This bit must be written with 1 to change the MAX bits. If 0 is written the value written to the MAX bits is ignored. Always read as 0 ."]
    #[inline(always)]
    pub fn maxcn(self) -> crate::common::RegisterFieldBool<15, 1, 0, Ctrl0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ctrl0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Identifier   ID. This field is freely readable and writable. Use to identify specific calculation."]
    #[inline(always)]
    pub fn id(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Ctrl0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Ctrl0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Identifier Condition Bit   IDCN. This bit must be written with 1 to change the ID field. If 0 is written the value written to the ID field is ignored. Always read as 0 ."]
    #[inline(always)]
    pub fn idcn(self) -> crate::common::RegisterFieldBool<23, 1, 0, Ctrl0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, Ctrl0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Current Shaping Enable   CSH. Write 1 to enable current shaping. See   8220 Current        Shaping  8221  on Page  160 14 for information."]
    #[inline(always)]
    pub fn csh(self) -> crate::common::RegisterFieldBool<24, 1, 0, Ctrl0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ctrl0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Current Shaping Enable Condition Bit   CSHCN. This bit must be written with 1 to change the CSH field. If 0 is written the value written to the CSH field is ignored. Always read as 0 ."]
    #[inline(always)]
    pub fn cshcn(self) -> crate::common::RegisterFieldBool<25, 1, 0, Ctrl0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ctrl0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Half Speed   HSPEED. This bit will switch the AMU core to run at a speed equivalent to SDOWN 1 when written with 1 . No effect if SDOWN is already at maximum permissible value of 11 ."]
    #[inline(always)]
    pub fn hspeed(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Ctrl0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ctrl0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Identifier Condition Bit   HSPEEDCN. This bit must be written with 1 to change the ID field. If 0 is written the value written to the HSPEED field is ignored. Always read as 0 ."]
    #[inline(always)]
    pub fn hspeedcn(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Ctrl0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, Ctrl0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Current Shaping Status   CSHS"]
    #[inline(always)]
    pub fn cshs(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, Ctrl0_SPEC, crate::common::R> {
        crate::common::RegisterField::<30,0x3,1,0,u8, Ctrl0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ctrl0 {
    #[inline(always)]
    fn default() -> Ctrl0 {
        <crate::RegValueT<Ctrl0_SPEC> as RegisterValue<_>>::new(28672)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0Mir_SPEC;
impl crate::sealed::RegSpec for Ctrl0Mir_SPEC {
    type DataType = u32;
}
#[doc = "Control Register 0 Mirror\n resetvalue={Application Reset:0x7000}"]
pub type Ctrl0Mir = crate::RegValueT<Ctrl0Mir_SPEC>;

impl Ctrl0Mir {
    #[doc = "Enable Bit   EN. The flag is set if the module is running."]
    #[inline(always)]
    pub fn en(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ctrl0Mir_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ctrl0Mir_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Ready Flag   READY. This bit is set by hardware upon successful completion of the computation."]
    #[inline(always)]
    pub fn ready(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Ctrl0Mir_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ctrl0Mir_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Slowdown Flag   SDOWN. This flag reports the speed of the state machine which controls the algorithm."]
    #[inline(always)]
    pub fn sdown(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, Ctrl0Mir_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x3,1,0,u8, Ctrl0Mir_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Limit Interrupt   LI. This flag controls the state of the limit interrupt. This interrupt is raised when the AMU outer loop counter reaches the value provided in p10."]
    #[inline(always)]
    pub fn li(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ctrl0Mir_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ctrl0Mir_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Interrupt   ERR. This flag signifies that an error during the computation has occurred. An interrupt is raised."]
    #[inline(always)]
    pub fn err(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ctrl0Mir_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ctrl0Mir_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Limit to Ready Interrupt   LIM2RDY. If this flag is set  a limit interrupt is also signaled at the ready interrupt line."]
    #[inline(always)]
    pub fn lim2rdy(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Ctrl0Mir_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ctrl0Mir_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Ready to Limit Interrupt   RDY2LIM. If this flag is set  the ready interrupt is also signaled the limit interrupt line."]
    #[inline(always)]
    pub fn rdy2lim(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Ctrl0Mir_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ctrl0Mir_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Stop on Limit Interrupt   SL. If this flag is set  AMU stops on limit interrupt"]
    #[inline(always)]
    pub fn sl(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ctrl0Mir_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ctrl0Mir_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Allow Register Write when Active   REGACCEN. If this flag is set  writes to the P10SINT  P6SINTand OLCOMP registers when the AMU is active do not cause an error."]
    #[inline(always)]
    pub fn regaccen(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Ctrl0Mir_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ctrl0Mir_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Maximum Speed Control   MAX. The field is used to report the speed at which arithmetic operations complete. The speed varies between 1 clock cycle  when 0   and 8 clock cycles  when 7  ."]
    #[inline(always)]
    pub fn max(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, Ctrl0Mir_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x7,1,0,u8, Ctrl0Mir_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Identifier   ID. Used to identify a specific calculation."]
    #[inline(always)]
    pub fn id(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Ctrl0Mir_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Ctrl0Mir_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Current Shaping Enable   CSH. Write 1 to enable current shaping. See ok for information."]
    #[inline(always)]
    pub fn csh(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Ctrl0Mir_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ctrl0Mir_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Half Speed   HSPEED. This bit will switch the AMU core to run at a speed equivalent to SDOWN 1 when written with 1 . No effect if SDOWN is already at maximum permissible value of 11 ."]
    #[inline(always)]
    pub fn hspeed(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Ctrl0Mir_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ctrl0Mir_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Current Shaping Status   CSHS"]
    #[inline(always)]
    pub fn cshs(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, Ctrl0Mir_SPEC, crate::common::R> {
        crate::common::RegisterField::<30,0x3,1,0,u8, Ctrl0Mir_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ctrl0Mir {
    #[inline(always)]
    fn default() -> Ctrl0Mir {
        <crate::RegValueT<Ctrl0Mir_SPEC> as RegisterValue<_>>::new(28672)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1_SPEC;
impl crate::sealed::RegSpec for Ctrl1_SPEC {
    type DataType = u32;
}
#[doc = "Control Register 1\n resetvalue={Application Reset:0x0}"]
pub type Ctrl1 = crate::RegValueT<Ctrl1_SPEC>;

impl Ctrl1 {
    #[doc = "Exponent for Fixed Exponent Format   EXPFIX. This field stores the exponent to be used when V is stored in memory as a 16 bit  fixed point value. See the related VFRMT field."]
    #[inline(always)]
    pub fn expfix(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Ctrl1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8, Ctrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Parameter Matrix V Format   VFRMT. This field controls the storage format of parameter matrix V. It can have the following values"]
    #[inline(always)]
    pub fn vfrmt(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Ctrl1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Ctrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Configuration Set   CFGSET. When set to 1   z is initialised to p 5 . When set to 0   z is not initialised and returns whatever value is present in the register from previous operations."]
    #[inline(always)]
    pub fn cfgset(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Ctrl1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ctrl1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ctrl1 {
    #[inline(always)]
    fn default() -> Ctrl1 {
        <crate::RegValueT<Ctrl1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eer_SPEC;
impl crate::sealed::RegSpec for Eer_SPEC {
    type DataType = u32;
}
#[doc = "Error Enable Register\n resetvalue={Application Reset:0x40007F}"]
pub type Eer = crate::RegValueT<Eer_SPEC>;

impl Eer {
    #[doc = "Enable Interrupt on P7 Over Maximum Error   P7MAX. Enable interrupt on P7MAX error."]
    #[inline(always)]
    pub fn p7max(self) -> crate::common::RegisterFieldBool<0, 1, 0, Eer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Eer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt on Input Infinity Error   IINF. Enable interrupt if input value is infinity"]
    #[inline(always)]
    pub fn iinf(self) -> crate::common::RegisterFieldBool<1, 1, 0, Eer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Eer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt on Input NaN Error   INAN. Enable interrupt if input value is not a number  NaN"]
    #[inline(always)]
    pub fn inan(self) -> crate::common::RegisterFieldBool<2, 1, 0, Eer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Eer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt on Output Overflow   OOV. Enable interrupt if output overflows."]
    #[inline(always)]
    pub fn oov(self) -> crate::common::RegisterFieldBool<3, 1, 0, Eer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Eer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt on Memory Error   ME. Enable interrupt on memory error"]
    #[inline(always)]
    pub fn me(self) -> crate::common::RegisterFieldBool<4, 1, 0, Eer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Eer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt on Register Access during Computation Error   REGACC. This flag enables interrupt generation if registers are written during computation"]
    #[inline(always)]
    pub fn regacc(self) -> crate::common::RegisterFieldBool<5, 1, 0, Eer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Eer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt on Input Denormalised Number Error   DENORM. Enable interrupt if a 32 bit input value contains a denormalised number"]
    #[inline(always)]
    pub fn denorm(self) -> crate::common::RegisterFieldBool<6, 1, 0, Eer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Eer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Stop on ECC Error   ECC STOP. Enable the use of an ECC error detected during a read from a system        memory to stop the AMU and flag an interrupt"]
    #[inline(always)]
    pub fn ecc_stop(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Eer_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Eer_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Stop on Input Denormalised Number Error.   DENEN. Stop execution if a 32 bit input value contains a denormalised number"]
    #[inline(always)]
    pub fn denen(self) -> crate::common::RegisterFieldBool<22, 1, 0, Eer_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Eer_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Stop Condition Bit   EECN. Must be written with 1 to allow the        state of bit 22 to be changed"]
    #[inline(always)]
    pub fn eecn(self) -> crate::common::RegisterFieldBool<23, 1, 0, Eer_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, Eer_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Eer {
    #[inline(always)]
    fn default() -> Eer {
        <crate::RegValueT<Eer_SPEC> as RegisterValue<_>>::new(4194431)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errsr_SPEC;
impl crate::sealed::RegSpec for Errsr_SPEC {
    type DataType = u32;
}
#[doc = "Error Status Register\n resetvalue={Application Reset:0x0F00}"]
pub type Errsr = crate::RegValueT<Errsr_SPEC>;

impl Errsr {
    #[doc = "P7 Over Maximum Error   P7MAX. The value p7 is larger than supported by hardware. The maximum supported value is 32."]
    #[inline(always)]
    pub fn p7max(self) -> crate::common::RegisterFieldBool<0, 1, 0, Errsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Errsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Infinity Error   IINF. Input value is infinity"]
    #[inline(always)]
    pub fn iinf(self) -> crate::common::RegisterFieldBool<1, 1, 0, Errsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Errsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Input NaN Error   INAN. Input value is not a number  NaN"]
    #[inline(always)]
    pub fn inan(self) -> crate::common::RegisterFieldBool<2, 1, 0, Errsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Errsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Output Overflow   OOV. An overflow occurred during computation. The output will be infinite or NaN."]
    #[inline(always)]
    pub fn oov(self) -> crate::common::RegisterFieldBool<3, 1, 0, Errsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Errsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Memory Error   ME. This error flag is generated by the DMA move engine. An error has        occurred while accessing the memory."]
    #[inline(always)]
    pub fn me(self) -> crate::common::RegisterFieldBool<4, 1, 0, Errsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Errsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Register Access during Computation Error   REGACC. This error occurs when the AMU parameter registers are being written while the computation is being performed using these parameters."]
    #[inline(always)]
    pub fn regacc(self) -> crate::common::RegisterFieldBool<5, 1, 0, Errsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Errsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Input Denormalised Number Error   DENORM. A 32 bit input value contains a denormalised number"]
    #[inline(always)]
    pub fn denorm(self) -> crate::common::RegisterFieldBool<6, 1, 0, Errsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Errsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Algorithm Position   ALGOPOS. This bit field shows the state of the AMU state machine when the error occurs. This allows inferring the error cause. Coding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn algopos(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Errsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Errsr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Uncorrectable Memory Error   MEU. A memory error has occurred which could not be corrected by the ECC logic"]
    #[inline(always)]
    pub fn meu(self) -> crate::common::RegisterFieldBool<13, 1, 0, Errsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Errsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Uncorrectable Memory Error   SRI ECC. A memory error has occurred when accessing a system memory resource"]
    #[inline(always)]
    pub fn sri_ecc(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Errsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Errsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Errsr {
    #[inline(always)]
    fn default() -> Errsr {
        <crate::RegValueT<Errsr_SPEC> as RegisterValue<_>>::new(3840)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcurrpfloat_SPEC;
impl crate::sealed::RegSpec for Lcurrpfloat_SPEC {
    type DataType = u32;
}
#[doc = "Current Coefficient Matrix L Pointer Register\n resetvalue={Application Reset:0x0}"]
pub type Lcurrpfloat = crate::RegValueT<Lcurrpfloat_SPEC>;

impl Lcurrpfloat {
    #[doc = "AMU Coefficient Register  Pointer to Float    LPFLOAT. This bit field holds a pointer to the coefficient array L M  of the algorithm."]
    #[inline(always)]
    pub fn lpfloat(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Lcurrpfloat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Lcurrpfloat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Lcurrpfloat {
    #[inline(always)]
    fn default() -> Lcurrpfloat {
        <crate::RegValueT<Lcurrpfloat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpfloat_SPEC;
impl crate::sealed::RegSpec for Lpfloat_SPEC {
    type DataType = u32;
}
#[doc = "Coefficient Array L Register\n resetvalue={Application Reset:0x0}"]
pub type Lpfloat = crate::RegValueT<Lpfloat_SPEC>;

impl Lpfloat {
    #[doc = "AMU Coefficient Array L  Pointer to Float    LPFLOAT. This bit field holds a pointer to the coefficient array L M  of the algorithm."]
    #[inline(always)]
    pub fn lpfloat(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Lpfloat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Lpfloat_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Lpfloat {
    #[inline(always)]
    fn default() -> Lpfloat {
        <crate::RegValueT<Lpfloat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Olcomp_SPEC;
impl crate::sealed::RegSpec for Olcomp_SPEC {
    type DataType = u32;
}
#[doc = "Parameter p8 Register\n resetvalue={Application Reset:0x0}"]
pub type Olcomp = crate::RegValueT<Olcomp_SPEC>;

impl Olcomp {
    #[doc = "Trigger Value for switch from half to full speed   CP8H2N. This register should be programmed with the p8 value when the AMU should switch from running at half programmed speed to full programmed speed. For instructions to use  please refer to the section about current shaping."]
    #[inline(always)]
    pub fn cp8h2n(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Olcomp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Olcomp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Value for switch from full to half speed   CPHN2H. This register should be programmed with the p8 value when the AMU should switch from running at full programmed speed to half programmed speed. For instructions to use  please refer to the section about current shaping."]
    #[inline(always)]
    pub fn cphn2h(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Olcomp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Olcomp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Olcomp {
    #[inline(always)]
    fn default() -> Olcomp {
        <crate::RegValueT<Olcomp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P10Sint_SPEC;
impl crate::sealed::RegSpec for P10Sint_SPEC {
    type DataType = u32;
}
#[doc = "Parameter p10 Register\n resetvalue={Application Reset:0x0}"]
pub type P10Sint = crate::RegValueT<P10Sint_SPEC>;

impl P10Sint {
    #[doc = "Loop Counter Value to Raise Limit Interrupt   NCROSSING. AMU will raise the limit interrupt if p8 is equal to or greater than this value."]
    #[inline(always)]
    pub fn ncrossing(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, P10Sint_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, P10Sint_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for P10Sint {
    #[inline(always)]
    fn default() -> P10Sint {
        <crate::RegValueT<P10Sint_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1Currpfloat_SPEC;
impl crate::sealed::RegSpec for P1Currpfloat_SPEC {
    type DataType = u32;
}
#[doc = "Current Parameter p1 Pointer Register\n resetvalue={Application Reset:0x0}"]
pub type P1Currpfloat = crate::RegValueT<P1Currpfloat_SPEC>;

impl P1Currpfloat {
    #[doc = "AMU Input Vector Register  Pointer to Float    P1PFLOAT. This bit field holds a pointer to the input vector array p1  M  of the algorithm."]
    #[inline(always)]
    pub fn p1pfloat(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, P1Currpfloat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, P1Currpfloat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for P1Currpfloat {
    #[inline(always)]
    fn default() -> P1Currpfloat {
        <crate::RegValueT<P1Currpfloat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P1Pfloat_SPEC;
impl crate::sealed::RegSpec for P1Pfloat_SPEC {
    type DataType = u32;
}
#[doc = "Parameter p1 Register\n resetvalue={Application Reset:0x0}"]
pub type P1Pfloat = crate::RegValueT<P1Pfloat_SPEC>;

impl P1Pfloat {
    #[doc = "AMU P1 Parameter Register  Pointer to Float    P1PFLOAT. This bit field holds a pointer to the parameter array p1 M  of the algorithm."]
    #[inline(always)]
    pub fn p1pfloat(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, P1Pfloat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, P1Pfloat_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for P1Pfloat {
    #[inline(always)]
    fn default() -> P1Pfloat {
        <crate::RegValueT<P1Pfloat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2Currpfloat_SPEC;
impl crate::sealed::RegSpec for P2Currpfloat_SPEC {
    type DataType = u32;
}
#[doc = "Current Parameter p2 Pointer Register\n resetvalue={Application Reset:0x0}"]
pub type P2Currpfloat = crate::RegValueT<P2Currpfloat_SPEC>;

impl P2Currpfloat {
    #[doc = "AMU Input Vector Register  Pointer to Float    P2PFLOAT. This bit field holds a pointer to the input vector array p2  M  of the algorithm."]
    #[inline(always)]
    pub fn p2pfloat(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, P2Currpfloat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, P2Currpfloat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for P2Currpfloat {
    #[inline(always)]
    fn default() -> P2Currpfloat {
        <crate::RegValueT<P2Currpfloat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P2Pfloat_SPEC;
impl crate::sealed::RegSpec for P2Pfloat_SPEC {
    type DataType = u32;
}
#[doc = "Parameter p2 Register\n resetvalue={Application Reset:0x0}"]
pub type P2Pfloat = crate::RegValueT<P2Pfloat_SPEC>;

impl P2Pfloat {
    #[doc = "AMU P2 Parameter Register  Pointer to Float    P2PFLOAT. This bit field holds a pointer to the parameter array p2  M  of the algorithm."]
    #[inline(always)]
    pub fn p2pfloat(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, P2Pfloat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, P2Pfloat_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for P2Pfloat {
    #[inline(always)]
    fn default() -> P2Pfloat {
        <crate::RegValueT<P2Pfloat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3Currpfloat_SPEC;
impl crate::sealed::RegSpec for P3Currpfloat_SPEC {
    type DataType = u32;
}
#[doc = "Current Parameter p3 Pointer Register\n resetvalue={Application Reset:0x0}"]
pub type P3Currpfloat = crate::RegValueT<P3Currpfloat_SPEC>;

impl P3Currpfloat {
    #[doc = "AMU Input Vector Register  Pointer to Float    P3PFLOAT. This bit field holds a pointer to the input vector array p3  M  of the algorithm."]
    #[inline(always)]
    pub fn p3pfloat(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, P3Currpfloat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, P3Currpfloat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for P3Currpfloat {
    #[inline(always)]
    fn default() -> P3Currpfloat {
        <crate::RegValueT<P3Currpfloat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P3Pfloat_SPEC;
impl crate::sealed::RegSpec for P3Pfloat_SPEC {
    type DataType = u32;
}
#[doc = "Parameter p3 Register\n resetvalue={Application Reset:0x0}"]
pub type P3Pfloat = crate::RegValueT<P3Pfloat_SPEC>;

impl P3Pfloat {
    #[doc = "AMU P3 Parameter Register  Pointer to Float    P3PFLOAT. This bit field holds a pointer to the parameter array p3  N  of the algorithm."]
    #[inline(always)]
    pub fn p3pfloat(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, P3Pfloat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, P3Pfloat_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for P3Pfloat {
    #[inline(always)]
    fn default() -> P3Pfloat {
        <crate::RegValueT<P3Pfloat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P4Float_SPEC;
impl crate::sealed::RegSpec for P4Float_SPEC {
    type DataType = u32;
}
#[doc = "Parameter p4 Register\n resetvalue={Application Reset:0x0}"]
pub type P4Float = crate::RegValueT<P4Float_SPEC>;

impl P4Float {
    #[doc = "AMU P4 Parameter Register  Float    P4FLOAT. This bit field holds the parameter p4 of the algorithm."]
    #[inline(always)]
    pub fn p4float(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, P4Float_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, P4Float_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for P4Float {
    #[inline(always)]
    fn default() -> P4Float {
        <crate::RegValueT<P4Float_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P5Float_SPEC;
impl crate::sealed::RegSpec for P5Float_SPEC {
    type DataType = u32;
}
#[doc = "Parameter p5 Register\n resetvalue={Application Reset:0x0}"]
pub type P5Float = crate::RegValueT<P5Float_SPEC>;

impl P5Float {
    #[doc = "AMU P5 Parameter Register  Float    P5FLOAT. This bit field holds the parameter p5 of the algorithm."]
    #[inline(always)]
    pub fn p5float(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, P5Float_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, P5Float_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for P5Float {
    #[inline(always)]
    fn default() -> P5Float {
        <crate::RegValueT<P5Float_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P6Sint_SPEC;
impl crate::sealed::RegSpec for P6Sint_SPEC {
    type DataType = u32;
}
#[doc = "Parameter p6 Register\n resetvalue={Application Reset:0x0}"]
pub type P6Sint = crate::RegValueT<P6Sint_SPEC>;

impl P6Sint {
    #[doc = "Max Iteration Counter for Outer Loop   NEND. This register contains the row dimension of matrix V. It is the maximum number of iterations of the outer loop of the algorithm."]
    #[inline(always)]
    pub fn nend(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, P6Sint_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, P6Sint_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for P6Sint {
    #[inline(always)]
    fn default() -> P6Sint {
        <crate::RegValueT<P6Sint_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P7Sint_SPEC;
impl crate::sealed::RegSpec for P7Sint_SPEC {
    type DataType = u32;
}
#[doc = "Parameter p7 Register\n resetvalue={Application Reset:0x0}"]
pub type P7Sint = crate::RegValueT<P7Sint_SPEC>;

impl P7Sint {
    #[doc = "Max Iteration Counter for Inner Loop   M. This register contains the column dimension of matrix V. It is the maximum number of iterations of the inner loop of the algorithm. Due to hardware constraints  this value cannot be larger than 32."]
    #[inline(always)]
    pub fn m(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, P7Sint_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, P7Sint_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for P7Sint {
    #[inline(always)]
    fn default() -> P7Sint {
        <crate::RegValueT<P7Sint_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P8Sint_SPEC;
impl crate::sealed::RegSpec for P8Sint_SPEC {
    type DataType = u32;
}
#[doc = "Parameter p8 Register\n resetvalue={Application Reset:0x0}"]
pub type P8Sint = crate::RegValueT<P8Sint_SPEC>;

impl P8Sint {
    #[doc = "Start Value for Outer Loop   NSTART. This register should be programmed with the start value for the outer loop of the algorithm. For instructions to use  please refer to the section about restarting AMU after an interrupted calculation."]
    #[inline(always)]
    pub fn nstart(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, P8Sint_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, P8Sint_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for P8Sint {
    #[inline(always)]
    fn default() -> P8Sint {
        <crate::RegValueT<P8Sint_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P8Sintmir_SPEC;
impl crate::sealed::RegSpec for P8Sintmir_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Status Register\n resetvalue={Application Reset:0x0}"]
pub type P8Sintmir = crate::RegValueT<P8Sintmir_SPEC>;

impl P8Sintmir {
    #[doc = "Start Value for Outer Loop   NSTART. Mirror of P8SINT field NSTART."]
    #[inline(always)]
    pub fn nstart(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, P8Sintmir_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, P8Sintmir_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for P8Sintmir {
    #[inline(always)]
    fn default() -> P8Sintmir {
        <crate::RegValueT<P8Sintmir_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P9Doublehigh32_SPEC;
impl crate::sealed::RegSpec for P9Doublehigh32_SPEC {
    type DataType = u32;
}
#[doc = "Parameter p9 Register\n resetvalue={Application Reset:0x0}"]
pub type P9Doublehigh32 = crate::RegValueT<P9Doublehigh32_SPEC>;

impl P9Doublehigh32 {
    #[doc = "AMU Parameter p9 Register  Double    P9DOUBLE. This bit field holds the higher 32 bit of parameter p9 of the algorithm."]
    #[inline(always)]
    pub fn p9double(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        P9Doublehigh32_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            P9Doublehigh32_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for P9Doublehigh32 {
    #[inline(always)]
    fn default() -> P9Doublehigh32 {
        <crate::RegValueT<P9Doublehigh32_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P9Doublelow32_SPEC;
impl crate::sealed::RegSpec for P9Doublelow32_SPEC {
    type DataType = u32;
}
#[doc = "Parameter p9 Register\n resetvalue={Application Reset:0x0}"]
pub type P9Doublelow32 = crate::RegValueT<P9Doublelow32_SPEC>;

impl P9Doublelow32 {
    #[doc = "AMU Parameter p9 Register  Double    P9DOUBLE. This bit field holds the lower 32 bit of parameter p9 of the algorithm."]
    #[inline(always)]
    pub fn p9double(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, P9Doublelow32_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            P9Doublelow32_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for P9Doublelow32 {
    #[inline(always)]
    fn default() -> P9Doublelow32 {
        <crate::RegValueT<P9Doublelow32_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ucurrpfloat_SPEC;
impl crate::sealed::RegSpec for Ucurrpfloat_SPEC {
    type DataType = u32;
}
#[doc = "Current Input Vector Pointer Register\n resetvalue={Application Reset:0x0}"]
pub type Ucurrpfloat = crate::RegValueT<Ucurrpfloat_SPEC>;

impl Ucurrpfloat {
    #[doc = "AMU Input Vector Register  Pointer to Float    UPFLOAT. This bit field holds a pointer to the input vector array u  M  of the algorithm."]
    #[inline(always)]
    pub fn upfloat(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Ucurrpfloat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Ucurrpfloat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ucurrpfloat {
    #[inline(always)]
    fn default() -> Ucurrpfloat {
        <crate::RegValueT<Ucurrpfloat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Upfloat_SPEC;
impl crate::sealed::RegSpec for Upfloat_SPEC {
    type DataType = u32;
}
#[doc = "Input Vector u Register\n resetvalue={Application Reset:0x0}"]
pub type Upfloat = crate::RegValueT<Upfloat_SPEC>;

impl Upfloat {
    #[doc = "AMU Input Vector Register  Pointer to Float    UPFLOAT. This bit field holds a pointer to the input vector array u  M  of the algorithm."]
    #[inline(always)]
    pub fn upfloat(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Upfloat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Upfloat_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Upfloat {
    #[inline(always)]
    fn default() -> Upfloat {
        <crate::RegValueT<Upfloat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vcurrpfloat_SPEC;
impl crate::sealed::RegSpec for Vcurrpfloat_SPEC {
    type DataType = u32;
}
#[doc = "Current Coefficient Matrix V Pointer Register\n resetvalue={Application Reset:0x0}"]
pub type Vcurrpfloat = crate::RegValueT<Vcurrpfloat_SPEC>;

impl Vcurrpfloat {
    #[doc = "AMU V Coefficient Register  Pointer to Float    VPFLOAT. This bit field holds a pointer to the coefficient vector array V  N M  of the algorithm."]
    #[inline(always)]
    pub fn vpfloat(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Vcurrpfloat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Vcurrpfloat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Vcurrpfloat {
    #[inline(always)]
    fn default() -> Vcurrpfloat {
        <crate::RegValueT<Vcurrpfloat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vpfloat_SPEC;
impl crate::sealed::RegSpec for Vpfloat_SPEC {
    type DataType = u32;
}
#[doc = "Coefficient Matrix V Register\n resetvalue={Application Reset:0x0}"]
pub type Vpfloat = crate::RegValueT<Vpfloat_SPEC>;

impl Vpfloat {
    #[doc = "AMU V Parameter Register  Pointer to Float    VPFLOAT. This bit field holds the base address of the coefficient matrix V NxM"]
    #[inline(always)]
    pub fn vpfloat(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Vpfloat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Vpfloat_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Vpfloat {
    #[inline(always)]
    fn default() -> Vpfloat {
        <crate::RegValueT<Vpfloat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ydoublehigh32_SPEC;
impl crate::sealed::RegSpec for Ydoublehigh32_SPEC {
    type DataType = u32;
}
#[doc = "Intermediate Result yRegister\n resetvalue={Application Reset:0x7FFFFFFF}"]
pub type Ydoublehigh32 = crate::RegValueT<Ydoublehigh32_SPEC>;

impl Ydoublehigh32 {
    #[doc = "AMU v Result Register  Double    YDOUBLE. This bit field holds the higher 32 bit of intermediate value yof the algorithm."]
    #[inline(always)]
    pub fn ydouble(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Ydoublehigh32_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Ydoublehigh32_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ydoublehigh32 {
    #[inline(always)]
    fn default() -> Ydoublehigh32 {
        <crate::RegValueT<Ydoublehigh32_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ydoublelow32_SPEC;
impl crate::sealed::RegSpec for Ydoublelow32_SPEC {
    type DataType = u32;
}
#[doc = "Intermediate Result yRegister\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Ydoublelow32 = crate::RegValueT<Ydoublelow32_SPEC>;

impl Ydoublelow32 {
    #[doc = "AMU v Result Register  Double    YDOUBLE. This bit field holds the lower 32 bit of intermediate value y of the algorithm."]
    #[inline(always)]
    pub fn ydouble(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Ydoublelow32_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Ydoublelow32_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ydoublelow32 {
    #[inline(always)]
    fn default() -> Ydoublelow32 {
        <crate::RegValueT<Ydoublelow32_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Yfloat_SPEC;
impl crate::sealed::RegSpec for Yfloat_SPEC {
    type DataType = u32;
}
#[doc = "Intermediate Result y Float Register\n resetvalue={Application Reset:0x7FFFFFFF}"]
pub type Yfloat = crate::RegValueT<Yfloat_SPEC>;

impl Yfloat {
    #[doc = "AMU v Result Register  Float    YFLOAT. This bit field holds the intermediate value v of the algorithm."]
    #[inline(always)]
    pub fn yfloat(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Yfloat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Yfloat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Yfloat {
    #[inline(always)]
    fn default() -> Yfloat {
        <crate::RegValueT<Yfloat_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Zdoublehigh32_SPEC;
impl crate::sealed::RegSpec for Zdoublehigh32_SPEC {
    type DataType = u32;
}
#[doc = "Return Value Register\n resetvalue={Application Reset:0x7FFFFFFF}"]
pub type Zdoublehigh32 = crate::RegValueT<Zdoublehigh32_SPEC>;

impl Zdoublehigh32 {
    #[doc = "AMU return value z Register  Double    ZDOUBLE. This bit field holds the higher 32 bit of return value  z of the algorithm."]
    #[inline(always)]
    pub fn zdouble(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Zdoublehigh32_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Zdoublehigh32_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Zdoublehigh32 {
    #[inline(always)]
    fn default() -> Zdoublehigh32 {
        <crate::RegValueT<Zdoublehigh32_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Zdoublelow32_SPEC;
impl crate::sealed::RegSpec for Zdoublelow32_SPEC {
    type DataType = u32;
}
#[doc = "Return Value Register\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Zdoublelow32 = crate::RegValueT<Zdoublelow32_SPEC>;

impl Zdoublelow32 {
    #[doc = "AMU Return value z Register  Double    ZDOUBLE. This bit field holds the lower 32 bit of return value z of the algorithm."]
    #[inline(always)]
    pub fn zdouble(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Zdoublelow32_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Zdoublelow32_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Zdoublelow32 {
    #[inline(always)]
    fn default() -> Zdoublelow32 {
        <crate::RegValueT<Zdoublelow32_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Zfloat_SPEC;
impl crate::sealed::RegSpec for Zfloat_SPEC {
    type DataType = u32;
}
#[doc = "Return Value Register\n resetvalue={Application Reset:0x7FFFFFFF}"]
pub type Zfloat = crate::RegValueT<Zfloat_SPEC>;

impl Zfloat {
    #[doc = "AMU Result Register  Float    ZFLOAT. This bit field holds the return value   z   of the algorithm."]
    #[inline(always)]
    pub fn zfloat(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Zfloat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Zfloat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Zfloat {
    #[inline(always)]
    fn default() -> Zfloat {
        <crate::RegValueT<Zfloat_SPEC> as RegisterValue<_>>::new(2147483647)
    }
}
