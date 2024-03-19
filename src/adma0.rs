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
#[doc = r"ADMA"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adma0(pub(super) *mut u8);
unsafe impl core::marker::Send for Adma0 {}
unsafe impl core::marker::Sync for Adma0 {}
impl Adma0 {
    #[doc = "Checksum Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn chksum(&self) -> crate::common::Reg<self::Chksum_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Counter Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cnt(&self) -> crate::common::Reg<self::Cnt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ctrl(&self) -> crate::common::Reg<self::Ctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Destination Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dadr(&self) -> crate::common::Reg<self::Dadr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Error Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn errsr(&self) -> crate::common::Reg<self::Errsr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }

    #[doc = "Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }

    #[doc = "Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0D1C005}"]
    #[inline(always)]
    pub const fn modid(&self) -> crate::common::Reg<self::Modid_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "OCDS Control and Status Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }

    #[doc = "OCDS Debug Access Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn oda(&self) -> crate::common::Reg<self::Oda_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(76usize)) }
    }

    #[doc = "Source Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sadr(&self) -> crate::common::Reg<self::Sadr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chksum_SPEC;
impl crate::sealed::RegSpec for Chksum_SPEC {
    type DataType = u32;
}
#[doc = "Checksum Register\n resetvalue={Application Reset:0x0}"]
pub type Chksum = crate::RegValueT<Chksum_SPEC>;

impl Chksum {
    #[doc = "DATA Checksum   CHKSUM. Hash value calculated from all data transferred by the ADMA since the register was last initialised."]
    #[inline(always)]
    pub fn chksum(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Chksum_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Chksum_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Chksum {
    #[inline(always)]
    fn default() -> Chksum {
        <crate::RegValueT<Chksum_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clc_SPEC;
impl crate::sealed::RegSpec for Clc_SPEC {
    type DataType = u32;
}
#[doc = "Clock Control Register\n resetvalue={Application Reset:0x0}"]
pub type Clc = crate::RegValueT<Clc_SPEC>;

impl Clc {
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control of the module."]
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
}
impl core::default::Default for Clc {
    #[inline(always)]
    fn default() -> Clc {
        <crate::RegValueT<Clc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cnt_SPEC;
impl crate::sealed::RegSpec for Cnt_SPEC {
    type DataType = u32;
}
#[doc = "Counter Register\n resetvalue={Application Reset:0x0}"]
pub type Cnt = crate::RegValueT<Cnt_SPEC>;

impl Cnt {
    #[doc = "Byte Counter   CNT. This register contains the number of bytes that the DMA engine is about to transfer. As the value contained by this register must be multiples of 8  bits 2 to 0 must be written with 0 and will always read as 0 . A value of 1 written to any of these bits will be ignored. As the destination for a transfer must always be within the AMU RAM  the count cannot exceed the size of the RAM. Unneeded bits will be silently ignored. For a 32 KiBAMU SRAM  bits 31 to 16 of the register are unneeded and will be silently ignored when written to and will always read 0 . For a 64 KiB AMU SRAM  bits 31 to 17 are unneeded and will be silently ignored when written and will always read 0 ."]
    #[inline(always)]
    pub fn cnt(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Cnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Cnt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Cnt {
    #[inline(always)]
    fn default() -> Cnt {
        <crate::RegValueT<Cnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl_SPEC;
impl crate::sealed::RegSpec for Ctrl_SPEC {
    type DataType = u32;
}
#[doc = "Control Register\n resetvalue={Application Reset:0x0}"]
pub type Ctrl = crate::RegValueT<Ctrl_SPEC>;

impl Ctrl {
    #[doc = "Enable Bit   EN. Turns on   off the engine. Will be cleared when the transfer was stopped or finished. The flag is set while the transfer is running."]
    #[inline(always)]
    pub fn en(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ctrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Condition Bit   ENCN. If the flag is 0  a change of EN bit is ignored.Read always as 0"]
    #[inline(always)]
    pub fn encn(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ctrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Ready Flag   READY. This bit is set by hardware upon successful completion of the DMA transfer."]
    #[inline(always)]
    pub fn ready(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ctrl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ctrl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Interrupt   ERR. This flag signifies that an error during the DMA transfer has occurred. An interrupt is raised."]
    #[inline(always)]
    pub fn err(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ctrl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ctrl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ECC Error Mode Bit   ECC STOP. When set to 1   a running DMA transfer will be stopped if an ECC error is detected in data read from system memory. When set to 0   the error will still be flagged but the DMA transaction will continue. This type of error is identified by the SRI ECC bit in the ERRSR register."]
    #[inline(always)]
    pub fn ecc_stop(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Ctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ctrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Checksum Enable Mode Bit   CHKEN. When set to 1 a cumulative hash value computed from all the data transferred by the ADMA will be stored in the CHKSUM register."]
    #[inline(always)]
    pub fn chken(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ctrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        <crate::RegValueT<Ctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dadr_SPEC;
impl crate::sealed::RegSpec for Dadr_SPEC {
    type DataType = u32;
}
#[doc = "Destination Address Register\n resetvalue={Application Reset:0x0}"]
pub type Dadr = crate::RegValueT<Dadr_SPEC>;

impl Dadr {
    #[doc = "Destination Address Register  Pointer    DADR. This register contains the destination address of the DMA transfer. This is the address to which the DMA engine writes that data it transfers. As the value contained by this register must be aligned to an 8 byte boundary  bits 2 to 0 must be written with 0 and will always read 0 . A value of 1 written to any of these bits will be ignored. As the destination address must always be within the AMU RAM  the high order bits not needed to access the AMU memory will cause a memory error if not set to the base address of the AMU RAM. e.g. for a 32 KiB AMU SRAM  bits 31 to 15 of the register are the base address and for a 64 KiB AMU SRAM bits 31 to 16 of the register are the base address."]
    #[inline(always)]
    pub fn dadr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Dadr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Dadr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dadr {
    #[inline(always)]
    fn default() -> Dadr {
        <crate::RegValueT<Dadr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errsr_SPEC;
impl crate::sealed::RegSpec for Errsr_SPEC {
    type DataType = u32;
}
#[doc = "Error Status Register\n resetvalue={Application Reset:0x0}"]
pub type Errsr = crate::RegValueT<Errsr_SPEC>;

impl Errsr {
    #[doc = "Memory Error   ME. This error flag is generated by the DMA move engine. An error has occurred while writing to the memory."]
    #[inline(always)]
    pub fn me(self) -> crate::common::RegisterFieldBool<0, 1, 0, Errsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Errsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "System Error   SE. This error flag is generated by the DMA move engine. An error has occurred while reading data from system memory"]
    #[inline(always)]
    pub fn se(self) -> crate::common::RegisterFieldBool<1, 1, 0, Errsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Errsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "System Memory ECC Error   SRI ECC. This error flag is generated by the DMA move engine. An ECC error has been detected in data read from system memory."]
    #[inline(always)]
    pub fn sri_ecc(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Errsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Errsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Errsr {
    #[inline(always)]
    fn default() -> Errsr {
        <crate::RegValueT<Errsr_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel registers are set. The RST bit will be cleared  reset to 0   after the kernel reset is executed."]
    #[inline(always)]
    pub fn rst(self) -> crate::common::RegisterFieldBool<0, 1, 0, Krst0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Krst0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Kernel Reset Status   RSTSTAT. This bit indicates whether a kernel reset was executed or not. This bit is set after the execution of a kernel reset in the same clock cycle both reset bits are cleared  ADMA KRST0.RST and ADMA KRST1.RST . This bit can be cleared by writing with  1  to the CLR bit in the related KRSTCLR register."]
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel reset registers is set. The RST bit will be cleared  re set to 0   after the kernel reset was executed."]
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
    #[doc = "Kernel Reset Status Clear   CLR. Read always as 0 ."]
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
pub struct Modid_SPEC;
impl crate::sealed::RegSpec for Modid_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0D1C005}"]
pub type Modid = crate::RegValueT<Modid_SPEC>;

impl Modid {
    #[doc = "Module Revision Number   MOD REV. This bit field defines the module revision number. The value of a module revision starts with 01H  first revision ."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Modid_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Modid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Module Type   MOD TYPE. The bit field is set to C0H which defines the module as a 32 bit module."]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Modid_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Modid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Module Number Value   MOD NUMBER. This bit field defines a module identification number. The value for the AMU is 00D1H."]
    #[inline(always)]
    pub fn mod_number(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Modid_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Modid_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Modid {
    #[inline(always)]
    fn default() -> Modid {
        <crate::RegValueT<Modid_SPEC> as RegisterValue<_>>::new(13746181)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ocs_SPEC;
impl crate::sealed::RegSpec for Ocs_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Control and Status Register\n resetvalue={Debug Reset:0x0}"]
pub type Ocs = crate::RegValueT<Ocs_SPEC>;

impl Ocs {
    #[doc = "OCDS Suspend Control   SUS. Controls the sensitivity to the suspend signal coming from the OCDS Trigger Switch  OTGS"]
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
pub struct Oda_SPEC;
impl crate::sealed::RegSpec for Oda_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Debug Access Register\n resetvalue={Debug Reset:0x0}"]
pub type Oda = crate::RegValueT<Oda_SPEC>;

impl Oda {
    #[doc = "Destructive Debug Read Enable   DDREN. For details see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ddren(self) -> crate::common::RegisterFieldBool<0, 1, 0, Oda_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Oda_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Destructive Read Enable   DREN. For details see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn dren(self) -> crate::common::RegisterFieldBool<1, 1, 0, Oda_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Oda_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Oda {
    #[inline(always)]
    fn default() -> Oda {
        <crate::RegValueT<Oda_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sadr_SPEC;
impl crate::sealed::RegSpec for Sadr_SPEC {
    type DataType = u32;
}
#[doc = "Source Address Register\n resetvalue={Application Reset:0x0}"]
pub type Sadr = crate::RegValueT<Sadr_SPEC>;

impl Sadr {
    #[doc = "Source Address Register  Pointer    SADR. This register contains the source address of the DMA transfer. This is the address from which the DMA engine reads data that is to be transferred. As the value contained by this register must be aligned to an 8 byte boundary  bits 2 to 0 must be written with 0 and will always read 0 . A value of 1 written to any of these bits will be ignored"]
    #[inline(always)]
    pub fn sadr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Sadr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Sadr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Sadr {
    #[inline(always)]
    fn default() -> Sadr {
        <crate::RegValueT<Sadr_SPEC> as RegisterValue<_>>::new(0)
    }
}
