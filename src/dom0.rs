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
#[doc = r"DOM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dom0(pub(super) *mut u8);
unsafe impl core::marker::Send for Dom0 {}
unsafe impl core::marker::Sync for Dom0 {}
impl Dom0 {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1264usize)) }
    }

    #[doc = "Domain 0 Bridge Control Register\n resetvalue={Application Reset:0x0,Application Reset:0x200,Application Reset:0x0}"]
    #[inline(always)]
    pub const fn brcon(&self) -> crate::common::Reg<self::Brcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1072usize)) }
    }

    #[doc = "Identification Register\n resetvalue={Application Reset:0x4D000}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1032usize)) }
    }

    #[doc = "Protocol Error Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pestat(&self) -> crate::common::Reg<self::Pestat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1040usize)) }
    }

    #[doc = "Transaction ID Enable Register\n resetvalue={Application Reset:0x0FFF3FFF}"]
    #[inline(always)]
    pub const fn tiden(&self) -> crate::common::Reg<self::Tiden_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1056usize)) }
    }

    #[doc = "Transaction ID Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tidstat(&self) -> crate::common::Reg<self::Tidstat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1048usize)) }
    }
    #[doc = "SCICTRL"]
    #[inline(always)]
    pub fn scictrl(self) -> [self::Scictrl; 16] {
        unsafe {
            [
                self::Scictrl(self.0.add(0x0usize + 0x0usize)),
                self::Scictrl(self.0.add(0x0usize + 0x20usize)),
                self::Scictrl(self.0.add(0x0usize + 0x40usize)),
                self::Scictrl(self.0.add(0x0usize + 0x60usize)),
                self::Scictrl(self.0.add(0x0usize + 0x80usize)),
                self::Scictrl(self.0.add(0x0usize + 0xa0usize)),
                self::Scictrl(self.0.add(0x0usize + 0xc0usize)),
                self::Scictrl(self.0.add(0x0usize + 0xe0usize)),
                self::Scictrl(self.0.add(0x0usize + 0x100usize)),
                self::Scictrl(self.0.add(0x0usize + 0x120usize)),
                self::Scictrl(self.0.add(0x0usize + 0x140usize)),
                self::Scictrl(self.0.add(0x0usize + 0x160usize)),
                self::Scictrl(self.0.add(0x0usize + 0x180usize)),
                self::Scictrl(self.0.add(0x0usize + 0x1a0usize)),
                self::Scictrl(self.0.add(0x0usize + 0x1c0usize)),
                self::Scictrl(self.0.add(0x0usize + 0x1e0usize)),
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
#[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accen0 = crate::RegValueT<Accen0_SPEC>;

impl Accen0 {
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
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
pub struct Brcon_SPEC;
impl crate::sealed::RegSpec for Brcon_SPEC {
    type DataType = u32;
}
#[doc = "Domain 0 Bridge Control Register\n resetvalue={Application Reset:0x0,Application Reset:0x200,Application Reset:0x0}"]
pub type Brcon = crate::RegValueT<Brcon_SPEC>;

impl Brcon {
    #[doc = "Online Data Acquisition Enable   OLDAEN. This bit is used to control trap generated for write accesses to the        OLDA address range associated with this domain."]
    #[inline(always)]
    pub fn oldaen(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Brcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Brcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Brcon {
    #[inline(always)]
    fn default() -> Brcon {
        <crate::RegValueT<Brcon_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Identification Register\n resetvalue={Application Reset:0x4D000}"]
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
    #[doc = "Module Type   MOD TYPE. The bit field is set to D0H which defines the module as a 32 bit module."]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MOD NUMBER. This bit field defines a module identification number. The value for the  lt Default   Font gt SRI Fabric is 0004H."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(315392)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pestat_SPEC;
impl crate::sealed::RegSpec for Pestat_SPEC {
    type DataType = u32;
}
#[doc = "Protocol Error Status Register\n resetvalue={Application Reset:0x0}"]
pub type Pestat = crate::RegValueT<Pestat_SPEC>;

impl Pestat {
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci0(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Pestat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Pestat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci1(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Pestat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Pestat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci2(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Pestat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Pestat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci3(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Pestat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Pestat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci4(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Pestat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Pestat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci5(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Pestat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Pestat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci6(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Pestat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Pestat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci7(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Pestat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Pestat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci8(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Pestat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Pestat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci9(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Pestat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Pestat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci10(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Pestat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Pestat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci11(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Pestat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Pestat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci12(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Pestat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Pestat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci13(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Pestat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Pestat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci14(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Pestat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Pestat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci15(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Pestat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Pestat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Pestat {
    #[inline(always)]
    fn default() -> Pestat {
        <crate::RegValueT<Pestat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tiden_SPEC;
impl crate::sealed::RegSpec for Tiden_SPEC {
    type DataType = u32;
}
#[doc = "Transaction ID Enable Register\n resetvalue={Application Reset:0x0FFF3FFF}"]
pub type Tiden = crate::RegValueT<Tiden_SPEC>;

impl Tiden {
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci9(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci0(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci1(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci2(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci3(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci4(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci5(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci6(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci7(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci8(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci9(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci10(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci11(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Tiden_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Tiden_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Tiden {
    #[inline(always)]
    fn default() -> Tiden {
        <crate::RegValueT<Tiden_SPEC> as RegisterValue<_>>::new(268386303)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tidstat_SPEC;
impl crate::sealed::RegSpec for Tidstat_SPEC {
    type DataType = u32;
}
#[doc = "Transaction ID Status Register\n resetvalue={Application Reset:0x0}"]
pub type Tidstat = crate::RegValueT<Tidstat_SPEC>;

impl Tidstat {
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci9(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci0(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci1(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci2(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci3(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci4(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci5(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci6(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci7(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci8(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci9(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci10(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci11(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Tidstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Tidstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Tidstat {
    #[inline(always)]
    fn default() -> Tidstat {
        <crate::RegValueT<Tidstat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc = "SCICTRL"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scictrl(pub(super) *mut u8);
unsafe impl core::marker::Send for Scictrl {}
unsafe impl core::marker::Sync for Scictrl {}
impl Scictrl {
    #[doc = "SCI 0 Error Address Capture Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn erraddrx(&self) -> crate::common::Reg<scictrl::ErraddRx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "SCI 0 Error Capture Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn errx(&self) -> crate::common::Reg<scictrl::ErRx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Protocol Error Control Register 0\n resetvalue={Application Reset:0x1}"]
    #[inline(always)]
    pub const fn peconx(&self) -> crate::common::Reg<scictrl::PecoNx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "SCI0 Arbiter Priority Register\n resetvalue={Application Reset:0x70000}"]
    #[inline(always)]
    pub const fn priorityx(
        &self,
    ) -> crate::common::Reg<scictrl::PrioritYx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
pub mod scictrl {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ErraddRx_SPEC;
    impl crate::sealed::RegSpec for ErraddRx_SPEC {
        type DataType = u32;
    }
    #[doc = "SCI 0 Error Address Capture Register\n resetvalue={Application Reset:0x0}"]
    pub type ErraddRx = crate::RegValueT<ErraddRx_SPEC>;

    impl ErraddRx {
        #[doc = "Transaction Address   ADDR. This bitfield contains the address of the erroneous transaction from the address phase"]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, ErraddRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, ErraddRx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for ErraddRx {
        #[inline(always)]
        fn default() -> ErraddRx {
            <crate::RegValueT<ErraddRx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ErRx_SPEC;
    impl crate::sealed::RegSpec for ErRx_SPEC {
        type DataType = u32;
    }
    #[doc = "SCI 0 Error Capture Register\n resetvalue={Application Reset:0x0}"]
    pub type ErRx = crate::RegValueT<ErRx_SPEC>;

    impl ErRx {
        #[doc = "Read Status   RD"]
        #[inline(always)]
        pub fn rd_n(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, ErRx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0, 1, 0, ErRx_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Write Status   WR"]
        #[inline(always)]
        pub fn wr_n(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, ErRx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<1, 1, 0, ErRx_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Supervisor Mode Status   SVM"]
        #[inline(always)]
        pub fn svm(self) -> crate::common::RegisterFieldBool<2, 1, 0, ErRx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<2, 1, 0, ErRx_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Operation Code   OPC. This field contains the opcode of the erroneous transaction  see CROSSREFERENCE for details."]
        #[inline(always)]
        pub fn opc(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, ErRx_SPEC, crate::common::R> {
            crate::common::RegisterField::<4,0xf,1,0,u8, ErRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Transaction ID   TR ID. This field contains the Master  8217 s transaction ID of the erroneous        transaction. The Transaction ID is built out of the Master  8217 s 6 bit        unique TAG ID  TR ID 5 0    and a 2 bit running number TR ID 7 6 ."]
        #[inline(always)]
        pub fn tr_id(
            self,
        ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, ErRx_SPEC, crate::common::R> {
            crate::common::RegisterField::<8,0xff,1,0,u8, ErRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Address Phase Error Detection Information   ADDR EDC. This field contains the Address Phase Error Detection Information of the        erroneous transaction."]
        #[inline(always)]
        pub fn addr_edc(
            self,
        ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, ErRx_SPEC, crate::common::R> {
            crate::common::RegisterField::<16,0xff,1,0,u8, ErRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "MCI Sideband Signals  7 0    MCI SBS. This bit field contains the MCI Sideband Signals  7 0  that are related        to the transaction information captured. In the AURIX  8482  TC3xx family  the sideband signals are used by the DMA to provide information        about the DMA requestor of a DMA transaction  for the encoding see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn mci_sbs(
            self,
        ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, ErRx_SPEC, crate::common::R> {
            crate::common::RegisterField::<24,0xff,1,0,u8, ErRx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for ErRx {
        #[inline(always)]
        fn default() -> ErRx {
            <crate::RegValueT<ErRx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PecoNx_SPEC;
    impl crate::sealed::RegSpec for PecoNx_SPEC {
        type DataType = u32;
    }
    #[doc = "Protocol Error Control Register 0\n resetvalue={Application Reset:0x1}"]
    pub type PecoNx = crate::RegValueT<PecoNx_SPEC>;

    impl PecoNx {
        #[doc = "Protocol Error Enable   PEEN"]
        #[inline(always)]
        pub fn peen(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, PecoNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,PecoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Set Protocol Error   SETPE. This allows SW to mimic a protocol error and present an indication        similar to a hardware detected error. After setting this bit  it is        automatically cleared by the hardware in the cycle after the write."]
        #[inline(always)]
        pub fn setpe(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, PecoNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,PecoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Protocol Error Acknowledge   PEACK. Writing a one to this bit while it  x2019 s set has the following results  The error lock of the corresponding ERRADDRx and ERRx are released  and the registers will be updated for the next protocol error detected  see CROSSREFERENCE  . After setting this bit  it is automatically cleared by the hardware in the cycle after the write."]
        #[inline(always)]
        pub fn peack(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, PecoNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,PecoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for PecoNx {
        #[inline(always)]
        fn default() -> PecoNx {
            <crate::RegValueT<PecoNx_SPEC> as RegisterValue<_>>::new(1)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PrioritYx_SPEC;
    impl crate::sealed::RegSpec for PrioritYx_SPEC {
        type DataType = u32;
    }
    #[doc = "SCI0 Arbiter Priority Register\n resetvalue={Application Reset:0x70000}"]
    pub type PrioritYx = crate::RegValueT<PrioritYx_SPEC>;

    impl PrioritYx {
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci0_p(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, PrioritYx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,PrioritYx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci1_p(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, PrioritYx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,PrioritYx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci2_p(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, PrioritYx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,PrioritYx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci3_p(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, PrioritYx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,PrioritYx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci4_p(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, PrioritYx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,PrioritYx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci5_p(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, PrioritYx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,PrioritYx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci6_p(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, PrioritYx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,PrioritYx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci7_p(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, PrioritYx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,PrioritYx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci8_p(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, PrioritYx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,PrioritYx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci9_p(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, PrioritYx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9,1,0,PrioritYx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci10_p(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, PrioritYx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,PrioritYx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci11_p(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, PrioritYx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<11,1,0,PrioritYx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "High Priority Round Share   HPRS. Number of transactions to give to the high priority round robin before a        transaction from low priority round  when request saturated . This        number may not be less than the number of high priority MCI programmed        via. MCIn P."]
        #[inline(always)]
        pub fn hprs(
            self,
        ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, PrioritYx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x7,1,0,u8, PrioritYx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for PrioritYx {
        #[inline(always)]
        fn default() -> PrioritYx {
            <crate::RegValueT<PrioritYx_SPEC> as RegisterValue<_>>::new(458752)
        }
    }
}
