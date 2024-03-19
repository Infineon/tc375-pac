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
#[doc = r"DMA"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma(pub(super) *mut u8);
unsafe impl core::marker::Send for Dma {}
unsafe impl core::marker::Sync for Dma {}
impl Dma {
    #[doc = "DMA Clock Control Register\n resetvalue={Application Reset:0x08}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "ME 1 Clear Error Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn clre0(&self) -> crate::common::Reg<self::Clre0_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(296usize)) }
    }

    #[doc = "ME 1 Clear Error Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn clre1(&self) -> crate::common::Reg<self::Clre1_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4392usize)) }
    }

    #[doc = "ME 1 Enable Error Register\n resetvalue={Application Reset:0x4030000}"]
    #[inline(always)]
    pub const fn eer0(&self) -> crate::common::Reg<self::Eer0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(288usize)) }
    }

    #[doc = "ME 1 Enable Error Register\n resetvalue={Application Reset:0x4030000}"]
    #[inline(always)]
    pub const fn eer1(&self) -> crate::common::Reg<self::Eer1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4384usize)) }
    }

    #[doc = "RP 0 Error Interrupt Set Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn errintrr(&self) -> [crate::common::Reg<self::ErrintRr_SPEC, crate::common::W>; 4] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x1320usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1320usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1320usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1320usize + 0xcusize)),
            ]
        }
    }

    #[doc = "ME 1 Error Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn errsr0(&self) -> crate::common::Reg<self::Errsr0_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(292usize)) }
    }

    #[doc = "ME 1 Error Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn errsr1(&self) -> crate::common::Reg<self::Errsr1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4388usize)) }
    }

    #[doc = "DMA Channel 000 Resource Partition Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hrrc(&self) -> [crate::common::Reg<self::HrRc_SPEC, crate::common::RW>; 128] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x5cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x60usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x64usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x68usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x6cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x70usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x74usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x78usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x7cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x80usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x84usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x88usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x8cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x90usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x94usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x98usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x9cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xa0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xa4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xa8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xacusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xb0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xb4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xb8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xbcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xc0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xc4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xc8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xccusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xd0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xd4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xd8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xdcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xe0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xe4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xe8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xecusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xf0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xf4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xf8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xfcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x100usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x104usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x108usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x10cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x110usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x114usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x118usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x11cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x120usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x124usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x128usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x12cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x130usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x134usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x138usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x13cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x140usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x144usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x148usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x14cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x150usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x154usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x158usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x15cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x160usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x164usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x168usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x16cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x170usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x174usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x178usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x17cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x180usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x184usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x188usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x18cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x190usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x194usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x198usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x19cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1a0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1a4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1a8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1acusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1b0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1b4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1b8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1bcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1c0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1c4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1c8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1ccusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1d0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1d4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1d8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1dcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1e0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1e4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1e8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1ecusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1f0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1f4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1f8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1fcusize)),
            ]
        }
    }

    #[doc = "DMA Identification Register\n resetvalue={Application Reset:0x087C000}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "ME 1 Read Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me00r(&self) -> crate::common::Reg<self::Me00R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(320usize)) }
    }

    #[doc = "ME 1 Read Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me01r(&self) -> crate::common::Reg<self::Me01R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(324usize)) }
    }

    #[doc = "ME 1 Read Register 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me02r(&self) -> crate::common::Reg<self::Me02R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(328usize)) }
    }

    #[doc = "ME 1 Read Register 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me03r(&self) -> crate::common::Reg<self::Me03R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(332usize)) }
    }

    #[doc = "ME 1 Read Register 4\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me04r(&self) -> crate::common::Reg<self::Me04R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(336usize)) }
    }

    #[doc = "ME 1 Read Register 5\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me05r(&self) -> crate::common::Reg<self::Me05R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(340usize)) }
    }

    #[doc = "ME 1 Read Register 6\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me06r(&self) -> crate::common::Reg<self::Me06R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(344usize)) }
    }

    #[doc = "ME 1 Read Register 7\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me07r(&self) -> crate::common::Reg<self::Me07R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(348usize)) }
    }

    #[doc = "ME 1 Channel Address and Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me0adicr(&self) -> crate::common::Reg<self::Me0Adicr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(400usize)) }
    }

    #[doc = "ME 1 Channel Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me0chcr(&self) -> crate::common::Reg<self::Me0Chcr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(404usize)) }
    }

    #[doc = "ME 1 Channel Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me0chsr(&self) -> crate::common::Reg<self::Me0Chsr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(412usize)) }
    }

    #[doc = "ME 1 Channel Destination Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me0dadr(&self) -> crate::common::Reg<self::Me0Dadr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(396usize)) }
    }

    #[doc = "ME 1 Channel Read Data CRC Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me0rdcrc(&self) -> crate::common::Reg<self::Me0Rdcrc_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(384usize)) }
    }

    #[doc = "ME 1 Channel Source Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me0sadr(&self) -> crate::common::Reg<self::Me0Sadr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(392usize)) }
    }

    #[doc = "ME 1 Channel Source and Destination Address CRC Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me0sdcrc(&self) -> crate::common::Reg<self::Me0Sdcrc_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(388usize)) }
    }

    #[doc = "ME 1 Channel Shadow Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me0shadr(&self) -> crate::common::Reg<self::Me0Shadr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(408usize)) }
    }

    #[doc = "ME 1 Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me0sr(&self) -> crate::common::Reg<self::Me0Sr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(304usize)) }
    }

    #[doc = "ME 1 Read Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me10r(&self) -> crate::common::Reg<self::Me10R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4416usize)) }
    }

    #[doc = "ME 1 Read Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me11r(&self) -> crate::common::Reg<self::Me11R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4420usize)) }
    }

    #[doc = "ME 1 Read Register 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me12r(&self) -> crate::common::Reg<self::Me12R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4424usize)) }
    }

    #[doc = "ME 1 Read Register 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me13r(&self) -> crate::common::Reg<self::Me13R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4428usize)) }
    }

    #[doc = "ME 1 Read Register 4\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me14r(&self) -> crate::common::Reg<self::Me14R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4432usize)) }
    }

    #[doc = "ME 1 Read Register 5\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me15r(&self) -> crate::common::Reg<self::Me15R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4436usize)) }
    }

    #[doc = "ME 1 Read Register 6\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me16r(&self) -> crate::common::Reg<self::Me16R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4440usize)) }
    }

    #[doc = "ME 1 Read Register 7\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me17r(&self) -> crate::common::Reg<self::Me17R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4444usize)) }
    }

    #[doc = "ME 1 Channel Address and Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me1adicr(&self) -> crate::common::Reg<self::Me1Adicr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4496usize)) }
    }

    #[doc = "ME 1 Channel Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me1chcr(&self) -> crate::common::Reg<self::Me1Chcr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4500usize)) }
    }

    #[doc = "ME 1 Channel Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me1chsr(&self) -> crate::common::Reg<self::Me1Chsr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4508usize)) }
    }

    #[doc = "ME 1 Channel Destination Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me1dadr(&self) -> crate::common::Reg<self::Me1Dadr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4492usize)) }
    }

    #[doc = "ME 1 Channel Read Data CRC Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me1rdcrc(&self) -> crate::common::Reg<self::Me1Rdcrc_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4480usize)) }
    }

    #[doc = "ME 1 Channel Source Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me1sadr(&self) -> crate::common::Reg<self::Me1Sadr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4488usize)) }
    }

    #[doc = "ME 1 Channel Source and Destination Address CRC Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me1sdcrc(&self) -> crate::common::Reg<self::Me1Sdcrc_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4484usize)) }
    }

    #[doc = "ME 1 Channel Shadow Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me1shadr(&self) -> crate::common::Reg<self::Me1Shadr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4504usize)) }
    }

    #[doc = "ME 1 Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me1sr(&self) -> crate::common::Reg<self::Me1Sr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4400usize)) }
    }

    #[doc = "RP 0 Mode Register\n resetvalue={Application Reset:0x1}"]
    #[inline(always)]
    pub const fn moder(&self) -> [crate::common::Reg<self::ModEr_SPEC, crate::common::RW>; 4] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x1300usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1300usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1300usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1300usize + 0xcusize)),
            ]
        }
    }

    #[doc = "DMA OCDS Trigger Set Select\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn otss(&self) -> crate::common::Reg<self::Otss_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4608usize)) }
    }

    #[doc = "DMA Pattern Read Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn prr0(&self) -> crate::common::Reg<self::Prr0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4616usize)) }
    }

    #[doc = "DMA Pattern Read Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn prr1(&self) -> crate::common::Reg<self::Prr1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4620usize)) }
    }

    #[doc = "DMA Channel 000 Suspend Acknowledge Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn susacrc(&self) -> [crate::common::Reg<self::SusacRc_SPEC, crate::common::R>; 128] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x5cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x60usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x64usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x68usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x6cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x70usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x74usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x78usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x7cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x80usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x84usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x88usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x8cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x90usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x94usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x98usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x9cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xa0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xa4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xa8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xacusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xb0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xb4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xb8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xbcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xc0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xc4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xc8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xccusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xd0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xd4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xd8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xdcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xe0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xe4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xe8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xecusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xf0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xf4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xf8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xfcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x100usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x104usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x108usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x10cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x110usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x114usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x118usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x11cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x120usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x124usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x128usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x12cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x130usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x134usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x138usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x13cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x140usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x144usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x148usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x14cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x150usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x154usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x158usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x15cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x160usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x164usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x168usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x16cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x170usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x174usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x178usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x17cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x180usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x184usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x188usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x18cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x190usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x194usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x198usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x19cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1a0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1a4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1a8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1acusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1b0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1b4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1b8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1bcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1c0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1c4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1c8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1ccusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1d0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1d4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1d8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1dcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1e0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1e4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1e8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1ecusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1f0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1f4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1f8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1fcusize)),
            ]
        }
    }

    #[doc = "DMA Channel 000 Suspend Enable Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn susenrc(
        &self,
    ) -> [crate::common::Reg<self::SusenRc_SPEC, crate::common::RW>; 128] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x5cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x60usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x64usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x68usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x6cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x70usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x74usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x78usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x7cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x80usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x84usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x88usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x8cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x90usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x94usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x98usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x9cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xa0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xa4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xa8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xacusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xb0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xb4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xb8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xbcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xc0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xc4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xc8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xccusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xd0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xd4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xd8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xdcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xe0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xe4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xe8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xecusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xf0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xf4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xf8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xfcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x100usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x104usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x108usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x10cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x110usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x114usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x118usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x11cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x120usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x124usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x128usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x12cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x130usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x134usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x138usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x13cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x140usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x144usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x148usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x14cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x150usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x154usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x158usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x15cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x160usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x164usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x168usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x16cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x170usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x174usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x178usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x17cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x180usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x184usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x188usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x18cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x190usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x194usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x198usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x19cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1a0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1a4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1a8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1acusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1b0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1b4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1b8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1bcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1c0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1c4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1c8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1ccusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1d0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1d4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1d8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1dcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1e0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1e4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1e8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1ecusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1f0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1f4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1f8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1fcusize)),
            ]
        }
    }

    #[doc = "DMA Time Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn time(&self) -> crate::common::Reg<self::Time_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4624usize)) }
    }

    #[doc = "DMA Channel 000 Transaction State Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tsrc(&self) -> [crate::common::Reg<self::TsRc_SPEC, crate::common::RW>; 128] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x5cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x60usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x64usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x68usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x6cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x70usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x74usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x78usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x7cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x80usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x84usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x88usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x8cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x90usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x94usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x98usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x9cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xa0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xa4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xa8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xacusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xb0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xb4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xb8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xbcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xc0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xc4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xc8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xccusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xd0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xd4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xd8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xdcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xe0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xe4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xe8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xecusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xf0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xf4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xf8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xfcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x100usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x104usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x108usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x10cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x110usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x114usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x118usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x11cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x120usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x124usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x128usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x12cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x130usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x134usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x138usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x13cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x140usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x144usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x148usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x14cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x150usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x154usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x158usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x15cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x160usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x164usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x168usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x16cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x170usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x174usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x178usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x17cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x180usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x184usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x188usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x18cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x190usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x194usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x198usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x19cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1a0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1a4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1a8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1acusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1b0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1b4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1b8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1bcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1c0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1c4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1c8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1ccusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1d0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1d4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1d8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1dcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1e0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1e4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1e8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1ecusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1f0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1f4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1f8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1fcusize)),
            ]
        }
    }
    #[doc = "ACCEN"]
    #[inline(always)]
    pub fn accen(self) -> [self::Accen; 4] {
        unsafe {
            [
                self::Accen(self.0.add(0x40usize + 0x0usize)),
                self::Accen(self.0.add(0x40usize + 0x8usize)),
                self::Accen(self.0.add(0x40usize + 0x10usize)),
                self::Accen(self.0.add(0x40usize + 0x18usize)),
            ]
        }
    }
    #[doc = "CH"]
    #[inline(always)]
    pub fn ch(self) -> [self::Ch; 128] {
        unsafe {
            [
                self::Ch(self.0.add(0x2000usize + 0x0usize)),
                self::Ch(self.0.add(0x2000usize + 0x20usize)),
                self::Ch(self.0.add(0x2000usize + 0x40usize)),
                self::Ch(self.0.add(0x2000usize + 0x60usize)),
                self::Ch(self.0.add(0x2000usize + 0x80usize)),
                self::Ch(self.0.add(0x2000usize + 0xa0usize)),
                self::Ch(self.0.add(0x2000usize + 0xc0usize)),
                self::Ch(self.0.add(0x2000usize + 0xe0usize)),
                self::Ch(self.0.add(0x2000usize + 0x100usize)),
                self::Ch(self.0.add(0x2000usize + 0x120usize)),
                self::Ch(self.0.add(0x2000usize + 0x140usize)),
                self::Ch(self.0.add(0x2000usize + 0x160usize)),
                self::Ch(self.0.add(0x2000usize + 0x180usize)),
                self::Ch(self.0.add(0x2000usize + 0x1a0usize)),
                self::Ch(self.0.add(0x2000usize + 0x1c0usize)),
                self::Ch(self.0.add(0x2000usize + 0x1e0usize)),
                self::Ch(self.0.add(0x2000usize + 0x200usize)),
                self::Ch(self.0.add(0x2000usize + 0x220usize)),
                self::Ch(self.0.add(0x2000usize + 0x240usize)),
                self::Ch(self.0.add(0x2000usize + 0x260usize)),
                self::Ch(self.0.add(0x2000usize + 0x280usize)),
                self::Ch(self.0.add(0x2000usize + 0x2a0usize)),
                self::Ch(self.0.add(0x2000usize + 0x2c0usize)),
                self::Ch(self.0.add(0x2000usize + 0x2e0usize)),
                self::Ch(self.0.add(0x2000usize + 0x300usize)),
                self::Ch(self.0.add(0x2000usize + 0x320usize)),
                self::Ch(self.0.add(0x2000usize + 0x340usize)),
                self::Ch(self.0.add(0x2000usize + 0x360usize)),
                self::Ch(self.0.add(0x2000usize + 0x380usize)),
                self::Ch(self.0.add(0x2000usize + 0x3a0usize)),
                self::Ch(self.0.add(0x2000usize + 0x3c0usize)),
                self::Ch(self.0.add(0x2000usize + 0x3e0usize)),
                self::Ch(self.0.add(0x2000usize + 0x400usize)),
                self::Ch(self.0.add(0x2000usize + 0x420usize)),
                self::Ch(self.0.add(0x2000usize + 0x440usize)),
                self::Ch(self.0.add(0x2000usize + 0x460usize)),
                self::Ch(self.0.add(0x2000usize + 0x480usize)),
                self::Ch(self.0.add(0x2000usize + 0x4a0usize)),
                self::Ch(self.0.add(0x2000usize + 0x4c0usize)),
                self::Ch(self.0.add(0x2000usize + 0x4e0usize)),
                self::Ch(self.0.add(0x2000usize + 0x500usize)),
                self::Ch(self.0.add(0x2000usize + 0x520usize)),
                self::Ch(self.0.add(0x2000usize + 0x540usize)),
                self::Ch(self.0.add(0x2000usize + 0x560usize)),
                self::Ch(self.0.add(0x2000usize + 0x580usize)),
                self::Ch(self.0.add(0x2000usize + 0x5a0usize)),
                self::Ch(self.0.add(0x2000usize + 0x5c0usize)),
                self::Ch(self.0.add(0x2000usize + 0x5e0usize)),
                self::Ch(self.0.add(0x2000usize + 0x600usize)),
                self::Ch(self.0.add(0x2000usize + 0x620usize)),
                self::Ch(self.0.add(0x2000usize + 0x640usize)),
                self::Ch(self.0.add(0x2000usize + 0x660usize)),
                self::Ch(self.0.add(0x2000usize + 0x680usize)),
                self::Ch(self.0.add(0x2000usize + 0x6a0usize)),
                self::Ch(self.0.add(0x2000usize + 0x6c0usize)),
                self::Ch(self.0.add(0x2000usize + 0x6e0usize)),
                self::Ch(self.0.add(0x2000usize + 0x700usize)),
                self::Ch(self.0.add(0x2000usize + 0x720usize)),
                self::Ch(self.0.add(0x2000usize + 0x740usize)),
                self::Ch(self.0.add(0x2000usize + 0x760usize)),
                self::Ch(self.0.add(0x2000usize + 0x780usize)),
                self::Ch(self.0.add(0x2000usize + 0x7a0usize)),
                self::Ch(self.0.add(0x2000usize + 0x7c0usize)),
                self::Ch(self.0.add(0x2000usize + 0x7e0usize)),
                self::Ch(self.0.add(0x2000usize + 0x800usize)),
                self::Ch(self.0.add(0x2000usize + 0x820usize)),
                self::Ch(self.0.add(0x2000usize + 0x840usize)),
                self::Ch(self.0.add(0x2000usize + 0x860usize)),
                self::Ch(self.0.add(0x2000usize + 0x880usize)),
                self::Ch(self.0.add(0x2000usize + 0x8a0usize)),
                self::Ch(self.0.add(0x2000usize + 0x8c0usize)),
                self::Ch(self.0.add(0x2000usize + 0x8e0usize)),
                self::Ch(self.0.add(0x2000usize + 0x900usize)),
                self::Ch(self.0.add(0x2000usize + 0x920usize)),
                self::Ch(self.0.add(0x2000usize + 0x940usize)),
                self::Ch(self.0.add(0x2000usize + 0x960usize)),
                self::Ch(self.0.add(0x2000usize + 0x980usize)),
                self::Ch(self.0.add(0x2000usize + 0x9a0usize)),
                self::Ch(self.0.add(0x2000usize + 0x9c0usize)),
                self::Ch(self.0.add(0x2000usize + 0x9e0usize)),
                self::Ch(self.0.add(0x2000usize + 0xa00usize)),
                self::Ch(self.0.add(0x2000usize + 0xa20usize)),
                self::Ch(self.0.add(0x2000usize + 0xa40usize)),
                self::Ch(self.0.add(0x2000usize + 0xa60usize)),
                self::Ch(self.0.add(0x2000usize + 0xa80usize)),
                self::Ch(self.0.add(0x2000usize + 0xaa0usize)),
                self::Ch(self.0.add(0x2000usize + 0xac0usize)),
                self::Ch(self.0.add(0x2000usize + 0xae0usize)),
                self::Ch(self.0.add(0x2000usize + 0xb00usize)),
                self::Ch(self.0.add(0x2000usize + 0xb20usize)),
                self::Ch(self.0.add(0x2000usize + 0xb40usize)),
                self::Ch(self.0.add(0x2000usize + 0xb60usize)),
                self::Ch(self.0.add(0x2000usize + 0xb80usize)),
                self::Ch(self.0.add(0x2000usize + 0xba0usize)),
                self::Ch(self.0.add(0x2000usize + 0xbc0usize)),
                self::Ch(self.0.add(0x2000usize + 0xbe0usize)),
                self::Ch(self.0.add(0x2000usize + 0xc00usize)),
                self::Ch(self.0.add(0x2000usize + 0xc20usize)),
                self::Ch(self.0.add(0x2000usize + 0xc40usize)),
                self::Ch(self.0.add(0x2000usize + 0xc60usize)),
                self::Ch(self.0.add(0x2000usize + 0xc80usize)),
                self::Ch(self.0.add(0x2000usize + 0xca0usize)),
                self::Ch(self.0.add(0x2000usize + 0xcc0usize)),
                self::Ch(self.0.add(0x2000usize + 0xce0usize)),
                self::Ch(self.0.add(0x2000usize + 0xd00usize)),
                self::Ch(self.0.add(0x2000usize + 0xd20usize)),
                self::Ch(self.0.add(0x2000usize + 0xd40usize)),
                self::Ch(self.0.add(0x2000usize + 0xd60usize)),
                self::Ch(self.0.add(0x2000usize + 0xd80usize)),
                self::Ch(self.0.add(0x2000usize + 0xda0usize)),
                self::Ch(self.0.add(0x2000usize + 0xdc0usize)),
                self::Ch(self.0.add(0x2000usize + 0xde0usize)),
                self::Ch(self.0.add(0x2000usize + 0xe00usize)),
                self::Ch(self.0.add(0x2000usize + 0xe20usize)),
                self::Ch(self.0.add(0x2000usize + 0xe40usize)),
                self::Ch(self.0.add(0x2000usize + 0xe60usize)),
                self::Ch(self.0.add(0x2000usize + 0xe80usize)),
                self::Ch(self.0.add(0x2000usize + 0xea0usize)),
                self::Ch(self.0.add(0x2000usize + 0xec0usize)),
                self::Ch(self.0.add(0x2000usize + 0xee0usize)),
                self::Ch(self.0.add(0x2000usize + 0xf00usize)),
                self::Ch(self.0.add(0x2000usize + 0xf20usize)),
                self::Ch(self.0.add(0x2000usize + 0xf40usize)),
                self::Ch(self.0.add(0x2000usize + 0xf60usize)),
                self::Ch(self.0.add(0x2000usize + 0xf80usize)),
                self::Ch(self.0.add(0x2000usize + 0xfa0usize)),
                self::Ch(self.0.add(0x2000usize + 0xfc0usize)),
                self::Ch(self.0.add(0x2000usize + 0xfe0usize)),
            ]
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clc_SPEC;
impl crate::sealed::RegSpec for Clc_SPEC {
    type DataType = u32;
}
#[doc = "DMA Clock Control Register\n resetvalue={Application Reset:0x08}"]
pub type Clc = crate::RegValueT<Clc_SPEC>;

impl Clc {
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control of the DMA"]
    #[inline(always)]
    pub fn disr(self) -> crate::common::RegisterFieldBool<0, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Disable Status Bit   DISS. Bit indicates the current status of the DMA"]
    #[inline(always)]
    pub fn diss(self) -> crate::common::RegisterFieldBool<1, 1, 0, Clc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Clc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Sleep Mode Enable Control   EDIS. Used for DMA sleep mode control."]
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
        <crate::RegValueT<Clc_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clre0_SPEC;
impl crate::sealed::RegSpec for Clre0_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Clear Error Register\n resetvalue={Application Reset:0x0}"]
pub type Clre0 = crate::RegValueT<Clre0_SPEC>;

impl Clre0 {
    #[doc = "Clear ME Source Error   CSER"]
    #[inline(always)]
    pub fn cser(self) -> crate::common::RegisterFieldBool<16, 1, 0, Clre0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Clre0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear ME Destination Error   CDER"]
    #[inline(always)]
    pub fn cder(self) -> crate::common::RegisterFieldBool<17, 1, 0, Clre0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Clre0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear SPB Error   CSPBER"]
    #[inline(always)]
    pub fn cspber(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Clre0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, Clre0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear SRI Error   CSRIER"]
    #[inline(always)]
    pub fn csrier(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Clre0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, Clre0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RAM Error   CRAMER"]
    #[inline(always)]
    pub fn cramer(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Clre0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Clre0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear SLL Error   CSLLER"]
    #[inline(always)]
    pub fn csller(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Clre0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Clre0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear DLL Error   CDLLER"]
    #[inline(always)]
    pub fn cdller(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Clre0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Clre0_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Clre0 {
    #[inline(always)]
    fn default() -> Clre0 {
        <crate::RegValueT<Clre0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clre1_SPEC;
impl crate::sealed::RegSpec for Clre1_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Clear Error Register\n resetvalue={Application Reset:0x0}"]
pub type Clre1 = crate::RegValueT<Clre1_SPEC>;

impl Clre1 {
    #[doc = "Clear ME Source Error   CSER"]
    #[inline(always)]
    pub fn cser(self) -> crate::common::RegisterFieldBool<16, 1, 0, Clre1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Clre1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear ME Destination Error   CDER"]
    #[inline(always)]
    pub fn cder(self) -> crate::common::RegisterFieldBool<17, 1, 0, Clre1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Clre1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear SPB Error   CSPBER"]
    #[inline(always)]
    pub fn cspber(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Clre1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, Clre1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear SRI Error   CSRIER"]
    #[inline(always)]
    pub fn csrier(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Clre1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, Clre1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear RAM Error   CRAMER"]
    #[inline(always)]
    pub fn cramer(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Clre1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Clre1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear SLL Error   CSLLER"]
    #[inline(always)]
    pub fn csller(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Clre1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Clre1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear DLL Error   CDLLER"]
    #[inline(always)]
    pub fn cdller(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Clre1_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Clre1_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Clre1 {
    #[inline(always)]
    fn default() -> Clre1 {
        <crate::RegValueT<Clre1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eer0_SPEC;
impl crate::sealed::RegSpec for Eer0_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Enable Error Register\n resetvalue={Application Reset:0x4030000}"]
pub type Eer0 = crate::RegValueT<Eer0_SPEC>;

impl Eer0 {
    #[doc = "Enable ME Source Error   ESER. This bit enables the generation of a ME source error interrupt."]
    #[inline(always)]
    pub fn eser(self) -> crate::common::RegisterFieldBool<16, 1, 0, Eer0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Eer0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable ME Destination Error   EDER. This bit enables the generation of a ME destination error interrupt."]
    #[inline(always)]
    pub fn eder(self) -> crate::common::RegisterFieldBool<17, 1, 0, Eer0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Eer0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable ME DMA Linked List Error   ELER. This bit enables the generation of a ME DMA Linked List error interrupt."]
    #[inline(always)]
    pub fn eler(self) -> crate::common::RegisterFieldBool<26, 1, 0, Eer0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Eer0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Eer0 {
    #[inline(always)]
    fn default() -> Eer0 {
        <crate::RegValueT<Eer0_SPEC> as RegisterValue<_>>::new(67305472)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eer1_SPEC;
impl crate::sealed::RegSpec for Eer1_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Enable Error Register\n resetvalue={Application Reset:0x4030000}"]
pub type Eer1 = crate::RegValueT<Eer1_SPEC>;

impl Eer1 {
    #[doc = "Enable ME Source Error   ESER. This bit enables the generation of a ME source error interrupt."]
    #[inline(always)]
    pub fn eser(self) -> crate::common::RegisterFieldBool<16, 1, 0, Eer1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Eer1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable ME Destination Error   EDER. This bit enables the generation of a ME destination error interrupt."]
    #[inline(always)]
    pub fn eder(self) -> crate::common::RegisterFieldBool<17, 1, 0, Eer1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Eer1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable ME DMA Linked List Error   ELER. This bit enables the generation of a ME DMA Linked List error interrupt."]
    #[inline(always)]
    pub fn eler(self) -> crate::common::RegisterFieldBool<26, 1, 0, Eer1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Eer1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Eer1 {
    #[inline(always)]
    fn default() -> Eer1 {
        <crate::RegValueT<Eer1_SPEC> as RegisterValue<_>>::new(67305472)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ErrintRr_SPEC;
impl crate::sealed::RegSpec for ErrintRr_SPEC {
    type DataType = u32;
}
#[doc = "RP 0 Error Interrupt Set Register\n resetvalue={Application Reset:0x0}"]
pub type ErrintRr = crate::RegValueT<ErrintRr_SPEC>;

impl ErrintRr {
    #[doc = "Set Error Interrupt Service Request   SIT. Reading this bit returns a 0."]
    #[inline(always)]
    pub fn sit(self) -> crate::common::RegisterFieldBool<0, 1, 0, ErrintRr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, ErrintRr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for ErrintRr {
    #[inline(always)]
    fn default() -> ErrintRr {
        <crate::RegValueT<ErrintRr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errsr0_SPEC;
impl crate::sealed::RegSpec for Errsr0_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Error Status Register\n resetvalue={Application Reset:0x0}"]
pub type Errsr0 = crate::RegValueT<Errsr0_SPEC>;

impl Errsr0 {
    #[doc = "ME Last Error Channel   LEC. This bit field indicates the DMA channel number of the last DMA channel        of ME generating an error i.e. RAM error DMA ERRSRm.RAMER  Safe Linked        List error DMA ERRSRm.SLLER  DMA Linked List error DMA ERRSRm.DLLER and        all on  160 chip  160 bus errors."]
    #[inline(always)]
    pub fn lec(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Errsr0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7f,1,0,u8, Errsr0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ME Source Error   SER. This bit is set whenever a ME error occurred during a source  read  move        of a DMA transfer  or a request could not been serviced due to the        access protection."]
    #[inline(always)]
    pub fn ser(self) -> crate::common::RegisterFieldBool<16, 1, 0, Errsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Errsr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ME Destination Error   DER. This bit is set whenever a ME error occurred during a destination         write  move of a DMA transfer  or a request could not been serviced due        to the access protection."]
    #[inline(always)]
    pub fn der(self) -> crate::common::RegisterFieldBool<17, 1, 0, Errsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Errsr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ME SPB Bus Error   SPBER. This bit is set when a ME DMA Move that has been started by the DMA SPB        master interface leads to an error on the SPB  160 Bus."]
    #[inline(always)]
    pub fn spber(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Errsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Errsr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ME SRI Bus Error   SRIER. This bit is set when a ME DMA Move that has been started by the DMA SRI        master interface leads to an error on the SRI  160 Bus."]
    #[inline(always)]
    pub fn srier(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Errsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Errsr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ME RAM Error   RAMER. This bit is set whenever a ME error occurred during the loading of a TCS        from the DMARAM to the ME channel registers."]
    #[inline(always)]
    pub fn ramer(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Errsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Errsr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ME Safe Linked List Error   SLLER. This bit is set when a ME error occurred during the comparison of a        SDCRC checksums during a safe linked list."]
    #[inline(always)]
    pub fn sller(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Errsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Errsr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ME DMA Linked List Error   DLLER. This bit is set when a ME error occurred during a DMALL  ACCLL  SAFLL or        CONLL operation when a new TCS is loaded from anywhere in memory to        overwrite the current TCS stored in the DMARAM."]
    #[inline(always)]
    pub fn dller(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Errsr0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Errsr0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Errsr0 {
    #[inline(always)]
    fn default() -> Errsr0 {
        <crate::RegValueT<Errsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errsr1_SPEC;
impl crate::sealed::RegSpec for Errsr1_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Error Status Register\n resetvalue={Application Reset:0x0}"]
pub type Errsr1 = crate::RegValueT<Errsr1_SPEC>;

impl Errsr1 {
    #[doc = "ME Last Error Channel   LEC. This bit field indicates the DMA channel number of the last DMA channel        of ME generating an error i.e. RAM error DMA ERRSRm.RAMER  Safe Linked        List error DMA ERRSRm.SLLER  DMA Linked List error DMA ERRSRm.DLLER and        all on  160 chip  160 bus errors."]
    #[inline(always)]
    pub fn lec(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Errsr1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7f,1,0,u8, Errsr1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ME Source Error   SER. This bit is set whenever a ME error occurred during a source  read  move        of a DMA transfer  or a request could not been serviced due to the        access protection."]
    #[inline(always)]
    pub fn ser(self) -> crate::common::RegisterFieldBool<16, 1, 0, Errsr1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Errsr1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ME Destination Error   DER. This bit is set whenever a ME error occurred during a destination         write  move of a DMA transfer  or a request could not been serviced due        to the access protection."]
    #[inline(always)]
    pub fn der(self) -> crate::common::RegisterFieldBool<17, 1, 0, Errsr1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Errsr1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ME SPB Bus Error   SPBER. This bit is set when a ME DMA Move that has been started by the DMA SPB        master interface leads to an error on the SPB  160 Bus."]
    #[inline(always)]
    pub fn spber(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Errsr1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Errsr1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ME SRI Bus Error   SRIER. This bit is set when a ME DMA Move that has been started by the DMA SRI        master interface leads to an error on the SRI  160 Bus."]
    #[inline(always)]
    pub fn srier(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Errsr1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Errsr1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ME RAM Error   RAMER. This bit is set whenever a ME error occurred during the loading of a TCS        from the DMARAM to the ME channel registers."]
    #[inline(always)]
    pub fn ramer(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Errsr1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Errsr1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ME Safe Linked List Error   SLLER. This bit is set when a ME error occurred during the comparison of a        SDCRC checksums during a safe linked list."]
    #[inline(always)]
    pub fn sller(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Errsr1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Errsr1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ME DMA Linked List Error   DLLER. This bit is set when a ME error occurred during a DMALL  ACCLL  SAFLL or        CONLL operation when a new TCS is loaded from anywhere in memory to        overwrite the current TCS stored in the DMARAM."]
    #[inline(always)]
    pub fn dller(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Errsr1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Errsr1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Errsr1 {
    #[inline(always)]
    fn default() -> Errsr1 {
        <crate::RegValueT<Errsr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HrRc_SPEC;
impl crate::sealed::RegSpec for HrRc_SPEC {
    type DataType = u32;
}
#[doc = "DMA Channel 000 Resource Partition Register\n resetvalue={Application Reset:0x0}"]
pub type HrRc = crate::RegValueT<HrRc_SPEC>;

impl HrRc {
    #[doc = "DMA Channel Resource Partition   HRP"]
    #[inline(always)]
    pub fn hrp(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, HrRc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, HrRc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for HrRc {
    #[inline(always)]
    fn default() -> HrRc {
        <crate::RegValueT<HrRc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "DMA Identification Register\n resetvalue={Application Reset:0x087C000}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MOD REV. This bit field defines the module revision number. See DMA Design Specification for MOD REV value."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MOD TYPE. The bit field is set to C0 H which        defines the module as a 32 bit module."]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MOD NUMBER. This bit field defines a module identification number."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(8896512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me00R_SPEC;
impl crate::sealed::RegSpec for Me00R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 0\n resetvalue={Application Reset:0x0}"]
pub type Me00R = crate::RegValueT<Me00R_SPEC>;

impl Me00R {
    #[doc = "DMA Read Move Data Byte   RD00"]
    #[inline(always)]
    pub fn rd00(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me00R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me00R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD01"]
    #[inline(always)]
    pub fn rd01(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me00R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me00R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD02"]
    #[inline(always)]
    pub fn rd02(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me00R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me00R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD03"]
    #[inline(always)]
    pub fn rd03(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me00R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me00R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me00R {
    #[inline(always)]
    fn default() -> Me00R {
        <crate::RegValueT<Me00R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me01R_SPEC;
impl crate::sealed::RegSpec for Me01R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 1\n resetvalue={Application Reset:0x0}"]
pub type Me01R = crate::RegValueT<Me01R_SPEC>;

impl Me01R {
    #[doc = "DMA Read Move Data Byte   RD10"]
    #[inline(always)]
    pub fn rd10(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me01R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me01R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD11"]
    #[inline(always)]
    pub fn rd11(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me01R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me01R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD12"]
    #[inline(always)]
    pub fn rd12(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me01R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me01R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD13"]
    #[inline(always)]
    pub fn rd13(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me01R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me01R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me01R {
    #[inline(always)]
    fn default() -> Me01R {
        <crate::RegValueT<Me01R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me02R_SPEC;
impl crate::sealed::RegSpec for Me02R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 2\n resetvalue={Application Reset:0x0}"]
pub type Me02R = crate::RegValueT<Me02R_SPEC>;

impl Me02R {
    #[doc = "DMA Read Move Data Byte   RD20"]
    #[inline(always)]
    pub fn rd20(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me02R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me02R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD21"]
    #[inline(always)]
    pub fn rd21(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me02R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me02R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD22"]
    #[inline(always)]
    pub fn rd22(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me02R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me02R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD23"]
    #[inline(always)]
    pub fn rd23(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me02R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me02R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me02R {
    #[inline(always)]
    fn default() -> Me02R {
        <crate::RegValueT<Me02R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me03R_SPEC;
impl crate::sealed::RegSpec for Me03R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 3\n resetvalue={Application Reset:0x0}"]
pub type Me03R = crate::RegValueT<Me03R_SPEC>;

impl Me03R {
    #[doc = "DMA Read Move Data Byte   RD30"]
    #[inline(always)]
    pub fn rd30(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me03R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me03R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD31"]
    #[inline(always)]
    pub fn rd31(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me03R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me03R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD32"]
    #[inline(always)]
    pub fn rd32(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me03R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me03R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD33"]
    #[inline(always)]
    pub fn rd33(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me03R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me03R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me03R {
    #[inline(always)]
    fn default() -> Me03R {
        <crate::RegValueT<Me03R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me04R_SPEC;
impl crate::sealed::RegSpec for Me04R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 4\n resetvalue={Application Reset:0x0}"]
pub type Me04R = crate::RegValueT<Me04R_SPEC>;

impl Me04R {
    #[doc = "DMA Read Move Data Byte   RD40"]
    #[inline(always)]
    pub fn rd40(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me04R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me04R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD41"]
    #[inline(always)]
    pub fn rd41(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me04R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me04R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD42"]
    #[inline(always)]
    pub fn rd42(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me04R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me04R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD43"]
    #[inline(always)]
    pub fn rd43(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me04R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me04R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me04R {
    #[inline(always)]
    fn default() -> Me04R {
        <crate::RegValueT<Me04R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me05R_SPEC;
impl crate::sealed::RegSpec for Me05R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 5\n resetvalue={Application Reset:0x0}"]
pub type Me05R = crate::RegValueT<Me05R_SPEC>;

impl Me05R {
    #[doc = "DMA Read Move Data Byte   RD50"]
    #[inline(always)]
    pub fn rd50(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me05R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me05R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD51"]
    #[inline(always)]
    pub fn rd51(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me05R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me05R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD52"]
    #[inline(always)]
    pub fn rd52(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me05R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me05R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD53"]
    #[inline(always)]
    pub fn rd53(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me05R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me05R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me05R {
    #[inline(always)]
    fn default() -> Me05R {
        <crate::RegValueT<Me05R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me06R_SPEC;
impl crate::sealed::RegSpec for Me06R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 6\n resetvalue={Application Reset:0x0}"]
pub type Me06R = crate::RegValueT<Me06R_SPEC>;

impl Me06R {
    #[doc = "DMA Read Move Data Byte   RD60"]
    #[inline(always)]
    pub fn rd60(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me06R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me06R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD61"]
    #[inline(always)]
    pub fn rd61(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me06R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me06R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD62"]
    #[inline(always)]
    pub fn rd62(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me06R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me06R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD63"]
    #[inline(always)]
    pub fn rd63(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me06R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me06R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me06R {
    #[inline(always)]
    fn default() -> Me06R {
        <crate::RegValueT<Me06R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me07R_SPEC;
impl crate::sealed::RegSpec for Me07R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 7\n resetvalue={Application Reset:0x0}"]
pub type Me07R = crate::RegValueT<Me07R_SPEC>;

impl Me07R {
    #[doc = "DMA Read Move Data Byte   RD70"]
    #[inline(always)]
    pub fn rd70(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me07R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me07R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD71"]
    #[inline(always)]
    pub fn rd71(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me07R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me07R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD72"]
    #[inline(always)]
    pub fn rd72(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me07R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me07R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD73"]
    #[inline(always)]
    pub fn rd73(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me07R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me07R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me07R {
    #[inline(always)]
    fn default() -> Me07R {
        <crate::RegValueT<Me07R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me0Adicr_SPEC;
impl crate::sealed::RegSpec for Me0Adicr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Address and Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
pub type Me0Adicr = crate::RegValueT<Me0Adicr_SPEC>;

impl Me0Adicr {
    #[doc = "Source Address Modification Factor   SMF. Active DMA channel source address modification factor."]
    #[inline(always)]
    pub fn smf(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Me0Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Increment of Source Address   INCS. Active DMA channel increment of source address control."]
    #[inline(always)]
    pub fn incs(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Me0Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Destination Address Modification Factor   DMF. Active DMA channel destination address modification factor."]
    #[inline(always)]
    pub fn dmf(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x7,1,0,u8, Me0Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Increment of Destination Address   INCD. Active DMA channel increment of destination address control."]
    #[inline(always)]
    pub fn incd(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Me0Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Circular Buffer Length Source   CBLS. Active DMA channel circular source buffer control."]
    #[inline(always)]
    pub fn cbls(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Me0Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Circular Buffer Length Destination   CBLD. Active DMA channel circular destination buffer control."]
    #[inline(always)]
    pub fn cbld(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Me0Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Shadow Control   SHCT. Active DMA channel control of shadow address register function."]
    #[inline(always)]
    pub fn shct(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Me0Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Source Circular Buffer Enable   SCBE. Active DMA channel circular source buffer enable disable."]
    #[inline(always)]
    pub fn scbe(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Me0Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Destination Circular Buffer Enable   DCBE. Active DMA channel circular destination buffer enable disable."]
    #[inline(always)]
    pub fn dcbe(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Me0Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Time Stamp   STAMP. Active DMA channel control to enable the appendage of a timestamp after        the end of the last DMA Move during a DMA transaction."]
    #[inline(always)]
    pub fn stamp(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Me0Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wrap Source Enable   WRPSE. Active DMA channel source buffer interrupt trigger enable disable."]
    #[inline(always)]
    pub fn wrpse(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Me0Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wrap Destination Enable   WRPDE. Active DMA channel destination buffer interrupt trigger enable disable."]
    #[inline(always)]
    pub fn wrpde(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Me0Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Control   INTCT. Active DMA channel interrupt service request control."]
    #[inline(always)]
    pub fn intct(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<26,0x3,1,0,u8, Me0Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Interrupt Raise Detect Value   IRDV. Active DMA channel control of the Threshold Limit of of DMA channel        CHSR.TCOUNT for triggering a channel interrupt service request."]
    #[inline(always)]
    pub fn irdv(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Me0Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me0Adicr {
    #[inline(always)]
    fn default() -> Me0Adicr {
        <crate::RegValueT<Me0Adicr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me0Chcr_SPEC;
impl crate::sealed::RegSpec for Me0Chcr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Control Register\n resetvalue={Application Reset:0x0}"]
pub type Me0Chcr = crate::RegValueT<Me0Chcr_SPEC>;

impl Me0Chcr {
    #[doc = "Transfer Reload Value   TREL. Active DMA channel Transfer Reload Value."]
    #[inline(always)]
    pub fn trel(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Me0Chcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Me0Chcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Block Mode   BLKM. Active DMA channel Block Mode."]
    #[inline(always)]
    pub fn blkm(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Me0Chcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Me0Chcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Request Only After Transaction   RROAT. Active DMA channel Reset Request Only After Transaction."]
    #[inline(always)]
    pub fn rroat(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Me0Chcr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Me0Chcr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel Operation Mode   CHMODE. Active DMA channel Channel Operation Mode."]
    #[inline(always)]
    pub fn chmode(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Me0Chcr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Me0Chcr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel Data Width   CHDW. Active DMA channel DMA move Channel Data Width."]
    #[inline(always)]
    pub fn chdw(
        self,
    ) -> crate::common::RegisterField<21, 0x7, 1, 0, u8, Me0Chcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<21,0x7,1,0,u8, Me0Chcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Pattern Select   PATSEL. Active DMA channel Pattern Select control."]
    #[inline(always)]
    pub fn patsel(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Me0Chcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Me0Chcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Swap Data CRC byte order   SWAP. Active DMA channel swap data CRC byte order."]
    #[inline(always)]
    pub fn swap(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Me0Chcr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Me0Chcr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Peripheral Request Select   PRSEL. Active DMA channel Peripheral Request Select."]
    #[inline(always)]
    pub fn prsel(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Me0Chcr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Me0Chcr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Me0Chcr {
    #[inline(always)]
    fn default() -> Me0Chcr {
        <crate::RegValueT<Me0Chcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me0Chsr_SPEC;
impl crate::sealed::RegSpec for Me0Chsr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Status Register\n resetvalue={Application Reset:0x0}"]
pub type Me0Chsr = crate::RegValueT<Me0Chsr_SPEC>;

impl Me0Chsr {
    #[doc = "Transfer Count Status   TCOUNT. Active DMA channel count of the number of DMA transfers. TCOUNT is        loaded with the DMA channel value of CHCFGR.TREL when TSR.CH becomes set         and TCOUNT  160    160 0 . After each DMA transfer  TCOUNT is decremented by 1."]
    #[inline(always)]
    pub fn tcount(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Me0Chsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Me0Chsr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Old Value of Pattern Detection   LXO. Active DMA channel compare result of a pattern compare operation when        8 bit or 16 bit data width is selected."]
    #[inline(always)]
    pub fn lxo(self) -> crate::common::RegisterFieldBool<15, 1, 0, Me0Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Me0Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wrap Source Buffer   WRPS. Active DMA channel Wrap Source Buffer status bit."]
    #[inline(always)]
    pub fn wrps(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Me0Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Me0Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wrap Destination Buffer   WRPD. Active DMA channel Wrap Destination Buffer status bit."]
    #[inline(always)]
    pub fn wrpd(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Me0Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Me0Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt from Channel   ICH. Active DMA channel detection of channel interrupt service request."]
    #[inline(always)]
    pub fn ich(self) -> crate::common::RegisterFieldBool<18, 1, 0, Me0Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Me0Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pattern Detection from Channel   IPM. Active DMA channel detection of pattern match interrupt service request."]
    #[inline(always)]
    pub fn ipm(self) -> crate::common::RegisterFieldBool<19, 1, 0, Me0Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Me0Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "DMA Double Buffering Active Buffer   BUFFER. Active DMA channel DMA Double Buffering Active Buffer status bit."]
    #[inline(always)]
    pub fn buffer(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Me0Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Me0Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "DMA Double Buffering Frozen Buffer   FROZEN. Active DMA channel DMA Double Buffering Frozen Buffer status bit."]
    #[inline(always)]
    pub fn frozen(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Me0Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Me0Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Me0Chsr {
    #[inline(always)]
    fn default() -> Me0Chsr {
        <crate::RegValueT<Me0Chsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me0Dadr_SPEC;
impl crate::sealed::RegSpec for Me0Dadr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Destination Address Register\n resetvalue={Application Reset:0x0}"]
pub type Me0Dadr = crate::RegValueT<Me0Dadr_SPEC>;

impl Me0Dadr {
    #[doc = "Destination Address   DADR. Active DMA channel 32 bit destination address used for DMA write moves."]
    #[inline(always)]
    pub fn dadr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Me0Dadr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Me0Dadr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me0Dadr {
    #[inline(always)]
    fn default() -> Me0Dadr {
        <crate::RegValueT<Me0Dadr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me0Rdcrc_SPEC;
impl crate::sealed::RegSpec for Me0Rdcrc_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Read Data CRC Register\n resetvalue={Application Reset:0x0}"]
pub type Me0Rdcrc = crate::RegValueT<Me0Rdcrc_SPEC>;

impl Me0Rdcrc {
    #[doc = "Read Data CRC   RDCRC. Active DMA channel read data CRC32 ethernet polynomial checksum."]
    #[inline(always)]
    pub fn rdcrc(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Me0Rdcrc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Me0Rdcrc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me0Rdcrc {
    #[inline(always)]
    fn default() -> Me0Rdcrc {
        <crate::RegValueT<Me0Rdcrc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me0Sadr_SPEC;
impl crate::sealed::RegSpec for Me0Sadr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Source Address Register\n resetvalue={Application Reset:0x0}"]
pub type Me0Sadr = crate::RegValueT<Me0Sadr_SPEC>;

impl Me0Sadr {
    #[doc = "Source Address   SADR. Active DMA channel 32 bit source address used for DMA read moves."]
    #[inline(always)]
    pub fn sadr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Me0Sadr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Me0Sadr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me0Sadr {
    #[inline(always)]
    fn default() -> Me0Sadr {
        <crate::RegValueT<Me0Sadr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me0Sdcrc_SPEC;
impl crate::sealed::RegSpec for Me0Sdcrc_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Source and Destination Address CRC Register\n resetvalue={Application Reset:0x0}"]
pub type Me0Sdcrc = crate::RegValueT<Me0Sdcrc_SPEC>;

impl Me0Sdcrc {
    #[doc = "Source and Destination Address CRC   SDCRC. Active DMA channel source and destination address CRC32 ethernet polynomial checksum."]
    #[inline(always)]
    pub fn sdcrc(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Me0Sdcrc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Me0Sdcrc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me0Sdcrc {
    #[inline(always)]
    fn default() -> Me0Sdcrc {
        <crate::RegValueT<Me0Sdcrc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me0Shadr_SPEC;
impl crate::sealed::RegSpec for Me0Shadr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Shadow Address Register\n resetvalue={Application Reset:0x0}"]
pub type Me0Shadr = crate::RegValueT<Me0Shadr_SPEC>;

impl Me0Shadr {
    #[doc = "Shadowed Address   SHADR. This bit field holds the 32 bit shadow address of the active DMA        channel. The function of the shadow address is set by the shadow control        settings."]
    #[inline(always)]
    pub fn shadr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Me0Shadr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Me0Shadr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me0Shadr {
    #[inline(always)]
    fn default() -> Me0Shadr {
        <crate::RegValueT<Me0Shadr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me0Sr_SPEC;
impl crate::sealed::RegSpec for Me0Sr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Status Register\n resetvalue={Application Reset:0x0}"]
pub type Me0Sr = crate::RegValueT<Me0Sr_SPEC>;

impl Me0Sr {
    #[doc = "ME Read Status   RS"]
    #[inline(always)]
    pub fn rs(self) -> crate::common::RegisterFieldBool<0, 1, 0, Me0Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Me0Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ME Write Status   WS"]
    #[inline(always)]
    pub fn ws(self) -> crate::common::RegisterFieldBool<4, 1, 0, Me0Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Me0Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ME Active Channel   CH. Indicates the number of the DMA Channel currently processed by the ME."]
    #[inline(always)]
    pub fn ch(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Me0Sr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Me0Sr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me0Sr {
    #[inline(always)]
    fn default() -> Me0Sr {
        <crate::RegValueT<Me0Sr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me10R_SPEC;
impl crate::sealed::RegSpec for Me10R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 0\n resetvalue={Application Reset:0x0}"]
pub type Me10R = crate::RegValueT<Me10R_SPEC>;

impl Me10R {
    #[doc = "DMA Read Move Data Byte   RD00"]
    #[inline(always)]
    pub fn rd00(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me10R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me10R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD01"]
    #[inline(always)]
    pub fn rd01(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me10R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me10R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD02"]
    #[inline(always)]
    pub fn rd02(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me10R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me10R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD03"]
    #[inline(always)]
    pub fn rd03(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me10R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me10R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me10R {
    #[inline(always)]
    fn default() -> Me10R {
        <crate::RegValueT<Me10R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me11R_SPEC;
impl crate::sealed::RegSpec for Me11R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 1\n resetvalue={Application Reset:0x0}"]
pub type Me11R = crate::RegValueT<Me11R_SPEC>;

impl Me11R {
    #[doc = "DMA Read Move Data Byte   RD10"]
    #[inline(always)]
    pub fn rd10(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me11R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me11R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD11"]
    #[inline(always)]
    pub fn rd11(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me11R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me11R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD12"]
    #[inline(always)]
    pub fn rd12(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me11R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me11R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD13"]
    #[inline(always)]
    pub fn rd13(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me11R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me11R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me11R {
    #[inline(always)]
    fn default() -> Me11R {
        <crate::RegValueT<Me11R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me12R_SPEC;
impl crate::sealed::RegSpec for Me12R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 2\n resetvalue={Application Reset:0x0}"]
pub type Me12R = crate::RegValueT<Me12R_SPEC>;

impl Me12R {
    #[doc = "DMA Read Move Data Byte   RD20"]
    #[inline(always)]
    pub fn rd20(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me12R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me12R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD21"]
    #[inline(always)]
    pub fn rd21(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me12R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me12R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD22"]
    #[inline(always)]
    pub fn rd22(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me12R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me12R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD23"]
    #[inline(always)]
    pub fn rd23(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me12R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me12R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me12R {
    #[inline(always)]
    fn default() -> Me12R {
        <crate::RegValueT<Me12R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me13R_SPEC;
impl crate::sealed::RegSpec for Me13R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 3\n resetvalue={Application Reset:0x0}"]
pub type Me13R = crate::RegValueT<Me13R_SPEC>;

impl Me13R {
    #[doc = "DMA Read Move Data Byte   RD30"]
    #[inline(always)]
    pub fn rd30(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me13R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me13R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD31"]
    #[inline(always)]
    pub fn rd31(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me13R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me13R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD32"]
    #[inline(always)]
    pub fn rd32(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me13R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me13R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD33"]
    #[inline(always)]
    pub fn rd33(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me13R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me13R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me13R {
    #[inline(always)]
    fn default() -> Me13R {
        <crate::RegValueT<Me13R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me14R_SPEC;
impl crate::sealed::RegSpec for Me14R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 4\n resetvalue={Application Reset:0x0}"]
pub type Me14R = crate::RegValueT<Me14R_SPEC>;

impl Me14R {
    #[doc = "DMA Read Move Data Byte   RD40"]
    #[inline(always)]
    pub fn rd40(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me14R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me14R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD41"]
    #[inline(always)]
    pub fn rd41(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me14R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me14R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD42"]
    #[inline(always)]
    pub fn rd42(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me14R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me14R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD43"]
    #[inline(always)]
    pub fn rd43(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me14R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me14R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me14R {
    #[inline(always)]
    fn default() -> Me14R {
        <crate::RegValueT<Me14R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me15R_SPEC;
impl crate::sealed::RegSpec for Me15R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 5\n resetvalue={Application Reset:0x0}"]
pub type Me15R = crate::RegValueT<Me15R_SPEC>;

impl Me15R {
    #[doc = "DMA Read Move Data Byte   RD50"]
    #[inline(always)]
    pub fn rd50(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me15R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me15R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD51"]
    #[inline(always)]
    pub fn rd51(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me15R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me15R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD52"]
    #[inline(always)]
    pub fn rd52(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me15R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me15R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD53"]
    #[inline(always)]
    pub fn rd53(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me15R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me15R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me15R {
    #[inline(always)]
    fn default() -> Me15R {
        <crate::RegValueT<Me15R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me16R_SPEC;
impl crate::sealed::RegSpec for Me16R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 6\n resetvalue={Application Reset:0x0}"]
pub type Me16R = crate::RegValueT<Me16R_SPEC>;

impl Me16R {
    #[doc = "DMA Read Move Data Byte   RD60"]
    #[inline(always)]
    pub fn rd60(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me16R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me16R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD61"]
    #[inline(always)]
    pub fn rd61(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me16R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me16R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD62"]
    #[inline(always)]
    pub fn rd62(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me16R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me16R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD63"]
    #[inline(always)]
    pub fn rd63(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me16R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me16R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me16R {
    #[inline(always)]
    fn default() -> Me16R {
        <crate::RegValueT<Me16R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me17R_SPEC;
impl crate::sealed::RegSpec for Me17R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 7\n resetvalue={Application Reset:0x0}"]
pub type Me17R = crate::RegValueT<Me17R_SPEC>;

impl Me17R {
    #[doc = "DMA Read Move Data Byte   RD70"]
    #[inline(always)]
    pub fn rd70(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me17R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me17R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD71"]
    #[inline(always)]
    pub fn rd71(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me17R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me17R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD72"]
    #[inline(always)]
    pub fn rd72(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me17R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me17R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD73"]
    #[inline(always)]
    pub fn rd73(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me17R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me17R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me17R {
    #[inline(always)]
    fn default() -> Me17R {
        <crate::RegValueT<Me17R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me1Adicr_SPEC;
impl crate::sealed::RegSpec for Me1Adicr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Address and Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
pub type Me1Adicr = crate::RegValueT<Me1Adicr_SPEC>;

impl Me1Adicr {
    #[doc = "Source Address Modification Factor   SMF. Active DMA channel source address modification factor."]
    #[inline(always)]
    pub fn smf(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Me1Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Increment of Source Address   INCS. Active DMA channel increment of source address control."]
    #[inline(always)]
    pub fn incs(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Me1Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Destination Address Modification Factor   DMF. Active DMA channel destination address modification factor."]
    #[inline(always)]
    pub fn dmf(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x7,1,0,u8, Me1Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Increment of Destination Address   INCD. Active DMA channel increment of destination address control."]
    #[inline(always)]
    pub fn incd(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Me1Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Circular Buffer Length Source   CBLS. Active DMA channel circular source buffer control."]
    #[inline(always)]
    pub fn cbls(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Me1Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Circular Buffer Length Destination   CBLD. Active DMA channel circular destination buffer control."]
    #[inline(always)]
    pub fn cbld(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Me1Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Shadow Control   SHCT. Active DMA channel control of shadow address register function."]
    #[inline(always)]
    pub fn shct(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Me1Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Source Circular Buffer Enable   SCBE. Active DMA channel circular source buffer enable disable."]
    #[inline(always)]
    pub fn scbe(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Me1Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Destination Circular Buffer Enable   DCBE. Active DMA channel circular destination buffer enable disable."]
    #[inline(always)]
    pub fn dcbe(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Me1Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Time Stamp   STAMP. Active DMA channel control to enable the appendage of a timestamp after        the end of the last DMA Move during a DMA transaction."]
    #[inline(always)]
    pub fn stamp(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Me1Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wrap Source Enable   WRPSE. Active DMA channel source buffer interrupt trigger enable disable."]
    #[inline(always)]
    pub fn wrpse(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Me1Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wrap Destination Enable   WRPDE. Active DMA channel destination buffer interrupt trigger enable disable."]
    #[inline(always)]
    pub fn wrpde(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Me1Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Control   INTCT. Active DMA channel interrupt service request control."]
    #[inline(always)]
    pub fn intct(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<26,0x3,1,0,u8, Me1Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Interrupt Raise Detect Value   IRDV. Active DMA channel control of the Threshold Limit of of DMA channel        CHSR.TCOUNT for triggering a channel interrupt service request."]
    #[inline(always)]
    pub fn irdv(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Me1Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me1Adicr {
    #[inline(always)]
    fn default() -> Me1Adicr {
        <crate::RegValueT<Me1Adicr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me1Chcr_SPEC;
impl crate::sealed::RegSpec for Me1Chcr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Control Register\n resetvalue={Application Reset:0x0}"]
pub type Me1Chcr = crate::RegValueT<Me1Chcr_SPEC>;

impl Me1Chcr {
    #[doc = "Transfer Reload Value   TREL. Active DMA channel Transfer Reload Value."]
    #[inline(always)]
    pub fn trel(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Me1Chcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Me1Chcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Block Mode   BLKM. Active DMA channel Block Mode."]
    #[inline(always)]
    pub fn blkm(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Me1Chcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Me1Chcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Request Only After Transaction   RROAT. Active DMA channel Reset Request Only After Transaction."]
    #[inline(always)]
    pub fn rroat(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Me1Chcr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Me1Chcr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel Operation Mode   CHMODE. Active DMA channel Channel Operation Mode."]
    #[inline(always)]
    pub fn chmode(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Me1Chcr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Me1Chcr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel Data Width   CHDW. Active DMA channel DMA move Channel Data Width."]
    #[inline(always)]
    pub fn chdw(
        self,
    ) -> crate::common::RegisterField<21, 0x7, 1, 0, u8, Me1Chcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<21,0x7,1,0,u8, Me1Chcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Pattern Select   PATSEL. Active DMA channel Pattern Select control."]
    #[inline(always)]
    pub fn patsel(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Me1Chcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Me1Chcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Swap Data CRC byte order   SWAP. Active DMA channel swap data CRC byte order."]
    #[inline(always)]
    pub fn swap(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Me1Chcr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Me1Chcr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Peripheral Request Select   PRSEL. Active DMA channel Peripheral Request Select."]
    #[inline(always)]
    pub fn prsel(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Me1Chcr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Me1Chcr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Me1Chcr {
    #[inline(always)]
    fn default() -> Me1Chcr {
        <crate::RegValueT<Me1Chcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me1Chsr_SPEC;
impl crate::sealed::RegSpec for Me1Chsr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Status Register\n resetvalue={Application Reset:0x0}"]
pub type Me1Chsr = crate::RegValueT<Me1Chsr_SPEC>;

impl Me1Chsr {
    #[doc = "Transfer Count Status   TCOUNT. Active DMA channel count of the number of DMA transfers. TCOUNT is        loaded with the DMA channel value of CHCFGR.TREL when TSR.CH becomes set         and TCOUNT  160    160 0 . After each DMA transfer  TCOUNT is decremented by 1."]
    #[inline(always)]
    pub fn tcount(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Me1Chsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Me1Chsr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Old Value of Pattern Detection   LXO. Active DMA channel compare result of a pattern compare operation when        8 bit or 16 bit data width is selected."]
    #[inline(always)]
    pub fn lxo(self) -> crate::common::RegisterFieldBool<15, 1, 0, Me1Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Me1Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wrap Source Buffer   WRPS. Active DMA channel Wrap Source Buffer status bit."]
    #[inline(always)]
    pub fn wrps(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Me1Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Me1Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wrap Destination Buffer   WRPD. Active DMA channel Wrap Destination Buffer status bit."]
    #[inline(always)]
    pub fn wrpd(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Me1Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Me1Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt from Channel   ICH. Active DMA channel detection of channel interrupt service request."]
    #[inline(always)]
    pub fn ich(self) -> crate::common::RegisterFieldBool<18, 1, 0, Me1Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Me1Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pattern Detection from Channel   IPM. Active DMA channel detection of pattern match interrupt service request."]
    #[inline(always)]
    pub fn ipm(self) -> crate::common::RegisterFieldBool<19, 1, 0, Me1Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Me1Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "DMA Double Buffering Active Buffer   BUFFER. Active DMA channel DMA Double Buffering Active Buffer status bit."]
    #[inline(always)]
    pub fn buffer(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Me1Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Me1Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "DMA Double Buffering Frozen Buffer   FROZEN. Active DMA channel DMA Double Buffering Frozen Buffer status bit."]
    #[inline(always)]
    pub fn frozen(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Me1Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Me1Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Me1Chsr {
    #[inline(always)]
    fn default() -> Me1Chsr {
        <crate::RegValueT<Me1Chsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me1Dadr_SPEC;
impl crate::sealed::RegSpec for Me1Dadr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Destination Address Register\n resetvalue={Application Reset:0x0}"]
pub type Me1Dadr = crate::RegValueT<Me1Dadr_SPEC>;

impl Me1Dadr {
    #[doc = "Destination Address   DADR. Active DMA channel 32 bit destination address used for DMA write moves."]
    #[inline(always)]
    pub fn dadr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Me1Dadr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Me1Dadr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me1Dadr {
    #[inline(always)]
    fn default() -> Me1Dadr {
        <crate::RegValueT<Me1Dadr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me1Rdcrc_SPEC;
impl crate::sealed::RegSpec for Me1Rdcrc_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Read Data CRC Register\n resetvalue={Application Reset:0x0}"]
pub type Me1Rdcrc = crate::RegValueT<Me1Rdcrc_SPEC>;

impl Me1Rdcrc {
    #[doc = "Read Data CRC   RDCRC. Active DMA channel read data CRC32 ethernet polynomial checksum."]
    #[inline(always)]
    pub fn rdcrc(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Me1Rdcrc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Me1Rdcrc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me1Rdcrc {
    #[inline(always)]
    fn default() -> Me1Rdcrc {
        <crate::RegValueT<Me1Rdcrc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me1Sadr_SPEC;
impl crate::sealed::RegSpec for Me1Sadr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Source Address Register\n resetvalue={Application Reset:0x0}"]
pub type Me1Sadr = crate::RegValueT<Me1Sadr_SPEC>;

impl Me1Sadr {
    #[doc = "Source Address   SADR. Active DMA channel 32 bit source address used for DMA read moves."]
    #[inline(always)]
    pub fn sadr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Me1Sadr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Me1Sadr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me1Sadr {
    #[inline(always)]
    fn default() -> Me1Sadr {
        <crate::RegValueT<Me1Sadr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me1Sdcrc_SPEC;
impl crate::sealed::RegSpec for Me1Sdcrc_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Source and Destination Address CRC Register\n resetvalue={Application Reset:0x0}"]
pub type Me1Sdcrc = crate::RegValueT<Me1Sdcrc_SPEC>;

impl Me1Sdcrc {
    #[doc = "Source and Destination Address CRC   SDCRC. Active DMA channel source and destination address CRC32 ethernet polynomial checksum."]
    #[inline(always)]
    pub fn sdcrc(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Me1Sdcrc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Me1Sdcrc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me1Sdcrc {
    #[inline(always)]
    fn default() -> Me1Sdcrc {
        <crate::RegValueT<Me1Sdcrc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me1Shadr_SPEC;
impl crate::sealed::RegSpec for Me1Shadr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Shadow Address Register\n resetvalue={Application Reset:0x0}"]
pub type Me1Shadr = crate::RegValueT<Me1Shadr_SPEC>;

impl Me1Shadr {
    #[doc = "Shadowed Address   SHADR. This bit field holds the 32 bit shadow address of the active DMA        channel. The function of the shadow address is set by the shadow control        settings."]
    #[inline(always)]
    pub fn shadr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Me1Shadr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Me1Shadr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me1Shadr {
    #[inline(always)]
    fn default() -> Me1Shadr {
        <crate::RegValueT<Me1Shadr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me1Sr_SPEC;
impl crate::sealed::RegSpec for Me1Sr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Status Register\n resetvalue={Application Reset:0x0}"]
pub type Me1Sr = crate::RegValueT<Me1Sr_SPEC>;

impl Me1Sr {
    #[doc = "ME Read Status   RS"]
    #[inline(always)]
    pub fn rs(self) -> crate::common::RegisterFieldBool<0, 1, 0, Me1Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Me1Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ME Write Status   WS"]
    #[inline(always)]
    pub fn ws(self) -> crate::common::RegisterFieldBool<4, 1, 0, Me1Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Me1Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ME Active Channel   CH. Indicates the number of the DMA Channel currently processed by the ME."]
    #[inline(always)]
    pub fn ch(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Me1Sr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Me1Sr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me1Sr {
    #[inline(always)]
    fn default() -> Me1Sr {
        <crate::RegValueT<Me1Sr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ModEr_SPEC;
impl crate::sealed::RegSpec for ModEr_SPEC {
    type DataType = u32;
}
#[doc = "RP 0 Mode Register\n resetvalue={Application Reset:0x1}"]
pub type ModEr = crate::RegValueT<ModEr_SPEC>;

impl ModEr {
    #[doc = "Resource Partition Supervisor Mode   MODE"]
    #[inline(always)]
    pub fn mode(self) -> crate::common::RegisterFieldBool<0, 1, 0, ModEr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, ModEr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for ModEr {
    #[inline(always)]
    fn default() -> ModEr {
        <crate::RegValueT<ModEr_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otss_SPEC;
impl crate::sealed::RegSpec for Otss_SPEC {
    type DataType = u32;
}
#[doc = "DMA OCDS Trigger Set Select\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
pub type Otss = crate::RegValueT<Otss_SPEC>;

impl Otss {
    #[doc = "Trigger Set for OTGB0 or OTGB1   TGS"]
    #[inline(always)]
    pub fn tgs(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Otss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Otss_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OTGB0 or OTGB1 Bus Select   BS"]
    #[inline(always)]
    pub fn bs(self) -> crate::common::RegisterFieldBool<7, 1, 0, Otss_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Otss_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Otss {
    #[inline(always)]
    fn default() -> Otss {
        <crate::RegValueT<Otss_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prr0_SPEC;
impl crate::sealed::RegSpec for Prr0_SPEC {
    type DataType = u32;
}
#[doc = "DMA Pattern Read Register 0\n resetvalue={Application Reset:0x0}"]
pub type Prr0 = crate::RegValueT<Prr0_SPEC>;

impl Prr0 {
    #[doc = "Pattern Data Byte   PAT00"]
    #[inline(always)]
    pub fn pat00(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Prr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Prr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pattern Data Byte   PAT01"]
    #[inline(always)]
    pub fn pat01(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Prr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Prr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pattern Data Byte   PAT02"]
    #[inline(always)]
    pub fn pat02(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Prr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Prr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pattern Data Byte   PAT03"]
    #[inline(always)]
    pub fn pat03(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Prr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Prr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Prr0 {
    #[inline(always)]
    fn default() -> Prr0 {
        <crate::RegValueT<Prr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prr1_SPEC;
impl crate::sealed::RegSpec for Prr1_SPEC {
    type DataType = u32;
}
#[doc = "DMA Pattern Read Register 1\n resetvalue={Application Reset:0x0}"]
pub type Prr1 = crate::RegValueT<Prr1_SPEC>;

impl Prr1 {
    #[doc = "Pattern Data Byte   PAT10"]
    #[inline(always)]
    pub fn pat10(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Prr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Prr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pattern Data Byte   PAT11"]
    #[inline(always)]
    pub fn pat11(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Prr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Prr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pattern Data Byte   PAT12"]
    #[inline(always)]
    pub fn pat12(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Prr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Prr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pattern Data Byte   PAT13"]
    #[inline(always)]
    pub fn pat13(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Prr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Prr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Prr1 {
    #[inline(always)]
    fn default() -> Prr1 {
        <crate::RegValueT<Prr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SusacRc_SPEC;
impl crate::sealed::RegSpec for SusacRc_SPEC {
    type DataType = u32;
}
#[doc = "DMA Channel 000 Suspend Acknowledge Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
pub type SusacRc = crate::RegValueT<SusacRc_SPEC>;

impl SusacRc {
    #[doc = "DMA Channel Suspend State or Frozen State Active for DMA Channel   SUSAC. Status bit indicates whether or not a DMA channel is in channel suspend        state or in the frozen state."]
    #[inline(always)]
    pub fn susac(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SusacRc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, SusacRc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for SusacRc {
    #[inline(always)]
    fn default() -> SusacRc {
        <crate::RegValueT<SusacRc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SusenRc_SPEC;
impl crate::sealed::RegSpec for SusenRc_SPEC {
    type DataType = u32;
}
#[doc = "DMA Channel 000 Suspend Enable Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
pub type SusenRc = crate::RegValueT<SusenRc_SPEC>;

impl SusenRc {
    #[doc = "Channel Suspend Enable for DMA Channel   SUSEN. Enables the DMA channel suspend capability. Channel suspend mode shall        be terminated when SUSEN is written with 0."]
    #[inline(always)]
    pub fn susen(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SusenRc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, SusenRc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for SusenRc {
    #[inline(always)]
    fn default() -> SusenRc {
        <crate::RegValueT<SusenRc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Time_SPEC;
impl crate::sealed::RegSpec for Time_SPEC {
    type DataType = u32;
}
#[doc = "DMA Time Register\n resetvalue={Application Reset:0x0}"]
pub type Time = crate::RegValueT<Time_SPEC>;

impl Time {
    #[doc = "Timestamp Count   COUNT. The count value used during the appendage of DMA timestamps."]
    #[inline(always)]
    pub fn count(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Time_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Time_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Time {
    #[inline(always)]
    fn default() -> Time {
        <crate::RegValueT<Time_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TsRc_SPEC;
impl crate::sealed::RegSpec for TsRc_SPEC {
    type DataType = u32;
}
#[doc = "DMA Channel 000 Transaction State Register\n resetvalue={Application Reset:0x0}"]
pub type TsRc = crate::RegValueT<TsRc_SPEC>;

impl TsRc {
    #[doc = "DMA Channel Reset   RST. The DMA channel reset bit is set by software  DMA channel TSR.RST   1         and cleared by hardware when the DMA channel has been reset."]
    #[inline(always)]
    pub fn rst(self) -> crate::common::RegisterFieldBool<0, 1, 0, TsRc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, TsRc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "DMA Channel Hardware Request Enable   HTRE    . See Functional Description for enable and disable. When a DMA channel          is configured for single mode  HTRE is reset when ME CHSR.TCOUNT is          decremented and ME CHSR.TCOUNT   0. When a DMA channel error is          reported or a pattern match is detected  DMA channel HTRE is reset."]
    #[inline(always)]
    pub fn htre(self) -> crate::common::RegisterFieldBool<1, 1, 0, TsRc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, TsRc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "DMA Channel Transaction Transfer Request Lost   TRL. This bit is reset by software clearing TRL  writing DMA channel TSR.CTL          1  or resetting the DMA channel  writing DMA channel TSR.RST  160    160 1 ."]
    #[inline(always)]
    pub fn trl(self) -> crate::common::RegisterFieldBool<2, 1, 0, TsRc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, TsRc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "DMA Channel Transaction Request State   CH. CH is reset when a pattern match is detected."]
    #[inline(always)]
    pub fn ch(self) -> crate::common::RegisterFieldBool<3, 1, 0, TsRc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, TsRc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable DMA Channel Transaction Transfer Request Lost Interrupt   ETRL. DMA channel control bit to enable the generation of an error interrupt        service request when DMA channel TSR.TRL is set."]
    #[inline(always)]
    pub fn etrl(self) -> crate::common::RegisterFieldBool<4, 1, 0, TsRc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, TsRc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "DMA Channel Halt Request   HLTREQ. The DMA channel halt request bit is set by software  writing DMA channel        TSR.HLTREQ   1  and cleared by software  writing DMA channel TSR.HLTCLR          1 ."]
    #[inline(always)]
    pub fn hltreq(self) -> crate::common::RegisterFieldBool<8, 1, 0, TsRc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, TsRc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "DMA Channel Halt Acknowledge   HLTACK"]
    #[inline(always)]
    pub fn hltack(self) -> crate::common::RegisterFieldBool<9, 1, 0, TsRc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, TsRc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable DMA Channel Hardware Transaction Request   ECH. See Functional Description. Reading this bit returns a 0."]
    #[inline(always)]
    pub fn ech(self) -> crate::common::RegisterFieldBool<16, 1, 0, TsRc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, TsRc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable DMA Channel Hardware Transaction Request   DCH. See Functional Description. Reading this bit returns a 0."]
    #[inline(always)]
    pub fn dch(self) -> crate::common::RegisterFieldBool<17, 1, 0, TsRc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, TsRc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear DMA Channel Transaction Transfer Request Lost   CTL. Software clear of the DMA channel TRL status flag. Reading this bit returns a 0."]
    #[inline(always)]
    pub fn ctl(self) -> crate::common::RegisterFieldBool<18, 1, 0, TsRc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, TsRc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear DMA Channel Halt Request and Acknowledge   HLTCLR. Reading this bit returns a 0."]
    #[inline(always)]
    pub fn hltclr(self) -> crate::common::RegisterFieldBool<24, 1, 0, TsRc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, TsRc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for TsRc {
    #[inline(always)]
    fn default() -> TsRc {
        <crate::RegValueT<TsRc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc = "ACCEN"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accen(pub(super) *mut u8);
unsafe impl core::marker::Send for Accen {}
unsafe impl core::marker::Sync for Accen {}
impl Accen {
    #[doc = "RP 0 Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accenr0(&self) -> crate::common::Reg<accen::AcceNr0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod accen {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AcceNr0_SPEC;
    impl crate::sealed::RegSpec for AcceNr0_SPEC {
        type DataType = u32;
    }
    #[doc = "RP 0 Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type AcceNr0 = crate::RegValueT<AcceNr0_SPEC>;

    impl AcceNr0 {
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en3(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en4(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en5(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en6(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en7(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en8(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<8,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en9(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en10(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en11(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<11,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en12(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en13(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en14(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en15(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en16(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<16,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en17(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<17,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en18(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<18,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en19(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<19,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en20(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<20,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en21(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<21,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en22(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<22,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en23(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<23,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en24(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<24,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en25(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<25,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en26(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<26,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en27(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<27,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en28(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<28,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en29(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<29,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en30(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<30,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en31(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, AcceNr0_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<31,1,0,AcceNr0_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for AcceNr0 {
        #[inline(always)]
        fn default() -> AcceNr0 {
            <crate::RegValueT<AcceNr0_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
}
#[doc = "CH"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch(pub(super) *mut u8);
unsafe impl core::marker::Send for Ch {}
unsafe impl core::marker::Sync for Ch {}
impl Ch {
    #[doc = "DMARAM Channel 000 Address and Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn adicrc(&self) -> crate::common::Reg<ch::AdicRc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "DMARAM Channel 000 Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn chcfgrc(&self) -> crate::common::Reg<ch::ChcfgRc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "DMARAM Channel 000 Control and Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn chcsrc(&self) -> crate::common::Reg<ch::ChcsRc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "DMARAM Channel 000 Destination Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dadrc(&self) -> crate::common::Reg<ch::DadRc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "DMARAM Channel 000 Read Data CRC Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rdcrcrc(&self) -> crate::common::Reg<ch::RdcrcRc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "DMARAM Channel 000 Source Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sadrc(&self) -> crate::common::Reg<ch::SadRc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "DMARAM Channel 000 Source and Destination Address CRC Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sdcrcrc(&self) -> crate::common::Reg<ch::SdcrcRc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "DMARAM Channel 000 Shadow Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn shadrc(&self) -> crate::common::Reg<ch::ShadRc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
}
pub mod ch {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdicRc_SPEC;
    impl crate::sealed::RegSpec for AdicRc_SPEC {
        type DataType = u32;
    }
    #[doc = "DMARAM Channel 000 Address and Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
    pub type AdicRc = crate::RegValueT<AdicRc_SPEC>;

    impl AdicRc {
        #[doc = "Source Address Modification Factor   SMF. DMA channel TCS 32 bit source address modification factor and the        channel data width CHDW determines an address offset value by which the        source address is modified after each DMA move. If SCBE   1 B and CBLS  160    160 0000 B then the        source address is not modified."]
        #[inline(always)]
        pub fn smf(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, AdicRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,u8, AdicRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Increment of Source Address   INCS. DMA channel TCS control bit to determine if the address offset selected        by SMF will be added to or subtracted from the source address after each        DMA move. If SCBE   1 B and CBLS  160    160 0000 B then the source address is not modified."]
        #[inline(always)]
        pub fn incs(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, AdicRc_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,AdicRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Destination Address Modification Factor   DMF. DMA channel TCS 32 bit destination address modification factor and the        channel data width CHDW determines an address offset value by which the        destination address is modified after each DMA move. If DCBE   1 B and CBLD  160    160 0000 B then the        destination address is not modified."]
        #[inline(always)]
        pub fn dmf(
            self,
        ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, AdicRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x7,1,0,u8, AdicRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Increment of Destination Address   INCD. DMA channel TCS control bit to determine if the address offset selected        by DMF will be added to or subtracted from the destination address after        each DMA move. If DCBE   1 B and CBLD  160    160 0000 B the destination address is not modified."]
        #[inline(always)]
        pub fn incd(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, AdicRc_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,AdicRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Circular Buffer Length Source   CBLS. DMA channel TCS circular buffer source address update control bit        determines which part of the 32 bit source address register remains        unchanged and is not updated after a DMA move operation. CBLS determines the size of the circular source buffer."]
        #[inline(always)]
        pub fn cbls(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, AdicRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xf,1,0,u8, AdicRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Circular Buffer Length Destination   CBLD. DMA channel TCS circular buffer destination address update control bit        determines which part of the 32 bit destination address register remains        unchanged and is not updated after a DMA move operation. CBLD determines the size of the circular destination buffer."]
        #[inline(always)]
        pub fn cbld(
            self,
        ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, AdicRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0xf,1,0,u8, AdicRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Shadow Control   SHCT. DMA channel TCS shadow control determines the function of the shadow        address register."]
        #[inline(always)]
        pub fn shct(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, AdicRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, AdicRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Source Circular Buffer Enable   SCBE. DMA channel TCS source circular buffer enable."]
        #[inline(always)]
        pub fn scbe(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, AdicRc_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<20,1,0,AdicRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Destination Circular Buffer Enable   DCBE. DMA channel TCS destination circular buffer enable."]
        #[inline(always)]
        pub fn dcbe(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, AdicRc_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<21,1,0,AdicRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Time Stamp   STAMP. DMA channel TCS control bit to enable the appendage of a timestamp after        the end of the last DMA Move during a DMA transaction."]
        #[inline(always)]
        pub fn stamp(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, AdicRc_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<22,1,0,AdicRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Wrap Source Enable   WRPSE. DMA channel TCS source buffer interrupt trigger enable disable."]
        #[inline(always)]
        pub fn wrpse(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, AdicRc_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<24,1,0,AdicRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Wrap Destination Enable   WRPDE. DMA channel TCS destination buffer interrupt trigger enable disable."]
        #[inline(always)]
        pub fn wrpde(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, AdicRc_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<25,1,0,AdicRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Interrupt Control   INTCT. DMA channel TCS interrupt control. If DMA channel CHCFGR.PRSEL   1 B for          the next lower priority channel then the channel transfer trigger          interrupt is disabled."]
        #[inline(always)]
        pub fn intct(
            self,
        ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, AdicRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<26,0x3,1,0,u8, AdicRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Interrupt Raise Detect Value   IRDV. DMA channel TCS interrupt threshold value defines the Threshold Limit of        CHSR.TCOUNT for which a channel interrupt trigger will be raised."]
        #[inline(always)]
        pub fn irdv(
            self,
        ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, AdicRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0xf,1,0,u8, AdicRc_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for AdicRc {
        #[inline(always)]
        fn default() -> AdicRc {
            <crate::RegValueT<AdicRc_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChcfgRc_SPEC;
    impl crate::sealed::RegSpec for ChcfgRc_SPEC {
        type DataType = u32;
    }
    #[doc = "DMARAM Channel 000 Configuration Register\n resetvalue={Application Reset:0x0}"]
    pub type ChcfgRc = crate::RegValueT<ChcfgRc_SPEC>;

    impl ChcfgRc {
        #[doc = "Transfer Reload Value   TREL. DMA channel TCS transfer reload value to control the number of DMA        transfers in a DMA transaction. The 14 bit transfer count value is        loaded into ME CHSR.TCOUNT at the start of a DMA transaction  when        TSR.CH becomes set and CHSR.TCOUNT  160    160 0 . A write to CHCFGR.TREL during a        running DMA transaction has no influence on the running DMA transaction. If CHCFGR.TREL  160    160 0 or if CHCFGR.TREL  160    160 1 then ME CHSR.TCOUNT will be        loaded with 1 when a new DMA transaction is started  at least one DMA        transfer must be executed per DMA transaction ."]
        #[inline(always)]
        pub fn trel(
            self,
        ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, ChcfgRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3fff,1,0,u16, ChcfgRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Block Mode   BLKM. Defines the number of DMA moves executed during one DMA transfer."]
        #[inline(always)]
        pub fn blkm(
            self,
        ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, ChcfgRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x7,1,0,u8, ChcfgRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Reset Request Only After Transaction   RROAT. DMA channel control bit to determine if the DMA request state flag  DMA        channel TSR.CH  is reset after each DMA transfer."]
        #[inline(always)]
        pub fn rroat(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, ChcfgRc_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<19,1,0,ChcfgRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Channel Operation Mode   CHMODE. DMA channel TCS control to determine TSR.HTRE reset condition."]
        #[inline(always)]
        pub fn chmode(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x1,
            1,
            0,
            chcfgrc::Chmode,
            ChcfgRc_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x1,
                1,
                0,
                chcfgrc::Chmode,
                ChcfgRc_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Channel Data Width   CHDW. DMA channel TCS data width for DMA read moves and DMA write moves."]
        #[inline(always)]
        pub fn chdw(
            self,
        ) -> crate::common::RegisterField<21, 0x7, 1, 0, u8, ChcfgRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<21,0x7,1,0,u8, ChcfgRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Pattern Select   PATSEL. DMA channel TCS bit field to select the pattern detection operation  see        Functional Description . If PATSEL 1 0  is not equal to 00 B then a ME pattern detection operation defined by the channel data width         CHDW  will be performed. PATSEL 2  selects the pattern read register.        If a pattern match is detected then a DMA channel interrupt shall be        triggered."]
        #[inline(always)]
        pub fn patsel(
            self,
        ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, ChcfgRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0x7,1,0,u8, ChcfgRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Swap Data CRC Byte Order   SWAP. DMA channel TCS swap data CRC byte order."]
        #[inline(always)]
        pub fn swap(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, ChcfgRc_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<27,1,0,ChcfgRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Peripheral Request Select   PRSEL. DMA channel TCS control bit field to select the source of a DMA request."]
        #[inline(always)]
        pub fn prsel(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, ChcfgRc_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<28,1,0,ChcfgRc_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for ChcfgRc {
        #[inline(always)]
        fn default() -> ChcfgRc {
            <crate::RegValueT<ChcfgRc_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod chcfgrc {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Chmode_SPEC;
        pub type Chmode = crate::EnumBitfieldStruct<u8, Chmode_SPEC>;
        impl Chmode {
            #[doc = "0 Single Mode is        selected for DMA channel. After a DMA transaction  DMA channel is        disabled for further hardware requests  TSR.HTRE is reset by hardware         TSR.HTRE must be set again by software for starting a new transaction."]
            pub const SINGLE_MODE_0: Self = Self::new(0);
            #[doc = "1 Continuous Mode is        selected for DMA channel. After a DMA transaction  bit TSR.HTRE remains        set."]
            pub const CONTINUOUS_MODE_1: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChcsRc_SPEC;
    impl crate::sealed::RegSpec for ChcsRc_SPEC {
        type DataType = u32;
    }
    #[doc = "DMARAM Channel 000 Control and Status Register\n resetvalue={Application Reset:0x0}"]
    pub type ChcsRc = crate::RegValueT<ChcsRc_SPEC>;

    impl ChcsRc {
        #[doc = "Transfer Count   TCOUNT. DMA channel status transfer count updated after DMARAM write back."]
        #[inline(always)]
        pub fn tcount(
            self,
        ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, ChcsRc_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0x3fff,1,0,u16, ChcsRc_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Old Value of Pattern Detection   LXO. DMA channel status bit to store the result of a pattern detection        operation when 8 bit or 16 bit data width is selected."]
        #[inline(always)]
        pub fn lxo(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, ChcsRc_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<15,1,0,ChcsRc_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Wrap Source Buffer   WRPS. Status bit indicates that a DMA channel has reached a wrap source buffer        boundary. Bit is reset by software  DMA channel CHCSR.CWRP   1 B or DMA channel reset TSR.RST   1 B  ."]
        #[inline(always)]
        pub fn wrps(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, ChcsRc_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<16,1,0,ChcsRc_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Wrap Destination Buffer   WRPD. Status bit indicates that a DMA channel has reached a wrap destination        buffer boundary. Bit is reset by software  DMA channel CHCSR.CWRP   1 B or DMA channel reset TSR.RST   1 B  ."]
        #[inline(always)]
        pub fn wrpd(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, ChcsRc_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<17,1,0,ChcsRc_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Interrupt from Channel   ICH. As soon as ME CHSR.TCOUNT decrements  if ADICR.INTCT 0    0 and        CHSR.COUNT   IRDV then ME CHSR.ICH is set else ME CHSR.ICH is set for        each decrement of CSR.TCOUNT. Bit is reset by software  DMA channel        CHCSR.CICH   1 B or by a DMA channel reset TSR.RST   1 B  ."]
        #[inline(always)]
        pub fn ich(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, ChcsRc_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<18,1,0,ChcsRc_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Pattern Detection from Channel   IPM. Status bit indicates that a pattern match has been detected for the DMA        channel when pattern detection is enabled. This bit is reset by software         DMA channel CHCSR.CICH   1 B or DMA channel reset TSR.RST   1 B  ."]
        #[inline(always)]
        pub fn ipm(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, ChcsRc_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<19,1,0,ChcsRc_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "DMA Double Buffering Active Buffer   BUFFER. During a DMA double buffering operation  the status bit indicates which        buffer is read or filled."]
        #[inline(always)]
        pub fn buffer(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, ChcsRc_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<22,1,0,ChcsRc_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "DMA Double Buffering Frozen Buffer   FROZEN. If a DMA channel is configured for double buffering operation  the        FROZEN bit indicates that one of the buffers is frozen and available for        processing by a cyclic software task. FROZEN bit shall only be set by the DMA and shall only be cleared by          software."]
        #[inline(always)]
        pub fn frozen(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, ChcsRc_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<23,1,0,ChcsRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "DMA Double Buffering Switch Buffer   SWB. When DMA double buffering is configured  the control bit is used to        re direct data from one buffer to the other buffer. If a DMALL  ACCLL  SAFLL or CONLL operation is configured then SWB          shall be 0 B ."]
        #[inline(always)]
        pub fn swb(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, ChcsRc_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<24,1,0,ChcsRc_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Wrap Buffer Interrupt   CWRP. Software clear of the DMA channel source and destination wrap buffer        flags stored at CHCSR.WRPS and CHCSR.WRPD. If the DMA channel is active        in a ME then clear ME bit fields CHSR.WRPS and CHSR.WRPD. Reading this        bit returns a 0."]
        #[inline(always)]
        pub fn cwrp(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, ChcsRc_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<25,1,0,ChcsRc_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Interrupt for DMA Channel   CICH. Software clear of the DMA channel flags stored at CHCSR.ICH and        CHCSR.IPM. If the DMA channel is active in a ME then clear ME bit fields        CHSR.ICH and CHSR.IPM. Reading this bit returns a 0."]
        #[inline(always)]
        pub fn cich(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, ChcsRc_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<26,1,0,ChcsRc_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Set Interrupt Trigger for DMA Channel   SIT. Reading this bit returns a 0. If a DMALL  ACCLL  SAFLL or CONLL operation is configured then SIT          must be 0 B ."]
        #[inline(always)]
        pub fn sit(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, ChcsRc_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<27,1,0,ChcsRc_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Set Transaction Request   SCH. Reading this bit returns a 0."]
        #[inline(always)]
        pub fn sch(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, ChcsRc_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<31,1,0,ChcsRc_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for ChcsRc {
        #[inline(always)]
        fn default() -> ChcsRc {
            <crate::RegValueT<ChcsRc_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DadRc_SPEC;
    impl crate::sealed::RegSpec for DadRc_SPEC {
        type DataType = u32;
    }
    #[doc = "DMARAM Channel 000 Destination Address Register\n resetvalue={Application Reset:0x0}"]
    pub type DadRc = crate::RegValueT<DadRc_SPEC>;

    impl DadRc {
        #[doc = "Destination Address   DADR. 32 bit destination address."]
        #[inline(always)]
        pub fn dadr(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, DadRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, DadRc_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for DadRc {
        #[inline(always)]
        fn default() -> DadRc {
            <crate::RegValueT<DadRc_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RdcrcRc_SPEC;
    impl crate::sealed::RegSpec for RdcrcRc_SPEC {
        type DataType = u32;
    }
    #[doc = "DMARAM Channel 000 Read Data CRC Register\n resetvalue={Application Reset:0x0}"]
    pub type RdcrcRc = crate::RegValueT<RdcrcRc_SPEC>;

    impl RdcrcRc {
        #[doc = "Read Data CRC   RDCRC. Checksum calculated for DMA read move data."]
        #[inline(always)]
        pub fn rdcrc(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, RdcrcRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, RdcrcRc_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for RdcrcRc {
        #[inline(always)]
        fn default() -> RdcrcRc {
            <crate::RegValueT<RdcrcRc_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SadRc_SPEC;
    impl crate::sealed::RegSpec for SadRc_SPEC {
        type DataType = u32;
    }
    #[doc = "DMARAM Channel 000 Source Address Register\n resetvalue={Application Reset:0x0}"]
    pub type SadRc = crate::RegValueT<SadRc_SPEC>;

    impl SadRc {
        #[doc = "Source Address   SADR. 32 bit source address."]
        #[inline(always)]
        pub fn sadr(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, SadRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, SadRc_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for SadRc {
        #[inline(always)]
        fn default() -> SadRc {
            <crate::RegValueT<SadRc_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdcrcRc_SPEC;
    impl crate::sealed::RegSpec for SdcrcRc_SPEC {
        type DataType = u32;
    }
    #[doc = "DMARAM Channel 000 Source and Destination Address CRC Register\n resetvalue={Application Reset:0x0}"]
    pub type SdcrcRc = crate::RegValueT<SdcrcRc_SPEC>;

    impl SdcrcRc {
        #[doc = "Source and Destination Address CRC   SDCRC. Checksum calculated for DMA move source and destination addresses."]
        #[inline(always)]
        pub fn sdcrc(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, SdcrcRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, SdcrcRc_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for SdcrcRc {
        #[inline(always)]
        fn default() -> SdcrcRc {
            <crate::RegValueT<SdcrcRc_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ShadRc_SPEC;
    impl crate::sealed::RegSpec for ShadRc_SPEC {
        type DataType = u32;
    }
    #[doc = "DMARAM Channel 000 Shadow Address Register\n resetvalue={Application Reset:0x0}"]
    pub type ShadRc = crate::RegValueT<ShadRc_SPEC>;

    impl ShadRc {
        #[doc = "Shadowed Address   SHADR. 32 bit shadow address."]
        #[inline(always)]
        pub fn shadr(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, ShadRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, ShadRc_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for ShadRc {
        #[inline(always)]
        fn default() -> ShadRc {
            <crate::RegValueT<ShadRc_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
