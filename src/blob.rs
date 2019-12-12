use cffi::Ptr;
use std::{
    mem,
    os::raw,
    ptr,
    slice,
};

use crate::ffi;

#[repr(transparent)]
pub struct Blob(Ptr<ffi::hb_blob_t>);
impl_ptr!(Blob, ffi::hb_blob_t);

impl cffi::Alloc for ffi::hb_blob_t {
    fn free(this: *mut Self) {
        unsafe { ffi::hb_blob_destroy(this) }
    }
}

// unimplemented:
// - hb_blob_create
impl Blob {
    /// Returns the singleton empty blob.
    pub fn empty() -> Blob {
        unsafe {
            let ptr = ffi::hb_blob_get_empty();
            Blob(Ptr::from_raw(ptr))
        }
    }

    pub fn into_raw(self) -> *mut ffi::hb_blob_t {
        Ptr::into_raw(self.0)
    }

    pub fn as_ptr(&self) -> *const ffi::hb_blob_t {
        Ptr::as_ptr(&self.0)
    }

    pub fn as_raw(&mut self) -> *mut ffi::hb_blob_t {
        Ptr::as_raw(&mut self.0)
    }
}

// unimplemented:
// - hb_blob_set_user_data
// - hb_blob_get_user_data
impl ffi::hb_blob_t {
    /// Return a new blob that represents a range of bytes in this blob. The new
    /// blob is always created with [`READONLY`], meaning that it will never
    /// modify data in the parent blob. This blob's data is not expected
    /// to be modified,  and will result in undefined behavior if it is.
    ///
    /// Makes this blob immutable.
    ///
    /// Return new blob, or the empty blob if something failed or if `length`
    /// is zero or `offset` is beyond the end of this blob's data.
    pub fn create_sub_blob(&mut self, offset: usize, length: usize) -> Blob {
        unsafe {
            let ptr = ffi::hb_blob_create_sub_blob(
                self,
                offset as raw::c_uint,
                length as raw::c_uint,
            );
            Blob(Ptr::from_raw(ptr))
        }
    }

    /// Makes a writable copy of @blob.
    pub fn copy_writable(&mut self) -> Blob {
        unsafe {
            let ptr = ffi::hb_blob_copy_writable_or_fail(self);
            if ptr == ::std::ptr::null_mut() {
                panic!("allocation failed");
            }
            Blob(Ptr::from_raw(ptr))
        }
    }

    pub fn is_immutable(&self) -> bool {
        // XXX: As of 1.8.8 hb_blob_is_immutable is constant, so this is safe.
        unsafe { ffi::hb_blob_is_immutable(mem::transmute(self)) != 0 }
    }

    pub fn make_immutable(&mut self) {
        unsafe { ffi::hb_blob_make_immutable(self) }
    }

    pub fn len(&self) -> usize {
        // XXX: As of 1.8.8 hb_blob_get_length is constant, so this is safe.
        unsafe { ffi::hb_blob_get_length(mem::transmute(self)) as usize }
    }

    pub fn data(&self) -> &[u8] {
        let mut len = 0;
        unsafe {
            // XXX: As of 1.8.8 hb_blob_get_data is constant, so this is safe.
            let ptr = ffi::hb_blob_get_data(mem::transmute(self), &mut len);
            slice::from_raw_parts(ptr as *const u8, len as usize)
        }
    }

    /// Try to make blob data writeable (possibly copying it).
    pub fn data_mut(&mut self) -> &mut [u8] {
        let mut len = 0;
        unsafe {
            let ptr = ffi::hb_blob_get_data_writable(self, &mut len);
            if ptr != ptr::null_mut() {
                panic!("allocation failed");
            }
            slice::from_raw_parts_mut(ptr as *mut u8, len as usize)
        }
    }
}
