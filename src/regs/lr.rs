// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2020 by the author(s)
//
// Author(s):
//   - Andre Richter <andre.o.richter@gmail.com>
//   - Alban Seurat <alban.seurat@me.com>

//! The link register

use register::cpu::RegisterReadWrite;

pub struct Reg;

impl RegisterReadWrite<u32, ()> for Reg {
    read_raw!(u32, "lr");
    write_raw!(u32, "lr");
}

pub static LR: Reg = Reg {};
