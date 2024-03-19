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
#[doc = r"FPI Bus Control Unit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbcu(pub(super) *mut u8);
unsafe impl core::marker::Send for Sbcu {}
unsafe impl core::marker::Sync for Sbcu {}
impl Sbcu {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "BCU EDC Alarm Clear Register 0\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn alclrx(&self) -> [crate::common::Reg<self::AlclRx_SPEC, crate::common::W>; 4] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x70usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x70usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x70usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x70usize + 0xcusize)),
            ]
        }
    }

    #[doc = "BCU EDC Alarm Control Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn alctrl(&self) -> crate::common::Reg<self::Alctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "BCU EDC Alarm Status Register 0\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn alstatx(&self) -> [crate::common::Reg<self::AlstaTx_SPEC, crate::common::R>; 4] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0xcusize)),
            ]
        }
    }

    #[doc = "BCU Control Register\n resetvalue={Application Reset:0x0FF09FFFF}"]
    #[inline(always)]
    pub const fn con(&self) -> crate::common::Reg<self::Con_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "BCU Debug Address 1 Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn dbadr1(&self) -> crate::common::Reg<self::Dbadr1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }

    #[doc = "BCU Debug Address 2 Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn dbadr2(&self) -> crate::common::Reg<self::Dbadr2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "BCU Debug Trapped Address Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn dbadrt(&self) -> crate::common::Reg<self::Dbadrt_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }

    #[doc = "BCU Debug Bus Operation Signals Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn dbbos(&self) -> crate::common::Reg<self::Dbbos_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "BCU Debug Trapped Bus Operation Signals Register\n resetvalue={Debug Reset:0x3180}"]
    #[inline(always)]
    pub const fn dbbost(&self) -> crate::common::Reg<self::Dbbost_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(76usize)) }
    }

    #[doc = "BCU Debug Control Register\n resetvalue={Debug Reset:0x7003}"]
    #[inline(always)]
    pub const fn dbcntl(&self) -> crate::common::Reg<self::Dbcntl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }

    #[doc = "BCU Debug Data Status Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn dbdat(&self) -> crate::common::Reg<self::Dbdat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }

    #[doc = "SBCU Debug Trapped Master Register\n resetvalue={Debug Reset:0x0FFFF}"]
    #[inline(always)]
    pub const fn dbgntt(&self) -> crate::common::Reg<self::Dbgntt_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }

    #[doc = "SBCU Debug Grant Mask Register\n resetvalue={Debug Reset:0x0FFFF}"]
    #[inline(always)]
    pub const fn dbgrnt(&self) -> crate::common::Reg<self::Dbgrnt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "BCU Error Address Capture Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn eadd(&self) -> crate::common::Reg<self::Eadd_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "BCU Error Control Capture Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn econ(&self) -> crate::common::Reg<self::Econ_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "BCU Error Data Capture Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn edat(&self) -> crate::common::Reg<self::Edat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }

    #[doc = "FPI Error Generation Control Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn fegen(&self) -> crate::common::Reg<self::Fegen_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x6A00}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Arbiter Priority Register High\n resetvalue={Application Reset:0x0FEDCBA98,Application Reset:0x0FEDC8888,Application Reset:0x0FEDCBA98}"]
    #[inline(always)]
    pub const fn prioh(&self) -> crate::common::Reg<self::Prioh_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Arbiter Priority Register Low\n resetvalue={Application Reset:0x76543210,Application Reset:0x088543210,Application Reset:0x76588210}"]
    #[inline(always)]
    pub const fn priol(&self) -> crate::common::Reg<self::Priol_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
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
pub struct AlclRx_SPEC;
impl crate::sealed::RegSpec for AlclRx_SPEC {
    type DataType = u32;
}
#[doc = "BCU EDC Alarm Clear Register 0\n resetvalue={Debug Reset:0x0}"]
pub type AlclRx = crate::RegValueT<AlclRx_SPEC>;

impl AlclRx {
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr00(self) -> crate::common::RegisterFieldBool<0, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr01(self) -> crate::common::RegisterFieldBool<1, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr02(self) -> crate::common::RegisterFieldBool<2, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr03(self) -> crate::common::RegisterFieldBool<3, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr04(self) -> crate::common::RegisterFieldBool<4, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr05(self) -> crate::common::RegisterFieldBool<5, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr06(self) -> crate::common::RegisterFieldBool<6, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr07(self) -> crate::common::RegisterFieldBool<7, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr08(self) -> crate::common::RegisterFieldBool<8, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr09(self) -> crate::common::RegisterFieldBool<9, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, AlclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, AlclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for AlclRx {
    #[inline(always)]
    fn default() -> AlclRx {
        <crate::RegValueT<AlclRx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Alctrl_SPEC;
impl crate::sealed::RegSpec for Alctrl_SPEC {
    type DataType = u32;
}
#[doc = "BCU EDC Alarm Control Register\n resetvalue={Debug Reset:0x0}"]
pub type Alctrl = crate::RegValueT<Alctrl_SPEC>;

impl Alctrl {
    #[doc = "Alarm Overflow. The ALOV bit is set if multiple FPI EDC alarms for the same FPI slave or        the same FPI master were detected while the related ALSTATx y  bit was        still set. Some errors result in a static fault situation  for example address          phase  data phase or data enable signal faults. Static faults do not          generate multiple alarms and will will not set the ALOV bit."]
    #[inline(always)]
    pub fn alov(self) -> crate::common::RegisterFieldBool<0, 1, 0, Alctrl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Alctrl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm Overflow Clear. The ALOVCLR bit is required to reset the ALOV bit."]
    #[inline(always)]
    pub fn alovclr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Alctrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Alctrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Alctrl {
    #[inline(always)]
    fn default() -> Alctrl {
        <crate::RegValueT<Alctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlstaTx_SPEC;
impl crate::sealed::RegSpec for AlstaTx_SPEC {
    type DataType = u32;
}
#[doc = "BCU EDC Alarm Status Register 0\n resetvalue={Debug Reset:0x0}"]
pub type AlstaTx = crate::RegValueT<AlstaTx_SPEC>;

impl AlstaTx {
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al0(self) -> crate::common::RegisterFieldBool<0, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al1(self) -> crate::common::RegisterFieldBool<1, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al2(self) -> crate::common::RegisterFieldBool<2, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al3(self) -> crate::common::RegisterFieldBool<3, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al4(self) -> crate::common::RegisterFieldBool<4, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al5(self) -> crate::common::RegisterFieldBool<5, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al6(self) -> crate::common::RegisterFieldBool<6, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al7(self) -> crate::common::RegisterFieldBool<7, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al8(self) -> crate::common::RegisterFieldBool<8, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al9(self) -> crate::common::RegisterFieldBool<9, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for AlstaTx {
    #[inline(always)]
    fn default() -> AlstaTx {
        <crate::RegValueT<AlstaTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Con_SPEC;
impl crate::sealed::RegSpec for Con_SPEC {
    type DataType = u32;
}
#[doc = "BCU Control Register\n resetvalue={Application Reset:0x0FF09FFFF}"]
pub type Con = crate::RegValueT<Con_SPEC>;

impl Con {
    #[doc = "BCU Bus Time Out Value. The bit field determines the number of System Peripheral Bus time out        cycles. Default after reset is FFFF H    65536 bus cycles . TOUT value must be  gt   5."]
    #[inline(always)]
    pub fn tout(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "BCU Debug Trace Enable. The bit enables disables the error capture mechanism for the registers        BCU ECON  BCU EADD  BCU EDAT. The bit does not affect the SMU alarm or the BCU interrupt that are send        on detection case of an error condition."]
    #[inline(always)]
    pub fn dbg(self) -> crate::common::RegisterFieldBool<16, 1, 0, Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Starvation Period Control. Determines the sample period for the starvation counter. Must be larger        than the number of masters. The reset value is FF H ."]
    #[inline(always)]
    pub fn spc(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Con_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Con {
    #[inline(always)]
    fn default() -> Con {
        <crate::RegValueT<Con_SPEC> as RegisterValue<_>>::new(4278845439)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbadr1_SPEC;
impl crate::sealed::RegSpec for Dbadr1_SPEC {
    type DataType = u32;
}
#[doc = "BCU Debug Address 1 Register\n resetvalue={Debug Reset:0x0}"]
pub type Dbadr1 = crate::RegValueT<Dbadr1_SPEC>;

impl Dbadr1 {
    #[doc = "Debug Trigger Address 1. This register contains the address for the address 1 trigger event        generation."]
    #[inline(always)]
    pub fn adr1(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Dbadr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Dbadr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dbadr1 {
    #[inline(always)]
    fn default() -> Dbadr1 {
        <crate::RegValueT<Dbadr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbadr2_SPEC;
impl crate::sealed::RegSpec for Dbadr2_SPEC {
    type DataType = u32;
}
#[doc = "BCU Debug Address 2 Register\n resetvalue={Debug Reset:0x0}"]
pub type Dbadr2 = crate::RegValueT<Dbadr2_SPEC>;

impl Dbadr2 {
    #[doc = "Debug Trigger Address 2. This register contains the address for the address 2 trigger event        generation."]
    #[inline(always)]
    pub fn adr2(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Dbadr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Dbadr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dbadr2 {
    #[inline(always)]
    fn default() -> Dbadr2 {
        <crate::RegValueT<Dbadr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbadrt_SPEC;
impl crate::sealed::RegSpec for Dbadrt_SPEC {
    type DataType = u32;
}
#[doc = "BCU Debug Trapped Address Register\n resetvalue={Debug Reset:0x0}"]
pub type Dbadrt = crate::RegValueT<Dbadrt_SPEC>;

impl Dbadrt {
    #[doc = "FPI Bus Address Status. This register contains the FPI  160 Bus address that was captured when the        OCDS break trigger event occurred."]
    #[inline(always)]
    pub fn fpiadr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Dbadrt_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Dbadrt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Dbadrt {
    #[inline(always)]
    fn default() -> Dbadrt {
        <crate::RegValueT<Dbadrt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbbos_SPEC;
impl crate::sealed::RegSpec for Dbbos_SPEC {
    type DataType = u32;
}
#[doc = "BCU Debug Bus Operation Signals Register\n resetvalue={Debug Reset:0x0}"]
pub type Dbbos = crate::RegValueT<Dbbos_SPEC>;

impl Dbbos {
    #[doc = "Opcode for Signal Status Debug Trigger. This bit field determines the type  opcode  of an FPI  160 Bus transaction        for which a signal status debug trigger event is generated  if enabled        by DBCNTL.ONBOS0  160    160 1 . Other bit combinations are reserved."]
    #[inline(always)]
    pub fn opc(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Dbbos_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Dbbos_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SVM Signal for Status Debug Trigger. This bit determines the mode of an FPI  160 Bus transaction for which a        signal status debug trigger event is generated  if enabled by        DBCNTL.ONBOS1  160    160 1 ."]
    #[inline(always)]
    pub fn svm(self) -> crate::common::RegisterFieldBool<4, 1, 0, Dbbos_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Dbbos_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Write Signal for Status Debug Trigger. This bit determines the state of the WR signal of an FPI  160 Bus transaction for which a signal status debug trigger        event is generated  if enabled by DBCNTL.ONBOS2  160    160 1 ."]
    #[inline(always)]
    pub fn wr(self) -> crate::common::RegisterFieldBool<8, 1, 0, Dbbos_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Dbbos_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Write Signal for Status Debug Trigger. This bit determines the state of the RD signal of an FPI  160 Bus transaction for which a signal status debug trigger        event is generated  if enabled by DBCNTL.ONBOS3  160    160 1 ."]
    #[inline(always)]
    pub fn rd(self) -> crate::common::RegisterFieldBool<12, 1, 0, Dbbos_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Dbbos_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Dbbos {
    #[inline(always)]
    fn default() -> Dbbos {
        <crate::RegValueT<Dbbos_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbbost_SPEC;
impl crate::sealed::RegSpec for Dbbost_SPEC {
    type DataType = u32;
}
#[doc = "BCU Debug Trapped Bus Operation Signals Register\n resetvalue={Debug Reset:0x3180}"]
pub type Dbbost = crate::RegValueT<Dbbost_SPEC>;

impl Dbbost {
    #[doc = "FPI Bus Opcode Status. This bit field indicates the type  opcode  of the FPI  160 Bus transaction        captured from the FPI  160 Bus signal lines when the BCU break trigger event        occurred. Other bit combinations are reserved."]
    #[inline(always)]
    pub fn fpiopc(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Dbbost_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Dbbost_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "FPI Bus Supervisor Mode Status. This bit indicates the state of the Supervisor Mode signal captured from        the FPI  160 Bus signal lines when the BCU break trigger event occurred."]
    #[inline(always)]
    pub fn fpisvm(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Dbbost_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Dbbost_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "FPI Bus Acknowledge Status. This bit field indicates the acknowledge signal status captured from the        FPI  160 Bus signal lines when the BCU break trigger event occurred. Coding        see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn fpiack(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, Dbbost_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x3,1,0,u8, Dbbost_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "FPI Bus Ready Status. This bit indicates the ready signal status captured from the FPI  160 Bus        signal lines when the BCU break trigger event occurred."]
    #[inline(always)]
    pub fn fpirdy(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dbbost_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Dbbost_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "FPI Bus Write Indication Status. This bit indicates the write signal status captured from the FPI  160 Bus        signal lines when the BCU break trigger event occurred."]
    #[inline(always)]
    pub fn fpiwr(self) -> crate::common::RegisterFieldBool<8, 1, 0, Dbbost_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Dbbost_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "FPI Bus Reset Status. This bit field indicates the reset signal status captured from the        FPI  160 Bus signal lines when the BCU break trigger event occurred. Others  160   160   160   160   160 Reserved"]
    #[inline(always)]
    pub fn fpirst(
        self,
    ) -> crate::common::RegisterField<9, 0x3, 1, 0, u8, Dbbost_SPEC, crate::common::R> {
        crate::common::RegisterField::<9,0x3,1,0,u8, Dbbost_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "FPI Bus OCDS Suspend Status. This bit indicates the OCDS suspend signal status captured from the        FPI  160 Bus signal lines when the BCU break trigger event occurred."]
    #[inline(always)]
    pub fn fpiops(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dbbost_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Dbbost_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "FPI Bus Read Indication Status. This bit indicates the read signal status captured from the FPI  160 Bus        signal lines when the BCU break trigger event occurred."]
    #[inline(always)]
    pub fn fpird(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dbbost_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Dbbost_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "FPI Bus Abort Status. This bit indicates the abort signal status captured from the FPI  160 Bus        signal lines when the BCU break trigger event occurred."]
    #[inline(always)]
    pub fn fpiabort(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Dbbost_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Dbbost_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "FPI Bus Time out Status. This bit indicates the time out signal status captured from the FPI  160 Bus        signal lines when the BCU break trigger event occurred."]
    #[inline(always)]
    pub fn fpitout(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Dbbost_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Dbbost_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "FPI Bus Endinit Status. This bit indicates the ENDINIT signal status captured from the FPI  160 Bus        signal lines when the BCU break trigger event occurred."]
    #[inline(always)]
    pub fn endinit(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Dbbost_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Dbbost_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "FPI Bus Master TAG Status. This bit field indicates the master TAG captured from the FPI  160 Bus signal        lines when the BCU break trigger event occurred  see CROSSREFERENCE  .        The master TAG identifies the master of the transfer which generated BCU        break trigger event."]
    #[inline(always)]
    pub fn fpitag(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Dbbost_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Dbbost_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Dbbost {
    #[inline(always)]
    fn default() -> Dbbost {
        <crate::RegValueT<Dbbost_SPEC> as RegisterValue<_>>::new(12672)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbcntl_SPEC;
impl crate::sealed::RegSpec for Dbcntl_SPEC {
    type DataType = u32;
}
#[doc = "BCU Debug Control Register\n resetvalue={Debug Reset:0x7003}"]
pub type Dbcntl = crate::RegValueT<Dbcntl_SPEC>;

impl Dbcntl {
    #[doc = "Status of BCU Debug Support Enable. This bit is controlled by the Cerberus and enables the BCU debug support."]
    #[inline(always)]
    pub fn eo(self) -> crate::common::RegisterFieldBool<0, 1, 0, Dbcntl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dbcntl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Status of BCU Breakpoint Logic. The OA bit is set by writing a 1 to bit RA. When OA is set  registers        DBGNTT  DBADRT and DBDAT are reset. Also DBBOST is reset with the        exception of the bit field FPIRST."]
    #[inline(always)]
    pub fn oa(self) -> crate::common::RegisterFieldBool<1, 1, 0, Dbcntl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Dbcntl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Rearm BCU Breakpoint Logic. Writing a 1 to this bit rearms BCU breakpoint logic and sets bit OA  160    160 1.        RA is always reads as 0."]
    #[inline(always)]
    pub fn ra(self) -> crate::common::RegisterFieldBool<4, 1, 0, Dbcntl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Dbcntl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Status of HSM Transaction Trace Logic"]
    #[inline(always)]
    pub fn hsmtrtren(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, Dbcntl_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x3,1,0,u8, Dbcntl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Status of HSM Debug Mode"]
    #[inline(always)]
    pub fn hsmdbgen(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Dbcntl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Dbcntl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Grant and Address Trigger Relation"]
    #[inline(always)]
    pub fn concom0(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dbcntl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Dbcntl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Address 1 and Address 2 Trigger Relation"]
    #[inline(always)]
    pub fn concom1(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Dbcntl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Dbcntl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Address and Signal Trigger Relation"]
    #[inline(always)]
    pub fn concom2(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Dbcntl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Dbcntl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Grant Trigger Enable"]
    #[inline(always)]
    pub fn ong(self) -> crate::common::RegisterFieldBool<16, 1, 0, Dbcntl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Dbcntl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Address 1 Trigger Control. See also CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ona1(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Dbcntl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Dbcntl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Address 2 Trigger Control. See also CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ona2(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, Dbcntl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, Dbcntl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Op code Signal Status Trigger Condition"]
    #[inline(always)]
    pub fn onbos0(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Dbcntl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Dbcntl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Supervisor Mode Signal Trigger Condition"]
    #[inline(always)]
    pub fn onbos1(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Dbcntl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Dbcntl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Write Signal Trigger Condition"]
    #[inline(always)]
    pub fn onbos2(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Dbcntl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Dbcntl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Read Signal Trigger Condition"]
    #[inline(always)]
    pub fn onbos3(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Dbcntl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Dbcntl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Dbcntl {
    #[inline(always)]
    fn default() -> Dbcntl {
        <crate::RegValueT<Dbcntl_SPEC> as RegisterValue<_>>::new(28675)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbdat_SPEC;
impl crate::sealed::RegSpec for Dbdat_SPEC {
    type DataType = u32;
}
#[doc = "BCU Debug Data Status Register\n resetvalue={Debug Reset:0x0}"]
pub type Dbdat = crate::RegValueT<Dbdat_SPEC>;

impl Dbdat {
    #[doc = "FPI Bus Data Status. This register contains the FPI  160 Bus data that was captured when the OCDS        break trigger event occurred."]
    #[inline(always)]
    pub fn fpidata(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Dbdat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Dbdat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Dbdat {
    #[inline(always)]
    fn default() -> Dbdat {
        <crate::RegValueT<Dbdat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgntt_SPEC;
impl crate::sealed::RegSpec for Dbgntt_SPEC {
    type DataType = u32;
}
#[doc = "SBCU Debug Trapped Master Register\n resetvalue={Debug Reset:0x0FFFF}"]
pub type Dbgntt = crate::RegValueT<Dbgntt_SPEC>;

impl Dbgntt {
    #[doc = "DMA   Cerberus FPI Bus Master Status. This bit indicates whether the DMA or Cerberus was FPI  160 Bus master when        the break trigger event occurred."]
    #[inline(always)]
    pub fn dma(self) -> crate::common::RegisterFieldBool<0, 1, 0, Dbgntt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dbgntt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "HSSL 0 FPI Bus Master Status. This bit indicates whether the HSSL 0 was FPI  160 Bus master when the break        trigger event occurred."]
    #[inline(always)]
    pub fn hssl0(self) -> crate::common::RegisterFieldBool<3, 1, 0, Dbgntt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Dbgntt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU0 FPI Bus Master Status. This bit indicates whether the CPU0 was FPI  160 Bus master when the break        trigger event occurred. Position   180 6  180  of the Req Lock Grant BCU input vector set is used in the          EDC implementation for Req Lock error injection. Therefore this          position shall be used on the SPB for CPU  on BBB for the Bridge           SRI2BBB ."]
    #[inline(always)]
    pub fn cpu0(self) -> crate::common::RegisterFieldBool<6, 1, 0, Dbgntt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Dbgntt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 FPI Bus Master Status. This bit indicates whether the CPU1 was FPI  160 Bus master when the break        trigger event occurred."]
    #[inline(always)]
    pub fn cpu1(self) -> crate::common::RegisterFieldBool<7, 1, 0, Dbgntt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Dbgntt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 Grant Trigger Enable"]
    #[inline(always)]
    pub fn cpu2(self) -> crate::common::RegisterFieldBool<8, 1, 0, Dbgntt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Dbgntt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "HSM Register FPI Bus Master Interface Status. This bit indicates whether the HSM was FPI  160 Bus master when the break        trigger event occurred."]
    #[inline(always)]
    pub fn hsmrmi(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dbgntt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Dbgntt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "HSM Cache FPI Bus Master Interface Status. This bit indicates whether the HSM was FPI  160 Bus master when the break        trigger event occurred."]
    #[inline(always)]
    pub fn hsmcmi(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Dbgntt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Dbgntt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Dbgntt {
    #[inline(always)]
    fn default() -> Dbgntt {
        <crate::RegValueT<Dbgntt_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgrnt_SPEC;
impl crate::sealed::RegSpec for Dbgrnt_SPEC {
    type DataType = u32;
}
#[doc = "SBCU Debug Grant Mask Register\n resetvalue={Debug Reset:0x0FFFF}"]
pub type Dbgrnt = crate::RegValueT<Dbgrnt_SPEC>;

impl Dbgrnt {
    #[doc = "DMA   Cerberus Trigger Enable. TC39xA  DMAH"]
    #[inline(always)]
    pub fn dma(self) -> crate::common::RegisterFieldBool<0, 1, 0, Dbgrnt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dbgrnt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "HSSL0 Trigger Enable"]
    #[inline(always)]
    pub fn hssl0(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Dbgrnt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Dbgrnt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU0 Grant Trigger Enable"]
    #[inline(always)]
    pub fn cpu0(self) -> crate::common::RegisterFieldBool<6, 1, 0, Dbgrnt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Dbgrnt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 Grant Trigger Enable"]
    #[inline(always)]
    pub fn cpu1(self) -> crate::common::RegisterFieldBool<7, 1, 0, Dbgrnt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Dbgrnt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 Grant Trigger Enable"]
    #[inline(always)]
    pub fn cpu2(self) -> crate::common::RegisterFieldBool<8, 1, 0, Dbgrnt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Dbgrnt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "HSM Register Master Interface Grant Trigger Enable"]
    #[inline(always)]
    pub fn hsmrmi(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Dbgrnt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Dbgrnt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "HSM Cache Master Interface Grant Trigger Enable"]
    #[inline(always)]
    pub fn hsmcmi(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Dbgrnt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Dbgrnt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Dbgrnt {
    #[inline(always)]
    fn default() -> Dbgrnt {
        <crate::RegValueT<Dbgrnt_SPEC> as RegisterValue<_>>::new(65535)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eadd_SPEC;
impl crate::sealed::RegSpec for Eadd_SPEC {
    type DataType = u32;
}
#[doc = "BCU Error Address Capture Register\n resetvalue={Application Reset:0x0}"]
pub type Eadd = crate::RegValueT<Eadd_SPEC>;

impl Eadd {
    #[doc = "Captured FPI Bus Address   FPIADR. This bit field holds the 32 bit FPI Bus address that has been captured at an FPI Bus error. Note that if multiple bus errors occurred  only the address of the first bus error is captured."]
    #[inline(always)]
    pub fn fpiadr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Eadd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Eadd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Eadd {
    #[inline(always)]
    fn default() -> Eadd {
        <crate::RegValueT<Eadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Econ_SPEC;
impl crate::sealed::RegSpec for Econ_SPEC {
    type DataType = u32;
}
#[doc = "BCU Error Control Capture Register\n resetvalue={Application Reset:0x0}"]
pub type Econ = crate::RegValueT<Econ_SPEC>;

impl Econ {
    #[doc = "FPI Bus Error Counter. ERRCNT is incremented on every occurrence of an FPI  160 Bus error. ERRCNT is        reset to 0 after the ECON register is read. Aborted        accesses to a 0 wait state SPB slave may also increment ERRCNT when the        slave generates an error acknowledge."]
    #[inline(always)]
    pub fn errcnt(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Econ_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Econ_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "State of FPI Bus Time Out Signal. This bit indicates the state of the time out signal at an FBI  160 Bus error."]
    #[inline(always)]
    pub fn tout(self) -> crate::common::RegisterFieldBool<14, 1, 0, Econ_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Econ_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "State of FPI Bus Ready Signal. This bit indicates the state of the ready signal at an FBI  160 Bus error."]
    #[inline(always)]
    pub fn rdy(self) -> crate::common::RegisterFieldBool<15, 1, 0, Econ_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Econ_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "State of FPI Bus Abort Signal. This bit indicates the state of the abort signal at an FBI  160 Bus error."]
    #[inline(always)]
    pub fn abt(self) -> crate::common::RegisterFieldBool<16, 1, 0, Econ_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Econ_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "State of FPI Bus Acknowledge Signals. This bit field indicates the acknowledge code that has been output by        the selected slave at an FPI  160 Bus error. Coding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ack(
        self,
    ) -> crate::common::RegisterField<17, 0x3, 1, 0, u8, Econ_SPEC, crate::common::RW> {
        crate::common::RegisterField::<17,0x3,1,0,u8, Econ_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "State of FPI Bus Supervisor Mode Signal. This bit indicates whether the FPI  160 Bus error occurred in Supervisor Mode        or in User Mode."]
    #[inline(always)]
    pub fn svm(self) -> crate::common::RegisterFieldBool<19, 1, 0, Econ_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Econ_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "State of FPI Bus Write Signal. This bit indicates whether the FPI  160 Bus error occurred at a write cycle         see CROSSREFERENCE  ."]
    #[inline(always)]
    pub fn wrn(self) -> crate::common::RegisterFieldBool<20, 1, 0, Econ_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Econ_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "State of FPI Bus Read Signal. This bit indicates whether the FPI  160 Bus error occurred at a read cycle         see CROSSREFERENCE  ."]
    #[inline(always)]
    pub fn rdn(self) -> crate::common::RegisterFieldBool<21, 1, 0, Econ_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Econ_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "FPI Bus Master Tag Number Signals. This bit field indicates the FPI  160 Bus master TAG number  definitions see CROSSREFERENCE  ."]
    #[inline(always)]
    pub fn tag(
        self,
    ) -> crate::common::RegisterField<22, 0x3f, 1, 0, u8, Econ_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3f,1,0,u8, Econ_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FPI Bus Operation Code Signals. The FPI  160 Bus operation codes are defined in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn opc(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Econ_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Econ_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Econ {
    #[inline(always)]
    fn default() -> Econ {
        <crate::RegValueT<Econ_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edat_SPEC;
impl crate::sealed::RegSpec for Edat_SPEC {
    type DataType = u32;
}
#[doc = "BCU Error Data Capture Register\n resetvalue={Application Reset:0x0}"]
pub type Edat = crate::RegValueT<Edat_SPEC>;

impl Edat {
    #[doc = "Captured FPI Bus Data   FPIDAT. This bit field holds the 32 bit FPI Bus data that has been captured at an FPI Bus error. Note that if multiple bus errors occurred  only the data of the first bus error is captured. No data are captured from transactions generated by the SHE module."]
    #[inline(always)]
    pub fn fpidat(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Edat_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Edat_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Edat {
    #[inline(always)]
    fn default() -> Edat {
        <crate::RegValueT<Edat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fegen_SPEC;
impl crate::sealed::RegSpec for Fegen_SPEC {
    type DataType = u32;
}
#[doc = "FPI Error Generation Control Register\n resetvalue={Debug Reset:0x0}"]
pub type Fegen = crate::RegValueT<Fegen_SPEC>;

impl Fegen {
    #[doc = "SEDM  Slave Encoder . The errors are injected in the FPI EDC encoders related to the BCU  SPB . Other bit combinations are reserved and do not inject errors."]
    #[inline(always)]
    pub fn sedm(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Fegen_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Fegen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MEDM  Master Encoder  Type of Error. The errors are injected in the FPI EDC encoders related to CPU0  SPB . Other bit combinations are reserved and do not inject errors."]
    #[inline(always)]
    pub fn medm(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Fegen_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Fegen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Signal Type of Error. The enable signals errors are injected by inverting the signal. For each        group of enable signals that is checked   gt  1 signal active   error  the        enable signal related to BCU   CPU0 is inverted. Other bit combinations are reserved and do not inject errors."]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Fegen_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Fegen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "BCU Type of Error. Other bit combinations are reserved and do not inject errors."]
    #[inline(always)]
    pub fn bcu(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, Fegen_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, Fegen_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Fegen {
    #[inline(always)]
    fn default() -> Fegen {
        <crate::RegValueT<Fegen_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x6A00}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MOD REV. This bit field defines the module revision number. The value of a module revision starts with 01H  first revision ."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MOD NUMBER. This bit field defines a module identification number. The value for the LBCU module is 006AH."]
    #[inline(always)]
    pub fn mod_number(
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(27136)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prioh_SPEC;
impl crate::sealed::RegSpec for Prioh_SPEC {
    type DataType = u32;
}
#[doc = "Arbiter Priority Register High\n resetvalue={Application Reset:0x0FEDCBA98,Application Reset:0x0FEDC8888,Application Reset:0x0FEDCBA98}"]
pub type Prioh = crate::RegValueT<Prioh_SPEC>;

impl Prioh {
    #[doc = "CPU2 Priority  Index 8 . This bit field defines the priority on the SPB for CPU2 access to the        SPB."]
    #[inline(always)]
    pub fn cpu2(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Prioh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Prioh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSMRMI Priority  Index 12 . This bit field defines the priority on the SPB for HSMRMI access to the        SPB."]
    #[inline(always)]
    pub fn hsmrmi(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Prioh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Prioh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSMCMI Priority  Index 13 . This bit field defines the priority on the SPB for HSMCMI access to the        SPB."]
    #[inline(always)]
    pub fn hsmcmi(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Prioh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Prioh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Prioh {
    #[inline(always)]
    fn default() -> Prioh {
        <crate::RegValueT<Prioh_SPEC> as RegisterValue<_>>::new(4275865736)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Priol_SPEC;
impl crate::sealed::RegSpec for Priol_SPEC {
    type DataType = u32;
}
#[doc = "Arbiter Priority Register Low\n resetvalue={Application Reset:0x76543210,Application Reset:0x088543210,Application Reset:0x76588210}"]
pub type Priol = crate::RegValueT<Priol_SPEC>;

impl Priol {
    #[doc = "DMA   Cerberus Priority  Index 0 . This bit field defines the priority on the SPB for DMA and Cerberus        access to the SPB."]
    #[inline(always)]
    pub fn dma(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Priol_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Priol_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSSL0 Priority  Index 3 . This bit field defines the priority on the SPB for HSSL0 access to the        SPB."]
    #[inline(always)]
    pub fn hssl0(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Priol_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Priol_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU0 Priority  Index 6 . This bit field defines the priority on the SPB for CPU0 access to the        SPB. Position   180 6  180  of the Req Lock Grant BCU input vector set is used in the          EDC implementation for Req Lock error injection. Therefore this          position shall be used for a CPU."]
    #[inline(always)]
    pub fn cpu0(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Priol_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Priol_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU1 Priority  Index 7 . This bit field defines the priority on the SPB for CPU1 access to the        SPB. This bit field contains the master priority for master connected to BCU        request input 7."]
    #[inline(always)]
    pub fn cpu1(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Priol_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Priol_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Priol {
    #[inline(always)]
    fn default() -> Priol {
        <crate::RegValueT<Priol_SPEC> as RegisterValue<_>>::new(5243408)
    }
}
