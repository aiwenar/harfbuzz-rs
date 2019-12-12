//! Bindings to `hb-blob.h`.

#![allow(non_camel_case_types)]

use std::os::raw::*;

use crate::ffi::common::*;

/// Note re various memory-modes:
///
/// - In no case shall the HarfBuzz client modify memory
///   that is passed to HarfBuzz in a blob.  If there is
///   any such possibility, [`MODE_DUPLICATE`] should be used
///   such that HarfBuzz makes a copy immediately,
///
/// - Use [`MODE_READONLY`] otherwise, unless you really really
///   really know what you are doing,
///
/// - [`MODE_WRITABLE`] is appropriate if you really made a
///   copy of data solely for the purpose of passing to
///   HarfBuzz and doing that just once (no reuse!),
///
/// - If the font is `mmap()`ed, it's ok to use
///   [`READONLY_MAY_MAKE_WRITABLE`], however, using that mode
///   correctly is very tricky.  Use [`MODE_READONLY`] instead.
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub enum hb_memory_mode_t {
  DUPLICATE,
  READONLY,
  WRITABLE,
  READONLY_MAY_MAKE_WRITABLE
}

#[repr(C)]
pub struct hb_blob_t(cffi::Private);

extern "C" {
    /// Creates a new “blob” object wrapping `data`. The `mode` parameter
    /// is used to negotiate ownership and lifecycle of `data`.
    ///
    /// Return new blob, or the empty blob if something failed or if `length` is
    /// zero.
    ///
    /// ## Arguments
    ///
    /// - `data`: Pointer to blob data.
    /// - `length`: Length of `data` in bytes.
    /// - `mode`: Memory mode for `data`.
    /// - `user_data`: Data parameter to pass to `destroy`.
    /// - `destroy`: Callback to call when `data` is not needed anymore.
    pub fn hb_blob_create(
        data: *const c_char,
        length: c_uint,
        mode: hb_memory_mode_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    ) -> *mut hb_blob_t;

    /// Returns a blob that represents a range of bytes in `parent`. The new blob
    /// is always created with [`READONLY`], meaning that it will never modify
    /// data in the parent blob. The parent data is not expected to be modified,
    /// and will result in undefined behavior if it is.
    ///
    /// Makes `parent` immutable.
    ///
    /// Return new blob, or the empty blob if something failed or if `length`
    /// is zero or `offset` is beyond the end of `parent`'s data.
    ///
    /// ## Arguments
    ///
    /// - `parent`: Parent blob.
    /// - `offset`: Start offset of sub-blob within `parent`, in bytes.
    /// - `length`: Length of sub-blob.
    pub fn hb_blob_create_sub_blob(
        blob: *mut hb_blob_t,
        offset: c_uint,
        length: c_uint,
    ) -> *mut hb_blob_t;

    /// Makes a writable copy of @blob.
    pub fn hb_blob_copy_writable_or_fail(blob: *mut hb_blob_t) -> *mut hb_blob_t;

    /// Returns the singleton empty blob.
    pub fn hb_blob_get_empty() -> *mut hb_blob_t;

    pub fn hb_blob_reference(blob: *mut hb_blob_t) -> *mut hb_blob_t;

    pub fn hb_blob_destroy(blob: *mut hb_blob_t);

    pub fn hb_blob_set_user_data(
        blob: *mut hb_blob_t,
        key: *mut hb_user_data_key_t,
        data: *mut c_void,
        destroy: hb_destroy_func_t,
        replace: hb_bool_t,
    ) -> hb_bool_t;

    pub fn hb_blob_get_user_data(
        blob: *mut hb_blob_t,
        key: *mut hb_user_data_key_t,
    ) -> *mut c_void;

    pub fn hb_blob_make_immutable(blob: *mut hb_blob_t);

    pub fn hb_blob_is_immutable(blob: *mut hb_blob_t) -> hb_bool_t;

    pub fn hb_blob_get_length(blob: *mut hb_blob_t) -> c_uint;

    pub fn hb_blob_get_data(
        blob: *mut hb_blob_t,
        length: *mut c_uint,
    ) -> *const c_char;

    /// Tries to make blob data writable (possibly copying it) and return
    /// pointer to data.
    ///
    /// Fails if blob has been made immutable, or if memory allocation fails.
    ///
    /// Return writable blob data, or null if failed.
    pub fn hb_blob_get_data_writable(
        blob: *mut hb_blob_t,
        length: *mut c_uint,
    ) -> *mut c_char;

    pub fn hb_blob_create_from_file(file_name: *const c_char) -> *mut hb_blob_t;
}
