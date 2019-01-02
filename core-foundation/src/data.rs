// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Core Foundation byte buffers.

pub use core_foundation_sys::data::*;
use core_foundation_sys::base::{CFIndex, CFRange};
use core_foundation_sys::base::{kCFAllocatorDefault};
use std::mem;
use std::ops::{Deref, DerefMut};
use std::slice;

use base::{CFIndexConvertible, TCFType};


declare_TCFType!{
    /// A byte buffer.
    CFData, CFDataRef
}
impl_TCFType!(CFData, CFDataRef, CFDataGetTypeID);
impl_CFTypeDescription!(CFData);

impl CFData {
    pub fn from_buffer(buffer: &[u8]) -> CFData {
        unsafe {
            let data_ref = CFDataCreate(kCFAllocatorDefault,
                                        buffer.as_ptr(),
                                        buffer.len().to_CFIndex());
            TCFType::wrap_under_create_rule(data_ref)
        }
    }

    /// Returns a pointer to the underlying bytes in this data. Note that this byte buffer is
    /// read-only.
    #[inline]
    pub fn bytes<'a>(&'a self) -> &'a [u8] {
        unsafe {
            slice::from_raw_parts(CFDataGetBytePtr(self.0), self.len() as usize)
        }
    }

    /// Returns the length of this byte buffer.
    #[inline]
    pub fn len(&self) -> CFIndex {
        unsafe {
            CFDataGetLength(self.0)
        }
    }
}

impl Deref for CFData {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.bytes()
    }
}

declare_TCFType!{
    /// A mutable byte buffer.
    ///
    /// *Warning:* Instances of this type must be effectively singly-owned, even though the
    /// underlying Core Foundation type is reference counted, to avoid introducing undefined
    /// behavior. Otherwise it would be possible to get, e.g. two `&mut [u8]`s to the same data.
    CFMutableData, CFMutableDataRef
}
// FIXME: THIS TYPE SHOULD NOT IMPLEMENT CLONE
impl_TCFType!(CFMutableData, CFMutableDataRef, CFDataGetTypeID);
impl_CFTypeDescription!(CFMutableData);

impl CFMutableData {
    /// Returns a new instance who's maximum capacity is not limited.
    pub fn new() -> CFMutableData {
        CFMutableData::with_maximum_capacity(0)
    }

    /// Returns a new instance with the given maximum capacity. A maximum capacity of `0` does not
    /// limit the capacity.
    pub fn with_maximum_capacity(maximum_capacity: usize) -> CFMutableData {
        unsafe {
            let data_ref = CFDataCreateMutable(kCFAllocatorDefault, maximum_capacity.to_CFIndex());
            TCFType::wrap_under_create_rule(data_ref)
        }
    }

    /// Returns a pointer to the underlying bytes in this data.
    #[inline]
    pub fn bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(CFDataGetBytePtr(self.0), self.len() as usize)
        }
    }

    /// Returns a mutable pointer to the underlying bytes in this data.
    #[inline]
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(CFDataGetMutableBytePtr(self.0), self.len() as usize)
        }
    }

    #[inline]
    pub fn extend_from_slice(&mut self, bytes: &[u8]) {
        unsafe {
            CFDataReplaceBytes(self.0,
                               CFRange { location: self.len(), length: 0 },
                               bytes.as_ptr(),
                               bytes.len().to_CFIndex());
        }
    }

    /// Returns the length of this byte buffer.
    #[inline]
    pub fn len(&self) -> CFIndex {
        unsafe {
            CFDataGetLength(self.0)
        }
    }

    /// Sets the length of this byte buffer to the given value. If that value is less than the
    /// current lenn, it truncates the excess bytes. If that value is greater than the current len,
    /// it zero-fills the extension to the byte buffer.
    #[inline]
    pub fn set_len(&mut self, len: usize) {
        unsafe {
            CFDataSetLength(self.0, len.to_CFIndex());
        }
    }

    /// Converts this `CFMutableData` into its immutable counterpart.
    ///
    /// *Note:* This method consumes self, because having a `CFData` and a `CFMutableData`
    /// referencing the same buffer could lead to undefined behavior.
    #[inline]
    pub fn into_immutable(self) -> CFData {
        let reference = self.0;
        mem::forget(self);
        CFData(reference)
    }
}

impl Deref for CFMutableData {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.bytes()
    }
}

impl DerefMut for CFMutableData {
    #[inline]
    fn deref_mut(&mut self) -> &mut [u8] {
        self.bytes_mut()
    }
}
