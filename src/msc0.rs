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
#[doc = r"MSC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msc0(pub(super) *mut u8);
unsafe impl core::marker::Send for Msc0 {}
unsafe impl core::marker::Sync for Msc0 {}
impl Msc0 {
    #[doc = "Asynchronous Block Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn abc(&self) -> crate::common::Reg<self::Abc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Downstream Command Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dc(&self) -> crate::common::Reg<self::Dc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Downstream Command Extension Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dce(&self) -> crate::common::Reg<self::Dce_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(124usize)) }
    }

    #[doc = "Downstream Command Mirror Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dcm(&self) -> crate::common::Reg<self::Dcm_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(120usize)) }
    }

    #[doc = "Downstream Data Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dd(&self) -> crate::common::Reg<self::Dd_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Downstream Data Extension Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dde(&self) -> crate::common::Reg<self::Dde_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(116usize)) }
    }

    #[doc = "Downstream Data Mirror Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ddm(&self) -> crate::common::Reg<self::Ddm_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(112usize)) }
    }

    #[doc = "Downstream Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dsc(&self) -> crate::common::Reg<self::Dsc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Downstream Control Enhanced Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dsce(&self) -> crate::common::Reg<self::Dsce_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(88usize)) }
    }

    #[doc = "Downstream Select Data Source High Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dsdsh(&self) -> crate::common::Reg<self::Dsdsh_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }

    #[doc = "Downstream Select Data Source High Extension Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dsdshe(&self) -> crate::common::Reg<self::Dsdshe_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(100usize)) }
    }

    #[doc = "Downstream Select Data Source Low Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dsdsl(&self) -> crate::common::Reg<self::Dsdsl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "Downstream Select Data Source Low Extension Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dsdsle(&self) -> crate::common::Reg<self::Dsdsle_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }

    #[doc = "Downstream Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dss(&self) -> crate::common::Reg<self::Dss_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "Downstream Timing Extension Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dste(&self) -> crate::common::Reg<self::Dste_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(108usize)) }
    }

    #[doc = "Emergency Stop Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn esr(&self) -> crate::common::Reg<self::Esr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }

    #[doc = "Emergency Stop Extension Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn esre(&self) -> crate::common::Reg<self::Esre_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(104usize)) }
    }

    #[doc = "Fractional Divider Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fdr(&self) -> crate::common::Reg<self::Fdr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }

    #[doc = "Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn icr(&self) -> crate::common::Reg<self::Icr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x28C010}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Interrupt Set Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn isc(&self) -> crate::common::Reg<self::Isc_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }

    #[doc = "Interrupt Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn isr(&self) -> crate::common::Reg<self::Isr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }

    #[doc = "Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(244usize)) }
    }

    #[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(240usize)) }
    }

    #[doc = "Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(236usize)) }
    }

    #[doc = "Output Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ocr(&self) -> crate::common::Reg<self::Ocr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(76usize)) }
    }

    #[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(232usize)) }
    }

    #[doc = "Upstream Data Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn udx(&self) -> [crate::common::Reg<self::UDx_SPEC, crate::common::RW>; 4] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xcusize)),
            ]
        }
    }

    #[doc = "Upstream Control Enhanced Register 1\n resetvalue={Application Reset:0x0FF}"]
    #[inline(always)]
    pub const fn usce(&self) -> crate::common::Reg<self::Usce_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(92usize)) }
    }

    #[doc = "Upstream Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn usr(&self) -> crate::common::Reg<self::Usr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Abc_SPEC;
impl crate::sealed::RegSpec for Abc_SPEC {
    type DataType = u32;
}
#[doc = "Asynchronous Block Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Abc = crate::RegValueT<Abc_SPEC>;

impl Abc {
    #[doc = "Duration of the Low Phase of the Shift Clock   LOW. Defines the duration of the low phase of the shift clock in periods of f A in the range of 1 to 16."]
    #[inline(always)]
    pub fn low(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Abc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0xf, 1, 0, u8, Abc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Duration of the High Phase of the Shift Clock   HIGH. Defines the duration of the high phase of the shift clock in periods of f A in the range of 1 to 16."]
    #[inline(always)]
    pub fn high(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Abc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, Abc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Overflow Interrupt Node Pointer   OIP. OIP selects the service request output line SRn  n   0 3  for the overflow interrupt."]
    #[inline(always)]
    pub fn oip(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Abc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8, 0x3, 1, 0, u8, Abc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Overflow Alternate Service Request   OASR. Selects if the interrupt signal is routed to the alternate service request node. See CROSSREFERENCE ."]
    #[inline(always)]
    pub fn oasr(self) -> crate::common::RegisterFieldBool<10, 1, 0, Abc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Abc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Overflow Flag   OVF. Indicates if overflow error has occurred."]
    #[inline(always)]
    pub fn ovf(self) -> crate::common::RegisterFieldBool<12, 1, 0, Abc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Abc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Overflow Flag Modify   OFM. Sets or clears the overflow flag OVF."]
    #[inline(always)]
    pub fn ofm(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, Abc_SPEC, crate::common::W> {
        crate::common::RegisterField::<13, 0x3, 1, 0, u8, Abc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Overflow Interrupt Enable   OIE. Enables or disables the path of the interrupt signal towards the        interrupt node. If enabled  an overflow event triggers an interrupt  and        if disabled then not. The OVF flag always indicates the occurrence of an overflow event         independently of OIE."]
    #[inline(always)]
    pub fn oie(self) -> crate::common::RegisterFieldBool<15, 1, 0, Abc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Abc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "N Divider ABRA   NDA. Defines the division ratio in the range of 1 to 8."]
    #[inline(always)]
    pub fn nda(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Abc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Abc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Underflow Interrupt Node Pointer   UIP. UIP selects the service request output line SRn  n  160    160 0 3  for the        underflow interrupt."]
    #[inline(always)]
    pub fn uip(
        self,
    ) -> crate::common::RegisterField<19, 0x3, 1, 0, u8, Abc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<19,0x3,1,0,u8, Abc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Underflow Alternate Service Request   UASR. Selects if the interrupt signal is routed to the alternate service request node. See CROSSREFERENCE ."]
    #[inline(always)]
    pub fn uasr(self) -> crate::common::RegisterFieldBool<21, 1, 0, Abc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Abc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Underflow Flag   UNF. Indicates if underflow error has occurred."]
    #[inline(always)]
    pub fn unf(self) -> crate::common::RegisterFieldBool<23, 1, 0, Abc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Abc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Underflow Flag Modify   UFM. Sets or clears the underflow flag UNF."]
    #[inline(always)]
    pub fn ufm(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, Abc_SPEC, crate::common::W> {
        crate::common::RegisterField::<24, 0x3, 1, 0, u8, Abc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Underflow Interrupt Enable   UIE. Enables or disables the path of the interrupt signal towards the        interrupt node. If enabled  an underflow event triggers an interrupt         and if disabled then not. The UNF flag always indicates the occurrence of an underflow event         independently of UIE."]
    #[inline(always)]
    pub fn uie(self) -> crate::common::RegisterFieldBool<26, 1, 0, Abc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Abc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clock Select   CLKSEL. Selects the clock source for the ABRA block."]
    #[inline(always)]
    pub fn clksel(
        self,
    ) -> crate::common::RegisterField<27, 0x7, 1, 0, u8, Abc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x7,1,0,u8, Abc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Asynchronous Block Bypass   ABB. Defines if the asynchronous block and the n divider of the MSC downstream path  located parallel to the fractional divider  are used or not. If bypassed  then also disabled."]
    #[inline(always)]
    pub fn abb(self) -> crate::common::RegisterFieldBool<31, 1, 0, Abc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Abc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Abc {
    #[inline(always)]
    fn default() -> Abc {
        <crate::RegValueT<Abc_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
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
    #[doc = "Sleep Mode Enable Control   EDIS. Used to control module  8217 s sleep mode."]
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
pub struct Dc_SPEC;
impl crate::sealed::RegSpec for Dc_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Command Register\n resetvalue={Application Reset:0x0}"]
pub type Dc = crate::RegValueT<Dc_SPEC>;

impl Dc {
    #[doc = "Downstream Command for SRL Shift Register   DCL. Contains the data bits to be transmitted during the SRL active phase of a command frame."]
    #[inline(always)]
    pub fn dcl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Dc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Dc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Downstream Command for SRH Shift Register   DCH. Contains the data bits to be transmitted during the SRH active phase of a command frame."]
    #[inline(always)]
    pub fn dch(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Dc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Dc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dc {
    #[inline(always)]
    fn default() -> Dc {
        <crate::RegValueT<Dc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dce_SPEC;
impl crate::sealed::RegSpec for Dce_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Command Extension Register\n resetvalue={Application Reset:0x0}"]
pub type Dce = crate::RegValueT<Dce_SPEC>;

impl Dce {
    #[doc = "Downstream Command Extension for SRH Shift Register   DCEH. Contains the data bits to be transmitted during the second half of the command frame in CX mode."]
    #[inline(always)]
    pub fn dceh(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Dce_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Dce_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dce {
    #[inline(always)]
    fn default() -> Dce {
        <crate::RegValueT<Dce_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcm_SPEC;
impl crate::sealed::RegSpec for Dcm_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Command Mirror Register\n resetvalue={Application Reset:0x0}"]
pub type Dcm = crate::RegValueT<Dcm_SPEC>;

impl Dcm {
    #[doc = "Downstream Command Mirror of the DC.DCL Bit Field   DCLM. Contains alternate write location for the command bits to be transmitted during the SRL active phase of a command frame."]
    #[inline(always)]
    pub fn dclm(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Dcm_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Dcm_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Downstream Command Mirror of the DC.DCH Bit Field   DCHM. Contains the alternate write location for the command bits to be transmitted during the SRH active phase of a command frame."]
    #[inline(always)]
    pub fn dchm(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Dcm_SPEC, crate::common::W> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Dcm_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Dcm {
    #[inline(always)]
    fn default() -> Dcm {
        <crate::RegValueT<Dcm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dd_SPEC;
impl crate::sealed::RegSpec for Dd_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Data Register\n resetvalue={Application Reset:0x0}"]
pub type Dd = crate::RegValueT<Dd_SPEC>;

impl Dd {
    #[doc = "Downstream Data for SRL Shift Register   DDL. Contains the data bits to be transmitted during the SRL active phase of a data frame."]
    #[inline(always)]
    pub fn ddl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Dd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Dd_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Downstream Data for SRH Shift Register   DDH. Contains the data bits to be transmitted during the SRH active phase of a data frame."]
    #[inline(always)]
    pub fn ddh(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Dd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Dd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dd {
    #[inline(always)]
    fn default() -> Dd {
        <crate::RegValueT<Dd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dde_SPEC;
impl crate::sealed::RegSpec for Dde_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Data Extension Register\n resetvalue={Application Reset:0x0}"]
pub type Dde = crate::RegValueT<Dde_SPEC>;

impl Dde {
    #[doc = "Downstream Data Extension for SRL Shift Register   DDLE. Contains the data bits to be transmitted during the SRL active phase of        a data frame."]
    #[inline(always)]
    pub fn ddle(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Dde_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Dde_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Downstream Data Extension for SRH Shift Register   DDHE. Contains the data bits to be transmitted during the SRH active phase of        a data frame."]
    #[inline(always)]
    pub fn ddhe(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Dde_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Dde_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dde {
    #[inline(always)]
    fn default() -> Dde {
        <crate::RegValueT<Dde_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ddm_SPEC;
impl crate::sealed::RegSpec for Ddm_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Data Mirror Register\n resetvalue={Application Reset:0x0}"]
pub type Ddm = crate::RegValueT<Ddm_SPEC>;

impl Ddm {
    #[doc = "Downstream Data Mirror for SRL Shift Register   DDLM. Contains the data bits to be transmitted during the SRL active phase of        a data frame."]
    #[inline(always)]
    pub fn ddlm(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Ddm_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Ddm_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Downstream Data Mirror for SRH Shift Register   DDHM. Contains the data bits to be transmitted during the SRH active phase of        a data frame."]
    #[inline(always)]
    pub fn ddhm(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Ddm_SPEC, crate::common::W> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Ddm_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Ddm {
    #[inline(always)]
    fn default() -> Ddm {
        <crate::RegValueT<Ddm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsc_SPEC;
impl crate::sealed::RegSpec for Dsc_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Control Register\n resetvalue={Application Reset:0x0}"]
pub type Dsc = crate::RegValueT<Dsc_SPEC>;

impl Dsc {
    #[doc = "Transmission Mode   TM. This bit selects the transmission mode of the downstream channel."]
    #[inline(always)]
    pub fn tm(self) -> crate::common::RegisterFieldBool<0, 1, 0, Dsc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dsc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Command Pending   CP. This bit is set when the downstream command register DC is written. CP        is cleared when the first bit of the related command frame is sent out."]
    #[inline(always)]
    pub fn cp(self) -> crate::common::RegisterFieldBool<1, 1, 0, Dsc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Dsc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Data Pending   DP. In Triggered Mode  this bit is set when the set data pending bit ISC.SDP        is set by software. In Data Repetition Mode  this bit is set by hardware        at the last passive time frame. At the start of the data frame  DP is        cleared by hardware."]
    #[inline(always)]
    pub fn dp(self) -> crate::common::RegisterFieldBool<2, 1, 0, Dsc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Dsc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Number of SRL Bits Shifted at Data Frames   NDBL. NDBL determines the number of shift register low part  SRL  bits that        are shifted out on SO during a data frame. Other bit combinations are        reserved  do not use these bit combinations."]
    #[inline(always)]
    pub fn ndbl(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, Dsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8, Dsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Number of SRH Bits Shifted at Data Frames   NDBH. NDBH determines the number of shift register high part  SRH  bits that        are shifted out on SO during a data frame. Other bit combinations are        reserved  do not use these bit combinations."]
    #[inline(always)]
    pub fn ndbh(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, Dsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, Dsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable SRL Active Phase Selection Bit   ENSELL. This bit determines whether a low level selection bit is inserted at the        beginning of a data frame  8217 s SRL active phase."]
    #[inline(always)]
    pub fn ensell(self) -> crate::common::RegisterFieldBool<13, 1, 0, Dsc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Dsc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable SRH Active Phase Selection Bit   ENSELH. This bit determines whether a low level selection bit is inserted at the        beginning of a data frame  8217 s SRH active phase."]
    #[inline(always)]
    pub fn enselh(self) -> crate::common::RegisterFieldBool<14, 1, 0, Dsc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Dsc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Downstream Disable   DSDIS. This bit indicates the state of the downstream channel operation."]
    #[inline(always)]
    pub fn dsdis(self) -> crate::common::RegisterFieldBool<15, 1, 0, Dsc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Dsc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Number of Bits Shifted at Command Frames   NBC. This bit field determines how many bits of the SRL SRH shift registers        are shifted out during transmission of a command frame."]
    #[inline(always)]
    pub fn nbc(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Dsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Dsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Passive Phase Length at Data Frames   PPD. This bit field determines the length of the passive phase of a data        frame."]
    #[inline(always)]
    pub fn ppd(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, Dsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, Dsc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dsc {
    #[inline(always)]
    fn default() -> Dsc {
        <crate::RegValueT<Dsc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsce_SPEC;
impl crate::sealed::RegSpec for Dsce_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Control Enhanced Register 1\n resetvalue={Application Reset:0x0}"]
pub type Dsce = crate::RegValueT<Dsce_SPEC>;

impl Dsce {
    #[doc = "Number of SRH Bits Shifted at Data Frames Extension   NDBHE. Additional MSB bit extension of the NDBH bit field. Adds 16 to the        resulting NDBH value if set."]
    #[inline(always)]
    pub fn ndbhe(self) -> crate::common::RegisterFieldBool<0, 1, 0, Dsce_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dsce_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Number of SRL Bits Shifted at Data Frames Extension   NDBLE. Additional MSB bit extension of the NDBL bit field. Adds 16 to the        resulting NDBL value if set."]
    #[inline(always)]
    pub fn ndble(self) -> crate::common::RegisterFieldBool<1, 1, 0, Dsce_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Dsce_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Extension Enable   EXEN. Enables the extension bit fields."]
    #[inline(always)]
    pub fn exen(self) -> crate::common::RegisterFieldBool<14, 1, 0, Dsce_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Dsce_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Command Command Flag   CCF. This bit flags that a second command frame has been written without data        frame to be sent in between. It is active only if CDCM 1. Otherwise it        is always low."]
    #[inline(always)]
    pub fn ccf(self) -> crate::common::RegisterFieldBool<15, 1, 0, Dsce_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Dsce_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Injection Enable of the Pin 0 Signal   INJENP0. This bit selects if an external signal is injected in a data frame."]
    #[inline(always)]
    pub fn injenp0(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Dsce_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Dsce_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Injection Position of the Pin 0 Signal   INJPOSP0. This bit selects the position of the injected external one bit signal         at the bit position in range of 0 to 63 in the data frame. If both PIN0POS and PIN1POS point to a same bit  PIN0POS has the higher        priority  that is PIN0 level will be visible in the data frame."]
    #[inline(always)]
    pub fn injposp0(
        self,
    ) -> crate::common::RegisterField<17, 0x3f, 1, 0, u8, Dsce_SPEC, crate::common::RW> {
        crate::common::RegisterField::<17,0x3f,1,0,u8, Dsce_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Injection Enable of the Pin 1 Signal   INJENP1. This bit selects if an external signal is injected in a data frame."]
    #[inline(always)]
    pub fn injenp1(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Dsce_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Dsce_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Injection Position of the Pin 1 Signal   INJPOSP1. This bit selects the position of the injected external one bit signal         at the bit position in range of 0 to 63 in the data frame."]
    #[inline(always)]
    pub fn injposp1(
        self,
    ) -> crate::common::RegisterField<25, 0x3f, 1, 0, u8, Dsce_SPEC, crate::common::RW> {
        crate::common::RegisterField::<25,0x3f,1,0,u8, Dsce_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Command Data Command in Data Repetition Mode   CDCM. This bit selects if a data frame is automatically inserted between two        consecutive command frame requests."]
    #[inline(always)]
    pub fn cdcm(self) -> crate::common::RegisterFieldBool<31, 1, 0, Dsce_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Dsce_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Dsce {
    #[inline(always)]
    fn default() -> Dsce {
        <crate::RegValueT<Dsce_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsdsh_SPEC;
impl crate::sealed::RegSpec for Dsdsh_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Select Data Source High Register\n resetvalue={Application Reset:0x0}"]
pub type Dsdsh = crate::RegValueT<Dsdsh_SPEC>;

impl Dsdsh {
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Dsdsh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Dsdsh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Dsdsh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Dsdsh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh4(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Dsdsh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh5(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Dsdsh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh6(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Dsdsh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh7(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Dsdsh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh8(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Dsdsh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh9(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, Dsdsh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh10(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Dsdsh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh11(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, Dsdsh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh12(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, Dsdsh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh13(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, Dsdsh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3,1,0,u8, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh14(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, Dsdsh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh15(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, Dsdsh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dsdsh {
    #[inline(always)]
    fn default() -> Dsdsh {
        <crate::RegValueT<Dsdsh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsdshe_SPEC;
impl crate::sealed::RegSpec for Dsdshe_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Select Data Source High Extension Register\n resetvalue={Application Reset:0x0}"]
pub type Dsdshe = crate::RegValueT<Dsdshe_SPEC>;

impl Dsdshe {
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh16(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Dsdshe_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh17(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Dsdshe_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh18(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Dsdshe_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh19(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Dsdshe_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh20(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Dsdshe_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh21(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Dsdshe_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh22(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Dsdshe_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh23(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Dsdshe_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh24(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Dsdshe_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh25(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, Dsdshe_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh26(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Dsdshe_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh27(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, Dsdshe_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh28(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, Dsdshe_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh29(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, Dsdshe_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3,1,0,u8, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh30(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, Dsdshe_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh31(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, Dsdshe_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dsdshe {
    #[inline(always)]
    fn default() -> Dsdshe {
        <crate::RegValueT<Dsdshe_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsdsl_SPEC;
impl crate::sealed::RegSpec for Dsdsl_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Select Data Source Low Register\n resetvalue={Application Reset:0x0}"]
pub type Dsdsl = crate::RegValueT<Dsdsl_SPEC>;

impl Dsdsl {
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Dsdsl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Dsdsl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Dsdsl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Dsdsl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl4(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Dsdsl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl5(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Dsdsl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl6(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Dsdsl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl7(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Dsdsl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl8(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Dsdsl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl9(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, Dsdsl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl10(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Dsdsl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl11(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, Dsdsl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl12(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, Dsdsl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl13(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, Dsdsl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3,1,0,u8, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl14(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, Dsdsl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl15(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, Dsdsl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dsdsl {
    #[inline(always)]
    fn default() -> Dsdsl {
        <crate::RegValueT<Dsdsl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsdsle_SPEC;
impl crate::sealed::RegSpec for Dsdsle_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Select Data Source Low Extension Register\n resetvalue={Application Reset:0x0}"]
pub type Dsdsle = crate::RegValueT<Dsdsle_SPEC>;

impl Dsdsle {
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl16(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Dsdsle_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl17(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Dsdsle_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl18(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Dsdsle_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl19(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Dsdsle_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl20(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Dsdsle_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl21(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Dsdsle_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl22(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Dsdsle_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl23(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Dsdsle_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl24(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Dsdsle_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl25(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, Dsdsle_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl26(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Dsdsle_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl27(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, Dsdsle_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl28(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, Dsdsle_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl29(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, Dsdsle_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3,1,0,u8, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl30(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, Dsdsle_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl31(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, Dsdsle_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dsdsle {
    #[inline(always)]
    fn default() -> Dsdsle {
        <crate::RegValueT<Dsdsle_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dss_SPEC;
impl crate::sealed::RegSpec for Dss_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Status Register\n resetvalue={Application Reset:0x0}"]
pub type Dss = crate::RegValueT<Dss_SPEC>;

impl Dss {
    #[doc = "Passive Time Frame Counter   PFC. In Data Repetition Mode  this bit field indicates the count of passive        time frames that are currently transmitted. In Triggered Mode PFC        remains at 0000 B ."]
    #[inline(always)]
    pub fn pfc(self) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Dss_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xf, 1, 0, u8, Dss_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Number Of Passive Time Frames   NPTF. This bit field indicates the number of passive time frames that are        inserted in Data Repetition Mode between two data frames."]
    #[inline(always)]
    pub fn nptf(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Dss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8, 0xf, 1, 0, u8, Dss_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Downstream Counter   DC. This bit field indicates the number of downstream shift clock periods        that have been elapsed since the start of the current frame. DC is reset        at the end of a downstream frame."]
    #[inline(always)]
    pub fn dc(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Dss_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Dss_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Data Frame Active   DFA. This bit indicates if a data frame is currently sent out  active phase only."]
    #[inline(always)]
    pub fn dfa(self) -> crate::common::RegisterFieldBool<24, 1, 0, Dss_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Dss_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Command Frame Active   CFA. This bit indicates if a command frame is currently sent out  active phase only."]
    #[inline(always)]
    pub fn cfa(self) -> crate::common::RegisterFieldBool<25, 1, 0, Dss_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Dss_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Dss {
    #[inline(always)]
    fn default() -> Dss {
        <crate::RegValueT<Dss_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dste_SPEC;
impl crate::sealed::RegSpec for Dste_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Timing Extension Register\n resetvalue={Application Reset:0x0}"]
pub type Dste = crate::RegValueT<Dste_SPEC>;

impl Dste {
    #[doc = "Passive Phase Length at Data Frames Extension   PPDE. Additional MSB bits extension of the PPD bit field."]
    #[inline(always)]
    pub fn ppde(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Dste_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Dste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Passive Phase Length at Control Frames Extension   PPCE. Additional MSB bits extension of the fixed length of 2 of the command        frames passive phase. The final length is 2  160    160 PPCE resulting in a range        of 2 to 65."]
    #[inline(always)]
    pub fn ppce(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, Dste_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8, Dste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N Divider Downstream   NDD. Defines the division ratio in the range of 1 to 16."]
    #[inline(always)]
    pub fn ndd(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Dste_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Dste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PPCE Extension Bit on the MSB Side   PPCEM. This bit is the MSB extension bit for the PPCE bit field  extending        command frame passive phase to the value of 127. The values of 128 and        129 are not allowed."]
    #[inline(always)]
    pub fn ppcem(self) -> crate::common::RegisterFieldBool<12, 1, 0, Dste_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Dste_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fast Mode   FM. Activates the fast mode and writing to the ABRA FIFO with 100  160 MBaud        input baud rate  in order to provide 80MBaud output baud rate. It is        also recommended for baud rate of 26 67  160 MBaud  as defined in the CROSSREFERENCE .        FM 0 is the compatibility setting with the previous generations of the        MSC."]
    #[inline(always)]
    pub fn fm(self) -> crate::common::RegisterFieldBool<16, 1, 0, Dste_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Dste_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Command Extension Mode   CX. Activates 64 bit command frame feature."]
    #[inline(always)]
    pub fn cx(self) -> crate::common::RegisterFieldBool<28, 1, 0, Dste_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Dste_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Unlock CX and FM for one write access   UL1. Write access with UL1  160    160 1 unlocks the CX and FM bit fields for the        current write access. Write accesses to these bit fields with UL1  160    160 0 do        not have any effect. Returns zero on read."]
    #[inline(always)]
    pub fn ul1(self) -> crate::common::RegisterFieldBool<31, 1, 0, Dste_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Dste_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Dste {
    #[inline(always)]
    fn default() -> Dste {
        <crate::RegValueT<Dste_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Esr_SPEC;
impl crate::sealed::RegSpec for Esr_SPEC {
    type DataType = u32;
}
#[doc = "Emergency Stop Register\n resetvalue={Application Reset:0x0}"]
pub type Esr = crate::RegValueT<Esr_SPEC>;

impl Esr {
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh0(self) -> crate::common::RegisterFieldBool<16, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh1(self) -> crate::common::RegisterFieldBool<17, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh2(self) -> crate::common::RegisterFieldBool<18, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh3(self) -> crate::common::RegisterFieldBool<19, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh4(self) -> crate::common::RegisterFieldBool<20, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh5(self) -> crate::common::RegisterFieldBool<21, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh6(self) -> crate::common::RegisterFieldBool<22, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh7(self) -> crate::common::RegisterFieldBool<23, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh8(self) -> crate::common::RegisterFieldBool<24, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh9(self) -> crate::common::RegisterFieldBool<25, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh10(self) -> crate::common::RegisterFieldBool<26, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh11(self) -> crate::common::RegisterFieldBool<27, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh12(self) -> crate::common::RegisterFieldBool<28, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh13(self) -> crate::common::RegisterFieldBool<29, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh14(self) -> crate::common::RegisterFieldBool<30, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh15(self) -> crate::common::RegisterFieldBool<31, 1, 0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Esr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Esr {
    #[inline(always)]
    fn default() -> Esr {
        <crate::RegValueT<Esr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Esre_SPEC;
impl crate::sealed::RegSpec for Esre_SPEC {
    type DataType = u32;
}
#[doc = "Emergency Stop Extension Register\n resetvalue={Application Reset:0x0}"]
pub type Esre = crate::RegValueT<Esre_SPEC>;

impl Esre {
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl16(self) -> crate::common::RegisterFieldBool<0, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl17(self) -> crate::common::RegisterFieldBool<1, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl18(self) -> crate::common::RegisterFieldBool<2, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl19(self) -> crate::common::RegisterFieldBool<3, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl20(self) -> crate::common::RegisterFieldBool<4, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl21(self) -> crate::common::RegisterFieldBool<5, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl22(self) -> crate::common::RegisterFieldBool<6, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl23(self) -> crate::common::RegisterFieldBool<7, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl24(self) -> crate::common::RegisterFieldBool<8, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl25(self) -> crate::common::RegisterFieldBool<9, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl26(self) -> crate::common::RegisterFieldBool<10, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl27(self) -> crate::common::RegisterFieldBool<11, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl28(self) -> crate::common::RegisterFieldBool<12, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl29(self) -> crate::common::RegisterFieldBool<13, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl30(self) -> crate::common::RegisterFieldBool<14, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl31(self) -> crate::common::RegisterFieldBool<15, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh25(self) -> crate::common::RegisterFieldBool<25, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh26(self) -> crate::common::RegisterFieldBool<26, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh27(self) -> crate::common::RegisterFieldBool<27, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh28(self) -> crate::common::RegisterFieldBool<28, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh29(self) -> crate::common::RegisterFieldBool<29, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh30(self) -> crate::common::RegisterFieldBool<30, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh31(self) -> crate::common::RegisterFieldBool<31, 1, 0, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Esre_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Esre {
    #[inline(always)]
    fn default() -> Esre {
        <crate::RegValueT<Esre_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdr_SPEC;
impl crate::sealed::RegSpec for Fdr_SPEC {
    type DataType = u32;
}
#[doc = "Fractional Divider Register\n resetvalue={Application Reset:0x0}"]
pub type Fdr = crate::RegValueT<Fdr_SPEC>;

impl Fdr {
    #[doc = "Step Value   STEP. Reload or addition value for RESULT."]
    #[inline(always)]
    pub fn step(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Fdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Fdr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Divider Mode   DM. DM selects normal or fractional divider mode."]
    #[inline(always)]
    pub fn dm(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Fdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Fdr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Result Value   RESULT. Bit field for the addition result."]
    #[inline(always)]
    pub fn result(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Fdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Fdr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Enable Hardware Clock Control   ENHW. Controls operation of ECEN input and DISCLK bit."]
    #[inline(always)]
    pub fn enhw(self) -> crate::common::RegisterFieldBool<30, 1, 0, Fdr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Fdr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable Clock   DISCLK. Hardware controlled disable for f MSC signal."]
    #[inline(always)]
    pub fn disclk(self) -> crate::common::RegisterFieldBool<31, 1, 0, Fdr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Fdr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Fdr {
    #[inline(always)]
    fn default() -> Fdr {
        <crate::RegValueT<Fdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr_SPEC;
impl crate::sealed::RegSpec for Icr_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
pub type Icr = crate::RegValueT<Icr_SPEC>;

impl Icr {
    #[doc = "Data Frame Interrupt Node Pointer   EDIP. EDIP selects the service request output line SRn  n   0 3  for the data frame interrupt."]
    #[inline(always)]
    pub fn edip(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0x3, 1, 0, u8, Icr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Data Frame Interrupt Enable   EDIE. This bit field determines the enable conditions for the data frame interrupt."]
    #[inline(always)]
    pub fn edie(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2, 0x3, 1, 0, u8, Icr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Command Frame Interrupt Node Pointer   ECIP. ECIP selects the service request output line SRn  n   0 3  for the command frame interrupt."]
    #[inline(always)]
    pub fn ecip(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4, 0x3, 1, 0, u8, Icr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Command Frame Interrupt Enable   ECIE. This bit enables the command frame interrupt."]
    #[inline(always)]
    pub fn ecie(self) -> crate::common::RegisterFieldBool<7, 1, 0, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Icr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Time Frame Interrupt Pointer   TFIP. TFIP selects the service request output line SRn  n   0 3  for the time frame interrupt."]
    #[inline(always)]
    pub fn tfip(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8, 0x3, 1, 0, u8, Icr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Time Frame Interrupt Enable   TFIE. This bit enables the time frame interrupt."]
    #[inline(always)]
    pub fn tfie(self) -> crate::common::RegisterFieldBool<11, 1, 0, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Icr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive Data Interrupt Pointer   RDIP. RDIP selects the service request output line SRn  n   0 3  for the receive data interrupt."]
    #[inline(always)]
    pub fn rdip(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Data Interrupt Enable   RDIE. This bit field determines the enable conditions for the receive data interrupt."]
    #[inline(always)]
    pub fn rdie(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        <crate::RegValueT<Icr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x28C010}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MODREV. This bit field defines the module revision number. The value of a module        revision starts with 01 H  first        revision ."]
    #[inline(always)]
    pub fn modrev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MODTYPE. This bit field defines the module as a 32 bit module  C0 H"]
    #[inline(always)]
    pub fn modtype(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MODNUM. This bit field defines the module identification number for the MSC          0028 H"]
    #[inline(always)]
    pub fn modnum(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Id_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Id {
    #[inline(always)]
    fn default() -> Id {
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(2670608)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isc_SPEC;
impl crate::sealed::RegSpec for Isc_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Set Clear Register\n resetvalue={Application Reset:0x0}"]
pub type Isc = crate::RegValueT<Isc_SPEC>;

impl Isc {
    #[doc = "Clear DEDI Flag   CDEDI"]
    #[inline(always)]
    pub fn cdedi(self) -> crate::common::RegisterFieldBool<0, 1, 0, Isc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Isc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear DECI Flag   CDECI"]
    #[inline(always)]
    pub fn cdeci(self) -> crate::common::RegisterFieldBool<1, 1, 0, Isc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Isc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear DTFI Flag   CDTFI"]
    #[inline(always)]
    pub fn cdtfi(self) -> crate::common::RegisterFieldBool<2, 1, 0, Isc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Isc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear URDI Flag   CURDI"]
    #[inline(always)]
    pub fn curdi(self) -> crate::common::RegisterFieldBool<3, 1, 0, Isc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Isc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear DP Flag   CDP"]
    #[inline(always)]
    pub fn cdp(self) -> crate::common::RegisterFieldBool<4, 1, 0, Isc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Isc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear CP Flag   CCP"]
    #[inline(always)]
    pub fn ccp(self) -> crate::common::RegisterFieldBool<5, 1, 0, Isc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Isc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear DSDIS Flag   CDDIS"]
    #[inline(always)]
    pub fn cddis(self) -> crate::common::RegisterFieldBool<6, 1, 0, Isc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Isc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set DEDI Flag   SDEDI"]
    #[inline(always)]
    pub fn sdedi(self) -> crate::common::RegisterFieldBool<16, 1, 0, Isc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Isc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set DECI Flag   SDECI"]
    #[inline(always)]
    pub fn sdeci(self) -> crate::common::RegisterFieldBool<17, 1, 0, Isc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Isc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set DTFI Flag   SDTFI"]
    #[inline(always)]
    pub fn sdtfi(self) -> crate::common::RegisterFieldBool<18, 1, 0, Isc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Isc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set URDI Flag   SURDI"]
    #[inline(always)]
    pub fn surdi(self) -> crate::common::RegisterFieldBool<19, 1, 0, Isc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, Isc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set DP Bit   SDP"]
    #[inline(always)]
    pub fn sdp(self) -> crate::common::RegisterFieldBool<20, 1, 0, Isc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, Isc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set CP Flag   SCP"]
    #[inline(always)]
    pub fn scp(self) -> crate::common::RegisterFieldBool<21, 1, 0, Isc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, Isc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set DSDIS Flag   SDDIS"]
    #[inline(always)]
    pub fn sddis(self) -> crate::common::RegisterFieldBool<22, 1, 0, Isc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, Isc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Isc {
    #[inline(always)]
    fn default() -> Isc {
        <crate::RegValueT<Isc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr_SPEC;
impl crate::sealed::RegSpec for Isr_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Status Register\n resetvalue={Application Reset:0x0}"]
pub type Isr = crate::RegValueT<Isr_SPEC>;

impl Isr {
    #[doc = "Data Frame Interrupt Flag   DEDI. This flag is always set by hardware when a downstream channel data frame        interrupt is generated. DEDI can be set or cleared by software when        writing to register ISC with the appropriate bits ISC.SDEDI or ISC.CDEDI        set."]
    #[inline(always)]
    pub fn dedi(self) -> crate::common::RegisterFieldBool<0, 1, 0, Isr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Isr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Command Frame Interrupt Flag   DECI. This flag is always set by hardware when a downstream channel command        frame interrupt is generated  whether or not it is enabled. DECI can be        set or cleared by software when writing to register ISC with the        appropriate bits SDECI or CDECI set."]
    #[inline(always)]
    pub fn deci(self) -> crate::common::RegisterFieldBool<1, 1, 0, Isr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Isr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Time Frame Interrupt Flag   DTFI. This flag is always set by hardware when a downstream channel time frame        interrupt is generated  whether or not it is enabled. DTFI can be set or        cleared by software when writing to register ISC with the appropriate        bits SDTFI or CDTFI set."]
    #[inline(always)]
    pub fn dtfi(self) -> crate::common::RegisterFieldBool<2, 1, 0, Isr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Isr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive Data Interrupt Flag   URDI. This flag is always set by hardware when an upstream channel receive        data interrupt is generated  whether or not it is enabled. URDI can be        set or cleared by software when writing to register ISC with the        appropriate bits SURDI or CURDI set."]
    #[inline(always)]
    pub fn urdi(self) -> crate::common::RegisterFieldBool<3, 1, 0, Isr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Isr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Isr {
    #[inline(always)]
    fn default() -> Isr {
        <crate::RegValueT<Isr_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel        reset will be executed if the reset bits of both kernel registers are        set. The RST bit will be cleared  reset to   180 0  180   by the BPI FPI after the        kernel reset was executed."]
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
#[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
pub type Krst1 = crate::RegValueT<Krst1_SPEC>;

impl Krst1 {
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel reset registers is set. The RST bit will be cleared  reset to  0   by the BPI FPI after the kernel reset was executed."]
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
pub struct Ocr_SPEC;
impl crate::sealed::RegSpec for Ocr_SPEC {
    type DataType = u32;
}
#[doc = "Output Control Register\n resetvalue={Application Reset:0x0}"]
pub type Ocr = crate::RegValueT<Ocr_SPEC>;

impl Ocr {
    #[doc = "FCLP Line Polarity   CLP"]
    #[inline(always)]
    pub fn clp(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ocr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ocr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SOP Line Polarity   SLP"]
    #[inline(always)]
    pub fn slp(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ocr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ocr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Chip Selection Lines Polarity   CSLP. Bit CSLP is buffered during a frame transmission. This means that any        change of CSLP becomes valid first with the start of the next frame        transmission."]
    #[inline(always)]
    pub fn cslp(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ocr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ocr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SDI Line Polarity   ILP"]
    #[inline(always)]
    pub fn ilp(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ocr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ocr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clock Control   CLKCTRL. This bit determines the activation of clock output FCL."]
    #[inline(always)]
    pub fn clkctrl(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ocr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ocr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Chip Enable Selection for ENL   CSL. This bit field selects the chip enable output ENx that becomes active        during the SRL active phase  ENL  160    160 1  of a data frame. The active level        of ENx is defined by bit CSLP."]
    #[inline(always)]
    pub fn csl(
        self,
    ) -> crate::common::RegisterField<9, 0x3, 1, 0, u8, Ocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9, 0x3, 1, 0, u8, Ocr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Chip Enable Selection for ENH   CSH. This bit field selects the chip enable output ENx that becomes active        during the SRH active phase  ENH  160    160 1  of a data frame. The active level        of ENx is defined by bit CSLP."]
    #[inline(always)]
    pub fn csh(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, u8, Ocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x3,1,0,u8, Ocr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Chip Enable Selection for ENC   CSC. This bit field selects the chip enable output ENx that becomes active        during the active phase  ENC  160    160 1  of a command frame. The active level        of ENx is defined by bit CSLP."]
    #[inline(always)]
    pub fn csc(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, Ocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x3,1,0,u8, Ocr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Serial Data Input Selection   SDISEL. This bit field selects the source for the serial data input SDI of the        upstream channel."]
    #[inline(always)]
    pub fn sdisel(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Ocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Ocr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ocr {
    #[inline(always)]
    fn default() -> Ocr {
        <crate::RegValueT<Ocr_SPEC> as RegisterValue<_>>::new(0)
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
pub struct UDx_SPEC;
impl crate::sealed::RegSpec for UDx_SPEC {
    type DataType = u32;
}
#[doc = "Upstream Data Register 0\n resetvalue={Application Reset:0x0}"]
pub type UDx = crate::RegValueT<UDx_SPEC>;

impl UDx {
    #[doc = "Received Data   DATA. This bit field contains the 8 bit receive data."]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, UDx_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, UDx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Valid Bit   V. This bit is set by hardware when the received data is written to UDx. Writing bit C   1 clears V. If hardware setting and software clearing of the valid bit occur simultaneously  bit V will be cleared."]
    #[inline(always)]
    pub fn v(self) -> crate::common::RegisterFieldBool<16, 1, 0, UDx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, UDx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Parity Bit   P. This flag contains the parity bit that has been received with the data frame."]
    #[inline(always)]
    pub fn p(self) -> crate::common::RegisterFieldBool<17, 1, 0, UDx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, UDx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit   C. C is always read as 0."]
    #[inline(always)]
    pub fn c(self) -> crate::common::RegisterFieldBool<18, 1, 0, UDx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, UDx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Lower Address Bit Field   LABF. This bit field contains the two address bits A 1 0  of the 4 bit address field  16 bit data frame . If 12 bit data frame is selected  LABF is always set to 00 B ."]
    #[inline(always)]
    pub fn labf(
        self,
    ) -> crate::common::RegisterField<19, 0x3, 1, 0, u8, UDx_SPEC, crate::common::R> {
        crate::common::RegisterField::<19, 0x3, 1, 0, u8, UDx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Internal Parity Flag   IPF. This bit contains the parity bit that has been calculated in the MSC during data frame reception."]
    #[inline(always)]
    pub fn ipf(self) -> crate::common::RegisterFieldBool<21, 1, 0, UDx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, UDx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Parity Error   PERR. This bit indicates if a start bit error  parity error  or stop bit error occurred during frame reception."]
    #[inline(always)]
    pub fn perr(self) -> crate::common::RegisterFieldBool<22, 1, 0, UDx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, UDx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for UDx {
    #[inline(always)]
    fn default() -> UDx {
        <crate::RegValueT<UDx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usce_SPEC;
impl crate::sealed::RegSpec for Usce_SPEC {
    type DataType = u32;
}
#[doc = "Upstream Control Enhanced Register 1\n resetvalue={Application Reset:0x0FF}"]
pub type Usce = crate::RegValueT<Usce_SPEC>;

impl Usce {
    #[doc = "Upstream Time out Prescaler   USTOPRE. Prescaler for the upstream time out limit. Expressed in upstream bit        times. 2 n divider in the range of 1        to 16384. Write to this bit field triggers new start of the watchdog timer."]
    #[inline(always)]
    pub fn ustopre(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Usce_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Usce_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Upstream Time out Value   USTOVAL. Upstream time out value for the N divider in the range of 1 to 16         expressed in number of upstream bit time lengths  the upstream bit time        is the reciprocal value of the upstream baud rate . Time out   BitTime          2  USTOPRE 1     USTOVAL 1 . Write to this bit field triggers new start of the watchdog timer."]
    #[inline(always)]
    pub fn ustoval(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Usce_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Usce_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Upstream Time out Interrupt Enable   USTOEN. Enable bit for the upstream time out interrupt. The Time out counter runs continuously independently of the USTOEN bit. The USTOEN enables only the interrupt generation."]
    #[inline(always)]
    pub fn ustoen(self) -> crate::common::RegisterFieldBool<8, 1, 0, Usce_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Usce_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Upstream Time out Flag   USTF. Signals an upstream timer event. If set by software  the interrupt is also triggered  if enabled. The watchdog timer runs continuously and sets the USTF flag at overflow  independently from the enable bit. Therefore this bit must be cleared with the same write access which enables the interrupt."]
    #[inline(always)]
    pub fn ustf(self) -> crate::common::RegisterFieldBool<9, 1, 0, Usce_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Usce_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Upstream Time out Clear   USTC. Clears the USTF flag. Write only bit  reads as 0."]
    #[inline(always)]
    pub fn ustc(self) -> crate::common::RegisterFieldBool<10, 1, 0, Usce_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Usce_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Upstream Time out Set   USTS. Sets the USTF flag. Write only bit  reads as 0."]
    #[inline(always)]
    pub fn usts(self) -> crate::common::RegisterFieldBool<11, 1, 0, Usce_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, Usce_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Upstream Time out Alternate Service Request   UTASR. Selects if the interrupt signal is routed to the alternate service request node. See CROSSREFERENCE ."]
    #[inline(always)]
    pub fn utasr(self) -> crate::common::RegisterFieldBool<13, 1, 0, Usce_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Usce_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Upstream Time out Interrupt Node Pointer   USTOIP. USTOIP selects the service request output line SRn  n   0 3  for the time out interrupt."]
    #[inline(always)]
    pub fn ustoip(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Usce_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Usce_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Usce {
    #[inline(always)]
    fn default() -> Usce {
        <crate::RegValueT<Usce_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usr_SPEC;
impl crate::sealed::RegSpec for Usr_SPEC {
    type DataType = u32;
}
#[doc = "Upstream Status Register\n resetvalue={Application Reset:0x0}"]
pub type Usr = crate::RegValueT<Usr_SPEC>;

impl Usr {
    #[doc = "Upstream Channel Frame Type   UFT. This bit determines the frame type used by the upstream channel for data        reception."]
    #[inline(always)]
    pub fn uft(self) -> crate::common::RegisterFieldBool<0, 1, 0, Usr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Usr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Upstream Channel Receiving Rate   URR. This bit field determines the baud rate for the upstream channel."]
    #[inline(always)]
    pub fn urr(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, Usr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1, 0x7, 1, 0, u8, Usr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Parity Control   PCTR. This bit determines the parity mode used by the upstream channel for        data reception."]
    #[inline(always)]
    pub fn pctr(self) -> crate::common::RegisterFieldBool<4, 1, 0, Usr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Usr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Service Request Delay Control   SRDC. This bit determines the additional delay inserted by the upstream        channel when triggering the receive interrupt. Only the hardware        generated interrupt can be delayed. The software generated interrupt by        writing the ISC.SURDI bit is never delayed."]
    #[inline(always)]
    pub fn srdc(self) -> crate::common::RegisterFieldBool<5, 1, 0, Usr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Usr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Upstream Counter   UC. This bit field indicates the content of the upstream counter that counts the bits during upstream channel reception."]
    #[inline(always)]
    pub fn uc(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Usr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Usr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Usr {
    #[inline(always)]
    fn default() -> Usr {
        <crate::RegValueT<Usr_SPEC> as RegisterValue<_>>::new(0)
    }
}
