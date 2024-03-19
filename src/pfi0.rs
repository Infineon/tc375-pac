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
#[doc = r"PFI"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfi0(pub(super) *mut u8);
unsafe impl core::marker::Send for Pfi0 {}
unsafe impl core::marker::Sync for Pfi0 {}
impl Pfi0 {
    #[doc = "ECC Read Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn eccr(&self) -> crate::common::Reg<self::Eccr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "ECC Status Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn eccs(&self) -> crate::common::Reg<self::Eccs_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "DBAB"]
    #[inline(always)]
    pub fn dbab(self) -> [self::Dbab; 2] {
        unsafe {
            [
                self::Dbab(self.0.add(0x4000usize + 0x0usize)),
                self::Dbab(self.0.add(0x4000usize + 0x20usize)),
            ]
        }
    }
    #[doc = "MBAB"]
    #[inline(always)]
    pub fn mbab(self) -> self::Mbab {
        unsafe { self::Mbab(self.0.add(32768usize)) }
    }
    #[doc = "SBAB"]
    #[inline(always)]
    pub fn sbab(self) -> [self::Sbab; 17] {
        unsafe {
            [
                self::Sbab(self.0.add(0x2000usize + 0x0usize)),
                self::Sbab(self.0.add(0x2000usize + 0x20usize)),
                self::Sbab(self.0.add(0x2000usize + 0x40usize)),
                self::Sbab(self.0.add(0x2000usize + 0x60usize)),
                self::Sbab(self.0.add(0x2000usize + 0x80usize)),
                self::Sbab(self.0.add(0x2000usize + 0xa0usize)),
                self::Sbab(self.0.add(0x2000usize + 0xc0usize)),
                self::Sbab(self.0.add(0x2000usize + 0xe0usize)),
                self::Sbab(self.0.add(0x2000usize + 0x100usize)),
                self::Sbab(self.0.add(0x2000usize + 0x120usize)),
                self::Sbab(self.0.add(0x2000usize + 0x140usize)),
                self::Sbab(self.0.add(0x2000usize + 0x160usize)),
                self::Sbab(self.0.add(0x2000usize + 0x180usize)),
                self::Sbab(self.0.add(0x2000usize + 0x1a0usize)),
                self::Sbab(self.0.add(0x2000usize + 0x1c0usize)),
                self::Sbab(self.0.add(0x2000usize + 0x1e0usize)),
                self::Sbab(self.0.add(0x2000usize + 0x200usize)),
            ]
        }
    }
    #[doc = "ZBAB"]
    #[inline(always)]
    pub fn zbab(self) -> [self::Zbab; 4] {
        unsafe {
            [
                self::Zbab(self.0.add(0xc000usize + 0x0usize)),
                self::Zbab(self.0.add(0xc000usize + 0x20usize)),
                self::Zbab(self.0.add(0xc000usize + 0x40usize)),
                self::Zbab(self.0.add(0xc000usize + 0x60usize)),
            ]
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccr_SPEC;
impl crate::sealed::RegSpec for Eccr_SPEC {
    type DataType = u32;
}
#[doc = "ECC Read Register\n resetvalue={System Reset:0x0}"]
pub type Eccr = crate::RegValueT<Eccr_SPEC>;

impl Eccr {
    #[doc = "Error Correction Read Code   RCODE. ECC code  read from the Flash read buffer with last data read operation."]
    #[inline(always)]
    pub fn rcode(
        self,
    ) -> crate::common::RegisterField<0, 0x3fffff, 1, 0, u32, Eccr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3fffff,1,0,u32, Eccr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Eccr {
    #[inline(always)]
    fn default() -> Eccr {
        <crate::RegValueT<Eccr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccs_SPEC;
impl crate::sealed::RegSpec for Eccs_SPEC {
    type DataType = u32;
}
#[doc = "ECC Status Register\n resetvalue={System Reset:0x0}"]
pub type Eccs = crate::RegValueT<Eccs_SPEC>;

impl Eccs {
    #[doc = "Read Access Single Bit ECC Error   ERR1. The flag reports a single bit ECC failure during the last NVM read access."]
    #[inline(always)]
    pub fn err1(self) -> crate::common::RegisterFieldBool<0, 1, 0, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Eccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Read Access Double Bit ECC Error   ERR2. The flag reports a double bit ECC failure during the last NVM read access."]
    #[inline(always)]
    pub fn err2(self) -> crate::common::RegisterFieldBool<1, 1, 0, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Eccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Read Access Multi bit ECC Error   ERRM. The flag reports multi bit ECC failure during the last NVM read access."]
    #[inline(always)]
    pub fn errm(self) -> crate::common::RegisterFieldBool<3, 1, 0, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Eccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Read Access ECC Error Within the Address   ERRA. The flag reports an address error during the last NVM read access."]
    #[inline(always)]
    pub fn erra(self) -> crate::common::RegisterFieldBool<4, 1, 0, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Eccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Read Access All Zeros   ALL0. The flag reports the All Zeros condition during the last NVM read access."]
    #[inline(always)]
    pub fn all0(self) -> crate::common::RegisterFieldBool<5, 1, 0, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Eccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "All Ones   ALL1. The flag reports the All Ones condition during the last NVM read access."]
    #[inline(always)]
    pub fn all1(self) -> crate::common::RegisterFieldBool<6, 1, 0, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Eccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Any Read Access ECC Error   ERRANY. The flag reports any ECC failure during the last NVM read access."]
    #[inline(always)]
    pub fn errany(self) -> crate::common::RegisterFieldBool<7, 1, 0, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Eccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated Single Bit ECC Errors   AER1. The flag accumulates single bit failures during NVM read operations."]
    #[inline(always)]
    pub fn aer1(self) -> crate::common::RegisterFieldBool<16, 1, 0, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Eccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated Double Bit ECC Errors   AER2. The flag accumulates double bit failures during NVM read operations."]
    #[inline(always)]
    pub fn aer2(self) -> crate::common::RegisterFieldBool<17, 1, 0, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Eccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated Multi bit ECC Errors   AERM. The flag accumulates multi bit failures during NVM read accesses."]
    #[inline(always)]
    pub fn aerm(self) -> crate::common::RegisterFieldBool<19, 1, 0, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Eccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated ECC Error Within the Address   ARRA. The flag accumulates an address errors during NVM read accesses."]
    #[inline(always)]
    pub fn arra(self) -> crate::common::RegisterFieldBool<20, 1, 0, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Eccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated All Zeros   AAL0. The flag accumulates the All Zeros condition during NVM read accesses."]
    #[inline(always)]
    pub fn aal0(self) -> crate::common::RegisterFieldBool<21, 1, 0, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Eccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated All Ones   AAL1. The flag accumulates the All Ones condition during NVM read accesses."]
    #[inline(always)]
    pub fn aal1(self) -> crate::common::RegisterFieldBool<22, 1, 0, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Eccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Accumulated Any Read Access ECC Error   AERANY. The status bit accumulates ECC failures during NVM read accesses."]
    #[inline(always)]
    pub fn aerany(self) -> crate::common::RegisterFieldBool<23, 1, 0, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Eccs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Eccs {
    #[inline(always)]
    fn default() -> Eccs {
        <crate::RegValueT<Eccs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc = "DBAB"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbab(pub(super) *mut u8);
unsafe impl core::marker::Send for Dbab {}
unsafe impl core::marker::Sync for Dbab {}
impl Dbab {
    #[doc = "DBAB Record 0\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn dbabrecordx(
        &self,
    ) -> crate::common::Reg<dbab::DbabrecorDx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod dbab {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbabrecorDx_SPEC;
    impl crate::sealed::RegSpec for DbabrecorDx_SPEC {
        type DataType = u32;
    }
    #[doc = "DBAB Record 0\n resetvalue={System Reset:0x0}"]
    pub type DbabrecorDx = crate::RegValueT<DbabrecorDx_SPEC>;

    impl DbabrecorDx {
        #[doc = "Address   ADDR. Captured address of local PFLASH bank with detected error."]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffff, 1, 0, u32, DbabrecorDx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<5,0x7ffff,1,0,u32, DbabrecorDx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Erase Counter Marker   ECMK. The captured address is from the Flash or Erase Counter."]
        #[inline(always)]
        pub fn ecmk(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, DbabrecorDx_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,DbabrecorDx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Reserved   RES. Always read as 0  should be written with 0."]
        #[inline(always)]
        pub fn res(
            self,
        ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, DbabrecorDx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<28,0x7,1,0,u8, DbabrecorDx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Valid   VLD"]
        #[inline(always)]
        pub fn vld(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, DbabrecorDx_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,DbabrecorDx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for DbabrecorDx {
        #[inline(always)]
        fn default() -> DbabrecorDx {
            <crate::RegValueT<DbabrecorDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "MBAB"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbab(pub(super) *mut u8);
unsafe impl core::marker::Send for Mbab {}
unsafe impl core::marker::Sync for Mbab {}
impl Mbab {
    #[doc = "MBAB Record 0\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn mbabrecordx(
        &self,
    ) -> crate::common::Reg<mbab::MbabrecorDx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod mbab {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MbabrecorDx_SPEC;
    impl crate::sealed::RegSpec for MbabrecorDx_SPEC {
        type DataType = u32;
    }
    #[doc = "MBAB Record 0\n resetvalue={System Reset:0x0}"]
    pub type MbabrecorDx = crate::RegValueT<MbabrecorDx_SPEC>;

    impl MbabrecorDx {
        #[doc = "Address   ADDR. Captured address of local PFLASH bank with detected error."]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffff, 1, 0, u32, MbabrecorDx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<5,0x7ffff,1,0,u32, MbabrecorDx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Erase Counter Marker   ECMK. The captured address is from the Flash or Erase Counter."]
        #[inline(always)]
        pub fn ecmk(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, MbabrecorDx_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,MbabrecorDx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Reserved   RES. Always read as 0  should be written with 0."]
        #[inline(always)]
        pub fn res(
            self,
        ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, MbabrecorDx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<28,0x7,1,0,u8, MbabrecorDx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Valid   VLD"]
        #[inline(always)]
        pub fn vld(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, MbabrecorDx_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,MbabrecorDx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for MbabrecorDx {
        #[inline(always)]
        fn default() -> MbabrecorDx {
            <crate::RegValueT<MbabrecorDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "SBAB"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbab(pub(super) *mut u8);
unsafe impl core::marker::Send for Sbab {}
unsafe impl core::marker::Sync for Sbab {}
impl Sbab {
    #[doc = "SBAB Record 0\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn sbabrecordx(
        &self,
    ) -> crate::common::Reg<sbab::SbabrecorDx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod sbab {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbabrecorDx_SPEC;
    impl crate::sealed::RegSpec for SbabrecorDx_SPEC {
        type DataType = u32;
    }
    #[doc = "SBAB Record 0\n resetvalue={System Reset:0x0}"]
    pub type SbabrecorDx = crate::RegValueT<SbabrecorDx_SPEC>;

    impl SbabrecorDx {
        #[doc = "Address   ADDR. Captured local PFLASH bank address of the page with detected error."]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffff, 1, 0, u32, SbabrecorDx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<5,0x7ffff,1,0,u32, SbabrecorDx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Erase Counter Marker   ECMK. The captured address is from the Flash or Erase Counter."]
        #[inline(always)]
        pub fn ecmk(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, SbabrecorDx_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,SbabrecorDx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Reserved   RES. Always read as 0  should be written with 0."]
        #[inline(always)]
        pub fn res(
            self,
        ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, SbabrecorDx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<28,0x7,1,0,u8, SbabrecorDx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Valid   VLD"]
        #[inline(always)]
        pub fn vld(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, SbabrecorDx_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,SbabrecorDx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for SbabrecorDx {
        #[inline(always)]
        fn default() -> SbabrecorDx {
            <crate::RegValueT<SbabrecorDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "ZBAB"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Zbab(pub(super) *mut u8);
unsafe impl core::marker::Send for Zbab {}
unsafe impl core::marker::Sync for Zbab {}
impl Zbab {
    #[doc = "ZBAB Record 0\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn zbabrecordx(
        &self,
    ) -> crate::common::Reg<zbab::ZbabrecorDx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod zbab {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ZbabrecorDx_SPEC;
    impl crate::sealed::RegSpec for ZbabrecorDx_SPEC {
        type DataType = u32;
    }
    #[doc = "ZBAB Record 0\n resetvalue={System Reset:0x0}"]
    pub type ZbabrecorDx = crate::RegValueT<ZbabrecorDx_SPEC>;

    impl ZbabrecorDx {
        #[doc = "Address   ADDR. Captured address of local PFLASH bank with detected error."]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffff, 1, 0, u32, ZbabrecorDx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<5,0x7ffff,1,0,u32, ZbabrecorDx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Erase Counter Marker   ECMK. The captured address is from the Flash or Erase Counter."]
        #[inline(always)]
        pub fn ecmk(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, ZbabrecorDx_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,ZbabrecorDx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Reserved   RES. Always read as 0  should be written with 0."]
        #[inline(always)]
        pub fn res(
            self,
        ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, ZbabrecorDx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<28,0x7,1,0,u8, ZbabrecorDx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Valid   VLD"]
        #[inline(always)]
        pub fn vld(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, ZbabrecorDx_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,ZbabrecorDx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for ZbabrecorDx {
        #[inline(always)]
        fn default() -> ZbabrecorDx {
            <crate::RegValueT<ZbabrecorDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
