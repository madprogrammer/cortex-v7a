// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (c) 2018-2020 by the author(s)
//
// Author(s):
//   - Jorge Aparicio
//   - Andre Richter <andre.o.richter@gmail.com>

//! Miscellaneous assembly instructions

/// The classic no-op
#[inline(always)]
pub fn nop() {
    #[cfg(target_arch = "arm")]
    unsafe {
        llvm_asm!("nop" :::: "volatile")
    }

    #[cfg(not(target_arch = "arm"))]
    unimplemented!()
}

