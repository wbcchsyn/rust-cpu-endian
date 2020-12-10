// Copyright 2020 Shin Yoshida
//
// "LGPL-3.0-or-later OR Apache-2.0 OR BSD-2-Clause OR MIT"
//
// This is part of cpu-endian
//
//  cpu-endian is free software: you can redistribute it and/or modify
//  it under the terms of the GNU Lesser General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  any later version.
//
//  cpu-endian is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU Lesser General Public License for more details.
//
//  You should have received a copy of the GNU Lesser General Public License
//  along with cpu-endian.  If not, see <http://www.gnu.org/licenses/>.
//
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//
// Redistribution and use in source and binary forms, with or without modification, are permitted
// provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of
//    conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright notice, this
//    list of conditions and the following disclaimer in the documentation and/or other
//    materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
// IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
// INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
// NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
// WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.
//
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next paragraph) shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![deny(missing_docs)]

//! # cpu-endian
//!
//! `cpu-endian` is a portable crate to detect CPU byte order.
//!
//! It detects how CPU native scalar type is ordered; little-endian or big-endian, or something else (like PDP-endian, mixed-endian, middle-endian, and so on.)
//!
//! ## Examples
//!
//! ```
//! use cpu_endian::{Endian, working};
//!
//! // Takes first octet of 0x00ff: u16.
//! let v: u16 = 0x00ff;
//! let first_octet: u8 = unsafe {
//!     let ptr = &v as *const u16;
//!     let ptr = ptr as *const u8;
//!     *ptr
//! };
//!
//! // If the byte-order is little-endian, the first octet should be 0xff, or if big-endian,
//! // it should be 0x00.
//! match working() {
//!     Endian::Little => assert_eq!(0xff, first_octet),
//!     Endian::Big => assert_eq!(0x00, first_octet),
//!     _ => {},
//! }
//! ```

use core::sync::atomic::{AtomicU8, Ordering};
use std::os::raw::c_int;

/// Byte order of scalar types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    /// little-endian
    Little,
    /// big-endian
    Big,
    /// Neither little-endian nor big-endian. For example, PDP-endian, mixed-endian, middle-endian, and so on.
    /// (Such endian is very rare today.)
    Minor,
}

/// Cache of native_().
///
/// Some CPUs change the byte order, however I don't think none of them changes it after process started.
static NATIVE: AtomicU8 = AtomicU8::new(0);

/// Returns the CPU byte order.
///
/// ```
/// use cpu_endian::*;
///
/// // Takes first octet of 0x00ff: u16.
/// let v: u16 = 0x00ff;
/// let first_octet: u8 = unsafe {
///     let ptr = &v as *const u16;
///     let ptr = ptr as *const u8;
///     *ptr
/// };
///
/// // If the byte-order is little-endian, the first octet should be 0xff, or if big-endian,
/// // it should be 0x00.
/// match working() {
///     Endian::Little => assert_eq!(0xff, first_octet),
///     Endian::Big => assert_eq!(0x00, first_octet),
///     _ => {},
/// }
/// ```
#[inline]
pub fn working() -> Endian {
    let mut cache = NATIVE.load(Ordering::Relaxed);

    // No cache is hit.
    // Because native()_ always returns the same value, Ordering::Relaxed will do.
    if cache == 0 {
        let order = unsafe { native_() };
        debug_assert_ne!(0, order);

        cache = order as u8;
        NATIVE.store(cache, Ordering::Relaxed);
    }

    match cache {
        1 => Endian::Little,
        2 => Endian::Big,
        _ => Endian::Minor,
    }
}

#[link(name = "native_endian_")]
extern "C" {
    /// Returns the cpu native endian.
    ///
    /// - Little endian: 1
    /// - Big endian: 2
    /// - Other: 3
    fn native_() -> c_int;
}
