//! Bindings to `hb-set.h`.

use std::os::raw::*;

use crate::ffi::common::*;

#[repr(C)]
pub struct hb_set_t(cffi::Private);

pub const HB_SET_VALUE_INVALID: hb_codepoint_t = hb_codepoint_t::max_value();

extern "C" {
    pub fn hb_set_create() -> *mut hb_set_t;
    pub fn hb_set_get_empty() -> *mut hb_set_t;
    pub fn hb_set_reference(set: *mut hb_set_t) -> *mut hb_set_t;
    pub fn hb_set_destroy(set: *mut hb_set_t);
    pub fn hb_set_set_user_data(
        set: *mut hb_set_t,
        key: *mut hb_user_data_key_t,
        data: *mut c_void,
        destroy: hb_destroy_func_t,
        replace: hb_bool_t,
    ) -> hb_bool_t;
    pub fn hb_set_get_user_data(
        set: *mut hb_set_t,
        key: *mut hb_user_data_key_t,
    ) -> *mut c_void;
    pub fn hb_set_allocation_successful(set: *const hb_set_t) -> hb_bool_t;
    pub fn hb_set_clear(set: *mut hb_set_t);
    pub fn hb_set_is_empty(set: *const hb_set_t);
    pub fn hb_set_has(set: *const hb_set_t, codepoint: hb_codepoint_t);
    pub fn hb_set_add(set: *mut hb_set_t, codepoint: hb_codepoint_t);
    pub fn hb_set_add_range(
        set: *mut hb_set_t,
        first: hb_codepoint_t,
        last: hb_codepoint_t,
    );
    pub fn hb_set_del(set: *mut hb_set_t, codepoint: hb_codepoint_t);
    pub fn hb_set_del_range(
        set: *mut hb_set_t,
        first: hb_codepoint_t,
        last: hb_codepoint_t,
    );
    pub fn hb_set_is_equal(set: *const hb_set_t, other: *const hb_set_t);
    pub fn hb_set_is_subset(set: *const hb_set_t, other: *const hb_set_t);
    pub fn hb_set_set(set: *mut hb_set_t, other: *const hb_set_t);
    pub fn hb_set_union(set: *mut hb_set_t, other: *const hb_set_t);
    pub fn hb_set_intersect(set: *mut hb_set_t, other: *const hb_set_t);
    pub fn hb_set_subtract(set: *mut hb_set_t, other: *const hb_set_t);
    pub fn hb_set_symmetric_difference(set: *mut hb_set_t, other: *const hb_set_t);
    pub fn hb_set_get_population(set: *const hb_set_t) -> c_uint;
    pub fn hb_set_get_min(set: *const hb_set_t) -> hb_codepoint_t;
    pub fn hb_set_get_max(set: *const hb_set_t) -> hb_codepoint_t;
    pub fn hb_set_next(
        set: *const hb_set_t,
        codepoint: *mut hb_codepoint_t,
    ) -> hb_bool_t;
    pub fn hb_set_previous(
        set: *const hb_set_t,
        codepoint: *mut hb_codepoint_t,
    ) -> hb_bool_t;
    pub fn hb_set_next_range(
        set: *const hb_set_t,
        first: *mut hb_codepoint_t,
        last: *mut hb_codepoint_t,
    ) -> hb_bool_t;
    pub fn hb_set_previous_range(
        set: *const hb_set_t,
        first: *mut hb_codepoint_t,
        last: *mut hb_codepoint_t,
    ) -> hb_bool_t;
}
