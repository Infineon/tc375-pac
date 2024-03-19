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
#[doc = r"FSI"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsi(pub(super) *mut u8);
unsafe impl core::marker::Send for Fsi {}
unsafe impl core::marker::Sync for Fsi {}
impl Fsi {
    #[doc = "Communication Register 1\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn comm_1(&self) -> crate::common::Reg<self::Comm1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }

    #[doc = "Communication Register 2\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn comm_2(&self) -> crate::common::Reg<self::Comm2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(5usize)) }
    }

    #[doc = "HSM Communication Register 1\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn hsmcomm_1(&self) -> crate::common::Reg<self::Hsmcomm1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(6usize)) }
    }

    #[doc = "HSM Communication Register 2\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn hsmcomm_2(&self) -> crate::common::Reg<self::Hsmcomm2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(7usize)) }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comm1_SPEC;
impl crate::sealed::RegSpec for Comm1_SPEC {
    type DataType = u8;
}
#[doc = "Communication Register 1\n resetvalue={System Reset:0x0}"]
pub type Comm1 = crate::RegValueT<Comm1_SPEC>;

impl Comm1 {
    #[doc = "FSI Communication 1   COMM1. This register can be written by FSI and DMU and is used to give        status handshake information between FSI and DMU."]
    #[inline(always)]
    pub fn comm1(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Comm1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Comm1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Comm1 {
    #[inline(always)]
    fn default() -> Comm1 {
        <crate::RegValueT<Comm1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comm2_SPEC;
impl crate::sealed::RegSpec for Comm2_SPEC {
    type DataType = u8;
}
#[doc = "Communication Register 2\n resetvalue={System Reset:0x0}"]
pub type Comm2 = crate::RegValueT<Comm2_SPEC>;

impl Comm2 {
    #[doc = "FSI Communication 2   COMM2. This register can be written by FSI and DMU and is used to give status handshake information between FSI and DMU."]
    #[inline(always)]
    pub fn comm2(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Comm2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Comm2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Comm2 {
    #[inline(always)]
    fn default() -> Comm2 {
        <crate::RegValueT<Comm2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsmcomm1_SPEC;
impl crate::sealed::RegSpec for Hsmcomm1_SPEC {
    type DataType = u8;
}
#[doc = "HSM Communication Register 1\n resetvalue={System Reset:0x0}"]
pub type Hsmcomm1 = crate::RegValueT<Hsmcomm1_SPEC>;

impl Hsmcomm1 {
    #[doc = "HSM FSI Communication 1   HSMCOMM1. This register can be written by FSI and DMU and is used to give status handshake information between FSI and DMU."]
    #[inline(always)]
    pub fn hsmcomm1(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Hsmcomm1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Hsmcomm1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Hsmcomm1 {
    #[inline(always)]
    fn default() -> Hsmcomm1 {
        <crate::RegValueT<Hsmcomm1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsmcomm2_SPEC;
impl crate::sealed::RegSpec for Hsmcomm2_SPEC {
    type DataType = u8;
}
#[doc = "HSM Communication Register 2\n resetvalue={System Reset:0x0}"]
pub type Hsmcomm2 = crate::RegValueT<Hsmcomm2_SPEC>;

impl Hsmcomm2 {
    #[doc = "HSM FSI Communication 2   HSMCOMM2. This register can be written by FSI and DMU and is used to give status handshake information between FSI and DMU."]
    #[inline(always)]
    pub fn hsmcomm2(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Hsmcomm2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Hsmcomm2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Hsmcomm2 {
    #[inline(always)]
    fn default() -> Hsmcomm2 {
        <crate::RegValueT<Hsmcomm2_SPEC> as RegisterValue<_>>::new(0)
    }
}
