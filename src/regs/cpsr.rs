// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2020 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>
//   - Sergey Anufrienko <sergey.anoufrienko@gmail.com>

//! Current Program Status Register
//!
//! Holds the current processor status

use register::{cpu::RegisterReadWrite, register_bitfields};

register_bitfields! {u32,
    pub CPSR [
        /// Negative condition flag.
        ///
        /// Set to the value of the N condition flag on taking an exception to EL1, and copied to
        /// the N condition flag on executing an exception return operation in EL1.
        ///
        /// Set to 1 if the result of the last flag-setting instruction was negative.
        N OFFSET(31) NUMBITS(1) [],

        /// Zero condition flag.
        ///
        /// Set to the value of the Z condition flag on taking an exception to EL1, and copied to
        /// the Z condition flag on executing an exception return operation in EL1.
        ///
        /// Set to 1 if the result of the last flag-setting instruction was zero, and to 0
        /// otherwise. A result of zero often indicates an equal result from a comparison.
        Z OFFSET(30) NUMBITS(1) [],

        /// Carry condition flag.
        ///
        /// Set to the value of the C condition flag on taking an exception to EL1, and copied to
        /// the C condition flag on executing an exception return operation in EL1.
        ///
        /// Set to 1 if the last flag-setting instruction resulted in a carry condition, for example
        /// an unsigned overflow on an addition.
        C OFFSET(29) NUMBITS(1) [],

        /// Overflow condition flag.
        ///
        /// Set to the value of the V condition flag on taking an exception to EL1, and copied to
        /// the V condition flag on executing an exception return operation in EL1.
        ///
        /// Set to 1 if the last flag-setting instruction resulted in an overflow condition, for
        /// example a signed overflow on an addition.
        V OFFSET(28) NUMBITS(1) [],

        /// Endianness execution state
        E OFFSET(9) NUMBITS(1) [
            Little = 0,
            Big = 1
        ],

        /// Asynchronous abort mask bit. The possible values of this bit are:
        ///
        /// 0 Exception not masked.
        /// 1 Exception masked.
        A OFFSET(8) NUMBITS(1) [
            Unmasked = 0,
            Masked = 1
        ],

        /// IRQ mask bit. The possible values of this bit are:
        ///
        /// 0 Exception not masked.
        /// 1 Exception masked.
        I OFFSET(7) NUMBITS(1) [
            Unmasked = 0,
            Masked = 1
        ],

        /// FIQ mask bit. The possible values of this bit are:
        ///
        /// 0 Exception not masked.
        /// 1 Exception masked.
        F OFFSET(6) NUMBITS(1) [
            Unmasked = 0,
            Masked = 1
        ],

        T OFFSET(5) NUMBITS(1) [
            ARM = 0,
            Thumb = 1
        ],

        M OFFSET(0) NUMBITS(5) [
            USR = 0b10000,
            FIQ = 0b10001,
            IRQ = 0b10010,
            SVC = 0b10011,
            ABT = 0b10111,
            UND = 0b11011,
            SYS = 0b11111
        ]
    ]
}

pub struct Reg;

impl RegisterReadWrite<u32, CPSR::Register> for Reg {
    sys_coproc_read_raw!(u32, "CPSR");
    sys_coproc_write_raw!(u32, "CPSR");
}

pub static CPSR: Reg = Reg {};
