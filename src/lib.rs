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
#![cfg_attr(not(feature = "tracing"), no_std)]
#![cfg_attr(target_arch = "tricore", feature(stdsimd))]
#![allow(non_camel_case_types)]
#![doc = "Default"]
pub mod common;
pub use common::*;

#[cfg(feature = "tracing")]
pub mod reg_name;
#[cfg(feature = "tracing")]
pub mod tracing;

#[cfg(feature = "adma0")]
pub mod adma0;
#[cfg(feature = "amu00")]
pub mod amu00;
#[cfg(feature = "amu01")]
pub mod amu01;
#[cfg(feature = "asclin0")]
pub mod asclin0;
#[cfg(feature = "asclin1")]
pub mod asclin1;
#[cfg(feature = "asclin10")]
pub mod asclin10;
#[cfg(feature = "asclin11")]
pub mod asclin11;
#[cfg(feature = "asclin2")]
pub mod asclin2;
#[cfg(feature = "asclin3")]
pub mod asclin3;
#[cfg(feature = "asclin4")]
pub mod asclin4;
#[cfg(feature = "asclin5")]
pub mod asclin5;
#[cfg(feature = "asclin6")]
pub mod asclin6;
#[cfg(feature = "asclin7")]
pub mod asclin7;
#[cfg(feature = "asclin8")]
pub mod asclin8;
#[cfg(feature = "asclin9")]
pub mod asclin9;
#[cfg(feature = "can0")]
pub mod can0;
#[cfg(feature = "can1")]
pub mod can1;
#[cfg(feature = "cbs")]
pub mod cbs;
#[cfg(feature = "ccu60")]
pub mod ccu60;
#[cfg(feature = "ccu61")]
pub mod ccu61;
#[cfg(feature = "convctrl")]
pub mod convctrl;
#[cfg(feature = "cpu0")]
pub mod cpu0;
#[cfg(feature = "cpu1")]
pub mod cpu1;
#[cfg(feature = "cpu2")]
pub mod cpu2;
#[cfg(feature = "dam0")]
pub mod dam0;
#[cfg(feature = "dma")]
pub mod dma;
#[cfg(feature = "dmu")]
pub mod dmu;
#[cfg(feature = "dom0")]
pub mod dom0;
#[cfg(feature = "edsadc")]
pub mod edsadc;
#[cfg(feature = "eray0")]
pub mod eray0;
#[cfg(feature = "evadc")]
pub mod evadc;
#[cfg(feature = "fce")]
pub mod fce;
#[cfg(feature = "fsi")]
pub mod fsi;
#[cfg(feature = "geth")]
pub mod geth;
#[cfg(feature = "gpt120")]
pub mod gpt120;
#[cfg(feature = "gtm")]
pub mod gtm;
#[cfg(feature = "hsct0")]
pub mod hsct0;
#[cfg(feature = "hsm")]
pub mod hsm;
#[cfg(feature = "hssl0")]
pub mod hssl0;
#[cfg(feature = "i2c0")]
pub mod i2c0;
#[cfg(feature = "int")]
pub mod int;
#[cfg(feature = "iom")]
pub mod iom;
#[cfg(feature = "minimcds")]
pub mod minimcds;
#[cfg(feature = "msc0")]
pub mod msc0;
#[cfg(feature = "msc1")]
pub mod msc1;
#[cfg(feature = "mtu")]
pub mod mtu;
#[cfg(feature = "p00")]
pub mod p00;
#[cfg(feature = "p01")]
pub mod p01;
#[cfg(feature = "p02")]
pub mod p02;
#[cfg(feature = "p10")]
pub mod p10;
#[cfg(feature = "p11")]
pub mod p11;
#[cfg(feature = "p12")]
pub mod p12;
#[cfg(feature = "p13")]
pub mod p13;
#[cfg(feature = "p14")]
pub mod p14;
#[cfg(feature = "p15")]
pub mod p15;
#[cfg(feature = "p20")]
pub mod p20;
#[cfg(feature = "p21")]
pub mod p21;
#[cfg(feature = "p22")]
pub mod p22;
#[cfg(feature = "p23")]
pub mod p23;
#[cfg(feature = "p32")]
pub mod p32;
#[cfg(feature = "p33")]
pub mod p33;
#[cfg(feature = "p34")]
pub mod p34;
#[cfg(feature = "p40")]
pub mod p40;
#[cfg(feature = "pfi0")]
pub mod pfi0;
#[cfg(feature = "pfi1")]
pub mod pfi1;
#[cfg(feature = "pms")]
pub mod pms;
#[cfg(feature = "pmu")]
pub mod pmu;
#[cfg(feature = "psi5")]
pub mod psi5;
#[cfg(feature = "psi5s")]
pub mod psi5s;
#[cfg(feature = "qspi0")]
pub mod qspi0;
#[cfg(feature = "qspi1")]
pub mod qspi1;
#[cfg(feature = "qspi2")]
pub mod qspi2;
#[cfg(feature = "qspi3")]
pub mod qspi3;
#[cfg(feature = "qspi4")]
pub mod qspi4;
#[cfg(feature = "sbcu")]
pub mod sbcu;
#[cfg(feature = "scu")]
pub mod scu;
#[cfg(feature = "sent")]
pub mod sent;
#[cfg(feature = "smu")]
pub mod smu;
#[cfg(feature = "src")]
pub mod src;
#[cfg(feature = "stm0")]
pub mod stm0;
#[cfg(feature = "stm1")]
pub mod stm1;
#[cfg(feature = "stm2")]
pub mod stm2;

#[cfg(feature = "adma0")]
pub const ADMA0: adma0::Adma0 = adma0::Adma0(0xf8508400u32 as _);
#[cfg(feature = "amu00")]
pub const AMU00: amu00::Amu00 = amu00::Amu00(0xf8508000u32 as _);
#[cfg(feature = "amu01")]
pub const AMU01: amu01::Amu01 = amu01::Amu01(0xf8508100u32 as _);
#[cfg(feature = "asclin0")]
pub const ASCLIN0: asclin0::Asclin0 = asclin0::Asclin0(0xf0000600u32 as _);
#[cfg(feature = "asclin1")]
pub const ASCLIN1: asclin1::Asclin1 = asclin1::Asclin1(0xf0000700u32 as _);
#[cfg(feature = "asclin10")]
pub const ASCLIN10: asclin10::Asclin10 = asclin10::Asclin10(0xf02c0a00u32 as _);
#[cfg(feature = "asclin11")]
pub const ASCLIN11: asclin11::Asclin11 = asclin11::Asclin11(0xf02c0b00u32 as _);
#[cfg(feature = "asclin2")]
pub const ASCLIN2: asclin2::Asclin2 = asclin2::Asclin2(0xf0000800u32 as _);
#[cfg(feature = "asclin3")]
pub const ASCLIN3: asclin3::Asclin3 = asclin3::Asclin3(0xf0000900u32 as _);
#[cfg(feature = "asclin4")]
pub const ASCLIN4: asclin4::Asclin4 = asclin4::Asclin4(0xf0000a00u32 as _);
#[cfg(feature = "asclin5")]
pub const ASCLIN5: asclin5::Asclin5 = asclin5::Asclin5(0xf0000b00u32 as _);
#[cfg(feature = "asclin6")]
pub const ASCLIN6: asclin6::Asclin6 = asclin6::Asclin6(0xf0000c00u32 as _);
#[cfg(feature = "asclin7")]
pub const ASCLIN7: asclin7::Asclin7 = asclin7::Asclin7(0xf0000d00u32 as _);
#[cfg(feature = "asclin8")]
pub const ASCLIN8: asclin8::Asclin8 = asclin8::Asclin8(0xf0000e00u32 as _);
#[cfg(feature = "asclin9")]
pub const ASCLIN9: asclin9::Asclin9 = asclin9::Asclin9(0xf0000f00u32 as _);
#[cfg(feature = "can0")]
pub const CAN0: can0::Can0 = can0::Can0(0xf0200000u32 as _);
#[cfg(feature = "can1")]
pub const CAN1: can1::Can1 = can1::Can1(0xf0210000u32 as _);
#[cfg(feature = "cbs")]
pub const CBS: cbs::Cbs = cbs::Cbs(0xf0000400u32 as _);
#[cfg(feature = "ccu60")]
pub const CCU60: ccu60::Ccu60 = ccu60::Ccu60(0xf0002a00u32 as _);
#[cfg(feature = "ccu61")]
pub const CCU61: ccu61::Ccu61 = ccu61::Ccu61(0xf0002b00u32 as _);
#[cfg(feature = "convctrl")]
pub const CONVCTRL: convctrl::Convctrl = convctrl::Convctrl(0xf0025000u32 as _);
#[cfg(feature = "cpu0")]
pub const CPU0: cpu0::Cpu0 = cpu0::Cpu0(0xf8800000u32 as _);
#[cfg(feature = "cpu1")]
pub const CPU1: cpu1::Cpu1 = cpu1::Cpu1(0xf8820000u32 as _);
#[cfg(feature = "cpu2")]
pub const CPU2: cpu2::Cpu2 = cpu2::Cpu2(0xf8840000u32 as _);
#[cfg(feature = "dam0")]
pub const DAM0: dam0::Dam0 = dam0::Dam0(0xf8500000u32 as _);
#[cfg(feature = "dma")]
pub const DMA: dma::Dma = dma::Dma(0xf0010000u32 as _);
#[cfg(feature = "dmu")]
pub const DMU: dmu::Dmu = dmu::Dmu(0xf8040000u32 as _);
#[cfg(feature = "dom0")]
pub const DOM0: dom0::Dom0 = dom0::Dom0(0xf8700000u32 as _);
#[cfg(feature = "edsadc")]
pub const EDSADC: edsadc::Edsadc = edsadc::Edsadc(0xf0024000u32 as _);
#[cfg(feature = "eray0")]
pub const ERAY0: eray0::Eray0 = eray0::Eray0(0xf001c000u32 as _);
#[cfg(feature = "evadc")]
pub const EVADC: evadc::Evadc = evadc::Evadc(0xf0020000u32 as _);
#[cfg(feature = "fce")]
pub const FCE: fce::Fce = fce::Fce(0xf0000000u32 as _);
#[cfg(feature = "fsi")]
pub const FSI: fsi::Fsi = fsi::Fsi(0xf8030000u32 as _);
#[cfg(feature = "geth")]
pub const GETH: geth::Geth = geth::Geth(0xf001d000u32 as _);
#[cfg(feature = "gpt120")]
pub const GPT120: gpt120::Gpt120 = gpt120::Gpt120(0xf0001800u32 as _);
#[cfg(feature = "gtm")]
pub const GTM: gtm::Gtm = gtm::Gtm(0xf0100000u32 as _);
#[cfg(feature = "hsct0")]
pub const HSCT0: hsct0::Hsct0 = hsct0::Hsct0(0xf0090000u32 as _);
#[cfg(feature = "hsm")]
pub const HSM: hsm::Hsm = hsm::Hsm(0xf0040000u32 as _);
#[cfg(feature = "hssl0")]
pub const HSSL0: hssl0::Hssl0 = hssl0::Hssl0(0xf0080000u32 as _);
#[cfg(feature = "i2c0")]
pub const I2C0: i2c0::I2C0 = i2c0::I2C0(0xf00c0000u32 as _);
#[cfg(feature = "int")]
pub const INT: int::Int = int::Int(0xf0037000u32 as _);
#[cfg(feature = "iom")]
pub const IOM: iom::Iom = iom::Iom(0xf0035000u32 as _);
#[cfg(feature = "minimcds")]
pub const MINIMCDS: minimcds::Minimcds = minimcds::Minimcds(0xfb718000u32 as _);
#[cfg(feature = "msc0")]
pub const MSC0: msc0::Msc0 = msc0::Msc0(0xf0002600u32 as _);
#[cfg(feature = "msc1")]
pub const MSC1: msc1::Msc1 = msc1::Msc1(0xf0002700u32 as _);
#[cfg(feature = "mtu")]
pub const MTU: mtu::Mtu = mtu::Mtu(0xf0060000u32 as _);
#[cfg(feature = "p00")]
pub const P00: p00::P00 = p00::P00(0xf003a000u32 as _);
#[cfg(feature = "p01")]
pub const P01: p01::P01 = p01::P01(0xf003a100u32 as _);
#[cfg(feature = "p02")]
pub const P02: p02::P02 = p02::P02(0xf003a200u32 as _);
#[cfg(feature = "p10")]
pub const P10: p10::P10 = p10::P10(0xf003aa00u32 as _);
#[cfg(feature = "p11")]
pub const P11: p11::P11 = p11::P11(0xf003ab00u32 as _);
#[cfg(feature = "p12")]
pub const P12: p12::P12 = p12::P12(0xf003ac00u32 as _);
#[cfg(feature = "p13")]
pub const P13: p13::P13 = p13::P13(0xf003ad00u32 as _);
#[cfg(feature = "p14")]
pub const P14: p14::P14 = p14::P14(0xf003ae00u32 as _);
#[cfg(feature = "p15")]
pub const P15: p15::P15 = p15::P15(0xf003af00u32 as _);
#[cfg(feature = "p20")]
pub const P20: p20::P20 = p20::P20(0xf003b400u32 as _);
#[cfg(feature = "p21")]
pub const P21: p21::P21 = p21::P21(0xf003b500u32 as _);
#[cfg(feature = "p22")]
pub const P22: p22::P22 = p22::P22(0xf003b600u32 as _);
#[cfg(feature = "p23")]
pub const P23: p23::P23 = p23::P23(0xf003b700u32 as _);
#[cfg(feature = "p32")]
pub const P32: p32::P32 = p32::P32(0xf003c000u32 as _);
#[cfg(feature = "p33")]
pub const P33: p33::P33 = p33::P33(0xf003c100u32 as _);
#[cfg(feature = "p34")]
pub const P34: p34::P34 = p34::P34(0xf003c200u32 as _);
#[cfg(feature = "p40")]
pub const P40: p40::P40 = p40::P40(0xf003c800u32 as _);
#[cfg(feature = "pfi0")]
pub const PFI0: pfi0::Pfi0 = pfi0::Pfi0(0xa8080000u32 as _);
#[cfg(feature = "pfi1")]
pub const PFI1: pfi1::Pfi1 = pfi1::Pfi1(0xa8380000u32 as _);
#[cfg(feature = "pms")]
pub const PMS: pms::Pms = pms::Pms(0xf0248000u32 as _);
#[cfg(feature = "pmu")]
pub const PMU: pmu::Pmu = pmu::Pmu(0xf8038000u32 as _);
#[cfg(feature = "psi5")]
pub const PSI5: psi5::Psi5 = psi5::Psi5(0xf0005000u32 as _);
#[cfg(feature = "psi5s")]
pub const PSI5S: psi5s::Psi5S = psi5s::Psi5S(0xf0007000u32 as _);
#[cfg(feature = "qspi0")]
pub const QSPI0: qspi0::Qspi0 = qspi0::Qspi0(0xf0001c00u32 as _);
#[cfg(feature = "qspi1")]
pub const QSPI1: qspi1::Qspi1 = qspi1::Qspi1(0xf0001d00u32 as _);
#[cfg(feature = "qspi2")]
pub const QSPI2: qspi2::Qspi2 = qspi2::Qspi2(0xf0001e00u32 as _);
#[cfg(feature = "qspi3")]
pub const QSPI3: qspi3::Qspi3 = qspi3::Qspi3(0xf0001f00u32 as _);
#[cfg(feature = "qspi4")]
pub const QSPI4: qspi4::Qspi4 = qspi4::Qspi4(0xf0002000u32 as _);
#[cfg(feature = "sbcu")]
pub const SBCU: sbcu::Sbcu = sbcu::Sbcu(0xf0030000u32 as _);
#[cfg(feature = "scu")]
pub const SCU: scu::Scu = scu::Scu(0xf0036000u32 as _);
#[cfg(feature = "sent")]
pub const SENT: sent::Sent = sent::Sent(0xf0003000u32 as _);
#[cfg(feature = "smu")]
pub const SMU: smu::Smu = smu::Smu(0xf0036800u32 as _);
#[cfg(feature = "src")]
pub const SRC: src::Src = src::Src(0xf0038000u32 as _);
#[cfg(feature = "stm0")]
pub const STM0: stm0::Stm0 = stm0::Stm0(0xf0001000u32 as _);
#[cfg(feature = "stm1")]
pub const STM1: stm1::Stm1 = stm1::Stm1(0xf0001100u32 as _);
#[cfg(feature = "stm2")]
pub const STM2: stm2::Stm2 = stm2::Stm2(0xf0001200u32 as _);
