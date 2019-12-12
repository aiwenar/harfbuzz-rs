//! Input and output buffers.
//!
//! This module contains raw bindings to `hb-buffer.h`.
//!
//! Buffers serve dual role in HarfBuzz; they hold the input characters that are
//! passed [`hb_shape()`], and after shaping they hold the output glyphs.

#![allow(non_camel_case_types)]

use std::{
    fmt,
    os::raw::*,
};

use crate::ffi::{
    common::*,
    font::*,
    unicode::*,
};

/// The `hb_glyph_info_t` is the structure that holds information about
/// the glyphs and their relation to input text.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct hb_glyph_info_t {
    /// Either a Unicode code point (before shaping) or a glyph index (after
    /// shaping).
    pub codepoint: hb_codepoint_t,
    pub mask: hb_mask_t,
    /// the index of the character in the original text that corresponds to this
    /// `hb_glyph_info_t` or whatever the client passes to [`hb_buffer_add()`].
    /// More than one `hb_glyph_info_t` can have the same `cluster` value,
    /// if they resulted from the same character (e.g. one to many glyph
    /// substitution), and when more than one character gets merged in the same
    /// glyph (e.g. many to one glyph substitution) the `hb_glyph_info_t` will
    /// have the smallest cluster value of them. By default some characters
    /// are merged into the same cluster (e.g. combining marks have the same
    /// cluster as their bases) even if they are separate glyphs,
    /// [`hb_buffer_set_cluster_level()`] allow selecting more fine-grained
    /// cluster handling.
    pub cluster: u32,
    _private1: hb_var_int_t,
    _private2: hb_var_int_t,
}

impl fmt::Debug for hb_glyph_info_t {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("hb_glyph_info_t")
            .field("codepoint", &self.codepoint)
            .field("mask", &self.mask)
            .field("cluster", &self.cluster)
            .finish()
    }
}

/// The [`hb_glyph_position_t`] is the structure that holds the positions of
/// the glyph in both horizontal and vertical directions. All positions
/// in `hb_glyph_position_t` are relative to the current point.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct hb_glyph_position_t {
    /// How much the line advances after drawing this glyph when setting text
    /// in horizontal direction.
    pub x_advance: hb_position_t,
    /// How much the line advances after drawing this glyph when setting text
    /// in vertical direction.
    pub y_advance: hb_position_t,
    /// How much the glyph moves on the X-axis before drawing it, this should
    /// not affect how much the line advances.
    pub x_offset: hb_position_t,
    /// How much the glyph moves on the Y-axis before drawing it, this should
    /// not affect how much the line advances.
    pub y_offset: hb_position_t,
    _private: hb_var_int_t,
}

impl fmt::Debug for hb_glyph_position_t {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("hb_glyph_position_t")
            .field("x_advance", &self.x_advance)
            .field("y_advance", &self.y_advance)
            .field("x_offset", &self.x_offset)
            .field("y_offset", &self.y_offset)
            .finish()
    }
}

/// The structure that holds various text properties of an [`hb_buffer_t`].
/// Can be set and retrieved using [`hb_buffer_set_segment_properties()`]
/// and [`hb_buffer_get_segment_properties()`], respectively.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct hb_segment_properties_t {
    /// The [`hb_direction_t`] of the buffer, see [`hb_buffer_set_direction()`].
    pub direction: hb_direction_t,
    /// The [`hb_script_t`] of the buffer, see [`hb_buffer_set_script()`].
    pub script: hb_script_t,
    /// The [`hb_language_t`] of the buffer, see [`hb_buffer_set_language()`].
    pub language: hb_language_t,
    _reserved1: *mut c_void,
    _reserved2: *mut c_void,
}

/*impl PartialEq for hb_segment_properties_t {
    fn eq(&self, other: &hb_segment_properties_t) -> bool {
        unsafe {
            hb_segment_properties_equal(self, other) != 0
        }
    }
}

impl Eq for hb_segment_properties_t {}

impl fmt::Debug for hb_segment_properties_t {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("hb_segment_properties_t")
            .field("direction", &self.direction)
            .field("script", &self.script)
            .field("language", &self.language)
            .finish()
    }
}*/

#[repr(C)]
pub struct hb_buffer_t(cffi::Private);

#[repr(C)]
pub enum hb_buffer_content_type_t {
    INVALID = 0,
    UNICODE,
    GLYPHS,
}

bitflags! {
    #[repr(C)]
    pub struct hb_buffer_flags_t: c_int {
        /// The default buffer flag.
        const BUFFER_FLAG_DEFAULT = 0x0000;
        /// Flag indicating that special handling of the beginning of text
        /// paragraph can be applied to this buffer. Should usually be set,
        /// unless you are passing to the buffer only part of the text without
        /// the full context.
        const BUFFER_FLAG_BOT = 0x0001;
        /// Flag indicating that special handling of the end of text paragraph
        /// can be applied to this buffer, similar to [`BUFFER_FLAG_BOT`].
        const BUFFER_FLAG_EOT = 0x0002;
        /// Flag indication that character with Default_Ignorable Unicode
        /// property should use the corresponding glyph from the font, instead
        /// of hiding them (done by replacing them with the space glyph and
        /// zeroing the advance width.)  This flag takes precedence over
        /// [`BUFFER_FLAG_REMOVE_DEFAULT_IGNORABLES`].
        const BUFFER_FLAG_PRESERVE_DEFAULT_IGNORABLES = 0x0004;
        /// Flag indication that character with Default_Ignorable Unicode
        /// property should use the corresponding glyph from the font, instead
        /// of hiding them (done by replacing them with the space glyph and
        /// zeroing the advance width.)  This flag takes precedence over
        /// [`HB_BUFFER_FLAG_REMOVE_DEFAULT_IGNORABLES`].
        const BUFFER_FLAG_REMOVE_DEFAULT_IGNORABLES = 0x0008;
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(C)]
pub enum hb_buffer_cluster_level_t {
    MONOTONE_GRAPHEMES = 0,
    MONOTONE_CHARACTERS = 1,
    CHARACTERS = 2,
}

/*impl Default for hb_buffer_cluster_level_t {
    fn default() -> Self {
        hb_buffer_cluster_level_t::MONOTONE_GRAPHEMES
    }
}*/

bitflags! {
    /// Flags that control what glyph information are serialized
    /// in [`hb_buffer_serialize_glyphs()`].
    #[repr(C)]
    pub struct hb_buffer_serialize_flags_t: c_int {
        /// Serialize glyph names, clusters and positions.
        const DEFAULT = 0x0000;
        /// Do not serialize glyph cluster.
        const NO_CLUSTERS = 0x0001;
        /// Do not serialize glyph position information.
        const NO_POSITIONS = 0x0002;
        /// Do no serialize glyph name.
        const NO_GLYPH_NAMES = 0x0004;
        /// Serialize glyph extents.
        const GLYPH_EXTENTS = 0x0008;
        /// Serialize glyph flags.
        const GLYPH_FLAGS = 0x0010;
        /// Do not serialize glyph advances, glyph offsets will reflect absolute
        /// glyph positions.
        const NO_ADVANCES = 0x0020;
    }
}

#[repr(C)]
pub enum hb_buffer_serialize_format_t {
  TEXT = 0x54455854,
  JSON = 0x4a534f4e,
}

bitflags! {
    #[repr(C)]
    pub struct hb_buffer_diff_flags_t: c_int { /*< flags >*/
        const EQUAL = 0x0000;
        /// Buffers with different content type cannot be meaningfully compared
        /// in any further detail.
        const CONTENT_TYPE_MISMATCH = 0x0001;
        /// For buffers with differing length, the per-glyph comparison is not
        /// attempted, though we do still scan reference for dottedcircle /
        /// .notdef glyphs.
        const LENGTH_MISMATCH = 0x0002;
        /// We want to know if dottedcircle / .notdef glyphs are present in the
        /// reference, as we may not care so much about other differences
        /// in this case.
        const NOTDEF_PRESENT = 0x0004;
        const DOTTED_CIRCLE_PRESENT = 0x0008;
        /// If the buffers have the same length, we compare them glyph-by-glyph
        /// and report which aspect(s) of the glyph info/position are different.
        const CODEPOINT_MISMATCH = 0x0010;
        const CLUSTER_MISMATCH = 0x0020;
        const GLYPH_FLAGS_MISMATCH = 0x0040;
        const POSITION_MISMATCH = 0x008;
    }
}

pub type hb_buffer_message_func_t = extern "C" fn(
    buffer: *mut hb_buffer_t,
    font: *mut hb_font_t,
    message: *const c_char,
    user_data: *mut c_void,
) -> hb_bool_t;

extern "C" {
    /// Checks the equality of two [`hb_segment_properties_t`]'s.
    pub fn hb_segment_properties_equal(
        a: *const hb_segment_properties_t,
        b: *const hb_segment_properties_t,
    ) -> hb_bool_t;

    /// Creates a hash representing `p`.
    pub fn hb_segment_properties_hash(p: *const hb_segment_properties_t) -> c_uint;

    /// Creates a new [`hb_buffer_t`] with all properties to defaults.
    ///
    /// A newly allocated [`hb_buffer_t`] with a reference count of 1.
    /// The initial reference count should be released with
    /// [`hb_buffer_destroy()`] when you are done using the [`hb_buffer_t`].
    /// This function never returns NULL. If memory cannot be allocated,
    /// a special [`hb_buffer_t`] object will be returned on which
    /// [`hb_buffer_allocation_successful()`] returns `false`.
    pub fn hb_buffer_create() -> *mut hb_buffer_t;

    pub fn hb_buffer_get_empty() -> *mut hb_buffer_t;

    /// Increases the reference count on `buffer` by one. This prevents `buffer`
    /// from being destroyed until a matching call to [`hb_buffer_destroy()`]
    /// is made.
    pub fn hb_buffer_reference(buffer: *mut hb_buffer_t) -> *mut hb_buffer_t;

    /// Deallocate the `buffer`.
    ///
    /// Decreases the reference count on `buffer` by one. If the result is zero,
    /// then `buffer` and all associated resources are freed.
    /// See [`hb_buffer_reference()`].
    pub fn hb_buffer_destroy(buffer: *mut hb_buffer_t);

    pub fn hb_buffer_set_user_data(
        buffer: *mut hb_buffer_t,
        key: *mut hb_user_data_key_t,
        data: *mut c_void,
        destroy: hb_destroy_func_t,
        replace: hb_bool_t,
    ) -> hb_bool_t;

    pub fn hb_buffer_get_user_data(
        buffer: *mut hb_buffer_t,
        key: *mut hb_user_data_key_t,
    ) -> *mut c_void;

    /// Sets the type of `buffer` contents, buffers are either empty, contain
    /// characters (before shaping) or glyphs (the result of shaping).
    pub fn hb_buffer_set_content_type(
        buffer: *mut hb_buffer_t,
        content_type: hb_buffer_content_type_t,
    );

    pub fn hb_buffer_get_content_type(buffer: *mut hb_buffer_t) -> hb_buffer_content_type_t;

    pub fn hb_buffer_set_unicode_funcs(
        buffer: *mut hb_buffer_t,
        functs: *mut hb_unicode_funcs_t,
    );

    pub fn hb_buffer_get_unicode_funcs(buffer: *mut hb_buffer_t) -> *mut hb_unicode_funcs_t;

    /// Set the text flow direction of the buffer. No shaping can happen without
    /// setting `buffer` direction, and it controls the visual direction for the
    /// output glyphs; for RTL direction the glyphs will be reversed. Many
    /// layout features depend on the proper setting of the direction,
    /// for example, reversing RTL text before shaping, then shaping with LTR
    /// direction is not the same as keeping the text in logical order
    /// and shaping with RTL direction.
    pub fn hb_buffer_set_direction(buffer: *mut hb_buffer_t, direction: hb_direction_t);

    pub fn hb_buffer_get_direction(buffer: &mut hb_buffer_t) -> hb_direction_t;

    /// Sets the script of `buffer` to `script`.
    ///
    /// Script is crucial for choosing the proper shaping behaviour for scripts
    /// that require it (e.g. Arabic) and the which OpenType features defined
    /// in the font to be applied.
    ///
    /// You can pass one of the predefined `hb_script_t` values, or use
    /// [`hb_script_from_string()`] or [`hb_script_from_iso15924_tag()`] to get
    /// the corresponding script from an ISO 15924 script tag.
    pub fn hb_buffer_set_script(buffer: *mut hb_buffer_t, script: hb_script_t);

    pub fn hb_buffer_get_script(buffer: *mut hb_buffer_t) -> hb_script_t;

    /// Sets the language of `buffer` to `language`.
    ///
    /// Languages are crucial for selecting which OpenType feature to apply
    /// to the buffer which can result in applying language-specific behaviour.
    /// Languages are orthogonal to the scripts, and though they are related,
    /// they are different concepts and should not be confused with each other.
    ///
    /// Use [`hb_language_from_string()`] to convert from ISO 639 language codes
    /// to [`hb_language_t`].
    pub fn hb_buffer_set_language(buffer: *mut hb_buffer_t, language: hb_language_t);

    pub fn hb_buffer_get_language(buffer: *mut hb_buffer_t) -> hb_language_t;

    /// Sets the segment properties of the buffer, a shortcut for calling
    /// [`hb_buffer_set_direction()`], [`hb_buffer_set_script()`] and
    /// [`hb_buffer_set_language()`] individually.
    pub fn hb_buffer_set_segment_properties(
        buffer: *mut hb_buffer_t,
        props: *const hb_segment_properties_t,
    );

    pub fn hb_buffer_get_segment_properties(
        buffer: &mut hb_buffer_t,
        props: *mut hb_segment_properties_t,
    );

    /// Sets unset buffer segment properties based on buffer Unicode contents.
    /// If buffer is not empty, it must have content type [`UNICODE`].
    ///
    /// If buffer script is not set (ie. is [`SCRIPT_INVALID`]), it will be set
    /// to the Unicode script of the first character in the buffer that has
    /// a script other than [`SCRIPT_COMMON`], [`SCRIPT_INHERITED`],
    /// and [`SCRIPT_UNKNOWN`].
    ///
    /// Next, if buffer direction is not set (ie. is [`DIRECTION_INVALID`]),
    /// it will be set to the natural horizontal direction of the buffer script
    /// as returned by [`hb_script_get_horizontal_direction()`].
    /// If [`hb_script_get_horizontal_direction()`] returns [`DIRECTION_INVALID`],
    /// then [`DIRECTION_LTR`] is used.
    ///
    /// Finally, if buffer language is not set (ie. is [`LANGUAGE_INVALID`]),
    /// it will be set to the process's default language as returned by
    /// [`hb_language_get_default()`]. This may change in the future by taking
    /// buffer script into consideration when choosing a language.
    pub fn hb_buffer_guess_segment_properties(buffer: *mut hb_buffer_t);

    /// Sets `buffer` flags to `flags`. See [`hb_buffer_flags_t`].
    pub fn hb_buffer_set_flags(buffer: *mut hb_buffer_t, flags: hb_buffer_flags_t);

    pub fn hb_buffer_get_flags(buffer: *mut hb_buffer_t) -> hb_buffer_flags_t;

    pub fn hb_buffer_set_cluster_level(
        buffer: *mut hb_buffer_t,
        cluster_level: hb_buffer_cluster_level_t,
    );

    pub fn hb_buffer_get_cluster_level(buffer: *mut hb_buffer_t) -> hb_buffer_cluster_level_t;

    /// Sets the [`hb_codepoint_t`] that replaces invalid entries for a given
    /// encoding when adding text to `buffer`.
    ///
    /// Default is [`HB_BUFFER_REPLACEMENT_CODEPOINT_DEFAULT`].
    pub fn hb_buffer_set_replacement_codepoint(buffer: *mut hb_buffer_t, replacement: hb_codepoint_t);

    pub fn hb_buffer_get_replacement_codepoint(buffer: *mut hb_buffer_t) -> hb_codepoint_t;

    /// Resets the buffer to its initial status, as if it was just newly created
    /// with [`hb_buffer_create()`].
    pub fn hb_buffer_reset(buffer: *mut hb_buffer_t);

    /// Similar to [`hb_buffer_reset()`], but does not clear the Unicode
    /// functions and the replacement code point.
    pub fn hb_buffer_clear_contents(buffer: *mut hb_buffer_t);

    /// Pre allocates memory for `buffer` to fit at least `size` number of items.
    pub fn hb_buffer_pre_allocate(buffer: *mut hb_buffer_t, size: c_uint);

    /// Check if allocating memory for the buffer succeeded.
    pub fn hb_buffer_allocation_successful(buffer: *mut hb_buffer_t) -> hb_bool_t;

    /// Reverses buffer contents.
    pub fn hb_buffer_reverse(buffer: *mut hb_buffer_t);

    /// Reverses buffer contents between start to end.
    pub fn hb_buffer_reverse_range(buffer: *mut hb_buffer_t, start: c_uint, end: c_uint);

    /// Reverses buffer clusters. That is, the buffer contents are reversed,
    /// then each cluster (consecutive items having the same cluster number) are
    /// reversed again.
    pub fn hb_buffer_reverse_clusters(buffer: *mut hb_buffer_t);

    /// Appends a character with the Unicode value of `codepoint` to `buffer`,
    /// and gives it the initial cluster value of `cluster`. Clusters can be any
    /// thing the client wants, they are usually used to refer to the index
    /// of the character in the input text stream and are output in
    /// `hb_glyph_info_t.cluster` field.
    ///
    /// This function does not check the validity of `codepoint`, it is up
    /// to the caller to ensure it is a valid Unicode code point.
    pub fn hb_buffer_add(buffer: *mut hb_buffer_t, codepoint: hb_codepoint_t, cluster: c_uint);

    /// See [`hb_buffer_add_codepoints()`].
    ///
    /// Replaces invalid UTF-8 characters with the `buffer` replacement code
    /// point, see [`hb_buffer_set_replacement_codepoint()`].
    pub fn hb_buffer_add_utf8(
        buffer: *mut hb_buffer_t,
        text: *const c_char,
        text_length: c_int,
        item_offset: c_uint,
        item_length: c_int,
    );

    /// See [`hb_buffer_add_codepoints()`].
    ///
    /// Replaces invalid UTF-16 characters with the `buffer` replacement code
    /// point, see [`hb_buffer_set_replacement_codepoint()`].
    pub fn hb_buffer_add_utf16(
        buffer: *mut hb_buffer_t,
        text: *const u16,
        text_length: c_int,
        item_offset: c_uint,
        item_length: c_int,
    );

    /// See [`hb_buffer_add_codepoints()`].
    ///
    /// Replaces invalid UTF-32 characters with the `buffer` replacement code
    /// point, see [`hb_buffer_set_replacement_codepoint()`].
    pub fn hb_buffer_add_utf32(
        buffer: *mut hb_buffer_t,
        text: *const u32,
        text_length: c_int,
        item_offset: c_uint,
        item_length: c_int,
    );

    /// Similar to [`hb_buffer_add_codepoints()`], but allows only access
    /// to first 256 Unicode code points that can fit in 8-bit strings.
    ///
    /// Note: Has nothing to do with non-Unicode Latin-1 encoding.
    pub fn hb_buffer_add_latin1(
        buffer: *mut hb_buffer_t,
        text: *const u8,
        text_length: c_int,
        item_offset: c_uint,
        item_length: c_int,
    );

    /// Appends characters from `text` array to `buffer`. The `item_offset`
    /// is the position of the first character from `text` that will
    /// be appended, and `item_length` is the number of character. When shaping
    /// part of a larger text (e.g. a run of text from a paragraph), instead
    /// of passing just the substring corresponding to the run, it is preferable
    /// to pass the whole paragraph and specify the run start and length
    /// as `item_offset` and `item_length`, respectively, to give HarfBuzz
    /// the full context to be able, for example, to do cross-run Arabic shaping
    /// or properly handle combining marks at stat of run.
    ///
    /// This function does not check the validity of `text`, it is up to
    /// the caller to ensure it contains a valid Unicode code points.
    ///
    /// ## Arguments
    ///
    /// - `text`: an array of Unicode code points to append.
    /// - `text_length`: the length of the @text, or `-1` if it is
    ///   NULL terminated.
    /// - `item_offset`: the offset of the first code point to add to
    ///   the `buffer`.
    /// - `item_length`: the number of code points to add to the `buffer`,
    ///   or `-1` for the end of `text` (assuming it is NULL terminated).
    pub fn hb_buffer_add_codepoints(
        buffer: *mut hb_buffer_t,
        text: *const hb_codepoint_t,
        text_length: c_int,
        item_offset: c_uint,
        item_length: c_int,
    );

    /// Append (part of) contents of another buffer to this buffer.
    pub fn hb_buffer_append(
        buffer: *mut hb_buffer_t,
        source: *mut hb_buffer_t,
        start: c_uint,
        end: c_uint,
    );

    /// Similar to [`hb_buffer_pre_allocate()`], but clears any new items added
    /// at the end.
    pub fn hb_buffer_set_length(buffer: *mut hb_buffer_t, length: c_uint) -> hb_bool_t;

    /// Returns the number of items in the buffer.
    pub fn hb_buffer_get_length(buffer: *mut hb_buffer_t) -> c_uint;

    /// Returns `buffer` glyph information array. Returned pointer  is valid as
    /// long as `buffer` contents are not modified.
    pub fn hb_buffer_get_glyph_infos(
        buffer: *mut hb_buffer_t,
        length: *mut c_uint,
    ) -> *mut hb_glyph_info_t;

    /// Returns `buffer` glyph position array. Returned pointer is valid as long
    /// as `buffer` contents are not modified.
    pub fn hb_buffer_get_glyph_positions(
        buffer: *mut hb_buffer_t,
        length: *mut c_uint,
    ) -> *mut hb_glyph_position_t;

    /// Reorders a glyph buffer to have canonical in-cluster glyph
    /// order / position. The resulting clusters should behave identical
    /// to pre-reordering clusters.
    ///
    /// Note: This has nothing to do with Unicode normalization.
    pub fn hb_buffer_normalize_glyphs(buffer: *mut hb_buffer_t);

    pub fn hb_buffer_serialize_format_from_string(
        str: *const c_char,
        len: c_int,
    ) -> hb_buffer_serialize_format_t;

    pub fn hb_buffer_serialize_format_to_string(format: hb_buffer_serialize_format_t) -> *const c_char;

    pub fn hb_buffer_serialize_list_formats() -> *const *const c_char;

    pub fn hb_buffer_serialize_glyphs(
        buffer: *mut hb_buffer_t,
        start: c_uint,
        end: c_uint,
        buf: *mut c_char,
        buf_size: c_uint,
        buf_consumed: *mut c_char,
        font: *mut hb_font_t,
        format: hb_buffer_serialize_format_t,
        flags: hb_buffer_serialize_flags_t,
    ) -> c_uint;

    pub fn hb_buffer_deserialize_glyphs(
        buffer: *mut hb_buffer_t,
        buf: *const c_char,
        buf_len: c_int,
        end_ptr: *const *const c_char,
        font: *mut hb_font_t,
        format: hb_buffer_serialize_format_t,
    ) -> hb_bool_t;

    /// If `dottedcircle_glyph` is `(hb_codepoint_t) -1` then
    /// [`DOTTED_CIRCLE_PRESENT`] and [`NOTDEF_PRESENT`] are never returned.
    /// This should be used by most callers if just comparing two buffers
    /// is needed.
    pub fn hb_buffer_diff(
        buffer: *mut hb_buffer_t,
        reference: *mut hb_buffer_t,
        dottedcircle_glyph: hb_codepoint_t,
        position_fuzz: c_uint,
    ) -> hb_buffer_diff_flags_t;

    pub fn hb_buffer_set_message_func(
        buffer: *mut hb_buffer_t,
        func: hb_buffer_message_func_t,
        user_data: *mut c_void,
        destroy: hb_destroy_func_t,
    );
}
