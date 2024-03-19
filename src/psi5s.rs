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
#[doc = r"PSI5S"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psi5S(pub(super) *mut u8);
unsafe impl core::marker::Send for Psi5S {}
unsafe impl core::marker::Sync for Psi5S {}
impl Psi5S {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(976usize)) }
    }

    #[doc = "Base Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn bar(&self) -> crate::common::Reg<self::Bar_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(212usize)) }
    }

    #[doc = "Baud Rate Timer Reload Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn bg(&self) -> crate::common::Reg<self::Bg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(532usize)) }
    }

    #[doc = "CPU Direct Write Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cdw(&self) -> crate::common::Reg<self::Cdw_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(368usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn con(&self) -> crate::common::Reg<self::Con_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(528usize)) }
    }

    #[doc = "Channel Trigger Value Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ctvx(&self) -> [crate::common::Reg<self::CtVx_SPEC, crate::common::RW>; 8] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x110usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x110usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x110usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x110usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x110usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x110usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x110usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x110usize + 0x1cusize)),
            ]
        }
    }

    #[doc = "Frame Counter Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcnt(&self) -> crate::common::Reg<self::Fcnt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "Fractional Divider for Output CLK Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fdo(&self) -> crate::common::Reg<self::Fdo_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(540usize)) }
    }

    #[doc = "PSI5 S Fractional Divider Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fdr(&self) -> crate::common::Reg<self::Fdr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }

    #[doc = "Fractional Divider Register for Time Stamp\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fdrt(&self) -> crate::common::Reg<self::Fdrt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "Fractional Divider Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fdv(&self) -> crate::common::Reg<self::Fdv_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(536usize)) }
    }

    #[doc = "Global Control Register\n resetvalue={Application Reset:0x1F}"]
    #[inline(always)]
    pub const fn gcr(&self) -> crate::common::Reg<self::Gcr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0D3C000}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Interrupt Node Pointer Register Global\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn inpg(&self) -> crate::common::Reg<self::Inpg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(788usize)) }
    }

    #[doc = "Interrupt Node Pointer Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn inpx(&self) -> [crate::common::Reg<self::InPx_SPEC, crate::common::RW>; 8] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x2e0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x2e0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x2e0usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x2e0usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x2e0usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x2e0usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x2e0usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x2e0usize + 0x1cusize)),
            ]
        }
    }

    #[doc = "Interrupt Clear Register Global\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn intclrg(&self) -> crate::common::Reg<self::Intclrg_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(780usize)) }
    }

    #[doc = "Interrupt Clear Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn intclrx(&self) -> [crate::common::Reg<self::IntclRx_SPEC, crate::common::W>; 8] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x2a0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x2a0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x2a0usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x2a0usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x2a0usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x2a0usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x2a0usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x2a0usize + 0x1cusize)),
            ]
        }
    }

    #[doc = "Interrupt Enable Register Global\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn inteng(&self) -> crate::common::Reg<self::Inteng_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(784usize)) }
    }

    #[doc = "Interrupt Enable Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn intenx(&self) -> [crate::common::Reg<self::InteNx_SPEC, crate::common::RW>; 8] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x2c0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x2c0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x2c0usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x2c0usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x2c0usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x2c0usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x2c0usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x2c0usize + 0x1cusize)),
            ]
        }
    }

    #[doc = "Interrupt Overview Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn intov(&self) -> crate::common::Reg<self::Intov_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(768usize)) }
    }

    #[doc = "Interrupt Set Register Global\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn intsetg(&self) -> crate::common::Reg<self::Intsetg_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(776usize)) }
    }

    #[doc = "Interrupt Set Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn intsetx(&self) -> [crate::common::Reg<self::IntseTx_SPEC, crate::common::W>; 8] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x1cusize)),
            ]
        }
    }

    #[doc = "Interrupt Status Register Global\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn intstatg(&self) -> crate::common::Reg<self::Intstatg_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(772usize)) }
    }

    #[doc = "Interrupt Status Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn intstatx(&self) -> [crate::common::Reg<self::IntstaTx_SPEC, crate::common::R>; 8] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x260usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x260usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x260usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x260usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x260usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x260usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x260usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x260usize + 0x1cusize)),
            ]
        }
    }

    #[doc = "Input and Output Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn iocr(&self) -> crate::common::Reg<self::Iocr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }

    #[doc = "Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(984usize)) }
    }

    #[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(988usize)) }
    }

    #[doc = "Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(992usize)) }
    }

    #[doc = "Number of Frames Control Register\n resetvalue={Application Reset:0x249249}"]
    #[inline(always)]
    pub const fn nfc(&self) -> crate::common::Reg<self::Nfc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(972usize)) }
    }

    #[doc = "Pulse Generation Control Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pgcx(&self) -> [crate::common::Reg<self::PgCx_SPEC, crate::common::RW>; 8] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0xf0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0xf0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0xf0usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0xf0usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0xf0usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0xf0usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0xf0usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0xf0usize + 0x1cusize)),
            ]
        }
    }

    #[doc = "Receive Buffer Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rbuf(&self) -> crate::common::Reg<self::Rbuf_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(548usize)) }
    }

    #[doc = "Receiver Control Register A0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rcrax(&self) -> [crate::common::Reg<self::RcrAx_SPEC, crate::common::RW>; 8] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x1cusize)),
            ]
        }
    }

    #[doc = "Receiver Control Register B0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rcrbx(&self) -> [crate::common::Reg<self::RcrBx_SPEC, crate::common::RW>; 8] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x50usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x50usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x50usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x50usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x50usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x50usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x50usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x50usize + 0x1cusize)),
            ]
        }
    }

    #[doc = "Receive Data Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rdr(&self) -> crate::common::Reg<self::Rdr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(180usize)) }
    }

    #[doc = "Receive Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rds(&self) -> crate::common::Reg<self::Rds_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(176usize)) }
    }

    #[doc = "Send Control Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn scrx(&self) -> [crate::common::Reg<self::ScRx_SPEC, crate::common::RW>; 8] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x1cusize)),
            ]
        }
    }

    #[doc = "Send Data Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sdrx(&self) -> [crate::common::Reg<self::SdRx_SPEC, crate::common::RW>; 8] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x150usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x150usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x150usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x150usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x150usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x150usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x150usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x150usize + 0x1cusize)),
            ]
        }
    }

    #[doc = "Target Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tar(&self) -> crate::common::Reg<self::Tar_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(208usize)) }
    }

    #[doc = "Transmit Buffer Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tbuf(&self) -> crate::common::Reg<self::Tbuf_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(544usize)) }
    }

    #[doc = "Time Stamp Count Register A\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tscnta(&self) -> crate::common::Reg<self::Tscnta_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Time Stamp Count Register B\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tscntb(&self) -> crate::common::Reg<self::Tscntb_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "Capture Register TSCR0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tscrx(&self) -> [crate::common::Reg<self::TscRx_SPEC, crate::common::R>; 8] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x90usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x90usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x90usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x90usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x90usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x90usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x90usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x90usize + 0x1cusize)),
            ]
        }
    }

    #[doc = "Time Stamp Mirror Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tsm(&self) -> crate::common::Reg<self::Tsm_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(184usize)) }
    }

    #[doc = "Watch Dog Timer Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn wdtx(&self) -> [crate::common::Reg<self::WdTx_SPEC, crate::common::RW>; 8] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x70usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x70usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x70usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x70usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x70usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x70usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x70usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x70usize + 0x1cusize)),
            ]
        }
    }

    #[doc = "Write Hardware Bits Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn whbcon(&self) -> crate::common::Reg<self::Whbcon_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(592usize)) }
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
pub struct Bar_SPEC;
impl crate::sealed::RegSpec for Bar_SPEC {
    type DataType = u32;
}
#[doc = "Base Address Register\n resetvalue={Application Reset:0x0}"]
pub type Bar = crate::RegValueT<Bar_SPEC>;

impl Bar {
    #[doc = "Base Address   BA. Contains the upper 30 bits of the base address for the DMA transfers. The 32 bit base address must be word aligned. Thus the 2 LSBs are fixed to 0."]
    #[inline(always)]
    pub fn ba(
        self,
    ) -> crate::common::RegisterField<2, 0x3fffffff, 1, 0, u32, Bar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3fffffff,1,0,u32, Bar_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Bar {
    #[inline(always)]
    fn default() -> Bar {
        <crate::RegValueT<Bar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bg_SPEC;
impl crate::sealed::RegSpec for Bg_SPEC {
    type DataType = u32;
}
#[doc = "Baud Rate Timer Reload Register\n resetvalue={Application Reset:0x0}"]
pub type Bg = crate::RegValueT<Bg_SPEC>;

impl Bg {
    #[doc = "Baud Rate Timer Reload Register Value   BR VALUE. Reading BR VALUE returns the 13 bit content of the baud rate timer. Writing BR VALUE loads the baud rate timer reload register. BG should only be written if CON.R   0."]
    #[inline(always)]
    pub fn br_value(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, Bg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1fff,1,0,u16, Bg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Bg {
    #[inline(always)]
    fn default() -> Bg {
        <crate::RegValueT<Bg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdw_SPEC;
impl crate::sealed::RegSpec for Cdw_SPEC {
    type DataType = u32;
}
#[doc = "CPU Direct Write Register\n resetvalue={Application Reset:0x0}"]
pub type Cdw = crate::RegValueT<Cdw_SPEC>;

impl Cdw {
    #[doc = "SD7   SD7. Send data of next ECU to Sensor frame."]
    #[inline(always)]
    pub fn sd0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cdw_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cdw_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD7   SD7. Send data of next ECU to Sensor frame."]
    #[inline(always)]
    pub fn sd1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cdw_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cdw_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD7   SD7. Send data of next ECU to Sensor frame."]
    #[inline(always)]
    pub fn sd2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Cdw_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cdw_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD7   SD7. Send data of next ECU to Sensor frame."]
    #[inline(always)]
    pub fn sd3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Cdw_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cdw_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD7   SD7. Send data of next ECU to Sensor frame."]
    #[inline(always)]
    pub fn sd4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Cdw_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cdw_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD7   SD7. Send data of next ECU to Sensor frame."]
    #[inline(always)]
    pub fn sd5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Cdw_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cdw_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD7   SD7. Send data of next ECU to Sensor frame."]
    #[inline(always)]
    pub fn sd6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Cdw_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cdw_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD7   SD7. Send data of next ECU to Sensor frame."]
    #[inline(always)]
    pub fn sd7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Cdw_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Cdw_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Trigger Pulse Indicator   TSI. If this bit is set  a sync pulse is assumed and thus a time stamp is captured when the command leaves the FIFO and is written to ASC TX Register. Bits SD 7 5  select the channel for which the time stamp is captured. RCRAx.TSTS must be cleared. If set  the time stamp is captured on Packet Frame reception only."]
    #[inline(always)]
    pub fn tsi(self) -> crate::common::RegisterFieldBool<8, 1, 0, Cdw_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cdw_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Cdw {
    #[inline(always)]
    fn default() -> Cdw {
        <crate::RegValueT<Cdw_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control of the module. This bit disables the kernel clocks f PSI5 S and the ASC clock f ASC ."]
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
    #[doc = "External Sleep Mode Request Disable Bit   EDIS. Used to control module s sleep mode. If this bit is cleared the kernel clocks f PSI5 S and the ASC clock f ASC are disabled during System Sleep Mode."]
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
pub struct Con_SPEC;
impl crate::sealed::RegSpec for Con_SPEC {
    type DataType = u32;
}
#[doc = "Control Register\n resetvalue={Application Reset:0x0}"]
pub type Con = crate::RegValueT<Con_SPEC>;

impl Con {
    #[doc = "Mode Selection   M"]
    #[inline(always)]
    pub fn m(self) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0x7, 1, 0, u8, Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Number of Stop Bit Selection   STP"]
    #[inline(always)]
    pub fn stp(self) -> crate::common::RegisterFieldBool<3, 1, 0, Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Receiver Enable Control   REN. Bit is reset by hardware after reception of a Byte in Synchronous Mode."]
    #[inline(always)]
    pub fn ren(self) -> crate::common::RegisterFieldBool<4, 1, 0, Con_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Con_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Parity Check Enable  asynchronous mode only    PEN"]
    #[inline(always)]
    pub fn pen(self) -> crate::common::RegisterFieldBool<5, 1, 0, Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Framing Check Enable  asynchronous mode only    FEN"]
    #[inline(always)]
    pub fn fen(self) -> crate::common::RegisterFieldBool<6, 1, 0, Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Overrun Check Enable   OEN"]
    #[inline(always)]
    pub fn oen(self) -> crate::common::RegisterFieldBool<7, 1, 0, Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ASC Parity Error Flag   PE. Set by hardware on a parity error  PEN   1 . Must be reset by software."]
    #[inline(always)]
    pub fn pe(self) -> crate::common::RegisterFieldBool<8, 1, 0, Con_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Con_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ASC Framing Error Flag   FE. Set by hardware on a framing error  FEN   1 . Must be reset by software."]
    #[inline(always)]
    pub fn fe(self) -> crate::common::RegisterFieldBool<9, 1, 0, Con_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Con_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ASC Overrun Error Flag   OE. Set by hardware on an overrun error  OEN   1 . Must be reset by software."]
    #[inline(always)]
    pub fn oe(self) -> crate::common::RegisterFieldBool<10, 1, 0, Con_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Con_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Fractional Divider Enable   FDE. FDE is don t care and assumed  0  in Synchr. Mode."]
    #[inline(always)]
    pub fn fde(self) -> crate::common::RegisterFieldBool<11, 1, 0, Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Parity Selection   ODD"]
    #[inline(always)]
    pub fn odd(self) -> crate::common::RegisterFieldBool<12, 1, 0, Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Baud Rate Selection   BRS. BRS is don t care if FDE   1  fractional divider enabled  FDE is don t care and assumed  0  in Synchr. Mode."]
    #[inline(always)]
    pub fn brs(self) -> crate::common::RegisterFieldBool<13, 1, 0, Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Loop back Mode Enable   LB"]
    #[inline(always)]
    pub fn lb(self) -> crate::common::RegisterFieldBool<14, 1, 0, Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Baud Rate Generator Run Control   R. Register BG should only be written if R   0."]
    #[inline(always)]
    pub fn r(self) -> crate::common::RegisterFieldBool<15, 1, 0, Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Mode Selection TX direction   MTX. While bit field M controls the RX path  MTX controls the mode for the TX path."]
    #[inline(always)]
    pub fn mtx(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Parity Selection TX direction   ODDTX. While bit field ODD controls the RX path  ODDTX controls the mode for the TX path."]
    #[inline(always)]
    pub fn oddtx(self) -> crate::common::RegisterFieldBool<28, 1, 0, Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Con {
    #[inline(always)]
    fn default() -> Con {
        <crate::RegValueT<Con_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtVx_SPEC;
impl crate::sealed::RegSpec for CtVx_SPEC {
    type DataType = u32;
}
#[doc = "Channel Trigger Value Register 0\n resetvalue={Application Reset:0x0}"]
pub type CtVx = crate::RegValueT<CtVx_SPEC>;

impl CtVx {
    #[doc = "Channel Trigger Value CTV   CTV. Contains the compare value  exact match  of Channel Trigger CTC at which a sync pulse is triggered for channel x and the counter CTC is cleared.  If cleared  CTC is stopped and no pulse triggered."]
    #[inline(always)]
    pub fn ctv(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, CtVx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, CtVx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel Trigger Counter   CTC. This bit field allows to read the current counter value of the reset timer cell CTVx. If GCR.ETCx is cleared  CTC can be written."]
    #[inline(always)]
    pub fn ctc(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, CtVx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, CtVx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for CtVx {
    #[inline(always)]
    fn default() -> CtVx {
        <crate::RegValueT<CtVx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcnt_SPEC;
impl crate::sealed::RegSpec for Fcnt_SPEC {
    type DataType = u32;
}
#[doc = "Frame Counter Register\n resetvalue={Application Reset:0x0}"]
pub type Fcnt = crate::RegValueT<Fcnt_SPEC>;

impl Fcnt {
    #[doc = "Frame Counter for Channel 7   FC7. Contains the number of received frames on Channel  160 x. Copied to RDR.FID        if RCRAx.FIDS is set."]
    #[inline(always)]
    pub fn fc0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Fcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0x7, 1, 0, u8, Fcnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Frame Counter for Channel 7   FC7. Contains the number of received frames on Channel  160 x. Copied to RDR.FID        if RCRAx.FIDS is set."]
    #[inline(always)]
    pub fn fc1(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, Fcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<3, 0x7, 1, 0, u8, Fcnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Frame Counter for Channel 7   FC7. Contains the number of received frames on Channel  160 x. Copied to RDR.FID        if RCRAx.FIDS is set."]
    #[inline(always)]
    pub fn fc2(
        self,
    ) -> crate::common::RegisterField<6, 0x7, 1, 0, u8, Fcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<6, 0x7, 1, 0, u8, Fcnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Frame Counter for Channel 7   FC7. Contains the number of received frames on Channel  160 x. Copied to RDR.FID        if RCRAx.FIDS is set."]
    #[inline(always)]
    pub fn fc3(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, Fcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<9, 0x7, 1, 0, u8, Fcnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Frame Counter for Channel 7   FC7. Contains the number of received frames on Channel  160 x. Copied to RDR.FID        if RCRAx.FIDS is set."]
    #[inline(always)]
    pub fn fc4(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, Fcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x7,1,0,u8, Fcnt_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Frame Counter for Channel 7   FC7. Contains the number of received frames on Channel  160 x. Copied to RDR.FID        if RCRAx.FIDS is set."]
    #[inline(always)]
    pub fn fc5(
        self,
    ) -> crate::common::RegisterField<15, 0x7, 1, 0, u8, Fcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<15,0x7,1,0,u8, Fcnt_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Frame Counter for Channel 7   FC7. Contains the number of received frames on Channel  160 x. Copied to RDR.FID        if RCRAx.FIDS is set."]
    #[inline(always)]
    pub fn fc6(
        self,
    ) -> crate::common::RegisterField<18, 0x7, 1, 0, u8, Fcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<18,0x7,1,0,u8, Fcnt_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Frame Counter for Channel 7   FC7. Contains the number of received frames on Channel  160 x. Copied to RDR.FID        if RCRAx.FIDS is set."]
    #[inline(always)]
    pub fn fc7(
        self,
    ) -> crate::common::RegisterField<21, 0x7, 1, 0, u8, Fcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<21,0x7,1,0,u8, Fcnt_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Clear Number of Frame Counter for Channel 7   NFCLR7. Clears the referring counter FCNT.FCx. Intended for use during recovery from TEI. If set while a frame is being received  this action results in FCx    1 . Thus FCx will never be  0  when RDI RSI signal a new receive frame. Use with care"]
    #[inline(always)]
    pub fn nfclr0(self) -> crate::common::RegisterFieldBool<24, 1, 0, Fcnt_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Fcnt_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Number of Frame Counter for Channel 7   NFCLR7. Clears the referring counter FCNT.FCx. Intended for use during recovery from TEI. If set while a frame is being received  this action results in FCx    1 . Thus FCx will never be  0  when RDI RSI signal a new receive frame. Use with care"]
    #[inline(always)]
    pub fn nfclr1(self) -> crate::common::RegisterFieldBool<25, 1, 0, Fcnt_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Fcnt_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Number of Frame Counter for Channel 7   NFCLR7. Clears the referring counter FCNT.FCx. Intended for use during recovery from TEI. If set while a frame is being received  this action results in FCx    1 . Thus FCx will never be  0  when RDI RSI signal a new receive frame. Use with care"]
    #[inline(always)]
    pub fn nfclr2(self) -> crate::common::RegisterFieldBool<26, 1, 0, Fcnt_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Fcnt_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Number of Frame Counter for Channel 7   NFCLR7. Clears the referring counter FCNT.FCx. Intended for use during recovery from TEI. If set while a frame is being received  this action results in FCx    1 . Thus FCx will never be  0  when RDI RSI signal a new receive frame. Use with care"]
    #[inline(always)]
    pub fn nfclr3(self) -> crate::common::RegisterFieldBool<27, 1, 0, Fcnt_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, Fcnt_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Number of Frame Counter for Channel 7   NFCLR7. Clears the referring counter FCNT.FCx. Intended for use during recovery from TEI. If set while a frame is being received  this action results in FCx    1 . Thus FCx will never be  0  when RDI RSI signal a new receive frame. Use with care"]
    #[inline(always)]
    pub fn nfclr4(self) -> crate::common::RegisterFieldBool<28, 1, 0, Fcnt_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Fcnt_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Number of Frame Counter for Channel 7   NFCLR7. Clears the referring counter FCNT.FCx. Intended for use during recovery from TEI. If set while a frame is being received  this action results in FCx    1 . Thus FCx will never be  0  when RDI RSI signal a new receive frame. Use with care"]
    #[inline(always)]
    pub fn nfclr5(self) -> crate::common::RegisterFieldBool<29, 1, 0, Fcnt_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, Fcnt_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Number of Frame Counter for Channel 7   NFCLR7. Clears the referring counter FCNT.FCx. Intended for use during recovery from TEI. If set while a frame is being received  this action results in FCx    1 . Thus FCx will never be  0  when RDI RSI signal a new receive frame. Use with care"]
    #[inline(always)]
    pub fn nfclr6(self) -> crate::common::RegisterFieldBool<30, 1, 0, Fcnt_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Fcnt_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Number of Frame Counter for Channel 7   NFCLR7. Clears the referring counter FCNT.FCx. Intended for use during recovery from TEI. If set while a frame is being received  this action results in FCx    1 . Thus FCx will never be  0  when RDI RSI signal a new receive frame. Use with care"]
    #[inline(always)]
    pub fn nfclr7(self) -> crate::common::RegisterFieldBool<31, 1, 0, Fcnt_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Fcnt_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Fcnt {
    #[inline(always)]
    fn default() -> Fcnt {
        <crate::RegValueT<Fcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdo_SPEC;
impl crate::sealed::RegSpec for Fdo_SPEC {
    type DataType = u32;
}
#[doc = "Fractional Divider for Output CLK Register\n resetvalue={Application Reset:0x0}"]
pub type Fdo = crate::RegValueT<Fdo_SPEC>;

impl Fdo {
    #[doc = "Step Value   STEP. Reload or addition value for internal accumulator."]
    #[inline(always)]
    pub fn step(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Fdo_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Fdo_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Divider Mode   DM. DM selects normal or fractional divider mode."]
    #[inline(always)]
    pub fn dm(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Fdo_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Fdo_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Fdo {
    #[inline(always)]
    fn default() -> Fdo {
        <crate::RegValueT<Fdo_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdr_SPEC;
impl crate::sealed::RegSpec for Fdr_SPEC {
    type DataType = u32;
}
#[doc = "PSI5 S Fractional Divider Register\n resetvalue={Application Reset:0x0}"]
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
}
impl core::default::Default for Fdr {
    #[inline(always)]
    fn default() -> Fdr {
        <crate::RegValueT<Fdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdrt_SPEC;
impl crate::sealed::RegSpec for Fdrt_SPEC {
    type DataType = u32;
}
#[doc = "Fractional Divider Register for Time Stamp\n resetvalue={Application Reset:0x0}"]
pub type Fdrt = crate::RegValueT<Fdrt_SPEC>;

impl Fdrt {
    #[doc = "Step Value   STEP. Reload or addition value for RESULT."]
    #[inline(always)]
    pub fn step(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Fdrt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Fdrt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Divider Mode   DM. DM selects normal or fractional divider mode."]
    #[inline(always)]
    pub fn dm(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Fdrt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Fdrt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Result Value   RESULT. Bit field for the addition result."]
    #[inline(always)]
    pub fn result(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Fdrt_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Fdrt_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "External Time Stamp Clear Source Select   ECS. Selects the external trigger line that clears the global Time Stamp        Counters TSCNTA B.CTS if this is enabled by ECEA ECEB."]
    #[inline(always)]
    pub fn ecs(
        self,
    ) -> crate::common::RegisterField<26, 0x7, 1, 0, u8, Fdrt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x7,1,0,u8, Fdrt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Time Stamp Clear Enable A   ECEA. Enables the external trigger line selected by ECS to clear the global Time Stamp Counter TSCNTA.CTS on rising edge of the external trigger."]
    #[inline(always)]
    pub fn ecea(self) -> crate::common::RegisterFieldBool<29, 1, 0, Fdrt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Fdrt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "External Time Stamp Clear Enable B   ECEB. Enables the external trigger line selected by ECS to clear the global Time Stamp Counter TSCNTB.CTS on rising edge of the external trigger."]
    #[inline(always)]
    pub fn eceb(self) -> crate::common::RegisterFieldBool<30, 1, 0, Fdrt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Fdrt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Fdrt {
    #[inline(always)]
    fn default() -> Fdrt {
        <crate::RegValueT<Fdrt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdv_SPEC;
impl crate::sealed::RegSpec for Fdv_SPEC {
    type DataType = u32;
}
#[doc = "Fractional Divider Register\n resetvalue={Application Reset:0x0}"]
pub type Fdv = crate::RegValueT<Fdv_SPEC>;

impl Fdv {
    #[doc = "Fractional Divider Register Value   FD VALUE. FD VALUE contains the 11 bit value n of the fractional divider which determines the fractional divider ratio n 2048  n   0 2047 . With n   0  the fractional divider is switched off  divider ratio   1 ."]
    #[inline(always)]
    pub fn fd_value(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Fdv_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Fdv_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Fdv {
    #[inline(always)]
    fn default() -> Fdv {
        <crate::RegValueT<Fdv_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gcr_SPEC;
impl crate::sealed::RegSpec for Gcr_SPEC {
    type DataType = u32;
}
#[doc = "Global Control Register\n resetvalue={Application Reset:0x1F}"]
pub type Gcr = crate::RegValueT<Gcr_SPEC>;

impl Gcr {
    #[doc = "CRCI   CRCI. is selected if bit is set."]
    #[inline(always)]
    pub fn crci(self) -> crate::common::RegisterFieldBool<0, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "XCRCI   XCRCI. is selected if bit is set."]
    #[inline(always)]
    pub fn xcrci(self) -> crate::common::RegisterFieldBool<1, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "TEI   TEI. is selected if bit is set."]
    #[inline(always)]
    pub fn tei(self) -> crate::common::RegisterFieldBool<2, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PE   PE. is selected if bit is set."]
    #[inline(always)]
    pub fn pe(self) -> crate::common::RegisterFieldBool<3, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "FE   FE. is selected if bit is set."]
    #[inline(always)]
    pub fn fe(self) -> crate::common::RegisterFieldBool<4, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "OE   OE. is selected if bit is set."]
    #[inline(always)]
    pub fn oe(self) -> crate::common::RegisterFieldBool<5, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "RBI   RBI. is selected if bit is set."]
    #[inline(always)]
    pub fn rbi(self) -> crate::common::RegisterFieldBool<6, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "HDI   HDI. is selected if bit is set."]
    #[inline(always)]
    pub fn hdi(self) -> crate::common::RegisterFieldBool<7, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel Trigger Counter CTV7.CTC   ETC7. This bit enables CTVx.CTC. The bits ETC0   8230  x can be set with one write        access to synchronously start all counters. This is required for proper        sync pulse staggering. If set  CTCx counts on  starting from its current        value. CTCx can be written only if ETCx is cleared  stopped ."]
    #[inline(always)]
    pub fn etc0(self) -> crate::common::RegisterFieldBool<8, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel Trigger Counter CTV7.CTC   ETC7. This bit enables CTVx.CTC. The bits ETC0   8230  x can be set with one write        access to synchronously start all counters. This is required for proper        sync pulse staggering. If set  CTCx counts on  starting from its current        value. CTCx can be written only if ETCx is cleared  stopped ."]
    #[inline(always)]
    pub fn etc1(self) -> crate::common::RegisterFieldBool<9, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel Trigger Counter CTV7.CTC   ETC7. This bit enables CTVx.CTC. The bits ETC0   8230  x can be set with one write        access to synchronously start all counters. This is required for proper        sync pulse staggering. If set  CTCx counts on  starting from its current        value. CTCx can be written only if ETCx is cleared  stopped ."]
    #[inline(always)]
    pub fn etc2(self) -> crate::common::RegisterFieldBool<10, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel Trigger Counter CTV7.CTC   ETC7. This bit enables CTVx.CTC. The bits ETC0   8230  x can be set with one write        access to synchronously start all counters. This is required for proper        sync pulse staggering. If set  CTCx counts on  starting from its current        value. CTCx can be written only if ETCx is cleared  stopped ."]
    #[inline(always)]
    pub fn etc3(self) -> crate::common::RegisterFieldBool<11, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel Trigger Counter CTV7.CTC   ETC7. This bit enables CTVx.CTC. The bits ETC0   8230  x can be set with one write        access to synchronously start all counters. This is required for proper        sync pulse staggering. If set  CTCx counts on  starting from its current        value. CTCx can be written only if ETCx is cleared  stopped ."]
    #[inline(always)]
    pub fn etc4(self) -> crate::common::RegisterFieldBool<12, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel Trigger Counter CTV7.CTC   ETC7. This bit enables CTVx.CTC. The bits ETC0   8230  x can be set with one write        access to synchronously start all counters. This is required for proper        sync pulse staggering. If set  CTCx counts on  starting from its current        value. CTCx can be written only if ETCx is cleared  stopped ."]
    #[inline(always)]
    pub fn etc5(self) -> crate::common::RegisterFieldBool<13, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel Trigger Counter CTV7.CTC   ETC7. This bit enables CTVx.CTC. The bits ETC0   8230  x can be set with one write        access to synchronously start all counters. This is required for proper        sync pulse staggering. If set  CTCx counts on  starting from its current        value. CTCx can be written only if ETCx is cleared  stopped ."]
    #[inline(always)]
    pub fn etc6(self) -> crate::common::RegisterFieldBool<14, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel Trigger Counter CTV7.CTC   ETC7. This bit enables CTVx.CTC. The bits ETC0   8230  x can be set with one write        access to synchronously start all counters. This is required for proper        sync pulse staggering. If set  CTCx counts on  starting from its current        value. CTCx can be written only if ETCx is cleared  stopped ."]
    #[inline(always)]
    pub fn etc7(self) -> crate::common::RegisterFieldBool<15, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel 7   CEN7. This bit enables PSI5 S Channel x. If cleared  all internal state        machines of the receiver and the sender are forced to default idle state        while all registers can be read and written. Used for configuration of a        channel. Frames received for a disabled channel are copied to ChID 0  FID 1 with        original IDs."]
    #[inline(always)]
    pub fn cen0(self) -> crate::common::RegisterFieldBool<16, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel 7   CEN7. This bit enables PSI5 S Channel x. If cleared  all internal state        machines of the receiver and the sender are forced to default idle state        while all registers can be read and written. Used for configuration of a        channel. Frames received for a disabled channel are copied to ChID 0  FID 1 with        original IDs."]
    #[inline(always)]
    pub fn cen1(self) -> crate::common::RegisterFieldBool<17, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel 7   CEN7. This bit enables PSI5 S Channel x. If cleared  all internal state        machines of the receiver and the sender are forced to default idle state        while all registers can be read and written. Used for configuration of a        channel. Frames received for a disabled channel are copied to ChID 0  FID 1 with        original IDs."]
    #[inline(always)]
    pub fn cen2(self) -> crate::common::RegisterFieldBool<18, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel 7   CEN7. This bit enables PSI5 S Channel x. If cleared  all internal state        machines of the receiver and the sender are forced to default idle state        while all registers can be read and written. Used for configuration of a        channel. Frames received for a disabled channel are copied to ChID 0  FID 1 with        original IDs."]
    #[inline(always)]
    pub fn cen3(self) -> crate::common::RegisterFieldBool<19, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel 7   CEN7. This bit enables PSI5 S Channel x. If cleared  all internal state        machines of the receiver and the sender are forced to default idle state        while all registers can be read and written. Used for configuration of a        channel. Frames received for a disabled channel are copied to ChID 0  FID 1 with        original IDs."]
    #[inline(always)]
    pub fn cen4(self) -> crate::common::RegisterFieldBool<20, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel 7   CEN7. This bit enables PSI5 S Channel x. If cleared  all internal state        machines of the receiver and the sender are forced to default idle state        while all registers can be read and written. Used for configuration of a        channel. Frames received for a disabled channel are copied to ChID 0  FID 1 with        original IDs."]
    #[inline(always)]
    pub fn cen5(self) -> crate::common::RegisterFieldBool<21, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel 7   CEN7. This bit enables PSI5 S Channel x. If cleared  all internal state        machines of the receiver and the sender are forced to default idle state        while all registers can be read and written. Used for configuration of a        channel. Frames received for a disabled channel are copied to ChID 0  FID 1 with        original IDs."]
    #[inline(always)]
    pub fn cen6(self) -> crate::common::RegisterFieldBool<22, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Channel 7   CEN7. This bit enables PSI5 S Channel x. If cleared  all internal state        machines of the receiver and the sender are forced to default idle state        while all registers can be read and written. Used for configuration of a        channel. Frames received for a disabled channel are copied to ChID 0  FID 1 with        original IDs."]
    #[inline(always)]
    pub fn cen7(self) -> crate::common::RegisterFieldBool<23, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Idle Time  GLOBAL VALUE FOR ALL CHANNELS    IDT. determines the number of stop bits in addition to the stop bit of the        last UART Frame that is required for SOF detection.  IDT 1  idle bit        times are allowed tolerated within one Packet Frame. Default is IDT   0         i.e. back to back transfer."]
    #[inline(always)]
    pub fn idt(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Gcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ASC only Mode   ASC. is selected if bit is set. The ASC registers are fully controllable by        SW via SPB. If cleared  the ASC is controlled by the message reassembly unit and the        message generation unit. RBUF and TBUF are no longer writable by SW and        interrupts are handled by the message reassembly block automatically."]
    #[inline(always)]
    pub fn asc(self) -> crate::common::RegisterFieldBool<31, 1, 0, Gcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Gcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Gcr {
    #[inline(always)]
    fn default() -> Gcr {
        <crate::RegValueT<Gcr_SPEC> as RegisterValue<_>>::new(31)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0D3C000}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MODREV. This bit field defines the module revision number. The value of a module revision starts with 01 H  first revision ."]
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
    #[doc = "Module Number Value   MODNUM. This bit field defines the module identification number for the PSI5 S  00D3 H"]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(13877248)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inpg_SPEC;
impl crate::sealed::RegSpec for Inpg_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Node Pointer Register Global\n resetvalue={Application Reset:0x0}"]
pub type Inpg = crate::RegValueT<Inpg_SPEC>;

impl Inpg {
    #[doc = "Interrupt Node Pointer for Interrupt TIR   TIR. This bit field defines the interrupt node  that is requested due to the        set condition for bit INTSTATG.TIR  if enabled by bit INTENG.TIR ."]
    #[inline(always)]
    pub fn tir(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Inpg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Inpg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for Interrupt RIR   RIR. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATG.RIR  if enabled by bit INTENG.RIR . For bit field definition  see TIR."]
    #[inline(always)]
    pub fn rir(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, Inpg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x7,1,0,u8, Inpg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for Interrupt EIR   EIR. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATG.EIR  if enabled by bit INTENG.EIR . For bit field definition  see TIR."]
    #[inline(always)]
    pub fn eir(
        self,
    ) -> crate::common::RegisterField<6, 0x7, 1, 0, u8, Inpg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x7,1,0,u8, Inpg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for Interrupt TBIR   TBIR. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATG.TBIR  if enabled by bit INTENG.TBIR . For bit field definition  see TIR."]
    #[inline(always)]
    pub fn tbir(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, Inpg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7,1,0,u8, Inpg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for Interrupt XCRCI   XCRCI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATG.XCRCI  if enabled by bit INTENG.XCRCI . For bit field definition  see TIR."]
    #[inline(always)]
    pub fn xcrci(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, Inpg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8, Inpg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for Interrupt FOI   FOI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATG.FOI  if enabled by bit INTENG.FOI . For bit field definition  see TIR."]
    #[inline(always)]
    pub fn foi(
        self,
    ) -> crate::common::RegisterField<15, 0x7, 1, 0, u8, Inpg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<15,0x7,1,0,u8, Inpg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Inpg {
    #[inline(always)]
    fn default() -> Inpg {
        <crate::RegValueT<Inpg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InPx_SPEC;
impl crate::sealed::RegSpec for InPx_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Node Pointer Register 0\n resetvalue={Application Reset:0x0}"]
pub type InPx = crate::RegValueT<InPx_SPEC>;

impl InPx {
    #[doc = "Interrupt Node Pointer for Interrupt RSI   RSI. This bit field defines the interrupt node  that is requested due to the        set condition for bit INTSTATx.RSI  if enabled by bit INTENx.RSI ."]
    #[inline(always)]
    pub fn rsi(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, InPx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for Interrupt RDI   RDI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATx.RDI  if enabled by bit INTENx.RDI . For bit field definition  see RSI."]
    #[inline(always)]
    pub fn rdi(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, InPx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x7,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for Interrupt RBI   RBI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATx.RBI  if enabled by bit INTENx.RBI . For bit field definition  see RSI."]
    #[inline(always)]
    pub fn rbi(
        self,
    ) -> crate::common::RegisterField<6, 0x7, 1, 0, u8, InPx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x7,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for Interrupt TEI   TEI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATx.TEI  if enabled by bit INTENx.TEI . For bit field definition  see RSI."]
    #[inline(always)]
    pub fn tei(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, InPx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for Interrupt CHCI   CHCI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATx.CHCI  if enabled by bit INTENx.CHCI . For bit field definition  see RSI."]
    #[inline(always)]
    pub fn chci(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, InPx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for Interrupt CRCI   CRCI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATx.CRCI. For bit field definition  see RSI."]
    #[inline(always)]
    pub fn crci(
        self,
    ) -> crate::common::RegisterField<15, 0x7, 1, 0, u8, InPx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<15,0x7,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for Interrupt TOI   TPI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATx.TPI  if enabled by bit INTENx.TPI . For bit field definition  see RSI."]
    #[inline(always)]
    pub fn tpi(
        self,
    ) -> crate::common::RegisterField<18, 0x7, 1, 0, u8, InPx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x7,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for TPOI   TPOI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATx.TPOI. For bit field definition  see RSI."]
    #[inline(always)]
    pub fn tpoi(
        self,
    ) -> crate::common::RegisterField<21, 0x7, 1, 0, u8, InPx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x7,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for HDI   HDI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATx.HDI. For bit field definition  see RSI."]
    #[inline(always)]
    pub fn hdi(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, InPx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for InPx {
    #[inline(always)]
    fn default() -> InPx {
        <crate::RegValueT<InPx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intclrg_SPEC;
impl crate::sealed::RegSpec for Intclrg_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Clear Register Global\n resetvalue={Application Reset:0x0}"]
pub type Intclrg = crate::RegValueT<Intclrg_SPEC>;

impl Intclrg {
    #[doc = "Clear Interrupt Request Flag TIR   TIR. Setting this bit clears bit INTSTATG.TIR. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tir(self) -> crate::common::RegisterFieldBool<0, 1, 0, Intclrg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Intclrg_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag RIR   RIR. Setting this bit clears bit INTSTATG.RIR. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rir(self) -> crate::common::RegisterFieldBool<1, 1, 0, Intclrg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Intclrg_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag EIR   EIR. Setting this bit clears bit INTSTATG.EIR. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn eir(self) -> crate::common::RegisterFieldBool<2, 1, 0, Intclrg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Intclrg_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag TBIR   TBIR. Setting this bit clears bit INTSTATG.TBIR. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tbir(self) -> crate::common::RegisterFieldBool<3, 1, 0, Intclrg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Intclrg_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag XCRCI   XCRCI. Setting this bit clears bit INTSTATG.XCRCI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn xcrci(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Intclrg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Intclrg_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag FOI   FOI. Setting this bit clears bit INTSTATG.FOI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn foi(self) -> crate::common::RegisterFieldBool<5, 1, 0, Intclrg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Intclrg_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Intclrg {
    #[inline(always)]
    fn default() -> Intclrg {
        <crate::RegValueT<Intclrg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntclRx_SPEC;
impl crate::sealed::RegSpec for IntclRx_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Clear Register 0\n resetvalue={Application Reset:0x0}"]
pub type IntclRx = crate::RegValueT<IntclRx_SPEC>;

impl IntclRx {
    #[doc = "Clear Interrupt Request Flag RSI   RSI. Setting this bit clears bit INTSTATx.RSI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi(self) -> crate::common::RegisterFieldBool<0, 1, 0, IntclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, IntclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag RDI   RDI. Setting this bit clears bit INTSTATx.RDI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi(self) -> crate::common::RegisterFieldBool<1, 1, 0, IntclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, IntclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag RBI   RBI. Setting this bit clears bit INTSTATx.RBI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rbi(self) -> crate::common::RegisterFieldBool<2, 1, 0, IntclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, IntclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag TEI   TEI. Setting this bit clears bit INTSTATx.TEI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei(self) -> crate::common::RegisterFieldBool<3, 1, 0, IntclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, IntclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag CHCI   CHCI. Setting this bit clears bit INTSTATx.CHCI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn chci(self) -> crate::common::RegisterFieldBool<4, 1, 0, IntclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, IntclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag CRCI   CRCI. Setting this bit clears bit INTSTATx.CRCI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci(self) -> crate::common::RegisterFieldBool<5, 1, 0, IntclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, IntclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag TPI   TPI. Setting this bit clears bit INTSTATx.TPI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tpi(self) -> crate::common::RegisterFieldBool<6, 1, 0, IntclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, IntclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag TPOI   TPOI. Setting this bit clears bit INTSTATx.TPOI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tpoi(self) -> crate::common::RegisterFieldBool<7, 1, 0, IntclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, IntclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Interrupt Request Flag HDI   HDI. Setting this bit clears bit INTSTATx.HDI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn hdi(self) -> crate::common::RegisterFieldBool<8, 1, 0, IntclRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, IntclRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for IntclRx {
    #[inline(always)]
    fn default() -> IntclRx {
        <crate::RegValueT<IntclRx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inteng_SPEC;
impl crate::sealed::RegSpec for Inteng_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Enable Register Global\n resetvalue={Application Reset:0x0}"]
pub type Inteng = crate::RegValueT<Inteng_SPEC>;

impl Inteng {
    #[doc = "Enable Interrupt Request TIR   TIR"]
    #[inline(always)]
    pub fn tir(self) -> crate::common::RegisterFieldBool<0, 1, 0, Inteng_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Inteng_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request RIR   RIR"]
    #[inline(always)]
    pub fn rir(self) -> crate::common::RegisterFieldBool<1, 1, 0, Inteng_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Inteng_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request EIR   EIR"]
    #[inline(always)]
    pub fn eir(self) -> crate::common::RegisterFieldBool<2, 1, 0, Inteng_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Inteng_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request TBIR   TBIR"]
    #[inline(always)]
    pub fn tbir(self) -> crate::common::RegisterFieldBool<3, 1, 0, Inteng_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Inteng_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request XCRCI   XCRCI"]
    #[inline(always)]
    pub fn xcrci(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Inteng_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Inteng_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request FOI   FOI"]
    #[inline(always)]
    pub fn foi(self) -> crate::common::RegisterFieldBool<5, 1, 0, Inteng_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Inteng_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Inteng {
    #[inline(always)]
    fn default() -> Inteng {
        <crate::RegValueT<Inteng_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InteNx_SPEC;
impl crate::sealed::RegSpec for InteNx_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Enable Register 0\n resetvalue={Application Reset:0x0}"]
pub type InteNx = crate::RegValueT<InteNx_SPEC>;

impl InteNx {
    #[doc = "Enable Interrupt Request RSI   RSI"]
    #[inline(always)]
    pub fn rsi(self) -> crate::common::RegisterFieldBool<0, 1, 0, InteNx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, InteNx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request RDI   RDI"]
    #[inline(always)]
    pub fn rdi(self) -> crate::common::RegisterFieldBool<1, 1, 0, InteNx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, InteNx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request RBI   RBI"]
    #[inline(always)]
    pub fn rbi(self) -> crate::common::RegisterFieldBool<2, 1, 0, InteNx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, InteNx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request TEI   TEI"]
    #[inline(always)]
    pub fn tei(self) -> crate::common::RegisterFieldBool<3, 1, 0, InteNx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, InteNx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request CHCI   CHCI"]
    #[inline(always)]
    pub fn chci(self) -> crate::common::RegisterFieldBool<4, 1, 0, InteNx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, InteNx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request CRCI   CRCI"]
    #[inline(always)]
    pub fn crci(self) -> crate::common::RegisterFieldBool<5, 1, 0, InteNx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, InteNx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request TPI   TPI"]
    #[inline(always)]
    pub fn tpi(self) -> crate::common::RegisterFieldBool<6, 1, 0, InteNx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, InteNx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request TPOI   TPOI"]
    #[inline(always)]
    pub fn tpoi(self) -> crate::common::RegisterFieldBool<7, 1, 0, InteNx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, InteNx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Interrupt Request HDI   HDI"]
    #[inline(always)]
    pub fn hdi(self) -> crate::common::RegisterFieldBool<8, 1, 0, InteNx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, InteNx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for InteNx {
    #[inline(always)]
    fn default() -> InteNx {
        <crate::RegValueT<InteNx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intov_SPEC;
impl crate::sealed::RegSpec for Intov_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Overview Register\n resetvalue={Application Reset:0x0}"]
pub type Intov = crate::RegValueT<Intov_SPEC>;

impl Intov {
    #[doc = "Interrupt Pending on Node Pointer RSI   RSI. If any interrupt requested flag is set for this Node Pointer in register  INTSTATx or INTSTATG  AND the referring interrupt is enabled in  INTENx or INTENG  then this bit is set. It is automatically reset if all flags in INTSTATx G are cleared for which the referring interrupt is enabled in INTENx G."]
    #[inline(always)]
    pub fn rsi(self) -> crate::common::RegisterFieldBool<0, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Node Pointer RDI   RDI. See details of INTOV.RSI."]
    #[inline(always)]
    pub fn rdi(self) -> crate::common::RegisterFieldBool<1, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Node Pointer RBI   RBI. See details of INTOV.RSI."]
    #[inline(always)]
    pub fn rbi(self) -> crate::common::RegisterFieldBool<2, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Node Pointer TEI   TEI. See details of INTOV.RSI."]
    #[inline(always)]
    pub fn tei(self) -> crate::common::RegisterFieldBool<3, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Node Pointer CHCI   CHCI. See details of INTOV.RSI."]
    #[inline(always)]
    pub fn chci(self) -> crate::common::RegisterFieldBool<4, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Node Pointer CRCI   CRCI. See details of INTOV.RSI."]
    #[inline(always)]
    pub fn crci(self) -> crate::common::RegisterFieldBool<5, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Node Pointer TPI   TPI. See details of INTOV.RSI."]
    #[inline(always)]
    pub fn tpi(self) -> crate::common::RegisterFieldBool<6, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Node Pointer TPOI   TPOI. See details of INTOV.RSI."]
    #[inline(always)]
    pub fn tpoi(self) -> crate::common::RegisterFieldBool<7, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Node Pointer HDI   HDI. See details of INTOV.HDI."]
    #[inline(always)]
    pub fn hdi(self) -> crate::common::RegisterFieldBool<8, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Node Pointer TIR   TIR. If any interrupt requested flag is set for this Node Pointer in register INTSTATG AND the referring interrupt is enabled in INTENG then this bit is set. It is automatically reset if all flags in INTSTATG are cleared for which the referring interrupt is enabled in INTENG."]
    #[inline(always)]
    pub fn tir(self) -> crate::common::RegisterFieldBool<9, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Node Pointer RIR   RIR. See details of INTOV.TIR."]
    #[inline(always)]
    pub fn rir(self) -> crate::common::RegisterFieldBool<10, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Node Pointer EIR   EIR. See details of INTOV.TIR."]
    #[inline(always)]
    pub fn eir(self) -> crate::common::RegisterFieldBool<11, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Node Pointer TBIR   TBIR. See details of INTOV.TIR."]
    #[inline(always)]
    pub fn tbir(self) -> crate::common::RegisterFieldBool<12, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Node Pointer XCRCI   XCRCI. See details of INTOV.TIR."]
    #[inline(always)]
    pub fn xcrci(self) -> crate::common::RegisterFieldBool<13, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Node Pointer FOI   FOI. See details of INTOV.TIR."]
    #[inline(always)]
    pub fn foi(self) -> crate::common::RegisterFieldBool<14, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Intov {
    #[inline(always)]
    fn default() -> Intov {
        <crate::RegValueT<Intov_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intsetg_SPEC;
impl crate::sealed::RegSpec for Intsetg_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Set Register Global\n resetvalue={Application Reset:0x0}"]
pub type Intsetg = crate::RegValueT<Intsetg_SPEC>;

impl Intsetg {
    #[doc = "Set Interrupt Request Flag TIR   TIR. Setting this bit set bit INTSTATG.TIR. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tir(self) -> crate::common::RegisterFieldBool<0, 1, 0, Intsetg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Intsetg_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag RIR   RIR. Setting this bit set bit INTSTATG.RIR. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rir(self) -> crate::common::RegisterFieldBool<1, 1, 0, Intsetg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Intsetg_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag EIR   EIR. Setting this bit set bit INTSTATG.EIR. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn eir(self) -> crate::common::RegisterFieldBool<2, 1, 0, Intsetg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Intsetg_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag TBIR   TBIR. Setting this bit set bit INTSTATG.TBIR. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tbir(self) -> crate::common::RegisterFieldBool<3, 1, 0, Intsetg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Intsetg_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag XCRCI   XCRCI. Setting this bit set bit INTSTATG.XCRCI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn xcrci(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Intsetg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Intsetg_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag FOI   FOI. Setting this bit set bit INTSTATG.FOI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn foi(self) -> crate::common::RegisterFieldBool<5, 1, 0, Intsetg_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Intsetg_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Intsetg {
    #[inline(always)]
    fn default() -> Intsetg {
        <crate::RegValueT<Intsetg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntseTx_SPEC;
impl crate::sealed::RegSpec for IntseTx_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Set Register 0\n resetvalue={Application Reset:0x0}"]
pub type IntseTx = crate::RegValueT<IntseTx_SPEC>;

impl IntseTx {
    #[doc = "Set Interrupt Request Flag RSI   RSI. Setting this bit set bit INTSTATx.RSI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rsi(self) -> crate::common::RegisterFieldBool<0, 1, 0, IntseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, IntseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag RDI   RDI. Setting this bit set bit INTSTATx.RDI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rdi(self) -> crate::common::RegisterFieldBool<1, 1, 0, IntseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, IntseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag RBI   RBI. Setting this bit set bit INTSTATx.RBI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn rbi(self) -> crate::common::RegisterFieldBool<2, 1, 0, IntseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, IntseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag TEI   TEI. Setting this bit set bit INTSTATx.TEI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tei(self) -> crate::common::RegisterFieldBool<3, 1, 0, IntseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, IntseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag CHCI   CHCI. Setting this bit set bit INTSTATx.CHCI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn chci(self) -> crate::common::RegisterFieldBool<4, 1, 0, IntseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, IntseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag CRCI   CRCI. Setting this bit set bit INTSTATx.CRCI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn crci(self) -> crate::common::RegisterFieldBool<5, 1, 0, IntseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, IntseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag TPI   TPI. Setting this bit set bit INTSTATx.TPI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tpi(self) -> crate::common::RegisterFieldBool<6, 1, 0, IntseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, IntseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag TPOI   TPOI. Setting this bit set bit INTSTATx.TPOI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn tpoi(self) -> crate::common::RegisterFieldBool<7, 1, 0, IntseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, IntseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Interrupt Request Flag HDI   HDI. Setting this bit set bit INTSTATx.HDI. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn hdi(self) -> crate::common::RegisterFieldBool<8, 1, 0, IntseTx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, IntseTx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for IntseTx {
    #[inline(always)]
    fn default() -> IntseTx {
        <crate::RegValueT<IntseTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstatg_SPEC;
impl crate::sealed::RegSpec for Intstatg_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Status Register Global\n resetvalue={Application Reset:0x0}"]
pub type Intstatg = crate::RegValueT<Intstatg_SPEC>;

impl Intstatg {
    #[doc = "Transmit Interrupt Request Flag   TIR. This bit can be cleared by bit INTCLRG.TIR. This bit can be set by bit INTSETG.TIR. This bit is set independently from INTENG."]
    #[inline(always)]
    pub fn tir(self) -> crate::common::RegisterFieldBool<0, 1, 0, Intstatg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Intstatg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive Interrupt Request Flag   RIR. This bit can be cleared by bit INTCLRG.RIR. This bit can be set by bit INTSETG.RIR. This bit is set independently from INTENG."]
    #[inline(always)]
    pub fn rir(self) -> crate::common::RegisterFieldBool<1, 1, 0, Intstatg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Intstatg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Interrupt Request Flag   EIR. The cause of an error interrupt request EIR  framing  parity  overrun error  can be identified by the error status flags CON.FE  CON.PE  and CON.OE  This bit can be cleared by bit INTCLRG.EIR. This bit can be set by bit INTSETG.EIR. This bit is set independently from INTENG."]
    #[inline(always)]
    pub fn eir(self) -> crate::common::RegisterFieldBool<2, 1, 0, Intstatg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Intstatg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmit Buffer Interrupt Request Flag   TBIR. This bit can be cleared by bit INTCLRG.TBIR. This bit can be set by bit INTSETG.TBIR. This bit is set independently from INTENG."]
    #[inline(always)]
    pub fn tbir(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Intstatg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Intstatg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "XCRC Error Request Flag   XCRCI. This bit is set if the CRC check on the enveloping Packet Frame fails including the case where XCRC check can not be performed  non recoverable frames  see CROSSREFERENCE .The received data is not reliable and stored in ChID  0   FID  1  with original IDs. This bit can be cleared by bit INTCLRG.XCRCI. This bit can be set by bit INTSETG.XCRCI. This bit is set independently from INTENG."]
    #[inline(always)]
    pub fn xcrci(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Intstatg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Intstatg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "FIFO Error Request Flag   FOI. This bit is set if the Transmit FIFO of the message generation is overrun i.e. a transfer to the FIFO was generated by the message generation unit or by CDW  CPU direct write register  while the FIFO was already full. This bit can be cleared by bit INTCLRG.FOI. This bit can be set by bit INTSETG.FOI. This bit is set independently from INTENG."]
    #[inline(always)]
    pub fn foi(self) -> crate::common::RegisterFieldBool<5, 1, 0, Intstatg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Intstatg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Intstatg {
    #[inline(always)]
    fn default() -> Intstatg {
        <crate::RegValueT<Intstatg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntstaTx_SPEC;
impl crate::sealed::RegSpec for IntstaTx_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Status Register 0\n resetvalue={Application Reset:0x0}"]
pub type IntstaTx = crate::RegValueT<IntstaTx_SPEC>;

impl IntstaTx {
    #[doc = "Receive Success Interrupt Request Flag   RSI. This bit is set at the successfully received end of a frame. It indicates that this frame is free of the errors CRCI  XCRCI  TEI  PE  FE  OE  RBI  HDI if selected to be taken into account in register GCR. This bit can be cleared by bit INTCLRx.RSI. This bit can be set by bit INTSETx.RSI. This bit is set independently from INTENx."]
    #[inline(always)]
    pub fn rsi(self) -> crate::common::RegisterFieldBool<0, 1, 0, IntstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, IntstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive Data Interrupt Request Flag   RDI. RDI is activated when a received frame is moved to a Receive Data Register RDR. Both RDI and RSI will be issued together at correct reception. This bit can be cleared by bit INTCLRx.RDI. This bit can be set by bit INTSETx.RDI. This bit is set independently from INTENx."]
    #[inline(always)]
    pub fn rdi(self) -> crate::common::RegisterFieldBool<1, 1, 0, IntstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, IntstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive Buffer Overflow Interrupt Request Flag   RBI. This bit is set after a frame has been received while the old one was not read from RDR. I.e. the kernel wants to set interrupt RDI and finds this interrupt already set. The old data is overwritten by the new data. This bit is NOT cleared by reading RDR. This bit can be cleared by bit INTCLRx.RBI. This bit can be set by bit INTSETx.RBI. This bit is set independently from INTENx."]
    #[inline(always)]
    pub fn rbi(self) -> crate::common::RegisterFieldBool<2, 1, 0, IntstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, IntstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Timing Error Interrupt Request Flag   TEI. This bit is set if the watch dog timer expired. Depending from RCRAx.WDMS either the distance between two RDIs is longer than specified in WDL or it expired without reception of CHCI in time. I.e. the time from issuing the sync pulse to reception of the last expected frame configured in NFC.NFx was too long. Note that the root cause might be a non recoverable frame  This bit can be cleared by bit INTCLRx.TEI. This bit can be set by bit INTSETx.TEI. This bit is set independently from INTENx."]
    #[inline(always)]
    pub fn tei(self) -> crate::common::RegisterFieldBool<3, 1, 0, IntstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, IntstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel Completed Interrupt Request Flag   CHCI. This bit is set if FCNT.FCx equals NFC.NFx. This bit can be cleared by bit INTCLRx.CHCI. This bit can be set by bit INTSETx.CHCI. This bit is set independently from INTENx."]
    #[inline(always)]
    pub fn chci(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, IntstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, IntstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Error Request Flag   CRCI. This bit is set if the CRC fails. This bit can be cleared by bit INTCLRx.CRCI. This bit can be set by bit INTSETx.CRCI. This bit is set independently from INTENx."]
    #[inline(always)]
    pub fn crci(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, IntstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, IntstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transfer Preparation Interrupt Request Flag   TPI. This bit is set after data to be transferred has been moved completely. Thus a new value can be written to SDRx. This bit can be cleared by bit INTCLRx.TPI. This bit can be set by bit INTSETx.TPI. This bit is set independently from INTENx."]
    #[inline(always)]
    pub fn tpi(self) -> crate::common::RegisterFieldBool<6, 1, 0, IntstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, IntstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmit Preparation Overflow Interrupt Request Flag   TPOI. This bit is set if SDR is written while TPF is set.  The old data is NOT overwritten. This bit can be cleared by bit INTCLRx.TPOI. This bit can be set by bit INTSETx.TPOI. This bit is set independently from INTENx."]
    #[inline(always)]
    pub fn tpoi(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, IntstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, IntstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Header Error Signalled Flag   HDI. This bit is set if at least one of the error signalling flags in the enveloping Packet Frame Err0 and Err1 is set. Up"]
    #[inline(always)]
    pub fn hdi(self) -> crate::common::RegisterFieldBool<8, 1, 0, IntstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, IntstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for IntstaTx {
    #[inline(always)]
    fn default() -> IntstaTx {
        <crate::RegValueT<IntstaTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iocr_SPEC;
impl crate::sealed::RegSpec for Iocr_SPEC {
    type DataType = u32;
}
#[doc = "Input and Output Control Register\n resetvalue={Application Reset:0x0}"]
pub type Iocr = crate::RegValueT<Iocr_SPEC>;

impl Iocr {
    #[doc = "Alternate Input Select   ALTI. Selects the alternate input for RX of the ASC"]
    #[inline(always)]
    pub fn alti(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Iocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Iocr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Iocr {
    #[inline(always)]
    fn default() -> Iocr {
        <crate::RegValueT<Iocr_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel registers are set. The RST bit will be cleared  re set to  0   by the BPI FPI after the kernel reset was executed."]
    #[inline(always)]
    pub fn rst(self) -> crate::common::RegisterFieldBool<0, 1, 0, Krst0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Krst0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Kernel Reset Status   RSTSTAT. This bit indicates wether a kernel reset was executed or not. This bit is set by the BPI FPI after the execution of a kernel reset in the same clock cycle both reset bits. This bit can be cleared by writing with  1  to the CLR bit in the related KRSTCLR register."]
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel reset registers is set. The RST bit will be cleared  re set to  0   by the BPI FPI after the kernel reset was executed."]
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
pub struct Nfc_SPEC;
impl crate::sealed::RegSpec for Nfc_SPEC {
    type DataType = u32;
}
#[doc = "Number of Frames Control Register\n resetvalue={Application Reset:0x249249}"]
pub type Nfc = crate::RegValueT<Nfc_SPEC>;

impl Nfc {
    #[doc = "Number of expected Frames on Channel 7   NF7"]
    #[inline(always)]
    pub fn nf0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Nfc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0x7, 1, 0, u8, Nfc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Number of expected Frames on Channel 7   NF7"]
    #[inline(always)]
    pub fn nf1(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, Nfc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3, 0x7, 1, 0, u8, Nfc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Number of expected Frames on Channel 7   NF7"]
    #[inline(always)]
    pub fn nf2(
        self,
    ) -> crate::common::RegisterField<6, 0x7, 1, 0, u8, Nfc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6, 0x7, 1, 0, u8, Nfc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Number of expected Frames on Channel 7   NF7"]
    #[inline(always)]
    pub fn nf3(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, Nfc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9, 0x7, 1, 0, u8, Nfc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Number of expected Frames on Channel 7   NF7"]
    #[inline(always)]
    pub fn nf4(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, Nfc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8, Nfc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Number of expected Frames on Channel 7   NF7"]
    #[inline(always)]
    pub fn nf5(
        self,
    ) -> crate::common::RegisterField<15, 0x7, 1, 0, u8, Nfc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<15,0x7,1,0,u8, Nfc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Number of expected Frames on Channel 7   NF7"]
    #[inline(always)]
    pub fn nf6(
        self,
    ) -> crate::common::RegisterField<18, 0x7, 1, 0, u8, Nfc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x7,1,0,u8, Nfc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Number of expected Frames on Channel 7   NF7"]
    #[inline(always)]
    pub fn nf7(
        self,
    ) -> crate::common::RegisterField<21, 0x7, 1, 0, u8, Nfc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x7,1,0,u8, Nfc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Nfc {
    #[inline(always)]
    fn default() -> Nfc {
        <crate::RegValueT<Nfc_SPEC> as RegisterValue<_>>::new(2396745)
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
    #[doc = "OCDS Suspend Control   SUS. Controls the sensitivity to the suspend signal coming from the OCDS Trigger Switch  OTGS  Suspend disables the kernel clocks f PSI5 S and the ASC clock f ASC ."]
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
pub struct PgCx_SPEC;
impl crate::sealed::RegSpec for PgCx_SPEC {
    type DataType = u32;
}
#[doc = "Pulse Generation Control Register 0\n resetvalue={Application Reset:0x0}"]
pub type PgCx = crate::RegValueT<PgCx_SPEC>;

impl PgCx {
    #[doc = "TX Command   TXCMD. Defines the value that is copied to the ASC FIFO for coding a  0 ."]
    #[inline(always)]
    pub fn txcmd(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, PgCx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, PgCx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alternate TX Command   ATXCMD. Defines the value that is copied to the ASC FIFO for the alternate pulse width i.e. for coding a  1 ."]
    #[inline(always)]
    pub fn atxcmd(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, PgCx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, PgCx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Time Base Select   TBS. This bit selects the clock source for CTVx"]
    #[inline(always)]
    pub fn tbs(self) -> crate::common::RegisterFieldBool<15, 1, 0, PgCx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, PgCx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "External Time Base Select   ETB. Selects the external clock line for counter CTVx."]
    #[inline(always)]
    pub fn etb(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, PgCx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8, PgCx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Periodic Trigger Enable   PTE. Periodic trigger is defined by CTVx.  Should be 0 if ETE is set."]
    #[inline(always)]
    pub fn pte(self) -> crate::common::RegisterFieldBool<19, 1, 0, PgCx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, PgCx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "External Trigger Select   ETS. Selects the external trigger line for pulse generation  e.g. angle        synchronous ."]
    #[inline(always)]
    pub fn ets(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, PgCx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x7,1,0,u8, PgCx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Trigger Enable   ETE.  Angle sync. trigger   external line is selected by ETS. Should be 0 if PTE is set."]
    #[inline(always)]
    pub fn ete(self) -> crate::common::RegisterFieldBool<23, 1, 0, PgCx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, PgCx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for PgCx {
    #[inline(always)]
    fn default() -> PgCx {
        <crate::RegValueT<PgCx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rbuf_SPEC;
impl crate::sealed::RegSpec for Rbuf_SPEC {
    type DataType = u32;
}
#[doc = "Receive Buffer Register\n resetvalue={Application Reset:0x0}"]
pub type Rbuf = crate::RegValueT<Rbuf_SPEC>;

impl Rbuf {
    #[doc = "Receive Data Register Value   RD VALUE. RBUF contains the received data bits and  depending on the selected mode  the parity bit in the asynchronous and synchronous operating modes of the ASC. In Asynchronous Mode  with CON.M   011 B  7 bit data   parity   the received parity bit is written into RBUF.7. In Asynchronous Mode  with CON.M   111 B  8 bit data   parity   the received parity bit is written into RBUF.8."]
    #[inline(always)]
    pub fn rd_value(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, Rbuf_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16, Rbuf_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Rbuf {
    #[inline(always)]
    fn default() -> Rbuf {
        <crate::RegValueT<Rbuf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RcrAx_SPEC;
impl crate::sealed::RegSpec for RcrAx_SPEC {
    type DataType = u32;
}
#[doc = "Receiver Control Register A0\n resetvalue={Application Reset:0x0}"]
pub type RcrAx = crate::RegValueT<RcrAx_SPEC>;

impl RcrAx {
    #[doc = "CRC or Parit5 Selection   CRC5. If set  a 3 bit CRC checksum is expected for the PSI5 S channel x in slot y. Else  1 bit Parity is assumed. This bit field is looked up before potential modification of FID according to FIDS."]
    #[inline(always)]
    pub fn crc0(self) -> crate::common::RegisterFieldBool<0, 1, 0, RcrAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, RcrAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC or Parit5 Selection   CRC5. If set  a 3 bit CRC checksum is expected for the PSI5 S channel x in slot y. Else  1 bit Parity is assumed. This bit field is looked up before potential modification of FID according to FIDS."]
    #[inline(always)]
    pub fn crc1(self) -> crate::common::RegisterFieldBool<1, 1, 0, RcrAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, RcrAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC or Parit5 Selection   CRC5. If set  a 3 bit CRC checksum is expected for the PSI5 S channel x in slot y. Else  1 bit Parity is assumed. This bit field is looked up before potential modification of FID according to FIDS."]
    #[inline(always)]
    pub fn crc2(self) -> crate::common::RegisterFieldBool<2, 1, 0, RcrAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, RcrAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC or Parit5 Selection   CRC5. If set  a 3 bit CRC checksum is expected for the PSI5 S channel x in slot y. Else  1 bit Parity is assumed. This bit field is looked up before potential modification of FID according to FIDS."]
    #[inline(always)]
    pub fn crc3(self) -> crate::common::RegisterFieldBool<3, 1, 0, RcrAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, RcrAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC or Parit5 Selection   CRC5. If set  a 3 bit CRC checksum is expected for the PSI5 S channel x in slot y. Else  1 bit Parity is assumed. This bit field is looked up before potential modification of FID according to FIDS."]
    #[inline(always)]
    pub fn crc4(self) -> crate::common::RegisterFieldBool<4, 1, 0, RcrAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, RcrAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC or Parit5 Selection   CRC5. If set  a 3 bit CRC checksum is expected for the PSI5 S channel x in slot y. Else  1 bit Parity is assumed. This bit field is looked up before potential modification of FID according to FIDS."]
    #[inline(always)]
    pub fn crc5(self) -> crate::common::RegisterFieldBool<5, 1, 0, RcrAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, RcrAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Time Stamp Enable   TSEN. Enables the time stamping for channel x"]
    #[inline(always)]
    pub fn tsen(self) -> crate::common::RegisterFieldBool<6, 1, 0, RcrAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, RcrAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Time Stamp Select   TSP. For non recoverable Packets  stored in ChID  0   FID  1  with original IDs   TSCNTA is captured in TSM and not in TSCR0  independent from TSP ."]
    #[inline(always)]
    pub fn tsp(self) -> crate::common::RegisterFieldBool<7, 1, 0, RcrAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, RcrAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Time Stamp Trigger Select   TSTS"]
    #[inline(always)]
    pub fn tsts(self) -> crate::common::RegisterFieldBool<8, 1, 0, RcrAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, RcrAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Frame ID Select   FIDS"]
    #[inline(always)]
    pub fn fids(self) -> crate::common::RegisterFieldBool<9, 1, 0, RcrAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, RcrAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Watch Dog Timer Mode Select   WDMS"]
    #[inline(always)]
    pub fn wdms(self) -> crate::common::RegisterFieldBool<10, 1, 0, RcrAx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, RcrAx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "UART Frame Count per Packet Frame in Slot 5   UFC5. This bit field defines the number of UART Frames per Packet Frame that are expected for Slot y. This bit field is looked up before potential modification of FID according to FIDS."]
    #[inline(always)]
    pub fn ufc0(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, RcrAx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, RcrAx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "UART Frame Count per Packet Frame in Slot 5   UFC5. This bit field defines the number of UART Frames per Packet Frame that are expected for Slot y. This bit field is looked up before potential modification of FID according to FIDS."]
    #[inline(always)]
    pub fn ufc1(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, RcrAx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, RcrAx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "UART Frame Count per Packet Frame in Slot 5   UFC5. This bit field defines the number of UART Frames per Packet Frame that are expected for Slot y. This bit field is looked up before potential modification of FID according to FIDS."]
    #[inline(always)]
    pub fn ufc2(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, RcrAx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, RcrAx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "UART Frame Count per Packet Frame in Slot 5   UFC5. This bit field defines the number of UART Frames per Packet Frame that are expected for Slot y. This bit field is looked up before potential modification of FID according to FIDS."]
    #[inline(always)]
    pub fn ufc3(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, RcrAx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8, RcrAx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "UART Frame Count per Packet Frame in Slot 5   UFC5. This bit field defines the number of UART Frames per Packet Frame that are expected for Slot y. This bit field is looked up before potential modification of FID according to FIDS."]
    #[inline(always)]
    pub fn ufc4(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, RcrAx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, RcrAx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "UART Frame Count per Packet Frame in Slot 5   UFC5. This bit field defines the number of UART Frames per Packet Frame that are expected for Slot y. This bit field is looked up before potential modification of FID according to FIDS."]
    #[inline(always)]
    pub fn ufc5(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, RcrAx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3,1,0,u8, RcrAx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for RcrAx {
    #[inline(always)]
    fn default() -> RcrAx {
        <crate::RegValueT<RcrAx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RcrBx_SPEC;
impl crate::sealed::RegSpec for RcrBx_SPEC {
    type DataType = u32;
}
#[doc = "Receiver Control Register B0\n resetvalue={Application Reset:0x0}"]
pub type RcrBx = crate::RegValueT<RcrBx_SPEC>;

impl RcrBx {
    #[doc = "Payload Data Length   PDL5. PDL determines the number of bits in a Packet Frame frame for Slot y. It        also determines the position of the CRC Parity bit. E.g. 8 defines 8        data bits on position  7 0 . On bit position 8 the CRC Parity is        located. See CROSSREFERENCE .        If PDLy is cleared    8216 0  8217   no frame is expected for this slot. Packet        Frames received for a slot with PDL     8216 0  8217  are copied to ChID 0  FID 1        with original IDs without further processing. This bit field is looked        up before potential modification of FID according to RCRAx.FIDS"]
    #[inline(always)]
    pub fn pdl0(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, RcrBx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, RcrBx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Payload Data Length   PDL5. PDL determines the number of bits in a Packet Frame frame for Slot y. It        also determines the position of the CRC Parity bit. E.g. 8 defines 8        data bits on position  7 0 . On bit position 8 the CRC Parity is        located. See CROSSREFERENCE .        If PDLy is cleared    8216 0  8217   no frame is expected for this slot. Packet        Frames received for a slot with PDL     8216 0  8217  are copied to ChID 0  FID 1        with original IDs without further processing. This bit field is looked        up before potential modification of FID according to RCRAx.FIDS"]
    #[inline(always)]
    pub fn pdl1(
        self,
    ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, RcrBx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1f,1,0,u8, RcrBx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Payload Data Length   PDL5. PDL determines the number of bits in a Packet Frame frame for Slot y. It        also determines the position of the CRC Parity bit. E.g. 8 defines 8        data bits on position  7 0 . On bit position 8 the CRC Parity is        located. See CROSSREFERENCE .        If PDLy is cleared    8216 0  8217   no frame is expected for this slot. Packet        Frames received for a slot with PDL     8216 0  8217  are copied to ChID 0  FID 1        with original IDs without further processing. This bit field is looked        up before potential modification of FID according to RCRAx.FIDS"]
    #[inline(always)]
    pub fn pdl2(
        self,
    ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, RcrBx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x1f,1,0,u8, RcrBx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Payload Data Length   PDL5. PDL determines the number of bits in a Packet Frame frame for Slot y. It        also determines the position of the CRC Parity bit. E.g. 8 defines 8        data bits on position  7 0 . On bit position 8 the CRC Parity is        located. See CROSSREFERENCE .        If PDLy is cleared    8216 0  8217   no frame is expected for this slot. Packet        Frames received for a slot with PDL     8216 0  8217  are copied to ChID 0  FID 1        with original IDs without further processing. This bit field is looked        up before potential modification of FID according to RCRAx.FIDS"]
    #[inline(always)]
    pub fn pdl3(
        self,
    ) -> crate::common::RegisterField<15, 0x1f, 1, 0, u8, RcrBx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<15,0x1f,1,0,u8, RcrBx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Payload Data Length   PDL5. PDL determines the number of bits in a Packet Frame frame for Slot y. It        also determines the position of the CRC Parity bit. E.g. 8 defines 8        data bits on position  7 0 . On bit position 8 the CRC Parity is        located. See CROSSREFERENCE .        If PDLy is cleared    8216 0  8217   no frame is expected for this slot. Packet        Frames received for a slot with PDL     8216 0  8217  are copied to ChID 0  FID 1        with original IDs without further processing. This bit field is looked        up before potential modification of FID according to RCRAx.FIDS"]
    #[inline(always)]
    pub fn pdl4(
        self,
    ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, RcrBx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x1f,1,0,u8, RcrBx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Payload Data Length   PDL5. PDL determines the number of bits in a Packet Frame frame for Slot y. It        also determines the position of the CRC Parity bit. E.g. 8 defines 8        data bits on position  7 0 . On bit position 8 the CRC Parity is        located. See CROSSREFERENCE .        If PDLy is cleared    8216 0  8217   no frame is expected for this slot. Packet        Frames received for a slot with PDL     8216 0  8217  are copied to ChID 0  FID 1        with original IDs without further processing. This bit field is looked        up before potential modification of FID according to RCRAx.FIDS"]
    #[inline(always)]
    pub fn pdl5(
        self,
    ) -> crate::common::RegisterField<25, 0x1f, 1, 0, u8, RcrBx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<25,0x1f,1,0,u8, RcrBx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for RcrBx {
    #[inline(always)]
    fn default() -> RcrBx {
        <crate::RegValueT<RcrBx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdr_SPEC;
impl crate::sealed::RegSpec for Rdr_SPEC {
    type DataType = u32;
}
#[doc = "Receive Data Register\n resetvalue={Application Reset:0x0}"]
pub type Rdr = crate::RegValueT<Rdr_SPEC>;

impl Rdr {
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd25(self) -> crate::common::RegisterFieldBool<25, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd26(self) -> crate::common::RegisterFieldBool<26, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PSI5 Receive Data   RD27. of last frame. D0 is on bit position 0."]
    #[inline(always)]
    pub fn rd27(self) -> crate::common::RegisterFieldBool<27, 1, 0, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Packet Frame Count   PFC. For data consistency  RDR  RDS and TSM are tagged with a Packet Frame count value. For each channel there is a separate internal packet counter. This allows to read a triplet of RDR  RDS and TSM from the system memory without looking at interrupts. If the PFC is identical all three values belong together. Otherwise they need to be read again until PFC is identical."]
    #[inline(always)]
    pub fn pfc(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Rdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<28, 0xf, 1, 0, u8, Rdr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Rdr {
    #[inline(always)]
    fn default() -> Rdr {
        <crate::RegValueT<Rdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rds_SPEC;
impl crate::sealed::RegSpec for Rds_SPEC {
    type DataType = u32;
}
#[doc = "Receive Status Register\n resetvalue={Application Reset:0x0}"]
pub type Rds = crate::RegValueT<Rds_SPEC>;

impl Rds {
    #[doc = "XCRC   XCRC5. CRC of last Packet Frame.  XCRC0 is on bit position 0."]
    #[inline(always)]
    pub fn xcrc0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Rds_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "XCRC   XCRC5. CRC of last Packet Frame.  XCRC0 is on bit position 0."]
    #[inline(always)]
    pub fn xcrc1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Rds_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "XCRC   XCRC5. CRC of last Packet Frame.  XCRC0 is on bit position 0."]
    #[inline(always)]
    pub fn xcrc2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Rds_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "XCRC   XCRC5. CRC of last Packet Frame.  XCRC0 is on bit position 0."]
    #[inline(always)]
    pub fn xcrc3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Rds_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "XCRC   XCRC5. CRC of last Packet Frame.  XCRC0 is on bit position 0."]
    #[inline(always)]
    pub fn xcrc4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Rds_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "XCRC   XCRC5. CRC of last Packet Frame.  XCRC0 is on bit position 0."]
    #[inline(always)]
    pub fn xcrc5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Rds_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "XCRC Error Flag   XCRCI. This bit is set if the CRC check on the enveloping Packet Frame fails including the case where XCRC check can not be performed  non recoverable frames  see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn xcrci(self) -> crate::common::RegisterFieldBool<6, 1, 0, Rds_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC   CRC2. of last frame. CRC0   Parity is on bit position 7. If Parity is used  CRC1 2 are always 0."]
    #[inline(always)]
    pub fn crc0(self) -> crate::common::RegisterFieldBool<7, 1, 0, Rds_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC   CRC2. of last frame. CRC0   Parity is on bit position 7. If Parity is used  CRC1 2 are always 0."]
    #[inline(always)]
    pub fn crc1(self) -> crate::common::RegisterFieldBool<8, 1, 0, Rds_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC   CRC2. of last frame. CRC0   Parity is on bit position 7. If Parity is used  CRC1 2 are always 0."]
    #[inline(always)]
    pub fn crc2(self) -> crate::common::RegisterFieldBool<9, 1, 0, Rds_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Error Flag   CRCI. This bit is set if the CRC or Parity check on the transported PSI5 frame fails."]
    #[inline(always)]
    pub fn crci(self) -> crate::common::RegisterFieldBool<10, 1, 0, Rds_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Error signalling Flag 0   ERR0. This bit represents the status of the error signalling flag Err0 in the enveloping Packet Frame."]
    #[inline(always)]
    pub fn err0(self) -> crate::common::RegisterFieldBool<11, 1, 0, Rds_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Error signalling Flag 1   ERR1. This bit represents the status of the error signalling flag Err1 in the enveloping Packet Frame."]
    #[inline(always)]
    pub fn err1(self) -> crate::common::RegisterFieldBool<12, 1, 0, Rds_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Header Error Signalled Flag   HDI. This bit is set if at least one of the error signalling flags in the enveloping Packet Frame Err0 and Err1 is set."]
    #[inline(always)]
    pub fn hdi(self) -> crate::common::RegisterFieldBool<13, 1, 0, Rds_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ASC Parity Error Flag   PE. This bit is set if the error flag signalling a parity error was set during reception of one of the ASC Bytes transporting this PSI5 frame."]
    #[inline(always)]
    pub fn pe(self) -> crate::common::RegisterFieldBool<14, 1, 0, Rds_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ASC Framing Error Flag   FE. This bit is set if the error flag signaling a framing error was set during reception of one of the ASC Bytes transporting this PSI5 frame."]
    #[inline(always)]
    pub fn fe(self) -> crate::common::RegisterFieldBool<15, 1, 0, Rds_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ASC Overrun Error Flag   OE. This bit is set if the error flag signaling an overrun error was set during reception of one of the ASC Bytes transporting this PSI5 frame."]
    #[inline(always)]
    pub fn oe(self) -> crate::common::RegisterFieldBool<16, 1, 0, Rds_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Time Error Flag   TEI. This bit is set if the watch dog timer expired. Depending from RCRAx.WDMS either the distance between two RDIs is longer than specified in WDL or it expired without reception of CHCI in time. I.e. the time from issuing the sync pulse to reception of the last expected frame configured in NFC.NFx was too long. If WDMS is in synchronous mode  all frames after TEI have TEI set in RDS until either CHCI is issued or INSTATx.TEI is cleared by SW. RDS.TEI flag is independently from INTENx.TEI."]
    #[inline(always)]
    pub fn tei(self) -> crate::common::RegisterFieldBool<17, 1, 0, Rds_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive Buffer Overflow Flag   RBI. This bit is set after a frame has been received while the old one was not read from RDR. I.e. the kernel wants to set interrupt RDI and finds RDI already set. The old data is overwritten by the new data."]
    #[inline(always)]
    pub fn rbi(self) -> crate::common::RegisterFieldBool<18, 1, 0, Rds_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Frame ID  Frame Number    FID. See bit RCRAx.FIDS for actual content."]
    #[inline(always)]
    pub fn fid(
        self,
    ) -> crate::common::RegisterField<19, 0x7, 1, 0, u8, Rds_SPEC, crate::common::R> {
        crate::common::RegisterField::<19, 0x7, 1, 0, u8, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel ID  Channel Number    CID"]
    #[inline(always)]
    pub fn cid(
        self,
    ) -> crate::common::RegisterField<22, 0x7, 1, 0, u8, Rds_SPEC, crate::common::R> {
        crate::common::RegisterField::<22, 0x7, 1, 0, u8, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Actual UART Frame Count   AFC. This bit field shows the number of UART frames actually received. This is used to further analyze non recoverable frames by SW or during debugging."]
    #[inline(always)]
    pub fn afc(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, Rds_SPEC, crate::common::R> {
        crate::common::RegisterField::<25, 0x7, 1, 0, u8, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Packet Frame Count   PFC. For data consistency  RDR  RDS and TSM are tagged with a Packet Frame count value. For each channel there is a separate internal packet counter. This allows to read a triplet of RDR  RDS and TSM from the system memory without looking at interrupts. If the PFC is identical all three values belong together. Otherwise they need to be read again until PFC is identical."]
    #[inline(always)]
    pub fn pfc(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Rds_SPEC, crate::common::R> {
        crate::common::RegisterField::<28, 0xf, 1, 0, u8, Rds_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Rds {
    #[inline(always)]
    fn default() -> Rds {
        <crate::RegValueT<Rds_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ScRx_SPEC;
impl crate::sealed::RegSpec for ScRx_SPEC {
    type DataType = u32;
}
#[doc = "Send Control Register 0\n resetvalue={Application Reset:0x0}"]
pub type ScRx = crate::RegValueT<ScRx_SPEC>;

impl ScRx {
    #[doc = "Pay Load Length of Registers SDRx   PLL. Defines the length that is taken into account. PLL needs to written before SDRx is used for proper operation."]
    #[inline(always)]
    pub fn pll(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, ScRx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, ScRx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enhanced Protocol Selection   EPS. EPS 0  controls the Bit Format and MSB Fill bit.  0  Tooth Gap  MSB Fill 1   1  PWM  MSB Fill 0  EPS 1  controls the Frame Format   0  Frame Format 1   3   1  Frame Format 4"]
    #[inline(always)]
    pub fn eps(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, ScRx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, ScRx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bit Stuffing Control   BSC. Depending from bit EPS 1   after 3 bits a  1  is inserted  Frame Format 1  3  or  after 6 bits a  0  is inserted  Frame Format 4"]
    #[inline(always)]
    pub fn bsc(self) -> crate::common::RegisterFieldBool<8, 1, 0, ScRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, ScRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Flush SDRx   FLUS. Setting this bit  stops shifting out the data in SDRx  the start sequence or CRC if any and clears the referring FSMs and counters  if EPS 0  0 SDRx is flushed by setting all bits if EPS 0  1 SDRx is flushed by clearing all bits clears TPF TPIx is issued at the end of successful flushing. Reads always as zero."]
    #[inline(always)]
    pub fn flus(self) -> crate::common::RegisterFieldBool<14, 1, 0, ScRx_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, ScRx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Generation Control   CRC"]
    #[inline(always)]
    pub fn crc(self) -> crate::common::RegisterFieldBool<22, 1, 0, ScRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, ScRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Start Sequence Generation Control   STA"]
    #[inline(always)]
    pub fn sta(self) -> crate::common::RegisterFieldBool<23, 1, 0, ScRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, ScRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmit in Progress Flag   TPF. If set  data preparation and transmission is in progress   start sequence or CRC or stuffing bits or data from SDRx are being transferred. It is cleared automatically after preparation and transmitting is finished. It gets set by HW immediately with the write access to SDRx and cleared by HW when the last pulse request is issued to the message generator  usually a CRC bit from the CRC Generator  If set  write access to SDRx will not change any data and issue TPOI."]
    #[inline(always)]
    pub fn tpf(self) -> crate::common::RegisterFieldBool<26, 1, 0, ScRx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, ScRx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for ScRx {
    #[inline(always)]
    fn default() -> ScRx {
        <crate::RegValueT<ScRx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdRx_SPEC;
impl crate::sealed::RegSpec for SdRx_SPEC {
    type DataType = u32;
}
#[doc = "Send Data Register 0\n resetvalue={Application Reset:0x0}"]
pub type SdRx = crate::RegValueT<SdRx_SPEC>;

impl SdRx {
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd0(self) -> crate::common::RegisterFieldBool<0, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd1(self) -> crate::common::RegisterFieldBool<1, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd2(self) -> crate::common::RegisterFieldBool<2, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd3(self) -> crate::common::RegisterFieldBool<3, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd4(self) -> crate::common::RegisterFieldBool<4, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd5(self) -> crate::common::RegisterFieldBool<5, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd6(self) -> crate::common::RegisterFieldBool<6, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd7(self) -> crate::common::RegisterFieldBool<7, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd8(self) -> crate::common::RegisterFieldBool<8, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd9(self) -> crate::common::RegisterFieldBool<9, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd10(self) -> crate::common::RegisterFieldBool<10, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd11(self) -> crate::common::RegisterFieldBool<11, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd12(self) -> crate::common::RegisterFieldBool<12, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd13(self) -> crate::common::RegisterFieldBool<13, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd14(self) -> crate::common::RegisterFieldBool<14, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd15(self) -> crate::common::RegisterFieldBool<15, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd16(self) -> crate::common::RegisterFieldBool<16, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd17(self) -> crate::common::RegisterFieldBool<17, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd18(self) -> crate::common::RegisterFieldBool<18, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd19(self) -> crate::common::RegisterFieldBool<19, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd20(self) -> crate::common::RegisterFieldBool<20, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd21(self) -> crate::common::RegisterFieldBool<21, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd22(self) -> crate::common::RegisterFieldBool<22, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD23   SD23. Send data of next ECU to Sensor frame. Each time a bit is shifted out  the whole content is shifted right by 1 position and the MSB is filled with  1  or  0  depending from EPS 0 . This allows to read back SDR and determine the status of the shift process by SW. The unused MSBs  bit position SCRx.PLL and higher  must be written with  0  if EPS 0  is set  PWM Method  and with  1  if EPS 0  is cleared  Tooth Gap Method . This is required  as the respective standard value is shifted into the register from MSB during shift out operation. SDRx will be filled with this value after shift out process."]
    #[inline(always)]
    pub fn sd23(self) -> crate::common::RegisterFieldBool<23, 1, 0, SdRx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, SdRx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for SdRx {
    #[inline(always)]
    fn default() -> SdRx {
        <crate::RegValueT<SdRx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tar_SPEC;
impl crate::sealed::RegSpec for Tar_SPEC {
    type DataType = u32;
}
#[doc = "Target Address Register\n resetvalue={Application Reset:0x0}"]
pub type Tar = crate::RegValueT<Tar_SPEC>;

impl Tar {
    #[doc = "Target Address   TA. Contains the upper 30 bit of the target address for the next DMA transfer. The 32 bit target address must be word aligned. Thus the 2 LSBs are fixed to 0. It is updated each time a new Packet Frame is received completely."]
    #[inline(always)]
    pub fn ta(
        self,
    ) -> crate::common::RegisterField<2, 0x3fffffff, 1, 0, u32, Tar_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x3fffffff,1,0,u32, Tar_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tar {
    #[inline(always)]
    fn default() -> Tar {
        <crate::RegValueT<Tar_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tbuf_SPEC;
impl crate::sealed::RegSpec for Tbuf_SPEC {
    type DataType = u32;
}
#[doc = "Transmit Buffer Register\n resetvalue={Application Reset:0x0}"]
pub type Tbuf = crate::RegValueT<Tbuf_SPEC>;

impl Tbuf {
    #[doc = "Transmit Data Register Value   TD VALUE. TBUF contains the data to be transmitted in the asynchronous and synchronous operating modes of the ASC. Data transmission is double buffered  therefore  a new value can be written to TBUF before the transmission of the previous value is complete."]
    #[inline(always)]
    pub fn td_value(
        self,
    ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, Tbuf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16, Tbuf_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Tbuf {
    #[inline(always)]
    fn default() -> Tbuf {
        <crate::RegValueT<Tbuf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tscnta_SPEC;
impl crate::sealed::RegSpec for Tscnta_SPEC {
    type DataType = u32;
}
#[doc = "Time Stamp Count Register A\n resetvalue={Application Reset:0x0}"]
pub type Tscnta = crate::RegValueT<Tscnta_SPEC>;

impl Tscnta {
    #[doc = "Current Time Stamp for the Module   CTS. This bit field shows the current time stamp."]
    #[inline(always)]
    pub fn cts(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, Tscnta_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffff,1,0,u32, Tscnta_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "External Time Base Select   ETB. Selects the external clock line for counter CTS if TBS is set."]
    #[inline(always)]
    pub fn etb(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Tscnta_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Tscnta_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Time Base Select   TBS. This bit selects the clock source for CTS"]
    #[inline(always)]
    pub fn tbs(self) -> crate::common::RegisterFieldBool<27, 1, 0, Tscnta_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Tscnta_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Time Base Enable TSCNTA   TBEA. This bit starts stops TSCNTA.CTS"]
    #[inline(always)]
    pub fn tbea(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Tscnta_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Tscnta_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Time Base Enable TSCNTB   TBEB. This bit starts stops TSCNTB.CTS"]
    #[inline(always)]
    pub fn tbeb(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Tscnta_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Tscnta_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Time Stamp Counter A   CLRA. This bit clears TSCNTA.CTS.  TSCNTA.CTS counts on  starting from 0."]
    #[inline(always)]
    pub fn clra(self) -> crate::common::RegisterFieldBool<30, 1, 0, Tscnta_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Tscnta_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Time Stamp Counter B   CLRB. This bit clears TSCNTB.CTS.  TSCNTB.CTS counts on  starting from 0."]
    #[inline(always)]
    pub fn clrb(self) -> crate::common::RegisterFieldBool<31, 1, 0, Tscnta_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Tscnta_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Tscnta {
    #[inline(always)]
    fn default() -> Tscnta {
        <crate::RegValueT<Tscnta_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tscntb_SPEC;
impl crate::sealed::RegSpec for Tscntb_SPEC {
    type DataType = u32;
}
#[doc = "Time Stamp Count Register B\n resetvalue={Application Reset:0x0}"]
pub type Tscntb = crate::RegValueT<Tscntb_SPEC>;

impl Tscntb {
    #[doc = "Current Time Stamp for the Module   CTS. This bit field shows the current time stamp."]
    #[inline(always)]
    pub fn cts(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, Tscntb_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffff,1,0,u32, Tscntb_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "External Time Base Select   ETB. Selects the external clock line for counter CTS."]
    #[inline(always)]
    pub fn etb(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Tscntb_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Tscntb_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Time Base Select   TBS. This bit selects the clock source for CTS"]
    #[inline(always)]
    pub fn tbs(self) -> crate::common::RegisterFieldBool<27, 1, 0, Tscntb_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Tscntb_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Tscntb {
    #[inline(always)]
    fn default() -> Tscntb {
        <crate::RegValueT<Tscntb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TscRx_SPEC;
impl crate::sealed::RegSpec for TscRx_SPEC {
    type DataType = u32;
}
#[doc = "Capture Register TSCR0\n resetvalue={Application Reset:0x0}"]
pub type TscRx = crate::RegValueT<TscRx_SPEC>;

impl TscRx {
    #[doc = "Time Stamp   TS. of the last sync pulse sent for channel x."]
    #[inline(always)]
    pub fn ts(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, TscRx_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffff,1,0,u32, TscRx_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for TscRx {
    #[inline(always)]
    fn default() -> TscRx {
        <crate::RegValueT<TscRx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsm_SPEC;
impl crate::sealed::RegSpec for Tsm_SPEC {
    type DataType = u32;
}
#[doc = "Time Stamp Mirror Register\n resetvalue={Application Reset:0x0}"]
pub type Tsm = crate::RegValueT<Tsm_SPEC>;

impl Tsm {
    #[doc = "Time Stamp   TS. of the last sync pulse sent on channel x."]
    #[inline(always)]
    pub fn ts(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, Tsm_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffff,1,0,u32, Tsm_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Packet Frame Count   PFC. For data consistency  RDR  RDS and TSM are tagged with a Packet Frame count value. For each channel there is a separate internal packet counter. This allows to read a triplet of RDR  RDS and TSM from the system memory without looking at interrupts. If the PFC is identical all three values belong together. Otherwise they need to be read again until PFC is identical."]
    #[inline(always)]
    pub fn pfc(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Tsm_SPEC, crate::common::R> {
        crate::common::RegisterField::<28, 0xf, 1, 0, u8, Tsm_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Tsm {
    #[inline(always)]
    fn default() -> Tsm {
        <crate::RegValueT<Tsm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdTx_SPEC;
impl crate::sealed::RegSpec for WdTx_SPEC {
    type DataType = u32;
}
#[doc = "Watch Dog Timer Register 0\n resetvalue={Application Reset:0x0}"]
pub type WdTx = crate::RegValueT<WdTx_SPEC>;

impl WdTx {
    #[doc = "Watch Dog Timer Limit   WDL. for channel x. If no watch dog is needed  WDL is cleared  the internal watch dog timer is stopped and no check is performed. CENx must be clear for write access."]
    #[inline(always)]
    pub fn wdl(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, WdTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffff,1,0,u32, WdTx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for WdTx {
    #[inline(always)]
    fn default() -> WdTx {
        <crate::RegValueT<WdTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Whbcon_SPEC;
impl crate::sealed::RegSpec for Whbcon_SPEC {
    type DataType = u32;
}
#[doc = "Write Hardware Bits Control Register\n resetvalue={Application Reset:0x0}"]
pub type Whbcon = crate::RegValueT<Whbcon_SPEC>;

impl Whbcon {
    #[doc = "Clear Receiver Enable Bit   CLRREN. Bit is always read as 0."]
    #[inline(always)]
    pub fn clrren(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Whbcon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Whbcon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Receiver Enable Bit   SETREN. Bit is always read as 0."]
    #[inline(always)]
    pub fn setren(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Whbcon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Whbcon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Parity Error Flag   CLRPE. Bit is always read as 0."]
    #[inline(always)]
    pub fn clrpe(self) -> crate::common::RegisterFieldBool<8, 1, 0, Whbcon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Whbcon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Framing Error Flag   CLRFE. Bit is always read as 0."]
    #[inline(always)]
    pub fn clrfe(self) -> crate::common::RegisterFieldBool<9, 1, 0, Whbcon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Whbcon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Overrun Error Flag   CLROE. Bit is always read as 0."]
    #[inline(always)]
    pub fn clroe(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Whbcon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Whbcon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Parity Error Flag   SETPE. Bit is always read as 0."]
    #[inline(always)]
    pub fn setpe(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Whbcon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, Whbcon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Framing Error Flag   SETFE. Bit is always read as 0."]
    #[inline(always)]
    pub fn setfe(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Whbcon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, Whbcon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Overrun Error Flag   SETOE. Bit is always read as 0."]
    #[inline(always)]
    pub fn setoe(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Whbcon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, Whbcon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Whbcon {
    #[inline(always)]
    fn default() -> Whbcon {
        <crate::RegValueT<Whbcon_SPEC> as RegisterValue<_>>::new(0)
    }
}
