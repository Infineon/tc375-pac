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
#[doc = r"DAM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dam0(pub(super) *mut u8);
unsafe impl core::marker::Send for Dam0 {}
unsafe impl core::marker::Sync for Dam0 {}
impl Dam0 {
    #[doc = "DAM Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "DAM Access Enable Register 1\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen1(&self) -> crate::common::Reg<self::Accen1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "DAM Clock Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "DAM Memory Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn memcon(&self) -> crate::common::Reg<self::Memcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "DAM Module ID Register\n resetvalue={Application Reset:0x088C003}"]
    #[inline(always)]
    pub const fn modid(&self) -> crate::common::Reg<self::Modid_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "RGNACCEN"]
    #[inline(always)]
    pub fn rgnaccen(self) -> [self::Rgnaccen; 8] {
        unsafe {
            [
                self::Rgnaccen(self.0.add(0xd8usize + 0x0usize)),
                self::Rgnaccen(self.0.add(0xd8usize + 0x10usize)),
                self::Rgnaccen(self.0.add(0xd8usize + 0x20usize)),
                self::Rgnaccen(self.0.add(0xd8usize + 0x30usize)),
                self::Rgnaccen(self.0.add(0xd8usize + 0x40usize)),
                self::Rgnaccen(self.0.add(0xd8usize + 0x50usize)),
                self::Rgnaccen(self.0.add(0xd8usize + 0x60usize)),
                self::Rgnaccen(self.0.add(0xd8usize + 0x70usize)),
            ]
        }
    }
    #[doc = "RGN"]
    #[inline(always)]
    pub fn rgn(self) -> [self::Rgn; 8] {
        unsafe {
            [
                self::Rgn(self.0.add(0x50usize + 0x0usize)),
                self::Rgn(self.0.add(0x50usize + 0x10usize)),
                self::Rgn(self.0.add(0x50usize + 0x20usize)),
                self::Rgn(self.0.add(0x50usize + 0x30usize)),
                self::Rgn(self.0.add(0x50usize + 0x40usize)),
                self::Rgn(self.0.add(0x50usize + 0x50usize)),
                self::Rgn(self.0.add(0x50usize + 0x60usize)),
                self::Rgn(self.0.add(0x50usize + 0x70usize)),
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
#[doc = "DAM Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accen0 = crate::RegValueT<Accen0_SPEC>;

impl Accen0 {
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
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
pub struct Accen1_SPEC;
impl crate::sealed::RegSpec for Accen1_SPEC {
    type DataType = u32;
}
#[doc = "DAM Access Enable Register 1\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accen1 = crate::RegValueT<Accen1_SPEC>;

impl Accen1 {
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en42(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en43(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en44(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en45(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en46(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en47(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en48(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en49(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en50(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en51(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en52(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en53(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en54(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en55(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en56(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en57(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en58(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en59(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en60(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en61(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en62(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en63(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Accen1 {
    #[inline(always)]
    fn default() -> Accen1 {
        <crate::RegValueT<Accen1_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clc_SPEC;
impl crate::sealed::RegSpec for Clc_SPEC {
    type DataType = u32;
}
#[doc = "DAM Clock Control Register\n resetvalue={Application Reset:0x0}"]
pub type Clc = crate::RegValueT<Clc_SPEC>;

impl Clc {
    #[doc = "DAM Disable Request Bit   DISR. This bit is used for enable disable control of the DAM ."]
    #[inline(always)]
    pub fn disr(self) -> crate::common::RegisterFieldBool<0, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "DAM Disable Status Bit   DISS. Current state of DAM ."]
    #[inline(always)]
    pub fn diss(self) -> crate::common::RegisterFieldBool<1, 1, 0, Clc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Clc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Clc {
    #[inline(always)]
    fn default() -> Clc {
        <crate::RegValueT<Clc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memcon_SPEC;
impl crate::sealed::RegSpec for Memcon_SPEC {
    type DataType = u32;
}
#[doc = "DAM Memory Control Register\n resetvalue={Application Reset:0x0}"]
pub type Memcon = crate::RegValueT<Memcon_SPEC>;

impl Memcon {
    #[doc = "Read Only Memory   ROM. Configure RAM to be Read Only Memory"]
    #[inline(always)]
    pub fn rom(self) -> crate::common::RegisterFieldBool<0, 1, 0, Memcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Memcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Internal ECC Error   INTERR. Flag set by hardware when the DAM logic detects an ECC error while accessing the RAM. This bit is cleared        by writing 0 but cannot be set by        software."]
    #[inline(always)]
    pub fn interr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Memcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Memcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Internal Read Modify Write Error   RMWERR. Flag set by hardware when the DAM logic detects an ECC error on the read phase of an internal RMW        operation. This bit is cleared by writing 0 but cannot be set by software."]
    #[inline(always)]
    pub fn rmwerr(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Memcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Memcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SRI Data Phase ECC Error   DATAERR. Flag set by hardware when the SRI interface detects an ECC error in the        data phase of an incoming write transaction. This bit is cleared by        writing 0 but cannot be set by software."]
    #[inline(always)]
    pub fn dataerr(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Memcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Memcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SRI Address Phase ECC Error   ADDERR. Flag set by hardware when the SRI interface detects an ECC error in the        address phase of an incoming transaction. This bit is cleared by writing 0 but cannot be set by software."]
    #[inline(always)]
    pub fn adderr(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Memcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Memcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Protection Bit for Memory Integrity Control Bit   PMIC. Will always return 0 when read"]
    #[inline(always)]
    pub fn pmic(self) -> crate::common::RegisterFieldBool<8, 1, 0, Memcon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Memcon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "ECC Error Disable   ERRDIS. When set SRI bus errors caused by ECC errors in data read from the SRAM        will be disabled"]
    #[inline(always)]
    pub fn errdis(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Memcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Memcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Memcon {
    #[inline(always)]
    fn default() -> Memcon {
        <crate::RegValueT<Memcon_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modid_SPEC;
impl crate::sealed::RegSpec for Modid_SPEC {
    type DataType = u32;
}
#[doc = "DAM Module ID Register\n resetvalue={Application Reset:0x088C003}"]
pub type Modid = crate::RegValueT<Modid_SPEC>;

impl Modid {
    #[doc = "Module Identification Value   ID VALUE"]
    #[inline(always)]
    pub fn id_value(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Modid_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Modid_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Modid {
    #[inline(always)]
    fn default() -> Modid {
        <crate::RegValueT<Modid_SPEC> as RegisterValue<_>>::new(8962051)
    }
}

#[doc = "RGNACCEN"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rgnaccen(pub(super) *mut u8);
unsafe impl core::marker::Send for Rgnaccen {}
unsafe impl core::marker::Sync for Rgnaccen {}
impl Rgnaccen {
    #[doc = "DAM Region Read Enable Register A\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn rgnaccenrax(
        &self,
    ) -> crate::common::Reg<rgnaccen::RgnaccenrAx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "DAM Region Read Enable Register B\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn rgnaccenrbx(
        &self,
    ) -> crate::common::Reg<rgnaccen::RgnaccenrBx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
}
pub mod rgnaccen {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnaccenrAx_SPEC;
    impl crate::sealed::RegSpec for RgnaccenrAx_SPEC {
        type DataType = u32;
    }
    #[doc = "DAM Region Read Enable Register A\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type RgnaccenrAx = crate::RegValueT<RgnaccenrAx_SPEC>;

    impl RgnaccenrAx {
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<0,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<1,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<2,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en3(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<3,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en4(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<4,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en5(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<5,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en6(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<6,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en7(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<7,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en8(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<8,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en9(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<9,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en10(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<10,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en11(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<11,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en12(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<12,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en13(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<13,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en14(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<14,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en15(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<15,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en16(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<16,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en17(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<17,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en18(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<18,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en19(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<19,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en20(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<20,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en21(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<21,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en22(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<22,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en23(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<23,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en24(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<24,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en25(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<25,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en26(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<26,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en27(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<27,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en28(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<28,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en29(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<29,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en30(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<30,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en31(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, RgnaccenrAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<31,1,0,RgnaccenrAx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for RgnaccenrAx {
        #[inline(always)]
        fn default() -> RgnaccenrAx {
            <crate::RegValueT<RgnaccenrAx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnaccenrBx_SPEC;
    impl crate::sealed::RegSpec for RgnaccenrBx_SPEC {
        type DataType = u32;
    }
    #[doc = "DAM Region Read Enable Register B\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type RgnaccenrBx = crate::RegValueT<RgnaccenrBx_SPEC>;

    impl RgnaccenrBx {
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en32(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<0,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en33(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<1,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en34(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<2,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en35(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<3,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en36(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<4,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en37(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<5,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en38(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<6,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en39(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<7,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en40(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<8,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en41(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<9,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en42(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<10,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en43(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<11,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en44(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<12,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en45(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<13,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en46(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<14,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en47(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<15,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en48(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<16,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en49(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<17,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en50(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<18,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en51(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<19,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en52(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<20,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en53(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<21,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en54(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<22,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en55(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<23,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en56(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<24,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en57(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<25,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en58(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<26,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en59(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<27,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en60(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<28,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en61(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<29,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en62(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<30,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en63(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, RgnaccenrBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<31,1,0,RgnaccenrBx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for RgnaccenrBx {
        #[inline(always)]
        fn default() -> RgnaccenrBx {
            <crate::RegValueT<RgnaccenrBx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
}
#[doc = "RGN"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rgn(pub(super) *mut u8);
unsafe impl core::marker::Send for Rgn {}
unsafe impl core::marker::Sync for Rgn {}
impl Rgn {
    #[doc = "DAM Region Write Enable Register A\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn rgnaccenwax(
        &self,
    ) -> crate::common::Reg<rgn::RgnaccenwAx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "DAM Region Write Enable Register B\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn rgnaccenwbx(
        &self,
    ) -> crate::common::Reg<rgn::RgnaccenwBx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "DAM Region Lower Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rgnlax(&self) -> crate::common::Reg<rgn::RgnlAx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "DAM Region Upper Address Register\n resetvalue={Application Reset:0x0FFFFFFE0}"]
    #[inline(always)]
    pub const fn rgnuax(&self) -> crate::common::Reg<rgn::RgnuAx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
}
pub mod rgn {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnaccenwAx_SPEC;
    impl crate::sealed::RegSpec for RgnaccenwAx_SPEC {
        type DataType = u32;
    }
    #[doc = "DAM Region Write Enable Register A\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type RgnaccenwAx = crate::RegValueT<RgnaccenwAx_SPEC>;

    impl RgnaccenwAx {
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<0,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<1,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<2,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en3(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<3,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en4(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<4,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en5(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<5,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en6(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<6,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en7(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<7,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en8(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<8,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en9(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<9,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en10(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<10,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en11(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<11,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en12(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<12,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en13(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<13,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en14(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<14,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en15(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<15,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en16(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<16,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en17(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<17,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en18(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<18,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en19(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<19,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en20(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<20,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en21(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<21,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en22(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<22,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en23(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<23,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en24(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<24,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en25(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<25,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en26(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<26,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en27(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<27,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en28(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<28,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en29(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<29,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en30(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<30,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en31(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, RgnaccenwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<31,1,0,RgnaccenwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for RgnaccenwAx {
        #[inline(always)]
        fn default() -> RgnaccenwAx {
            <crate::RegValueT<RgnaccenwAx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnaccenwBx_SPEC;
    impl crate::sealed::RegSpec for RgnaccenwBx_SPEC {
        type DataType = u32;
    }
    #[doc = "DAM Region Write Enable Register B\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type RgnaccenwBx = crate::RegValueT<RgnaccenwBx_SPEC>;

    impl RgnaccenwBx {
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en32(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<0,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en33(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<1,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en34(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<2,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en35(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<3,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en36(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<4,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en37(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<5,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en38(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<6,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en39(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<7,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en40(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<8,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en41(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<9,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en42(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<10,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en43(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<11,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en44(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<12,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en45(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<13,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en46(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<14,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en47(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<15,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en48(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<16,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en49(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<17,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en50(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<18,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en51(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<19,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en52(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<20,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en53(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<21,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en54(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<22,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en55(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<23,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en56(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<24,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en57(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<25,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en58(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<26,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en59(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<27,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en60(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<28,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en61(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<29,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en62(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<30,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en63(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, RgnaccenwBx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<31,1,0,RgnaccenwBx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for RgnaccenwBx {
        #[inline(always)]
        fn default() -> RgnaccenwBx {
            <crate::RegValueT<RgnaccenwBx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnlAx_SPEC;
    impl crate::sealed::RegSpec for RgnlAx_SPEC {
        type DataType = u32;
    }
    #[doc = "DAM Region Lower Address Register\n resetvalue={Application Reset:0x0}"]
    pub type RgnlAx = crate::RegValueT<RgnlAx_SPEC>;

    impl RgnlAx {
        #[doc = "Region Lower Address   ADDR. Bits 31 to 5 of the SRI address which is the lower bound of the defined        memory region"]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, RgnlAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7ffffff,1,0,u32, RgnlAx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for RgnlAx {
        #[inline(always)]
        fn default() -> RgnlAx {
            <crate::RegValueT<RgnlAx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnuAx_SPEC;
    impl crate::sealed::RegSpec for RgnuAx_SPEC {
        type DataType = u32;
    }
    #[doc = "DAM Region Upper Address Register\n resetvalue={Application Reset:0x0FFFFFFE0}"]
    pub type RgnuAx = crate::RegValueT<RgnuAx_SPEC>;

    impl RgnuAx {
        #[doc = "Region Lower Address   ADDR. Bits 31 to 5 of the SRI address which is the upper bound of the defined        memory region. i.e. the first address outside the protected region."]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, RgnuAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7ffffff,1,0,u32, RgnuAx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for RgnuAx {
        #[inline(always)]
        fn default() -> RgnuAx {
            <crate::RegValueT<RgnuAx_SPEC> as RegisterValue<_>>::new(4294967264)
        }
    }
}
