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
#[doc = r"HSCT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsct0(pub(super) *mut u8);
unsafe impl core::marker::Send for Hsct0 {}
unsafe impl core::marker::Sync for Hsct0 {}
impl Hsct0 {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(65532usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Configuration Physical Layer Register\n resetvalue={Application Reset:0x1F0000}"]
    #[inline(always)]
    pub const fn configphy(&self) -> crate::common::Reg<self::Configphy_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }

    #[doc = "Clear To Send Control Register\n resetvalue={Application Reset:0x1}"]
    #[inline(always)]
    pub const fn ctsctrl(&self) -> crate::common::Reg<self::Ctsctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Transmission Disable Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn disable(&self) -> crate::common::Reg<self::Disable_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0B4C002}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Interface Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ifctrl(&self) -> crate::common::Reg<self::Ifctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Interface Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ifstat(&self) -> crate::common::Reg<self::Ifstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }

    #[doc = "Initialization Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn init(&self) -> crate::common::Reg<self::Init_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "Interrupt register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn irq(&self) -> crate::common::Reg<self::Irq_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "Interrupt Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn irqclr(&self) -> crate::common::Reg<self::Irqclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }

    #[doc = "Interrupt Enable Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn irqen(&self) -> crate::common::Reg<self::Irqen_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }

    #[doc = "Reset Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(65524usize)) }
    }

    #[doc = "Reset Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(65520usize)) }
    }

    #[doc = "Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(65516usize)) }
    }

    #[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(65512usize)) }
    }

    #[doc = "Sleep Control Register\n resetvalue={Application Reset:0x200000}"]
    #[inline(always)]
    pub const fn sleepctrl(&self) -> crate::common::Reg<self::Sleepctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn stat(&self) -> crate::common::Reg<self::Stat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "STATPHY\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn statphy(&self) -> crate::common::Reg<self::Statphy_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "Test Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn testctrl(&self) -> crate::common::Reg<self::Testctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }

    #[doc = "Unsolicited Status Message Received\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn usmr(&self) -> crate::common::Reg<self::Usmr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }

    #[doc = "Unsolicited Status Message Send\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn usms(&self) -> crate::common::Reg<self::Usms_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(84usize)) }
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
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
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
pub struct Clc_SPEC;
impl crate::sealed::RegSpec for Clc_SPEC {
    type DataType = u32;
}
#[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
pub type Clc = crate::RegValueT<Clc_SPEC>;

impl Clc {
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control  clock and local access  of the module. This bit switches the clock off immediately  therefore the software has to take care that the HSSL and HSCT communication has been terminated properly before setting this bit. The disable is performed after an acknowledge signal from the core logic is received. The acknowledge signal is directly looped back from the request signal."]
    #[inline(always)]
    pub fn disr(self) -> crate::common::RegisterFieldBool<0, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Disable Status Bit   DISS. Bit indicates the current status of the module and is set after the requested disable is active."]
    #[inline(always)]
    pub fn diss(self) -> crate::common::RegisterFieldBool<1, 1, 0, Clc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Clc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Chip System Sleep Mode Control   EDIS. The EDIS bit in the CLC register controls whether or not a module is stopped during Chip System initiated Sleep Mode. If EDIS is 0  a Sleep Mode request can be recognized by the module and  when received  its clock is immediately shut off. Therefore the software has to take care that the HSSL and HSCT communication has been terminated properly before activating the sleep mode request. If EDIS is set to 1  a Sleep Mode request is disregarded by the module and the module continues its operation. Note  This chip system sleep mode has nothing to do with the HSCT protocol sleep mode."]
    #[inline(always)]
    pub fn edis(self) -> crate::common::RegisterFieldBool<3, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Clc {
    #[inline(always)]
    fn default() -> Clc {
        <crate::RegValueT<Clc_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Configphy_SPEC;
impl crate::sealed::RegSpec for Configphy_SPEC {
    type DataType = u32;
}
#[doc = "Configuration Physical Layer Register\n resetvalue={Application Reset:0x1F0000}"]
pub type Configphy = crate::RegValueT<Configphy_SPEC>;

impl Configphy {
    #[doc = "Physical Layer Power On.   PON"]
    #[inline(always)]
    pub fn pon(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Configphy_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Configphy_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Correlator phase enable   allows to enable disable each of the 5 Phase outputs separately.   CORCEN"]
    #[inline(always)]
    pub fn corcen(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Configphy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Configphy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Configphy {
    #[inline(always)]
    fn default() -> Configphy {
        <crate::RegValueT<Configphy_SPEC> as RegisterValue<_>>::new(2031616)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsctrl_SPEC;
impl crate::sealed::RegSpec for Ctsctrl_SPEC {
    type DataType = u32;
}
#[doc = "Clear To Send Control Register\n resetvalue={Application Reset:0x1}"]
pub type Ctsctrl = crate::RegValueT<Ctsctrl_SPEC>;

impl Ctsctrl {
    #[doc = "Transmit CTS Frame Generation   CTS FRAME. Dedicated CTS frames are generated after the receive data path is not        able to accept more data. The situation is indicated by the CTS header        bit in case there is currently no data to be transferred."]
    #[inline(always)]
    pub fn cts_frame(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Ctsctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ctsctrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable TX CTS signaling   CTS TXD. If this bit is set to 1  CTS signaling is not performed at the interface        and the status remains at the clear to send for every frame send."]
    #[inline(always)]
    pub fn cts_txd(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Ctsctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ctsctrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable RX CTS detection   CTS RXD. If this bit is set to 1  CTS detection is not performed at the receiver        and the status remains internally at clear to send for every frame        received."]
    #[inline(always)]
    pub fn cts_rxd(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Ctsctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ctsctrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable HSSL interface CTS Frame Blocking   HSSL CTS FBD. If this bit is set to 1  CTS signaling is not performed at the interface        and the status remains at the clear to send for every frame send."]
    #[inline(always)]
    pub fn hssl_cts_fbd(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Ctsctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ctsctrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ctsctrl {
    #[inline(always)]
    fn default() -> Ctsctrl {
        <crate::RegValueT<Ctsctrl_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Disable_SPEC;
impl crate::sealed::RegSpec for Disable_SPEC {
    type DataType = u32;
}
#[doc = "Transmission Disable Register\n resetvalue={Application Reset:0x3}"]
pub type Disable = crate::RegValueT<Disable_SPEC>;

impl Disable {
    #[doc = "Disable HSCT Transmit path in Master interface   TX DIS. Disable the transmit path of the HSCT interface. If this bit is set to 1          no transfer can be initiated and the LVDS driver is disabled."]
    #[inline(always)]
    pub fn tx_dis(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Disable_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Disable_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable HSCT Receive path in Master interface   RX DIS. Disable the receive line path of the HSCT interface. If this bit is set        to 1   no transfer from the other side can be received and the Master RX        path is in a low power state. This feature is only available in the Master interface. Slave interface        RX path can not be disabled and a write to the register has no effect."]
    #[inline(always)]
    pub fn rx_dis(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Disable_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Disable_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable RX Header Error Discard Payload data.   RX HEPD. Instead of discarding the Payload data at a header error the payload        data is passed to the higher layer  HSSL . Only channel data to HSSL is        affected. This function is available in Master and in Slave mode."]
    #[inline(always)]
    pub fn rx_hepd(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Disable_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Disable_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Disable {
    #[inline(always)]
    fn default() -> Disable {
        <crate::RegValueT<Disable_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0B4C002}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MOD REV"]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Type   MOD TYPE"]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number for module identification   MOD NUM"]
    #[inline(always)]
    pub fn mod_num(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Id_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Id {
    #[inline(always)]
    fn default() -> Id {
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(11845634)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifctrl_SPEC;
impl crate::sealed::RegSpec for Ifctrl_SPEC {
    type DataType = u32;
}
#[doc = "Interface Control Register\n resetvalue={Application Reset:0x0}"]
pub type Ifctrl = crate::RegValueT<Ifctrl_SPEC>;

impl Ifctrl {
    #[doc = "Master Mode   Trigger for Interface Control Value to be send to Slave interface   IFCVS. See the table  quot Interface Control Payload Values quot . Master IF Mode  The value is taken as control frame value send as        payload to the Slave IF. Slave IF Mode  The value is a new configuration of the Slave IF  not        recommended flow    ."]
    #[inline(always)]
    pub fn ifcvs(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Ifctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Ifctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Master Mode   Slave IF control frame trigger   SIFCV. Changing the interface configuration  software must guarantee not having        transfers active on the interface. This register bit always reads back zero."]
    #[inline(always)]
    pub fn sifcv(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ifctrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ifctrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Master Mode RX speed   MRXSPEED. Register setting only valid in interface Master mode."]
    #[inline(always)]
    pub fn mrxspeed(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Ifctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Ifctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Master Mode TX speed   MTXSPEED. Register setting only valid in interface Master mode."]
    #[inline(always)]
    pub fn mtxspeed(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, Ifctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, Ifctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interface TX Test Mode   IFTESTMD"]
    #[inline(always)]
    pub fn iftestmd(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Ifctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ifctrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ifctrl {
    #[inline(always)]
    fn default() -> Ifctrl {
        <crate::RegValueT<Ifctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifstat_SPEC;
impl crate::sealed::RegSpec for Ifstat_SPEC {
    type DataType = u32;
}
#[doc = "Interface Status Register\n resetvalue={Application Reset:0x0}"]
pub type Ifstat = crate::RegValueT<Ifstat_SPEC>;

impl Ifstat {
    #[doc = "HSCT Slave interface Status for RX link   RX STAT. Slave interface transmitter only"]
    #[inline(always)]
    pub fn rx_stat(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Ifstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Ifstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "HSCT Slave interface Status for TX link   TX STAT. Slave interface receiver only"]
    #[inline(always)]
    pub fn tx_stat(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, Ifstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x3,1,0,u8, Ifstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "HSCT LVDS Slave interface TX enable   TX EN. Slave interface only"]
    #[inline(always)]
    pub fn tx_en(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ifstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ifstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ifstat {
    #[inline(always)]
    fn default() -> Ifstat {
        <crate::RegValueT<Ifstat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Init_SPEC;
impl crate::sealed::RegSpec for Init_SPEC {
    type DataType = u32;
}
#[doc = "Initialization Register\n resetvalue={Application Reset:0x0}"]
pub type Init = crate::RegValueT<Init_SPEC>;

impl Init {
    #[doc = "Enable HSCT SysClk in Master interface   SYS CLK EN. SysClk enable activates the SysClk. This feature is only available in the Master interface. In Slave interface mode the register setting does not have an effect."]
    #[inline(always)]
    pub fn sys_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Init_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Init_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Select Interface Mode   IFM. Select the Interface Mode  Master IF or Slave IF ."]
    #[inline(always)]
    pub fn ifm(self) -> crate::common::RegisterFieldBool<3, 1, 0, Init_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Init_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Select Reference Clock Frequency Divider   SRCF. Defines physical layer reference frequency  PHY CLK  and respective        input frequency divider. The configuration is valid for Low speed and        Medium Speed mode."]
    #[inline(always)]
    pub fn srcf(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Init_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Init_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select SysClk Frequency Divider   SSCF. For master interface defines SysClk pad output frequency. The allowed        SysClk frequency is 10MHz or 20MHz."]
    #[inline(always)]
    pub fn sscf(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Init_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Init_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit High Speed Divider.   TXHD. The Transmit High Speed data rate can be reduced by dividing factors.        The Transmit High Speed data rate is separated from the Receive High        Speed data rate."]
    #[inline(always)]
    pub fn txhd(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Init_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Init_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive High Speed Divider.   RXHD. The Receive High Speed data rate can be reduced by dividing factors. The Receive High Speed data rate is separated from the Transmit High Speed data rate. For future use configuration leads to no output clock  pll phase o lt 4 0 gt   delivered to the correlator."]
    #[inline(always)]
    pub fn rxhd(
        self,
    ) -> crate::common::RegisterField<19, 0x7, 1, 0, u8, Init_SPEC, crate::common::RW> {
        crate::common::RegisterField::<19,0x7,1,0,u8, Init_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Init {
    #[inline(always)]
    fn default() -> Init {
        <crate::RegValueT<Init_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irq_SPEC;
impl crate::sealed::RegSpec for Irq_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt register\n resetvalue={Application Reset:0x0}"]
pub type Irq = crate::RegValueT<Irq_SPEC>;

impl Irq {
    #[doc = "Header error detected   HER. Not supported size  Received command at Slave interface 8 bit size only   other command          sizes generate an error. Received command ping answer at Master interface 32 bit size only            other command sizes generate an error Unsolicited data 32 bit only Logical data channel size 8 bit Not supported logical channel type  0b1xxx 0010  Slave interface control and Slave interface read"]
    #[inline(always)]
    pub fn her(self) -> crate::common::RegisterFieldBool<1, 1, 0, Irq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Irq_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Payload error detected   PYER. Payload does not fit the header size"]
    #[inline(always)]
    pub fn pyer(self) -> crate::common::RegisterFieldBool<2, 1, 0, Irq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Irq_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "HSCT command error   CER. Control command not valid or control payload size bigger than 8 bit.        Single Slave specific commands received in Multi Slave Mode do not        trigger CER error  but only MSCE error."]
    #[inline(always)]
    pub fn cer(self) -> crate::common::RegisterFieldBool<3, 1, 0, Irq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Irq_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "HSCT interface control frame send   IFCFS. The scheduled interface control command is send."]
    #[inline(always)]
    pub fn ifcfs(self) -> crate::common::RegisterFieldBool<4, 1, 0, Irq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Irq_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Speed Mode Switch Error  Master Mode only    SMER. Speed mode change did not work. Received PING payload not valid."]
    #[inline(always)]
    pub fn smer(self) -> crate::common::RegisterFieldBool<5, 1, 0, Irq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Irq_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Unsolicited message frame send finished   USMSF. Interrupt is indicated after the unsolicited message send is finished."]
    #[inline(always)]
    pub fn usmsf(self) -> crate::common::RegisterFieldBool<6, 1, 0, Irq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Irq_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PLL lost lock error   PLER. After the PLL locked  the PLL may loose lock  which is reflected by the        error"]
    #[inline(always)]
    pub fn pler(self) -> crate::common::RegisterFieldBool<7, 1, 0, Irq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Irq_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Unsolicited Message Received   USM. Unsolicited message received indication. Unsolicited message indicates a        system event to the other interface side."]
    #[inline(always)]
    pub fn usm(self) -> crate::common::RegisterFieldBool<8, 1, 0, Irq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Irq_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PING Answer Received   PAR. The received message was identified as PING."]
    #[inline(always)]
    pub fn par(self) -> crate::common::RegisterFieldBool<9, 1, 0, Irq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Irq_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TX transfer error occurred on a disabled TX channel.   TXTE. A disabled TX triggers an error interrupt  if  TX disabled on a pending or active data transfer. TX CTS configuration change on a active CTS frame."]
    #[inline(always)]
    pub fn txte(self) -> crate::common::RegisterFieldBool<10, 1, 0, Irq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Irq_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Synchronization FIFO overflow  in RX direction    SFO. Physical layer to Controller data synchronization FIFO in RX transfer        direction hit an overflow situation. This interrupt is an indication about a to slow SRI clock compared to          Physical layer clock  which results in a overflow situation.  Minimum          SRI frequency 40 MHz."]
    #[inline(always)]
    pub fn sfo(self) -> crate::common::RegisterFieldBool<11, 1, 0, Irq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Irq_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Synchronization FIFO underflow  in TX direction    SFU. Controller to Physical layer data synchronization FIFO in TX transfer        direction hit an underflow situation. This interrupt is an indication about a to slow SRI clock compared to          Physical layer clock  which results in a underflow situation.  Minimum          SRI frequency 40 MHz."]
    #[inline(always)]
    pub fn sfu(self) -> crate::common::RegisterFieldBool<12, 1, 0, Irq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Irq_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Multi Slave scenario Command Error   MSCE. This interrupt indicates a control command which is not allowed in multi        Slave scenario. In Master and Slave mode a not allowed command results        to an error."]
    #[inline(always)]
    pub fn msce(self) -> crate::common::RegisterFieldBool<13, 1, 0, Irq_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Irq_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Irq {
    #[inline(always)]
    fn default() -> Irq {
        <crate::RegValueT<Irq_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqclr_SPEC;
impl crate::sealed::RegSpec for Irqclr_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Clear Register\n resetvalue={Application Reset:0x0}"]
pub type Irqclr = crate::RegValueT<Irqclr_SPEC>;

impl Irqclr {
    #[doc = "Header error detected interrupt clear   HERCLR"]
    #[inline(always)]
    pub fn herclr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Payload error detected interrupt clear   PYERCLR"]
    #[inline(always)]
    pub fn pyerclr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "HSCT command error interrupt clear   CERCLR"]
    #[inline(always)]
    pub fn cerclr(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "HSCT interface control command send interrupt clear   IFCFSCLR"]
    #[inline(always)]
    pub fn ifcfsclr(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Speed Mode Switch Error interrupt clear   SMERCLR"]
    #[inline(always)]
    pub fn smerclr(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Unsolicited message frame send finished interrupt clear   USMSFCLR"]
    #[inline(always)]
    pub fn usmsfclr(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "PLL lost lock error interrupt clear   PLERCLR"]
    #[inline(always)]
    pub fn plerclr(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Unsolicited Message received clear   USMCLR"]
    #[inline(always)]
    pub fn usmclr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "PING Answer received clear   PARCLR"]
    #[inline(always)]
    pub fn parclr(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "TX disable error interrupt clear   TXTECLR"]
    #[inline(always)]
    pub fn txteclr(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Synchronization FIFO overflow  in RX direction  interrupt clear   SFOCLR"]
    #[inline(always)]
    pub fn sfoclr(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Synchronization FIFO underflow  in TX direction  interrupt clear   SFUCLR"]
    #[inline(always)]
    pub fn sfuclr(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Multi Slave scenario Command Error interrupt clear   MSCELR"]
    #[inline(always)]
    pub fn mscelr(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Irqclr {
    #[inline(always)]
    fn default() -> Irqclr {
        <crate::RegValueT<Irqclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqen_SPEC;
impl crate::sealed::RegSpec for Irqen_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Enable Register\n resetvalue={Application Reset:0x0}"]
pub type Irqen = crate::RegValueT<Irqen_SPEC>;

impl Irqen {
    #[doc = "Header error detected interrupt enable   HEREN"]
    #[inline(always)]
    pub fn heren(self) -> crate::common::RegisterFieldBool<1, 1, 0, Irqen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Irqen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Payload error detected interrupt enable   PYEREN"]
    #[inline(always)]
    pub fn pyeren(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Irqen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Irqen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "HSCT command error interrupt enable   CEREN"]
    #[inline(always)]
    pub fn ceren(self) -> crate::common::RegisterFieldBool<3, 1, 0, Irqen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Irqen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "HSCT interface control command send enable   IFCFSEN"]
    #[inline(always)]
    pub fn ifcfsen(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Irqen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Irqen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Speed Mode Switch Error interrupt enable   SMEREN"]
    #[inline(always)]
    pub fn smeren(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Irqen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Irqen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Unsolicited message frame send finished   USMSFEN"]
    #[inline(always)]
    pub fn usmsfen(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Irqen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Irqen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PLL lost lock error interrupt enable   PLEREN"]
    #[inline(always)]
    pub fn pleren(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Irqen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Irqen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Unsolicited Message received enable   USMEN"]
    #[inline(always)]
    pub fn usmen(self) -> crate::common::RegisterFieldBool<8, 1, 0, Irqen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Irqen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PING Answer Received enable   PAREN"]
    #[inline(always)]
    pub fn paren(self) -> crate::common::RegisterFieldBool<9, 1, 0, Irqen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Irqen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "TX disable error interrupt enable   TXTEEN"]
    #[inline(always)]
    pub fn txteen(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Irqen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Irqen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Synchronization FIFO overflow  in RX direction  interrupt enable   SFOEN"]
    #[inline(always)]
    pub fn sfoen(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Irqen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Irqen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Synchronization FIFO underflow  in TX direction  interrupt enable   SFUEN"]
    #[inline(always)]
    pub fn sfuen(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Irqen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Irqen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Multi Slave scenario Command Error interrupt enable   MSCEEN"]
    #[inline(always)]
    pub fn msceen(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Irqen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Irqen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Irqen {
    #[inline(always)]
    fn default() -> Irqen {
        <crate::RegValueT<Irqen_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krst0_SPEC;
impl crate::sealed::RegSpec for Krst0_SPEC {
    type DataType = u32;
}
#[doc = "Reset Register 0\n resetvalue={Application Reset:0x0}"]
pub type Krst0 = crate::RegValueT<Krst0_SPEC>;

impl Krst0 {
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel        reset will be executed if the reset bits of both kernel registers are        set. The RST bit will be cleared  re set to   180 0  180   by the BPI FPI after the        kernel reset was executed. It is strongly recommended to reset the HSCT and HSSL in sequence to avoid communication issues."]
    #[inline(always)]
    pub fn rst(self) -> crate::common::RegisterFieldBool<0, 1, 0, Krst0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Krst0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Kernel Reset Status   RSTSTAT. This bit indicates wether a kernel reset was executed or not. This bit        is set by the BPI FPI after the execution of a kernel reset in the same        clock cycle both reset bits. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related KRSTCLR register."]
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
#[doc = "Reset Register 1\n resetvalue={Application Reset:0x0}"]
pub type Krst1 = crate::RegValueT<Krst1_SPEC>;

impl Krst1 {
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel        reset will be executed if the reset bits of both kernel reset registers        is set. The RST bit will be cleared  re set to   180 0  180   by the BPI FPI after the        kernel reset was executed. It is strongly recommended to reset the HSCT and HSSL in sequence to avoid communication issues."]
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
#[doc = "Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
pub type Krstclr = crate::RegValueT<Krstclr_SPEC>;

impl Krstclr {
    #[doc = "Kernel Reset Status Clear   CLR. Read always as 0."]
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
pub struct Ocs_SPEC;
impl crate::sealed::RegSpec for Ocs_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0}"]
pub type Ocs = crate::RegValueT<Ocs_SPEC>;

impl Ocs {
    #[doc = "Trigger Set for OTGB0 1   TGS. Others  160   160 reserved  no Trigger Set selected"]
    #[inline(always)]
    pub fn tgs(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Ocs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0x3, 1, 0, u8, Ocs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "OTGB0 1 Bus Select   TGB"]
    #[inline(always)]
    pub fn tgb(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ocs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ocs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "TGS  TGB Write Protection   TG P. TGS and TGB are only written when TG P is 1  otherwise unchanged. Read        as 0."]
    #[inline(always)]
    pub fn tg_p(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ocs_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ocs_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "OCDS Suspend Control   SUS. Controls the sensitivity to the suspend signal coming from the OCDS        Trigger Switch  OTGS  Others  160   160 reserved After a Hard suspend the HSCT and higher layer protocol module must be          reset. A a new initialization sequence is required afterwards"]
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
pub struct Sleepctrl_SPEC;
impl crate::sealed::RegSpec for Sleepctrl_SPEC {
    type DataType = u32;
}
#[doc = "Sleep Control Register\n resetvalue={Application Reset:0x200000}"]
pub type Sleepctrl = crate::RegValueT<Sleepctrl_SPEC>;

impl Sleepctrl {
    #[doc = "Sleep mode enabled   SLPEN. Sleep mode is enabled and performed after receiving a 1 at the end of a        received frame or in transmission direction  if no more data to be send."]
    #[inline(always)]
    pub fn slpen(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Sleepctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Sleepctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Gating in Sleep Mode   SLPCLKG. In sleep mode the clock for HSCT  framer  deframer  can be gated in        order to minimize power consumption. Clock gating  Receiving path and transmitting path is separated."]
    #[inline(always)]
    pub fn slpclkg(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Sleepctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Sleepctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Counter Value for Determining the Wake up Time of the LVDS Line Driver   WKUPCNT. This counter value corresponds to wake up time the LVDS requires from        sleep to wake up. Counter is clocked by SRI clock."]
    #[inline(always)]
    pub fn wkupcnt(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Sleepctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Sleepctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Sleepctrl {
    #[inline(always)]
    fn default() -> Sleepctrl {
        <crate::RegValueT<Sleepctrl_SPEC> as RegisterValue<_>>::new(2097152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat_SPEC;
impl crate::sealed::RegSpec for Stat_SPEC {
    type DataType = u32;
}
#[doc = "Status Register\n resetvalue={Application Reset:0x0}"]
pub type Stat = crate::RegValueT<Stat_SPEC>;

impl Stat {
    #[doc = "RX  Receiving  Payload Size   RX PSIZE. Contains the payload size of the last received frame."]
    #[inline(always)]
    pub fn rx_psize(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Stat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0x7, 1, 0, u8, Stat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RX  Receiving  Logical Channel Type   RX CHANNEL. Contains the logical channel type of the last received frame. See Table         quot Logical Channel Type Coding quot ."]
    #[inline(always)]
    pub fn rx_channel(
        self,
    ) -> crate::common::RegisterField<3, 0xf, 1, 0, u8, Stat_SPEC, crate::common::R> {
        crate::common::RegisterField::<3, 0xf, 1, 0, u8, Stat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RX  Receiving  Sleep Mode Status   RX SLEEP"]
    #[inline(always)]
    pub fn rx_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Stat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Stat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "TX  Transmission  Sleep Mode Status   TX SLEEP"]
    #[inline(always)]
    pub fn tx_sleep(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Stat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Stat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Payload Size   TX PSIZE. Coding of the logical channel type is according to the Table  Payload        Size Coding. This value was used in the logical channel type field in        the header for the actual transfer in transmit direction."]
    #[inline(always)]
    pub fn tx_psize(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, Stat_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x7,1,0,u8, Stat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmission Logical Channel Type   TX CHANNEL TYPE. Coding of the logical channel type is according to the Table  Logical        Channel Type Coding. This value was used in the logical channel type        field in the header for the actual transfer in transmit direction."]
    #[inline(always)]
    pub fn tx_channel_type(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Stat_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Stat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Last Interface Control Command Received   LIFCCMDR. The bit value reflects the last control command received. The bit is        active in Slave interface mode only. In Master mode it reflects logic 0        always.  See CROSSREFERENCE"]
    #[inline(always)]
    pub fn lifccmdr(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Stat_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Stat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        <crate::RegValueT<Stat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Statphy_SPEC;
impl crate::sealed::RegSpec for Statphy_SPEC {
    type DataType = u32;
}
#[doc = "STATPHY\n resetvalue={Application Reset:0x0}"]
pub type Statphy = crate::RegValueT<Statphy_SPEC>;

impl Statphy {
    #[doc = "PLL locked   PLOCK"]
    #[inline(always)]
    pub fn plock(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Statphy_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Statphy_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmitter speed   TXSA"]
    #[inline(always)]
    pub fn txsa(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Statphy_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Statphy_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receiver speed   RXSA"]
    #[inline(always)]
    pub fn rxsa(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Statphy_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Statphy_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Statphy {
    #[inline(always)]
    fn default() -> Statphy {
        <crate::RegValueT<Statphy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Testctrl_SPEC;
impl crate::sealed::RegSpec for Testctrl_SPEC {
    type DataType = u32;
}
#[doc = "Test Control Register\n resetvalue={Application Reset:0x0}"]
pub type Testctrl = crate::RegValueT<Testctrl_SPEC>;

impl Testctrl {
    #[doc = "Enable Slave TX path  Slave interface mode only    TXENS. This function should be only used during interface testing the mode          and SW development. In functional mode the Slave interface should only          be controlled via transfer commands received from Master interface.           This trigger register reads back 0 always.  If TXENS and TXDISS are          written one  TXDISS has higher priority. The status is visible in          IFSTAT.TX EN."]
    #[inline(always)]
    pub fn txens(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Testctrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Testctrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable Slave TX path  Slave Interface mode only    TXDISS. This function should be only used during interface testing the mode          and SW development. In functional mode the Slave interface should only          be controlled via transfer commands received from Master interface.           This trigger register reads back 0 always.  If TXENS and TXDISS are          written one  TXDISS has higher priority. The status is visible in          IFSTAT.TX EN."]
    #[inline(always)]
    pub fn txdiss(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Testctrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Testctrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "LVDS loop back TX to RX enable   LLOPTXRX. Transmit data at LVDS is directly looped back to the LVDS RX. Data        transfer speed is defined by the TX speed configuration.  The data path        in the SoC is using the complete Transmit path through all functional        layers and is looped back at LVDS from TX to RX. Also at RX data path        all SoC data layers are active.  Requires same speed configuration for RX  and TX link before enabled."]
    #[inline(always)]
    pub fn lloptxrx(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Testctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Testctrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PRBS Pattern enable   PRBSEN. Enable of the PRBSEN bit allows a continuos PRBS stream with the        configured transfer speed Baud rate. This feature is available to        measure ISI during the time other SoC functions are running in an        applicative mode. This feature is for measurement purpose only."]
    #[inline(always)]
    pub fn prbsen(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Testctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Testctrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Testctrl {
    #[inline(always)]
    fn default() -> Testctrl {
        <crate::RegValueT<Testctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usmr_SPEC;
impl crate::sealed::RegSpec for Usmr_SPEC {
    type DataType = u32;
}
#[doc = "Unsolicited Status Message Received\n resetvalue={Application Reset:0x0}"]
pub type Usmr = crate::RegValueT<Usmr_SPEC>;

impl Usmr {
    #[doc = "Unsolicited status message received   USMR. The register contains the last received unsolicited status message."]
    #[inline(always)]
    pub fn usmr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Usmr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Usmr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Usmr {
    #[inline(always)]
    fn default() -> Usmr {
        <crate::RegValueT<Usmr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usms_SPEC;
impl crate::sealed::RegSpec for Usms_SPEC {
    type DataType = u32;
}
#[doc = "Unsolicited Status Message Send\n resetvalue={Application Reset:0x0}"]
pub type Usms = crate::RegValueT<Usms_SPEC>;

impl Usms {
    #[doc = "Unsolicited status message send   USMS. Writing to the register triggers an unsolicited status message to be        send to the other interface side."]
    #[inline(always)]
    pub fn usms(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Usms_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Usms_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Usms {
    #[inline(always)]
    fn default() -> Usms {
        <crate::RegValueT<Usms_SPEC> as RegisterValue<_>>::new(0)
    }
}
